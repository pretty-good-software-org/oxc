use cow_utils::CowUtils;
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
struct Config {
    #[serde(rename = "passwordWords")]
    password_words: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            password_words: vec![
                "password".into(),
                "pwd".into(),
                "passwd".into(),
                "passphrase".into(),
            ],
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct NoHardcodedPasswords(Config);

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows string literals assigned to names that suggest they contain passwords.
    NoHardcodedPasswords,
    oxc,
    suspicious,
    config = Config,
    version = "0.0.1",
    short_description = "Disallow hard-coded passwords.",
);

impl Rule for NoHardcodedPasswords {
    fn from_configuration(value: serde_json::Value) -> Result<Self, serde_json::error::Error> {
        if value.is_null() {
            return Ok(Self::default());
        }
        Ok(Self(serde_json::from_value(value)?))
    }

    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::StringLiteral(literal) = node.kind() else {
            return;
        };
        let Some(binding) = (match ctx.nodes().parent_node(node.id()).kind() {
            AstKind::VariableDeclarator(declarator) => declarator.id.get_identifier_name(),
            _ => None,
        }) else {
            return;
        };
        let binding = binding.cow_to_ascii_lowercase();
        if self.0.password_words.iter().any(|word| {
            binding.contains(word.cow_to_ascii_lowercase().as_ref()) && !literal.value.is_empty()
        }) {
            ctx.diagnostic(
                OxcDiagnostic::warn("Do not hard-code a password.")
                    .with_help("Use a secret manager or environment variable.")
                    .with_label(literal.span),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoHardcodedPasswords::NAME,
        NoHardcodedPasswords::PLUGIN,
        vec!["const password = process.env.PASSWORD;"],
        vec![r#"const password = "secret";"#],
    )
    .test_and_snapshot();
}
