use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct SingleCharacterAlternation;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Reports regular expressions that use alternation between single characters.
    SingleCharacterAlternation,
    oxc,
    suspicious,
    none,
    version = "0.0.1",
    short_description = "Disallow single-character regular-expression alternation.",
);

impl Rule for SingleCharacterAlternation {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::RegExpLiteral(literal) = node.kind() else { return };
        let pattern = literal.regex.pattern.text.as_str();
        if pattern.split('|').count() > 1
            && pattern.split('|').all(|part| part.chars().count() == 1)
        {
            ctx.diagnostic(
                OxcDiagnostic::warn(
                    "Replace this single-character alternation with a character class.",
                )
                .with_help("Use a character class such as `[ab]`.")
                .with_label(literal.span),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        SingleCharacterAlternation::NAME,
        SingleCharacterAlternation::PLUGIN,
        vec!["/ab|cd/"],
        vec!["/a|b/"],
    )
    .test_and_snapshot();
}
