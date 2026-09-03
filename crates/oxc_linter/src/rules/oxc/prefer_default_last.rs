use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct PreferDefaultLast;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Requires the default switch case to appear after all explicit cases.
    PreferDefaultLast,
    oxc,
    style,
    none,
    version = "0.0.1",
    short_description = "Prefer the default switch case last.",
);

impl Rule for PreferDefaultLast {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::SwitchStatement(statement) = node.kind() else { return };
        let Some(default_index) = statement.cases.iter().position(|case| case.test.is_none())
        else {
            return;
        };
        if default_index + 1 < statement.cases.len() {
            ctx.diagnostic(
                OxcDiagnostic::warn("Move the default case to the end of the switch.")
                    .with_help("Keep explicit cases together before the fallback case.")
                    .with_label(statement.cases[default_index].span),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        PreferDefaultLast::NAME,
        PreferDefaultLast::PLUGIN,
        vec!["switch (value) { case 1: one(); default: fallback(); }"],
        vec!["switch (value) { default: fallback(); case 1: one(); }"],
    )
    .test_and_snapshot();
}
