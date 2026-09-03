use oxc_ast::{
    AstKind,
    ast::{BindingPattern, Expression},
};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoEmptyCollection;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Reports reads from collections that are provably empty.
    NoEmptyCollection,
    oxc,
    suspicious,
    none,
    version = "0.0.1",
    short_description = "Disallow operations that can only observe an empty collection.",
);

impl Rule for NoEmptyCollection {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::VariableDeclarator(declaration) = node.kind() else { return };
        let Some(Expression::ArrayExpression(array)) = declaration.init.as_ref() else { return };
        if !array.elements.is_empty() {
            return;
        }
        let BindingPattern::BindingIdentifier(identifier) = &declaration.id else { return };
        let references =
            ctx.semantic().symbol_references(identifier.symbol_id()).collect::<Vec<_>>();
        if !references.is_empty() && references.iter().any(|reference| !reference.is_write()) {
            ctx.diagnostic(
                OxcDiagnostic::warn("Review this usage of an empty collection.")
                    .with_help(
                        "Populate the collection before reading from it or remove the operation.",
                    )
                    .with_label(identifier.span),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoEmptyCollection::NAME,
        NoEmptyCollection::PLUGIN,
        vec!["const values = [value]; consume(values);"],
        vec!["const values = []; values.length;"],
    )
    .test_and_snapshot();
}
