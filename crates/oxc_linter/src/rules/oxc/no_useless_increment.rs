use oxc_ast::{
    AstKind,
    ast::{Expression, UpdateOperator},
};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoUselessIncrement;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Reports postfix increments or decrements whose result is immediately discarded.
    NoUselessIncrement,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow useless postfix increments and decrements.",
);

impl Rule for NoUselessIncrement {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::ReturnStatement(statement) = node.kind() else {
            return;
        };
        let Some(Expression::UpdateExpression(update)) = statement.argument.as_ref() else {
            return;
        };
        if !update.prefix {
            report_postfix(update.operator, update.span, ctx);
        }
    }
}

fn report_postfix(operator: UpdateOperator, span: oxc_span::Span, ctx: &LintContext) {
    let name = if operator == UpdateOperator::Increment { "increment" } else { "decrement" };
    ctx.diagnostic(
        OxcDiagnostic::warn(format!("Remove this useless {name} or use its result."))
            .with_help(
                "Use the prefix form when the updated value is needed, or remove the update.",
            )
            .with_label(span),
    );
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoUselessIncrement::NAME,
        NoUselessIncrement::PLUGIN,
        vec!["return ++value;", "value = value + 1;"],
        vec!["return value++;"],
    )
    .test_and_snapshot();
}
