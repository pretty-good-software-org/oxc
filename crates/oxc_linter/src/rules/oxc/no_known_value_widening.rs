use oxc_ast::{
    AstKind,
    ast::{Expression, TSType},
};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::{GetSpan, Span};

use crate::{AstNode, context::LintContext, rule::Rule};

fn no_known_value_widening_diagnostic(span: Span, name: &str) -> OxcDiagnostic {
    OxcDiagnostic::warn(format!("Binding `{name}` discards known type evidence."))
        .with_help("Keep inference or use a named owner contract instead of a broad type.")
        .with_label(span)
}

#[derive(Debug, Default, Clone)]
pub struct NoKnownValueWidening;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows explicit broad annotations on bindings initialized with
    /// syntactically known values.
    ///
    /// ### Why is this bad?
    ///
    /// The annotation discards useful type evidence that TypeScript inferred
    /// from the initializer.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```typescript
    /// const value: unknown = { name: "Ada" };
    /// ```
    NoKnownValueWidening,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow widening known values with broad annotations.",
);

impl Rule for NoKnownValueWidening {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::VariableDeclarator(declarator) = node.kind() else {
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
        let Some(name) = declarator.id.get_identifier_name() else {
            return;
        };
        ctx.diagnostic(no_known_value_widening_diagnostic(declarator.id.span(), name.as_str()));
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

    let pass = vec!["const value: User = { name: 'Ada' };", "const value: unknown = getValue();"];
    let fail = vec!["const value: unknown = { name: 'Ada' };", "const value: object = []; "];

    Tester::new(NoKnownValueWidening::NAME, NoKnownValueWidening::PLUGIN, pass, fail)
        .test_and_snapshot();
}
