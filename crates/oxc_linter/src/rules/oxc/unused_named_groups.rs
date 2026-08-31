use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct UnusedNamedGroups;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Reports named regular-expression groups that are never referenced.
    UnusedNamedGroups,
    oxc,
    suspicious,
    none,
    version = "0.0.1",
    short_description = "Disallow unused named regex groups.",
);

impl Rule for UnusedNamedGroups {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::RegExpLiteral(literal) = node.kind() else { return };
        let pattern = literal.regex.pattern.text.as_str();
        let mut remainder = pattern;
        while let Some(start) = remainder.find("(?<") {
            let after = &remainder[start + 3..];
            let Some(end) = after.find('>') else { break };
            let name = &after[..end];
            if !name.is_empty() && !pattern.contains(&format!(r"\k<{name}>")) {
                ctx.diagnostic(
                    OxcDiagnostic::warn(format!("Remove the unused named group `{name}`."))
                        .with_help("Remove the group or reference it with a named backreference.")
                        .with_label(literal.span),
                );
            }
            remainder = &after[end + 1..];
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        UnusedNamedGroups::NAME,
        UnusedNamedGroups::PLUGIN,
        vec![r"/(?<word>\w+)\k<word>/"],
        vec![r"/(?<word>\w+)/"],
    )
    .test_and_snapshot();
}
