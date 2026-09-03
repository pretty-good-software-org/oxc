use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoEmptyAfterReluctant;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Reports reluctant quantifiers that occur at the end of a regex alternative.
    NoEmptyAfterReluctant,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow ineffective reluctant quantifiers.",
);

impl Rule for NoEmptyAfterReluctant {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::RegExpLiteral(literal) = node.kind() else { return };
        let pattern = literal.regex.pattern.text.as_str();
        if pattern.ends_with("*?") || pattern.ends_with("+?") || pattern.ends_with("??") {
            ctx.diagnostic(
                OxcDiagnostic::warn(
                    "Fix this reluctant quantifier that will only ever match one repetition.",
                )
                .with_help(
                    "Remove the reluctant modifier or add a following expression that requires it.",
                )
                .with_label(literal.span),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoEmptyAfterReluctant::NAME,
        NoEmptyAfterReluctant::PLUGIN,
        vec!["/a*?b/"],
        vec!["/a*?/"],
    )
    .test_and_snapshot();
}
