use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoClearTextProtocols;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows clear-text HTTP URLs in string literals.
    NoClearTextProtocols,
    oxc,
    suspicious,
    none,
    version = "0.0.1",
    short_description = "Disallow clear-text HTTP URLs.",
);

impl Rule for NoClearTextProtocols {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::StringLiteral(literal) = node.kind() else {
            return;
        };
        if literal.value.starts_with("http://") {
            ctx.diagnostic(
                OxcDiagnostic::warn("Use HTTPS instead of a clear-text HTTP URL.")
                    .with_help("Use an HTTPS URL to protect data in transit.")
                    .with_label(literal.span),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoClearTextProtocols::NAME,
        NoClearTextProtocols::PLUGIN,
        vec![r#"const url = "https://example.test";"#],
        vec![r#"const url = "http://example.test";"#],
    )
    .test_and_snapshot();
}
