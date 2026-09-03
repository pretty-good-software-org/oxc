use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::GetSpan;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
struct Config {
    threshold: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self { threshold: 4 }
    }
}

#[derive(Debug, Clone, Default)]
pub struct NoNestedFunctions(Box<Config>);

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows functions nested deeper than the configured threshold.
    NoNestedFunctions,
    oxc,
    suspicious,
    config = Config,
    version = "0.0.1",
    short_description = "Limit nested function depth.",
);

impl Rule for NoNestedFunctions {
    fn from_configuration(value: serde_json::Value) -> Result<Self, serde_json::error::Error> {
        if value.is_null() {
            return Ok(Self::default());
        }
        Ok(Self(Box::new(serde_json::from_value(value)?)))
    }

    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        if !node.kind().is_function_like() {
            return;
        }
        let depth = ctx
            .nodes()
            .ancestors(node.id())
            .filter(|ancestor| ancestor.kind().is_function_like())
            .count();
        if depth >= self.0.threshold {
            ctx.diagnostic(
                OxcDiagnostic::warn("Reduce the nesting depth of this function.")
                    .with_help("Extract the nested function or simplify the control flow.")
                    .with_label(node.kind().span()),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoNestedFunctions::NAME,
        NoNestedFunctions::PLUGIN,
        vec!["function outer() { function inner() {} }"],
        vec!["function a() { function b() { function c() { function d() { function e() {} } } } }"],
    )
    .test_and_snapshot();
}
