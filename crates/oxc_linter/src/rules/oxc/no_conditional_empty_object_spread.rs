use oxc_ast::{AstKind, ast::Expression};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::Span;

use crate::{AstNode, context::LintContext, rule::Rule};

fn no_conditional_empty_object_spread_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Avoid conditionally spreading an empty object.")
        .with_help("Build the object explicitly and add the property only when present.")
        .with_label(span)
}

#[derive(Debug, Default, Clone)]
pub struct NoConditionalEmptyObjectSpread;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows object spreads that conditionally spread an empty object.
    ///
    /// ### Why is this bad?
    ///
    /// Conditional empty-object spreads hide property omission semantics in a
    /// terse expression that is easy to misread.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```javascript
    /// const result = { ...(condition ? { value } : {}) };
    /// ```
    NoConditionalEmptyObjectSpread,
    oxc,
    style,
    none,
    version = "0.0.1",
    short_description = "Disallow conditional empty-object spreads.",
);

impl Rule for NoConditionalEmptyObjectSpread {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::SpreadElement(spread) = node.kind() else {
            return;
        };
        if !matches!(ctx.nodes().parent_kind(node.id()), AstKind::ObjectExpression(_)) {
            return;
        }
        let conditional = unwrap_parentheses(&spread.argument);
        let Expression::ConditionalExpression(conditional) = conditional else {
            return;
        };
        if is_empty_object(&conditional.consequent) || is_empty_object(&conditional.alternate) {
            ctx.diagnostic(no_conditional_empty_object_spread_diagnostic(spread.span));
        }
    }
}

fn unwrap_parentheses<'a>(expression: &'a Expression<'a>) -> &'a Expression<'a> {
    match expression {
        Expression::ParenthesizedExpression(parenthesized) => {
            unwrap_parentheses(&parenthesized.expression)
        }
        _ => expression,
    }
}

fn is_empty_object(expression: &Expression<'_>) -> bool {
    matches!(expression, Expression::ObjectExpression(object) if object.properties.is_empty())
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass =
        vec!["const result = { ...value };", "const result = { ...(condition ? value : other) };"];
    let fail = vec![
        "const result = { ...(condition ? { value } : {}) };",
        "const result = { ...(condition ? {} : { value }) };",
    ];

    Tester::new(
        NoConditionalEmptyObjectSpread::NAME,
        NoConditionalEmptyObjectSpread::PLUGIN,
        pass,
        fail,
    )
    .test_and_snapshot();
}
