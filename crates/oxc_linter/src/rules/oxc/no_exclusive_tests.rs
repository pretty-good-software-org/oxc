use oxc_ast::{AstKind, ast::Expression};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::GetSpan;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoExclusiveTests;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows exclusive test modifiers such as `it.only` and `describe.only`.
    NoExclusiveTests,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow exclusive test blocks.",
);

impl Rule for NoExclusiveTests {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::CallExpression(call) = node.kind() else { return };
        let Some(member) = call.callee.as_member_expression() else { return };
        if member.static_property_name() != Some("only") || !is_test_function(member.object()) {
            return;
        }
        ctx.diagnostic(
            OxcDiagnostic::warn("Remove the exclusive test modifier.")
                .with_help("Use the ordinary test function so the complete suite runs.")
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
        NoExclusiveTests::NAME,
        NoExclusiveTests::PLUGIN,
        vec!["it('works', testFn)"],
        vec!["it.only('works', testFn)", "describe.only('suite', testFn)"],
    )
    .test_and_snapshot();
}
