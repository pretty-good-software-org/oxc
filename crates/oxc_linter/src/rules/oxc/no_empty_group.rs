use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoEmptyGroup;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows empty groups in regular expressions.
    NoEmptyGroup,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow empty regular-expression groups.",
);

impl Rule for NoEmptyGroup {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::RegExpLiteral(literal) = node.kind() else {
            return;
        };
        if literal.regex.pattern.text.contains("()") {
            ctx.diagnostic(
                OxcDiagnostic::warn("Remove the empty regular-expression group.")
                    .with_help("Replace the empty group with the intended pattern.")
                    .with_label(literal.span),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoEmptyGroup::NAME,
        NoEmptyGroup::PLUGIN,
        vec!["const pattern = /value/;"],
        vec!["const pattern = /value()/;"],
    )
    .test_and_snapshot();
}
