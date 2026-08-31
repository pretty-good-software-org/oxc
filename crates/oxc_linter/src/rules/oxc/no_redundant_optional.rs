use oxc_ast::{AstKind, ast::TSType};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoRedundantOptional;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Reports optional TypeScript properties whose type explicitly includes `undefined`.
    NoRedundantOptional,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow redundant optional property markers.",
);

impl Rule for NoRedundantOptional {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::TSPropertySignature(property) = node.kind() else { return };
        let Some(annotation) = &property.type_annotation else { return };
        if property.optional && contains_undefined(&annotation.type_annotation) {
            ctx.diagnostic(
                OxcDiagnostic::warn(
                    "Remove either the `?` marker or the explicit `undefined` type.",
                )
                .with_help("Keep one representation of optionality.")
                .with_label(property.span),
            );
        }
    }
}

fn contains_undefined(ty: &TSType) -> bool {
    match ty {
        TSType::TSUndefinedKeyword(_) => true,
        TSType::TSUnionType(union) => union.types.iter().any(contains_undefined),
        _ => false,
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoRedundantOptional::NAME,
        NoRedundantOptional::PLUGIN,
        vec!["type User = { name?: string };"],
        vec!["type User = { name?: string | undefined };"],
    )
    .test_and_snapshot();
}
