use oxc_ast::{AstKind, ast::ImportDeclarationSpecifier};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct UnusedImport;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Reports imported bindings that have no references in the file.
    UnusedImport,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow unused imports.",
);

impl Rule for UnusedImport {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::ImportDeclaration(import) = node.kind() else { return };
        let Some(specifiers) = &import.specifiers else { return };
        for specifier in specifiers {
            let local = match specifier {
                ImportDeclarationSpecifier::ImportSpecifier(specifier) => &specifier.local,
                ImportDeclarationSpecifier::ImportDefaultSpecifier(specifier) => &specifier.local,
                ImportDeclarationSpecifier::ImportNamespaceSpecifier(specifier) => &specifier.local,
            };
            if ctx.semantic().symbol_references(local.symbol_id()).count() == 0 {
                ctx.diagnostic(
                    OxcDiagnostic::warn(format!("Remove the unused import `{}`.", local.name))
                        .with_help("Remove the import or use it in this file.")
                        .with_label(local.span),
                );
            }
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        UnusedImport::NAME,
        UnusedImport::PLUGIN,
        vec!["import { value } from 'module'; use(value);"],
        vec!["import { value } from 'module';"],
    )
    .test_and_snapshot();
}
