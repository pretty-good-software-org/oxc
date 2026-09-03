use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::Span;
use oxc_syntax::operator::UnaryOperator;

use crate::{AstNode, context::LintContext, rule::Rule};

fn no_runtime_typeof_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Avoid runtime `typeof` checks for external values.")
        .with_help("Decode input at its I/O boundary, then branch on the domain value.")
        .with_label(span)
}

#[derive(Debug, Default, Clone)]
pub struct NoRuntimeTypeof;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows runtime `typeof` checks.
    ///
    /// ### Why is this bad?
    ///
    /// A `typeof` check narrows a runtime representation without establishing
    /// the domain contract of external input.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```javascript
    /// if (typeof value === "string") {
    ///   useValue(value);
    /// }
    /// ```
    NoRuntimeTypeof,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow runtime `typeof` checks.",
);

impl Rule for NoRuntimeTypeof {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::UnaryExpression(expression) = node.kind() else {
            return;
        };
        if expression.operator == UnaryOperator::Typeof {
            ctx.diagnostic(no_runtime_typeof_diagnostic(expression.span));
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec!["value", "String(value)", "value === undefined"];
    let fail = vec!["typeof value", "typeof value === 'string'"];

    Tester::new(NoRuntimeTypeof::NAME, NoRuntimeTypeof::PLUGIN, pass, fail).test_and_snapshot();
}
