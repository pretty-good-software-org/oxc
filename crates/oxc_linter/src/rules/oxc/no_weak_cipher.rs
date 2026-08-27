use cow_utils::CowUtils;
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoWeakCipher;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows string literals naming weak or obsolete ciphers.
    NoWeakCipher,
    oxc,
    suspicious,
    none,
    version = "0.0.1",
    short_description = "Disallow weak cipher algorithms.",
);

impl Rule for NoWeakCipher {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::StringLiteral(literal) = node.kind() else {
            return;
        };
        let value = literal.value.cow_to_ascii_uppercase();
        let weak = ["DES", "3DES", "RC2", "RC4", "ECB"]
            .iter()
            .any(|cipher| value.split(['-', '/', '_']).any(|part| part == *cipher));
        if weak {
            ctx.diagnostic(
                OxcDiagnostic::warn("Do not use a weak cipher algorithm.")
                    .with_help("Use a modern authenticated encryption algorithm such as AES-GCM.")
                    .with_label(literal.span),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoWeakCipher::NAME,
        NoWeakCipher::PLUGIN,
        vec![r#"const cipher = "AES-256-GCM";"#],
        vec![r#"const cipher = "AES-128-ECB";"#, r#"const cipher = "RC4";"#],
    )
    .test_and_snapshot();
}
