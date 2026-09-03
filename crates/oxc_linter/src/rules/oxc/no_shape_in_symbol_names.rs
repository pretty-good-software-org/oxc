use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::Span;

use crate::{AstNode, context::LintContext, rule::Rule};

fn no_shape_in_symbol_names_diagnostic(span: Span, name: &str) -> OxcDiagnostic {
    OxcDiagnostic::warn(format!("Rename symbol `{name}` for its domain role."))
        .with_help("Use a name that describes ownership or domain meaning instead of structure.")
        .with_label(span)
}

#[derive(Debug, Default, Clone)]
pub struct NoShapeInSymbolNames;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows the case-insensitive substring `shape` in symbol names.
    ///
    /// ### Why is this bad?
    ///
    /// `shape` describes structure rather than ownership or domain meaning.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```typescript
    /// const userShape = parseUser(input);
    /// ```
    NoShapeInSymbolNames,
    oxc,
    style,
    none,
    version = "0.0.1",
    short_description = "Disallow `shape` in symbol names.",
);

impl Rule for NoShapeInSymbolNames {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let (name, span) = match node.kind() {
            AstKind::BindingIdentifier(identifier) => (identifier.name.as_str(), identifier.span),
            AstKind::IdentifierName(identifier) => (identifier.name.as_str(), identifier.span),
            AstKind::PrivateIdentifier(identifier) => (identifier.name.as_str(), identifier.span),
            _ => return,
        };
        if name.as_bytes().windows(5).any(|window| window.eq_ignore_ascii_case(b"shape")) {
            ctx.diagnostic(no_shape_in_symbol_names_diagnostic(span, name));
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec!["const user = value;", "class User { method() {} }"];
    let fail = vec!["const userShape = value;", "class ShapeParser {}"];

    Tester::new(NoShapeInSymbolNames::NAME, NoShapeInSymbolNames::PLUGIN, pass, fail)
        .test_and_snapshot();
}
