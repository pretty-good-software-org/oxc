use oxc_ast::{AstKind, ast::Statement};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoRedundantJump;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Reports an unlabelled, argument-less return at the end of a non-empty function.
    NoRedundantJump,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow redundant terminal returns.",
);

impl Rule for NoRedundantJump {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::FunctionBody(body) = node.kind() else { return };
        if body.statements.len() < 2 {
            return;
        }
        let Some(Statement::ReturnStatement(return_statement)) = body.statements.last() else {
            return;
        };
        if return_statement.argument.is_none() {
            ctx.diagnostic(
                OxcDiagnostic::warn("Remove this redundant jump.")
                    .with_help("Allow the function to reach its end naturally.")
                    .with_label(return_statement.span),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoRedundantJump::NAME,
        NoRedundantJump::PLUGIN,
        vec!["function run() { work(); return value; }"],
        vec!["function run() { work(); return; }"],
    )
    .test_and_snapshot();
}
