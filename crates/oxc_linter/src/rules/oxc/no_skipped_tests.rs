use oxc_ast::{AstKind, ast::Expression};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::GetSpan;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoSkippedTests;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows skipped test blocks such as `it.skip` and `describe.skip`.
    NoSkippedTests,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow skipped tests.",
);

impl Rule for NoSkippedTests {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::CallExpression(call) = node.kind() else { return };
        let Some(member) = call.callee.as_member_expression() else { return };
        if member.static_property_name() != Some("skip") || !is_test_function(member.object()) {
            return;
        }
        ctx.diagnostic(
            OxcDiagnostic::warn("Remove the skipped test modifier.")
                .with_help("Restore the test or remove it from the test suite.")
                .with_label(member.as_property_key().span()),
        );
    }
}

fn is_test_function(expression: &Expression) -> bool {
    matches!(expression, Expression::Identifier(identifier) if matches!(identifier.name.as_str(), "it" | "test" | "describe" | "suite" | "specify"))
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoSkippedTests::NAME,
        NoSkippedTests::PLUGIN,
        vec!["it('works', testFn)"],
        vec!["it.skip('works', testFn)", "describe.skip('suite', testFn)"],
    )
    .test_and_snapshot();
}
