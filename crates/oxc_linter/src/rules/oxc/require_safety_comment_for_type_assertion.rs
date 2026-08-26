use oxc_ast::{AstKind, ast::TSType};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::Span;

use crate::{AstNode, context::LintContext, rule::Rule};

fn missing_safety_comment_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("This type assertion has no `SAFETY:` justification.")
        .with_help(
            "State the invariant TypeScript cannot express immediately before the assertion.",
        )
        .with_label(span)
}

#[derive(Debug, Default, Clone)]
pub struct RequireSafetyCommentForTypeAssertion;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Requires a nearby `SAFETY:` comment for non-const TypeScript assertions.
    ///
    /// ### Why is this bad?
    ///
    /// Assertions bypass TypeScript's checks. A short invariant comment makes
    /// the unchecked boundary reviewable.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```typescript
    /// const user = value as User;
    /// ```
    RequireSafetyCommentForTypeAssertion,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Require safety comments for type assertions.",
);

impl Rule for RequireSafetyCommentForTypeAssertion {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let span = match node.kind() {
            AstKind::TSAsExpression(assertion) => {
                if is_const_assertion(&assertion.type_annotation) {
                    return;
                }
                assertion.span
            }
            AstKind::TSTypeAssertion(assertion) => {
                if is_const_assertion(&assertion.type_annotation) {
                    return;
                }
                assertion.span
            }
            _ => return,
        };
        let window_start = span.start.saturating_sub(512);
        let has_safety_comment = ctx.comments().iter().any(|comment| {
            comment.span.end <= span.start
                && comment.span.start >= window_start
                && ctx.source_range(comment.content_span()).contains("SAFETY:")
        });
        if !has_safety_comment {
            ctx.diagnostic(missing_safety_comment_diagnostic(span));
        }
    }
}

fn is_const_assertion(type_annotation: &TSType<'_>) -> bool {
    matches!(
        type_annotation,
        TSType::TSTypeReference(reference)
            if reference.type_name.is_identifier()
                && reference.type_name.get_identifier_reference().is_some_and(|name| name.name == "const")
    )
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec![
        "// SAFETY: validated by the parser\nconst user = value as User;",
        "const values = input as const;",
    ];
    let fail = vec!["const user = value as User;"];

    Tester::new(
        RequireSafetyCommentForTypeAssertion::NAME,
        RequireSafetyCommentForTypeAssertion::PLUGIN,
        pass,
        fail,
    )
    .test_and_snapshot();
}
