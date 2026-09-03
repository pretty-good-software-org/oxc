use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::GetSpan;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct UseTypeAlias;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Reports large repeated type compositions that should be named aliases.
    UseTypeAlias,
    oxc,
    style,
    none,
    version = "0.0.1",
    short_description = "Prefer type aliases for large type compositions.",
);

impl Rule for UseTypeAlias {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let type_count = match node.kind() {
            AstKind::TSUnionType(union) => union.types.len(),
            AstKind::TSIntersectionType(intersection) => intersection.types.len(),
            _ => return,
        };
        if type_count <= 2 {
            return;
        }
        let kind =
            if matches!(node.kind(), AstKind::TSUnionType(_)) { "union" } else { "intersection" };
        ctx.diagnostic(
            OxcDiagnostic::warn(format!("Replace this {kind} type with a type alias."))
                .with_help("Name the composed type and reuse the alias.")
                .with_label(node.kind().span()),
        );
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        UseTypeAlias::NAME,
        UseTypeAlias::PLUGIN,
        vec!["type User = string | number;"],
        vec!["let value: string | number | boolean = input;"],
    )
    .test_and_snapshot();
}
