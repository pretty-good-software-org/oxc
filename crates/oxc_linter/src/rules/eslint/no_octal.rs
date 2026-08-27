use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::Span;

use crate::{AstNode, context::LintContext, rule::Rule};

fn no_octal_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Don't use legacy octal literals.")
        .with_help("Use a decimal or explicit `0o` octal literal instead.")
        .with_label(span)
}

#[derive(Debug, Default, Clone)]
pub struct NoOctal;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows legacy octal literals such as `0123`.
    ///
    /// ### Why is this bad?
    ///
    /// Legacy octal syntax is ambiguous and is not valid in strict mode. Use
    /// decimal syntax or the explicit `0o` prefix instead.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```javascript
    /// const value = 0123;
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```javascript
    /// const value = 123;
    /// const value = 0o123;
    /// ```
    NoOctal,
    eslint,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow legacy octal literals.",
);

impl Rule for NoOctal {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::NumericLiteral(literal) = node.kind() else {
            return;
        };
        let Some(raw) = literal.raw.as_ref().map(oxc_str::Str::as_str) else {
            return;
        };
        if is_legacy_octal(raw) {
            ctx.diagnostic(no_octal_diagnostic(literal.span));
        }
    }
}

fn is_legacy_octal(raw: &str) -> bool {
    raw.len() > 1
        && raw.starts_with('0')
        && raw[1..].bytes().all(|byte| (b'0'..=b'7').contains(&byte))
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec!["0", "7", "10", "123", "0o123", "0x123", "0b101", "0.123", "1e2"];
    let fail = vec!["00", "01", "0123", "0777"];

    Tester::new(NoOctal::NAME, NoOctal::PLUGIN, pass, fail).test_and_snapshot();
}
