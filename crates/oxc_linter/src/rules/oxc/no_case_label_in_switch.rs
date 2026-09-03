use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoCaseLabelInSwitch;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows labels nested inside switch cases.
    NoCaseLabelInSwitch,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow labels inside switch cases.",
);

impl Rule for NoCaseLabelInSwitch {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::LabeledStatement(label) = node.kind() else {
            return;
        };
        for ancestor in ctx.nodes().ancestors(node.id()) {
            match ancestor.kind() {
                AstKind::SwitchCase(_) => {
                    ctx.diagnostic(
                        OxcDiagnostic::warn(format!(
                            "Remove this misleading `{}:` label.",
                            label.label.name
                        ))
                        .with_help("Use a case clause or a block instead of a label.")
                        .with_label(label.label.span),
                    );
                    return;
                }
                kind if kind.is_function_like() => return,
                _ => {}
            }
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoCaseLabelInSwitch::NAME,
        NoCaseLabelInSwitch::PLUGIN,
        vec!["switch (value) { case 1: break; }"],
        vec!["switch (value) { case 1: label: break; }"],
    )
    .test_and_snapshot();
}
