use oxc_ast::{
    AstKind,
    ast::{Expression, Statement},
};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct PreferSingleBooleanReturn;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Reports conditionals whose branches only return boolean literals.
    PreferSingleBooleanReturn,
    oxc,
    style,
    none,
    version = "0.0.1",
    short_description = "Prefer a single boolean return statement.",
);

impl Rule for PreferSingleBooleanReturn {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::IfStatement(statement) = node.kind() else { return };
        if matches!(ctx.nodes().parent_kind(node.id()), AstKind::IfStatement(_)) {
            return;
        }
        let Some(alternate) = &statement.alternate else { return };
        if returns_boolean(&statement.consequent) && returns_boolean(alternate) {
            ctx.diagnostic(
                OxcDiagnostic::warn(
                    "Replace this if-then-else flow with a single return statement.",
                )
                .with_help("Return the condition directly, negating it when necessary.")
                .with_label(statement.span),
            );
        }
    }
}

fn returns_boolean(statement: &Statement) -> bool {
    match statement {
        Statement::ReturnStatement(return_statement) => {
            is_boolean(return_statement.argument.as_ref())
        }
        Statement::BlockStatement(block) if block.body.len() == 1 => {
            returns_boolean(&block.body[0])
        }
        _ => false,
    }
}

fn is_boolean(expression: Option<&Expression>) -> bool {
    matches!(expression, Some(Expression::BooleanLiteral(_)))
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        PreferSingleBooleanReturn::NAME,
        PreferSingleBooleanReturn::PLUGIN,
        vec!["if (condition) return value; else return other;"],
        vec!["if (condition) return true; else return false;"],
    )
    .test_and_snapshot();
}
