use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::GetSpan;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoDuplicateInComposite;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows duplicate members in TypeScript unions and intersections.
    NoDuplicateInComposite,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow duplicate composite type members.",
);

impl Rule for NoDuplicateInComposite {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let members = match node.kind() {
            AstKind::TSUnionType(composite) => &composite.types,
            AstKind::TSIntersectionType(composite) => &composite.types,
            _ => return,
        };
        for (index, member) in members.iter().enumerate() {
            let text = ctx.source_range(member.span()).trim();
            if members[..index]
                .iter()
                .any(|previous| ctx.source_range(previous.span()).trim() == text)
            {
                ctx.diagnostic(
                    OxcDiagnostic::warn("Remove this duplicated composite type member.")
                        .with_help("Remove the duplicate or replace it with another type.")
                        .with_label(member.span()),
                );
            }
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoDuplicateInComposite::NAME,
        NoDuplicateInComposite::PLUGIN,
        vec!["type Value = string | number;"],
        vec!["type Value = string | number | string;", "type Value = A & B & A;"],
    )
    .test_and_snapshot();
}
