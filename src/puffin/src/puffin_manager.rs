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

pub mod cache;
pub mod file_accessor;
pub mod fs_puffin_manager;
pub mod stager;

#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use async_trait::async_trait;
use common_base::range_read::RangeReader;
use futures::AsyncRead;

use crate::blob_metadata::{BlobMetadata, CompressionCodec};
use crate::error::Result;
use crate::file_metadata::FileMetadata;

/// The `PuffinManager` trait provides a unified interface for creating `PuffinReader` and `PuffinWriter`.
#[async_trait]
pub trait PuffinManager {
    type Reader: PuffinReader;
    type Writer: PuffinWriter;
    type FileHandle: ToString + Clone + Send + Sync;

    /// Creates a `PuffinReader` for the specified `handle`.
    async fn reader(&self, handle: &Self::FileHandle) -> Result<Self::Reader>;

    /// Creates a `PuffinWriter` for the specified `handle`.
    async fn writer(&self, handle: &Self::FileHandle) -> Result<Self::Writer>;
}

/// The `PuffinWriter` trait provides methods for writing blobs and directories to a Puffin file.
#[async_trait]
pub trait PuffinWriter {
    /// Writes a blob associated with the specified `key` to the Puffin file.
    /// Returns the number of bytes written.
    async fn put_blob<R>(
        &mut self,
        key: &str,
        raw_data: R,
        options: PutOptions,
        properties: HashMap<String, String>,
    ) -> Result<u64>
    where
        R: AsyncRead + Send;

    /// Writes a directory associated with the specified `key` to the Puffin file.
    /// Returns the number of bytes written.
    ///
    /// The specified `dir` should be accessible from the filesystem.
    async fn put_dir(&mut self, key: &str, dir: PathBuf, options: PutOptions) -> Result<u64>;

    /// Sets whether the footer should be LZ4 compressed.
    fn set_footer_lz4_compressed(&mut self, lz4_compressed: bool);

    /// Finalizes the Puffin file after writing.
    async fn finish(self) -> Result<u64>;
}

/// Options available for `put_blob` and `put_dir` methods.
#[derive(Debug, Clone, Default)]
pub struct PutOptions {
    /// The compression codec to use for blob data.
    pub compression: Option<CompressionCodec>,
}

/// The `PuffinReader` trait provides methods for reading blobs and directories from a Puffin file.
#[async_trait]
pub trait PuffinReader {
    type Blob: BlobGuard;
    type Dir: DirGuard;

    fn with_file_size_hint(self, file_size_hint: Option<u64>) -> Self;

    /// Returns the metadata of the Puffin file.
    async fn metadata(&self) -> Result<Arc<FileMetadata>>;

    /// Reads a blob from the Puffin file.
    ///
    /// The returned `BlobWithMetadata` is used to access the blob data and its metadata.
    /// Users should hold the `BlobWithMetadata` until they are done with the blob data.
    async fn blob(&self, key: &str) -> Result<BlobWithMetadata<Self::Blob>>;

    /// Reads a directory from the Puffin file.
    ///
    /// The returned `DirGuard` is used to access the directory in the filesystem.
    /// The caller is responsible for holding the `DirGuard` until they are done with the directory.
    async fn dir(&self, key: &str) -> Result<Self::Dir>;
}

/// `BlobGuard` is provided by the `PuffinReader` to access the blob data.
/// Users should hold the `BlobGuard` until they are done with the blob data.
#[async_trait]
#[auto_impl::auto_impl(Arc)]
pub trait BlobGuard {
    type Reader: RangeReader;
    async fn reader(&self) -> Result<Self::Reader>;
}

/// `BlobWithMetadata` provides access to the blob data and its metadata.
pub struct BlobWithMetadata<B> {
    blob: B,
    metadata: BlobMetadata,
}

impl<B: BlobGuard> BlobWithMetadata<B> {
    /// Creates a new `BlobWithMetadata` instance.
    pub fn new(blob: B, metadata: BlobMetadata) -> Self {
        Self { blob, metadata }
    }

    /// Returns the reader for the blob data.
    pub async fn reader(&self) -> Result<B::Reader> {
        self.blob.reader().await
    }

    /// Returns the metadata of the blob.
    pub fn metadata(&self) -> &BlobMetadata {
        &self.metadata
    }
}

/// `DirGuard` is provided by the `PuffinReader` to access the directory in the filesystem.
/// Users should hold the `DirGuard` until they are done with the directory.
#[auto_impl::auto_impl(Arc)]
pub trait DirGuard {
    fn path(&self) -> &PathBuf;
}
