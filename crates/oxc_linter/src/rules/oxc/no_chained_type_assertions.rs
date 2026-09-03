use crate::{AstNode, context::LintContext, rule::Rule};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::{GetSpan, Span};

fn no_chained_type_assertions_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Avoid chained TypeScript assertions.")
        .with_help("Use one named type or validate the value at its boundary.")
        .with_label(span)
}

#[derive(Debug, Default, Clone)]
pub struct NoChainedTypeAssertions;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows chained TypeScript type assertions.
    ///
    /// ### Why is this bad?
    ///
    /// Multiple assertions hide unsound widening and narrowing at the call
    /// site. Use one named type or validate the value at its boundary.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```typescript
    /// const value = input as unknown as User;
    /// ```
    NoChainedTypeAssertions,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow chained TypeScript type assertions.",
);

impl Rule for NoChainedTypeAssertions {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let is_assertion =
            matches!(node.kind(), AstKind::TSAsExpression(_) | AstKind::TSTypeAssertion(_));
        if !is_assertion {
            return;
        }
        let parent = ctx.nodes().parent_kind(node.id());
        if matches!(parent, AstKind::TSAsExpression(_) | AstKind::TSTypeAssertion(_)) {
            ctx.diagnostic(no_chained_type_assertions_diagnostic(node.span()));
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec!["const value = input as User;"];
    let fail = vec!["const value = input as unknown as User;"];

    Tester::new(NoChainedTypeAssertions::NAME, NoChainedTypeAssertions::PLUGIN, pass, fail)
        .test_and_snapshot();
}
