// Copyright 2023 Greptime Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![no_main]

use std::collections::HashMap;
use std::sync::Arc;

use arbitrary::{Arbitrary, Unstructured};
use common_telemetry::info;
use libfuzzer_sys::fuzz_target;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;
use snafu::ResultExt;
use sqlx::{MySql, Pool};
use strum::{EnumIter, IntoEnumIterator};
use tests_fuzz::context::{TableContext, TableContextRef};
use tests_fuzz::error::{self, Result};
use tests_fuzz::fake::{
    merge_two_word_map_fn, random_capitalize_map, uppercase_and_keyword_backtick_map,
    MappedGenerator, WordGenerator,
};
use tests_fuzz::generator::alter_expr::{
    AlterExprAddColumnGeneratorBuilder, AlterExprDropColumnGeneratorBuilder,
    AlterExprModifyDataTypeGeneratorBuilder, AlterExprRenameGeneratorBuilder,
    AlterExprSetTableOptionsGeneratorBuilder, AlterExprUnsetTableOptionsGeneratorBuilder,
};
use tests_fuzz::generator::create_expr::CreateTableExprGeneratorBuilder;
use tests_fuzz::generator::Generator;
use tests_fuzz::ir::{
    droppable_columns, modifiable_columns, AlterTableExpr, AlterTableOption, CreateTableExpr,
};
use tests_fuzz::translator::mysql::alter_expr::AlterTableExprTranslator;
use tests_fuzz::translator::mysql::create_expr::CreateTableExprTranslator;
use tests_fuzz::translator::DslTranslator;
use tests_fuzz::utils::{
    get_gt_fuzz_input_max_columns, init_greptime_connections_via_env, Connections,
};
use tests_fuzz::validator;
struct FuzzContext {
    greptime: Pool<MySql>,
}

impl FuzzContext {
    async fn close(self) {
        self.greptime.close().await;
    }
}

#[derive(Clone, Debug)]
struct FuzzInput {
    seed: u64,
    actions: usize,
}

#[derive(Debug, EnumIter)]
enum AlterTableKind {
    AddColumn,
    DropColumn,
    RenameTable,
    ModifyDataType,
    SetTableOptions,
    UnsetTableOptions,
}

fn generate_create_table_expr<R: Rng + 'static>(rng: &mut R) -> Result<CreateTableExpr> {
    let max_columns = get_gt_fuzz_input_max_columns();
    let columns = rng.random_range(2..max_columns);
    let mut with_clause = HashMap::new();
    if rng.random_bool(0.5) {
        with_clause.insert("append_mode".to_string(), "true".to_string());
    }
    let create_table_generator = CreateTableExprGeneratorBuilder::default()
        .name_generator(Box::new(MappedGenerator::new(
            WordGenerator,
            merge_two_word_map_fn(random_capitalize_map, uppercase_and_keyword_backtick_map),
        )))
        .columns(columns)
        .engine("mito")
        .with_clause(with_clause)
        .build()
        .unwrap();
    create_table_generator.generate(rng)
}

fn generate_alter_table_expr<R: Rng + 'static>(
    table_ctx: TableContextRef,
    rng: &mut R,
) -> Result<AlterTableExpr> {
    let kinds = AlterTableKind::iter().collect::<Vec<_>>();
    match kinds[rng.random_range(0..kinds.len())] {
        AlterTableKind::DropColumn if !droppable_columns(&table_ctx.columns).is_empty() => {
            AlterExprDropColumnGeneratorBuilder::default()
                .table_ctx(table_ctx)
                .build()
                .unwrap()
                .generate(rng)
        }
        AlterTableKind::ModifyDataType if !modifiable_columns(&table_ctx.columns).is_empty() => {
            AlterExprModifyDataTypeGeneratorBuilder::default()
                .table_ctx(table_ctx)
                .build()
                .unwrap()
                .generate(rng)
        }
        AlterTableKind::RenameTable => AlterExprRenameGeneratorBuilder::default()
            .table_ctx(table_ctx)
            .name_generator(Box::new(MappedGenerator::new(
                WordGenerator,
                merge_two_word_map_fn(random_capitalize_map, uppercase_and_keyword_backtick_map),
            )))
            .build()
            .unwrap()
            .generate(rng),
        AlterTableKind::SetTableOptions => {
            let expr_generator = AlterExprSetTableOptionsGeneratorBuilder::default()
                .table_ctx(table_ctx)
                .build()
                .unwrap();
            expr_generator.generate(rng)
        }
        AlterTableKind::UnsetTableOptions => {
            let expr_generator = AlterExprUnsetTableOptionsGeneratorBuilder::default()
                .table_ctx(table_ctx)
                .build()
                .unwrap();
            expr_generator.generate(rng)
        }
        _ => {
            let location = rng.random_bool(0.5);
            let expr_generator = AlterExprAddColumnGeneratorBuilder::default()
                .table_ctx(table_ctx)
                .location(location)
                .build()
                .unwrap();
            expr_generator.generate(rng)
        }
    }
}

