use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct GeneratorWithoutYield;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows generator functions that never yield a value.
    GeneratorWithoutYield,
    oxc,
    suspicious,
    none,
    version = "0.0.1",
    short_description = "Disallow generators without yield statements.",
);

impl Rule for GeneratorWithoutYield {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::Function(function) = node.kind() else {
            return;
        };
        if !function.generator {
            return;
        }
        let source = ctx.source_range(function.span);
        let has_yield = source
            .split(|character: char| !character.is_ascii_alphanumeric() && character != '_')
            .any(|word| word == "yield");
        if !has_yield {
            ctx.diagnostic(
                OxcDiagnostic::warn("Remove the generator modifier or add a yield statement.")
                    .with_help("A generator should yield values or be an ordinary function.")
                    .with_label(function.span),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        GeneratorWithoutYield::NAME,
        GeneratorWithoutYield::PLUGIN,
        vec!["function* values() { yield 1; }", "function values() { return 1; }"],
        vec!["function* values() { return 1; }"],
    )
    .test_and_snapshot();
}
