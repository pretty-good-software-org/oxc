use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoEmptyAlternatives;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows empty alternatives in regular expressions.
    NoEmptyAlternatives,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow empty regular-expression alternatives.",
);

impl Rule for NoEmptyAlternatives {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::RegExpLiteral(literal) = node.kind() else {
            return;
        };
        let pattern = literal.regex.pattern.text.as_str();
        if pattern.starts_with('|')
            || pattern.ends_with('|')
            || pattern.contains("(|")
            || pattern.contains("|)")
        {
            ctx.diagnostic(
                OxcDiagnostic::warn("Remove the empty regular-expression alternative.")
                    .with_help("Replace the empty alternative with the intended pattern.")
                    .with_label(literal.span),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoEmptyAlternatives::NAME,
        NoEmptyAlternatives::PLUGIN,
        vec!["const pattern = /value|other/;"],
        vec!["const pattern = /value|/;", "const pattern = /(|value)/;"],
    )
    .test_and_snapshot();
}
