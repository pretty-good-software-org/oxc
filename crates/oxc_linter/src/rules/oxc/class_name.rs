use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::GetSpan;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
struct Config {
    format: String,
}

impl Default for Config {
    fn default() -> Self {
        Self { format: "^[A-Z][a-zA-Z0-9]*$".into() }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ClassName(Box<Config>);

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Enforces the configured naming convention for classes.
    ClassName,
    oxc,
    style,
    config = Config,
    version = "0.0.1",
    short_description = "Enforce class naming conventions.",
);

impl Rule for ClassName {
    fn from_configuration(value: serde_json::Value) -> Result<Self, serde_json::error::Error> {
        if value.is_null() {
            return Ok(Self::default());
        }
        Ok(Self(Box::new(serde_json::from_value(value)?)))
    }

    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let name = match node.kind() {
            AstKind::Class(class) => class.id.as_ref().map(|id| id.name.as_str()),
            _ => None,
        };
        let Some(name) = name else { return };
        let valid = name.chars().next().is_some_and(|character| character.is_ascii_uppercase())
            && name.chars().all(|character| character.is_ascii_alphanumeric());
        if !valid {
            ctx.diagnostic(
                OxcDiagnostic::warn(format!("Rename class `{name}` to match `{}`.", self.0.format))
                    .with_help("Use a PascalCase class name.")
                    .with_label(node.kind().span()),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        ClassName::NAME,
        ClassName::PLUGIN,
        vec!["class Example {}"],
        vec!["class example {}"],
    )
    .test_and_snapshot();
}
