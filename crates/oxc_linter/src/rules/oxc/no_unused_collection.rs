use oxc_ast::{
    AstKind,
    ast::{BindingPattern, Expression},
};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoUnusedCollection;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Reports collections that are assigned repeatedly without their contents being read.
    NoUnusedCollection,
    oxc,
    suspicious,
    none,
    version = "0.0.1",
    short_description = "Disallow unused collections.",
);

impl Rule for NoUnusedCollection {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::VariableDeclarator(declaration) = node.kind() else { return };
        let Some(Expression::ArrayExpression(_)) = declaration.init.as_ref() else { return };
        let BindingPattern::BindingIdentifier(identifier) = &declaration.id else { return };
        let references =
            ctx.semantic().symbol_references(identifier.symbol_id()).collect::<Vec<_>>();
        if references.len() > 1 && references.iter().all(|reference| reference.is_write()) {
            ctx.diagnostic(
                OxcDiagnostic::warn(
                    "Either use this collection's contents or remove the collection.",
                )
                .with_help("Read from the collection or remove redundant assignments.")
                .with_label(identifier.span),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoUnusedCollection::NAME,
        NoUnusedCollection::PLUGIN,
        vec!["const values = []; consume(values);"],
        vec!["let values = []; values = []; values = [];"],
    )
    .test_and_snapshot();
}
