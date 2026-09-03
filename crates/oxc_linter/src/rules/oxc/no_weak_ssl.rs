use cow_utils::CowUtils;
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoWeakSsl;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows obsolete SSL and TLS protocol versions in string literals.
    NoWeakSsl,
    oxc,
    suspicious,
    none,
    version = "0.0.1",
    short_description = "Disallow obsolete SSL and TLS versions.",
);

impl Rule for NoWeakSsl {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::StringLiteral(literal) = node.kind() else {
            return;
        };
        let value = literal.value.cow_to_ascii_uppercase();
        if ["SSLV2", "SSLV3", "TLSV1", "TLSV1.0", "TLSV1.1"].iter().any(|version| value == *version)
        {
            ctx.diagnostic(
                OxcDiagnostic::warn("Do not use an obsolete SSL or TLS protocol version.")
                    .with_help("Require TLS 1.2 or newer.")
                    .with_label(literal.span),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoWeakSsl::NAME,
        NoWeakSsl::PLUGIN,
        vec![r#"const version = "TLSv1.3";"#],
        vec![r#"const version = "TLSv1.1";"#, r#"const version = "SSLv3";"#],
    )
    .test_and_snapshot();
}
