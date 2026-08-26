use oxc_ast::{
    AstKind,
    ast::{TSType, TSTypeName},
};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::{GetSpan, Span};

use crate::{AstNode, context::LintContext, rule::Rule};

fn no_unknown_returns_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("This function exposes `unknown` to its caller.")
        .with_help("Parse the value at its boundary and return a named domain type.")
        .with_label(span)
}

#[derive(Debug, Default, Clone)]
pub struct NoUnknownReturns;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows explicit function return contracts containing `unknown`.
    ///
    /// ### Why is this bad?
    ///
    /// Unknown return values force every caller to repeat boundary validation.
    /// Parse once and expose a named domain type instead.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```typescript
    /// function load(): unknown { return input; }
    /// ```
    NoUnknownReturns,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow function returns containing `unknown`.",
);

impl Rule for NoUnknownReturns {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::TSTypeAnnotation(annotation) = node.kind() else {
            return;
        };
        let parent = ctx.nodes().parent_node(node.id());
        let grand_parent = ctx.nodes().parent_node(parent.id());
        if !(is_return_annotation(annotation.span, parent.kind())
            || is_return_annotation(annotation.span, grand_parent.kind()))
            || !contains_unknown(&annotation.type_annotation)
        {
            return;
        }
        ctx.diagnostic(no_unknown_returns_diagnostic(annotation.type_annotation.span()));
    }
}

fn is_return_annotation(span: Span, parent: AstKind<'_>) -> bool {
    match parent {
        AstKind::Function(function) => {
            function.return_type.as_ref().is_some_and(|ret| ret.span == span)
        }
        AstKind::ArrowFunctionExpression(function) => {
            function.return_type.as_ref().is_some_and(|ret| ret.span == span)
        }
        AstKind::TSFunctionType(function) => function.return_type.span == span,
        AstKind::TSConstructorType(function) => function.return_type.span == span,
        AstKind::TSCallSignatureDeclaration(signature) => {
            signature.return_type.as_ref().is_some_and(|ret| ret.span == span)
        }
        AstKind::TSMethodSignature(signature) => {
            signature.return_type.as_ref().is_some_and(|ret| ret.span == span)
        }
        AstKind::TSConstructSignatureDeclaration(signature) => {
            signature.return_type.as_ref().is_some_and(|ret| ret.span == span)
        }
        _ => false,
    }
}

fn contains_unknown(type_annotation: &TSType<'_>) -> bool {
    match type_annotation {
        TSType::TSUnknownKeyword(_) => true,
        TSType::TSParenthesizedType(parenthesized) => {
            contains_unknown(&parenthesized.type_annotation)
        }
        TSType::TSUnionType(union) => union.types.iter().any(contains_unknown),
        TSType::TSTypeReference(reference) => {
            let Some(name) = TSTypeName::get_identifier_reference(&reference.type_name) else {
                return false;
            };
            matches!(name.name.as_str(), "Promise" | "PromiseLike")
                && reference
                    .type_arguments
                    .as_ref()
                    .and_then(|arguments| arguments.params.first())
                    .is_some_and(contains_unknown)
        }
        _ => false,
    }
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec!["function load(): User { return input; }"];
    let fail = vec![
        "function load(): unknown { return input; }",
        "const load = (): Promise<unknown> => input;",
    ];

    Tester::new(NoUnknownReturns::NAME, NoUnknownReturns::PLUGIN, pass, fail).test_and_snapshot();
}
