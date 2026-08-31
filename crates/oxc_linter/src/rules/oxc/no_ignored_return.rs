use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoIgnoredReturn;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Reports standalone calls to methods whose return value is normally the useful result.
    NoIgnoredReturn,
    oxc,
    suspicious,
    none,
    version = "0.0.1",
    short_description = "Disallow ignored results from pure collection methods.",
);

const PURE_METHODS: &[&str] = &[
    "concat",
    "filter",
    "find",
    "findIndex",
    "includes",
    "indexOf",
    "join",
    "lastIndexOf",
    "map",
    "reduce",
    "reduceRight",
    "slice",
    "some",
    "every",
];

impl Rule for NoIgnoredReturn {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::CallExpression(call) = node.kind() else { return };
        if !matches!(ctx.nodes().parent_kind(node.id()), AstKind::ExpressionStatement(_)) {
            return;
        }
        let Some(member) = call.callee.as_member_expression() else { return };
        let Some(method) = member.static_property_name() else { return };
        if PURE_METHODS.contains(&method) {
            ctx.diagnostic(
                OxcDiagnostic::warn(format!(
                    "Use the result returned by `{method}` or remove this call."
                ))
                .with_help("Assign or return the result instead of discarding it.")
                .with_label(call.span),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoIgnoredReturn::NAME,
        NoIgnoredReturn::PLUGIN,
        vec!["const result = values.map(transform);"],
        vec!["values.map(transform);"],
    )
    .test_and_snapshot();
}
