use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoHardcodedSecrets;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows string literals that look like common service credentials.
    NoHardcodedSecrets,
    oxc,
    suspicious,
    none,
    version = "0.0.1",
    short_description = "Disallow hard-coded service credentials.",
);

impl Rule for NoHardcodedSecrets {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::StringLiteral(literal) = node.kind() else {
            return;
        };
        if looks_like_secret(literal.value.as_str()) {
            ctx.diagnostic(
                OxcDiagnostic::warn("Do not hard-code a service credential.")
                    .with_help("Load credentials from a secret manager or environment variable.")
                    .with_label(literal.span),
            );
        }
    }
}

fn looks_like_secret(value: &str) -> bool {
    let is_jwt = value.starts_with("eyJ") && value.matches('.').count() == 2;
    is_jwt
        || ["AKIA", "ghp_", "github_pat_", "xoxb-", "xoxp-", "sk-"]
            .iter()
            .any(|prefix| value.starts_with(prefix) && value.len() > prefix.len() + 8)
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoHardcodedSecrets::NAME,
        NoHardcodedSecrets::PLUGIN,
        vec!["const token = process.env.TOKEN;", r#"const value = "ordinary text";"#],
        vec![
            r#"const token = "ghp_1234567890abcdef";"#,
            r#"const token = "eyJheader.payload.signature";"#,
        ],
    )
    .test_and_snapshot();
}
