(function() {
    var type_impls = Object.fromEntries([["common_telemetry",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-TraceContextPropagator\" class=\"impl\"><a href=\"#impl-Clone-for-TraceContextPropagator\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for TraceContextPropagator</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; TraceContextPropagator</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/clone.rs.html#174\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: &amp;Self)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","common_telemetry::tracing_context::Propagator"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-TraceContextPropagator\" class=\"impl\"><a href=\"#impl-Debug-for-TraceContextPropagator\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for TraceContextPropagator</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","common_telemetry::tracing_context::Propagator"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Default-for-TraceContextPropagator\" class=\"impl\"><a href=\"#impl-Default-for-TraceContextPropagator\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for TraceContextPropagator</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.default\" class=\"method trait-impl\"><a href=\"#method.default\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default\" class=\"fn\">default</a>() -&gt; TraceContextPropagator</h4></section></summary><div class='docblock'>Returns the “default value” for a type. <a href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default\">Read more</a></div></details></div></details>","Default","common_telemetry::tracing_context::Propagator"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-TextMapPropagator-for-TraceContextPropagator\" class=\"impl\"><a href=\"#impl-TextMapPropagator-for-TraceContextPropagator\" class=\"anchor\">§</a><h3 class=\"code-header\">impl TextMapPropagator for TraceContextPropagator</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.inject_context\" class=\"method trait-impl\"><a href=\"#method.inject_context\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">inject_context</a>(&amp;self, cx: &amp;Context, injector: &amp;mut dyn Injector)</h4></section></summary><div class=\"docblock\"><p>Properly encodes the values of the <code>SpanContext</code> and injects them\ninto the <code>Injector</code>.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.extract_with_context\" class=\"method trait-impl\"><a href=\"#method.extract_with_context\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">extract_with_context</a>(\n    &amp;self,\n    cx: &amp;Context,\n    extractor: &amp;dyn Extractor,\n) -&gt; Context</h4></section></summary><div class=\"docblock\"><p>Retrieves encoded <code>SpanContext</code>s using the <code>Extractor</code>. It decodes\nthe <code>SpanContext</code> and returns it. If no <code>SpanContext</code> was retrieved\nOR if the retrieved SpanContext is invalid then an empty <code>SpanContext</code>\nis returned.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.fields\" class=\"method trait-impl\"><a href=\"#method.fields\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">fields</a>(&amp;self) -&gt; FieldIter&lt;'_&gt;</h4></section></summary><div class='docblock'>Returns iter of fields used by [<code>TextMapPropagator</code>]</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.inject\" class=\"method trait-impl\"><a href=\"#method.inject\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">inject</a>(&amp;self, injector: &amp;mut dyn Injector)</h4></section></summary><div class='docblock'>Properly encodes the values of the current <a href=\"crate::Context\"><code>Context</code></a> and injects them into\nthe <a href=\"crate::propagation::Injector\"><code>Injector</code></a>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.extract\" class=\"method trait-impl\"><a href=\"#method.extract\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">extract</a>(&amp;self, extractor: &amp;dyn Extractor) -&gt; Context</h4></section></summary><div class='docblock'>Retrieves encoded data using the provided [<code>Extractor</code>]. If no data for this\nformat was retrieved OR if the retrieved data is invalid, then the current\n<a href=\"crate::Context\"><code>Context</code></a> is returned.</div></details></div></details>","TextMapPropagator","common_telemetry::tracing_context::Propagator"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-TraceContextPropagator\" class=\"impl\"><a href=\"#impl-TraceContextPropagator\" class=\"anchor\">§</a><h3 class=\"code-header\">impl TraceContextPropagator</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">new</a>() -&gt; TraceContextPropagator</h4></section></summary><div class=\"docblock\"><p>Create a new <code>TraceContextPropagator</code>.</p>\n</div></details></div></details>",0,"common_telemetry::tracing_context::Propagator"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[8291]}