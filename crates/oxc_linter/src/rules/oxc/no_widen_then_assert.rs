use oxc_ast::{
    AstKind,
    ast::{Expression, TSType},
};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::{GetSpan, Span};

use crate::{AstNode, context::LintContext, rule::Rule};

fn no_widen_then_assert_diagnostic(span: Span, name: &str) -> OxcDiagnostic {
    OxcDiagnostic::warn(format!("Binding `{name}` discards type evidence before an assertion."))
        .with_help("Keep the precise type from initialization through use.")
        .with_label(span)
}

#[derive(Debug, Default, Clone)]
pub struct NoWidenThenAssert;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows local bindings that widen known values before asserting them
    /// to a narrower type.
    ///
    /// ### Why is this bad?
    ///
    /// The widening discards type evidence and the later assertion recreates
    /// it without a runtime check.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```typescript
    /// const value: unknown = knownValue;
    /// use(value as User);
    /// ```
    NoWidenThenAssert,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow widening a known value before asserting it.",
);

impl Rule for NoWidenThenAssert {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let expression = match node.kind() {
            AstKind::TSAsExpression(assertion) => {
                (&assertion.expression, &assertion.type_annotation)
            }
            AstKind::TSTypeAssertion(assertion) => {
                (&assertion.expression, &assertion.type_annotation)
            }
            _ => return,
        };
        let Expression::Identifier(identifier) = expression.0.without_parentheses() else {
            return;
        };
        if is_broad_type(expression.1) {
            return;
        }
        let Some(symbol_id) = ctx.scoping().get_reference(identifier.reference_id()).symbol_id()
        else {
            return;
        };
        let declaration = ctx.nodes().get_node(ctx.scoping().symbol_declaration(symbol_id));
        let Some(declarator) = declaration.kind().as_variable_declarator() else {
            return;
        };
        let Some(annotation) = &declarator.type_annotation else {
            return;
        };
        if !is_broad_type(&annotation.type_annotation)
            || !declarator.init.as_ref().is_some_and(is_known_expression)
        {
            return;
        }
        let name = identifier.name.as_str();
        ctx.diagnostic(no_widen_then_assert_diagnostic(node.span(), name));
    }
}

fn is_broad_type(type_annotation: &TSType<'_>) -> bool {
    matches!(
        type_annotation,
        TSType::TSUnknownKeyword(_) | TSType::TSAnyKeyword(_) | TSType::TSObjectKeyword(_)
    )
}

fn is_known_expression(expression: &Expression<'_>) -> bool {
    matches!(
        expression.without_parentheses(),
        Expression::ArrayExpression(_)
            | Expression::ClassExpression(_)
            | Expression::FunctionExpression(_)
            | Expression::NumericLiteral(_)
            | Expression::ObjectExpression(_)
            | Expression::StringLiteral(_)
            | Expression::TemplateLiteral(_)
    )
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec![
        "const value: User = knownValue;",
        "const value: unknown = getValue();\nvalue as User;",
    ];
    let fail = vec!["const value: unknown = {};\nvalue as User;"];

    Tester::new(NoWidenThenAssert::NAME, NoWidenThenAssert::PLUGIN, pass, fail).test_and_snapshot();
}
