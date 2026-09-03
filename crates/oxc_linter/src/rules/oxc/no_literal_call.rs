use oxc_ast::{AstKind, ast::Expression};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::GetSpan;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoLiteralCall;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows using literal expressions as functions.
    NoLiteralCall,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow calling literal expressions.",
);

impl Rule for NoLiteralCall {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::CallExpression(call) = node.kind() else { return };
        if is_literal_expression(&call.callee) {
            ctx.diagnostic(
                OxcDiagnostic::warn("Do not use a literal expression as a function.")
                    .with_help("Call a named function instead of a literal value.")
                    .with_label(call.callee.span()),
            );
        }
    }
}

fn is_literal_expression(expression: &Expression) -> bool {
    matches!(
        expression,
        Expression::ArrayExpression(_)
            | Expression::ClassExpression(_)
            | Expression::ObjectExpression(_)
            | Expression::StringLiteral(_)
            | Expression::NumericLiteral(_)
            | Expression::BooleanLiteral(_)
            | Expression::NullLiteral(_)
            | Expression::TemplateLiteral(_)
    )
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoLiteralCall::NAME,
        NoLiteralCall::PLUGIN,
        vec!["fn()"],
        vec!["'value'()", "[value]()"],
    )
    .test_and_snapshot();
}
