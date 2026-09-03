use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Clone)]
pub struct MaxSwitchCases {
    max: usize,
}

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Limits the number of cases in a switch statement.
    MaxSwitchCases,
    oxc,
    suspicious,
    config = usize,
    version = "0.0.1",
    short_description = "Limit the number of switch cases.",
);

impl Default for MaxSwitchCases {
    fn default() -> Self {
        Self { max: 30 }
    }
}

impl Rule for MaxSwitchCases {
    fn from_configuration(value: serde_json::Value) -> Result<Self, serde_json::error::Error> {
        match value {
            serde_json::Value::Null => Ok(Self::default()),
            serde_json::Value::Number(number) => {
                Ok(Self { max: usize::try_from(number.as_u64().unwrap_or(30)).unwrap_or(30) })
            }
            value => Ok(Self { max: serde_json::from_value(value)? }),
        }
    }

    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::SwitchStatement(statement) = node.kind() else {
            return;
        };
        if statement.cases.len() > self.max {
            ctx.diagnostic(
                OxcDiagnostic::warn(format!(
                    "Reduce this switch statement to at most {} cases.",
                    self.max
                ))
                .with_help("Split the switch into smaller decision structures.")
                .with_label(statement.span),
            );
        }
    }
}

#[test]
#[expect(clippy::format_collect)]
fn test() {
    use crate::tester::Tester;
    let cases = (1..=31).map(|case| format!("case {case}: handler();")).collect::<String>();
    Tester::new(
        MaxSwitchCases::NAME,
        MaxSwitchCases::PLUGIN,
        vec![String::from("switch (value) { case 1: one(); }")],
        vec![format!("switch (value) {{ {cases} }}")],
    )
    .test_and_snapshot();
}