impl Arbitrary<'_> for FuzzInput {
    fn arbitrary(u: &mut Unstructured<'_>) -> arbitrary::Result<Self> {
        let seed = u.int_in_range(u64::MIN..=u64::MAX)?;
        let mut rng = ChaChaRng::seed_from_u64(seed);
        let actions = rng.random_range(1..256);

        Ok(FuzzInput { seed, actions })
    }
}

async fn execute_alter_table(ctx: FuzzContext, input: FuzzInput) -> Result<()> {
    info!("input: {input:?}");
    let mut rng = ChaChaRng::seed_from_u64(input.seed);

    // Create table
    let expr = generate_create_table_expr(&mut rng).unwrap();
    let translator = CreateTableExprTranslator;
    let sql = translator.translate(&expr)?;
    let result = sqlx::query(&sql)
        .execute(&ctx.greptime)
        .await
        .context(error::ExecuteQuerySnafu { sql: &sql })?;
    info!("Create table: {sql}, result: {result:?}");

    // Alter table actions
    let mut table_ctx = Arc::new(TableContext::from(&expr));
    for _ in 0..input.actions {
        let expr = generate_alter_table_expr(table_ctx.clone(), &mut rng).unwrap();
        let translator = AlterTableExprTranslator;
        let sql = translator.translate(&expr)?;
        let result = sqlx::query(&sql)
            .execute(&ctx.greptime)
            .await
            .context(error::ExecuteQuerySnafu { sql: &sql })?;
        info!("Alter table: {sql}, result: {result:?}");
        // Applies changes
        table_ctx = Arc::new(Arc::unwrap_or_clone(table_ctx).alter(expr).unwrap());

        // Validates columns
        let mut column_entries = validator::column::fetch_columns(
            &ctx.greptime,
            "public".into(),
            table_ctx.name.clone(),
        )
        .await?;
        column_entries.sort_by(|a, b| a.column_name.cmp(&b.column_name));
        let mut columns = table_ctx.columns.clone();
        columns.sort_by(|a, b| a.name.value.cmp(&b.name.value));
        validator::column::assert_eq(&column_entries, &columns)?;

        // Validates table options
        let sql = format!("SHOW CREATE TABLE {}", table_ctx.name);
        let mut table_options = validator::table::fetch_table_options(&ctx.greptime, &sql).await?;
        table_options.sort_by(|a, b| a.key().cmp(b.key()));
        let mut expected_table_options = table_ctx.table_options.clone();
        expected_table_options.sort_by(|a, b| a.key().cmp(b.key()));
        table_options
            .iter()
            .zip(expected_table_options.iter())
            .for_each(|(a, b)| {
                if let (
                    AlterTableOption::TwcsMaxOutputFileSize(a),
                    AlterTableOption::TwcsMaxOutputFileSize(b),
                ) = (a, b)
                {
                    // to_string loses precision for ReadableSize, so the size in generated SQL is not the same as the size in the table context,
                    // but the string representation should be the same. For example:
                    //                                     to_string()                from_str()
                    // ReadableSize(13001360408898724524) ------------> "11547.5PiB" -----------> ReadableSize(13001329174265200640)
                    assert_eq!(a.to_string(), b.to_string());
                } else {
                    assert_eq!(a, b);
                }
            });
    }

    // Cleans up
    let table_name = table_ctx.name.clone();
    let sql = format!("DROP TABLE {}", table_name);
    let result = sqlx::query(&sql)
        .execute(&ctx.greptime)
        .await
        .context(error::ExecuteQuerySnafu { sql })?;
    info!("Drop table: {}, result: {result:?}", table_name);
    ctx.close().await;

    Ok(())
}

fuzz_target!(|input: FuzzInput| {
    common_telemetry::init_default_ut_logging();
    common_runtime::block_on_global(async {
        let Connections { mysql } = init_greptime_connections_via_env().await;
        let ctx = FuzzContext {
            greptime: mysql.expect("mysql connection init must be succeed"),
        };
        execute_alter_table(ctx, input)
            .await
            .unwrap_or_else(|err| panic!("fuzz test must be succeed: {err:?}"));
    })
});
