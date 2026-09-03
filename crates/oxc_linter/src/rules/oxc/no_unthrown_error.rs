use oxc_ast::{AstKind, ast::Expression};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoUnthrownError;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Reports error objects created as standalone expression statements.
    NoUnthrownError,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Require created errors to be thrown or used.",
);

impl Rule for NoUnthrownError {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::NewExpression(expression) = node.kind() else { return };
        if !matches!(ctx.nodes().parent_kind(node.id()), AstKind::ExpressionStatement(_)) {
            return;
        }
        let Expression::Identifier(identifier) = &expression.callee else { return };
        if !(identifier.name.ends_with("Error") || identifier.name.ends_with("Exception")) {
            return;
        }
        ctx.diagnostic(
            OxcDiagnostic::warn("Throw this error or remove this useless statement.")
                .with_help("Prefix the error construction with `throw` or use the error value.")
                .with_label(expression.span),
        );
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoUnthrownError::NAME,
        NoUnthrownError::PLUGIN,
        vec!["throw new Error('failure');", "const error = new Error('failure');"],
        vec!["new Error('failure');"],
    )
    .test_and_snapshot();
}
