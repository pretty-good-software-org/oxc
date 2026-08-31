use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct ConciseRegex;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Reports verbose regular-expression character classes with concise equivalents.
    ConciseRegex,
    oxc,
    style,
    none,
    version = "0.0.1",
    short_description = "Prefer concise regular-expression character classes.",
);

impl Rule for ConciseRegex {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::RegExpLiteral(literal) = node.kind() else { return };
        let pattern = literal.regex.pattern.text.as_str();
        if pattern.contains("[0-9]") || pattern.contains("[a-zA-Z0-9_]") {
            ctx.diagnostic(
                OxcDiagnostic::warn(
                    "Use concise character class syntax in this regular expression.",
                )
                .with_help("Replace `[0-9]` with `\\d` or `[a-zA-Z0-9_]` with `\\w`.")
                .with_label(literal.span),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(ConciseRegex::NAME, ConciseRegex::PLUGIN, vec!["/[a-z]+/"], vec!["/[0-9]+/"])
        .test_and_snapshot();
}
