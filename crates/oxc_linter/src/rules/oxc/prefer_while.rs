use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct PreferWhile;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Reports `for` loops without initialization or update clauses.
    PreferWhile,
    oxc,
    style,
    none,
    version = "0.0.1",
    short_description = "Prefer while loops for condition-only iteration.",
);

impl Rule for PreferWhile {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::ForStatement(statement) = node.kind() else { return };
        if statement.init.is_none() && statement.update.is_none() && statement.test.is_some() {
            ctx.diagnostic(
                OxcDiagnostic::warn("Replace this `for` loop with a `while` loop.")
                    .with_help(
                        "Use `while (condition)` when the loop has no initializer or update.",
                    )
                    .with_label(statement.span),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        PreferWhile::NAME,
        PreferWhile::PLUGIN,
        vec!["for (let i = 0; i < 3; i++) run(i);"],
        vec!["for (; ready(); ) run();"],
    )
    .test_and_snapshot();
}
