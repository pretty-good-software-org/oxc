use oxc_ast::{
    AstKind,
    ast::{Expression, UnaryOperator},
};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct VoidUse;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Reports unnecessary uses of the `void` operator.
    VoidUse,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow unnecessary `void` expressions.",
);

impl Rule for VoidUse {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::UnaryExpression(expression) = node.kind() else { return };
        if expression.operator != UnaryOperator::Void || is_allowed(&expression.argument) {
            return;
        }
        ctx.diagnostic(
            OxcDiagnostic::warn("Remove this use of the `void` operator.")
                .with_help(
                    "Use the expression directly unless intentionally discarding its result.",
                )
                .with_label(expression.span),
        );
    }
}

fn is_allowed(expression: &Expression) -> bool {
    matches!(expression, Expression::NumericLiteral(number) if number.value == 0.0)
        || matches!(expression, Expression::CallExpression(call) if matches!(call.callee, Expression::ArrowFunctionExpression(_) | Expression::FunctionExpression(_)))
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(VoidUse::NAME, VoidUse::PLUGIN, vec!["void 0"], vec!["void value"])
        .test_and_snapshot();
}
