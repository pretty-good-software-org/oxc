use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct ConstructorForSideEffects;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows constructing an object solely for its side effects.
    ConstructorForSideEffects,
    oxc,
    suspicious,
    none,
    version = "0.0.1",
    short_description = "Disallow constructors used only for side effects.",
);

impl Rule for ConstructorForSideEffects {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::NewExpression(expression) = node.kind() else {
            return;
        };
        if matches!(ctx.nodes().parent_node(node.id()).kind(), AstKind::ExpressionStatement(_)) {
            ctx.diagnostic(
                OxcDiagnostic::warn("Do not use a constructor only for its side effects.")
                    .with_help(
                        "Assign the constructed value or call an explicit initialization method.",
                    )
                    .with_label(expression.span),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        ConstructorForSideEffects::NAME,
        ConstructorForSideEffects::PLUGIN,
        vec!["const value = new Thing();"],
        vec!["new Thing();"],
    )
    .test_and_snapshot();
}
