use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::Span;

use crate::{AstNode, context::LintContext, rule::Rule};

fn no_commented_code_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Remove this commented out code.")
        .with_help("Delete obsolete code instead of keeping it in a comment.")
        .with_label(span)
}

#[derive(Debug, Default, Clone)]
pub struct NoCommentedCode;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows comments that contain likely source code.
    ///
    /// ### Why is this bad?
    ///
    /// Version control preserves old code without keeping it in source files.
    /// Commented-out code obscures the active implementation.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```javascript
    /// // const obsolete = 1;
    /// ```
    NoCommentedCode,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow commented-out source code.",
);

impl Rule for NoCommentedCode {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        if !matches!(node.kind(), AstKind::Program(_)) {
            return;
        }
        for comment in ctx.comments() {
            let content = ctx.source_range(comment.content_span());
            if is_likely_code(content.trim()) {
                ctx.diagnostic(no_commented_code_diagnostic(comment.span));
            }
        }
    }
}

fn is_likely_code(content: &str) -> bool {
    content.lines().any(|line| {
        let line = line.trim().trim_start_matches('*').trim();
        let starts_like_code = [
            "const ",
            "let ",
            "var ",
            "function ",
            "class ",
            "if (",
            "for (",
            "while (",
            "return ",
            "import ",
            "export ",
        ]
        .iter()
        .any(|prefix| line.starts_with(prefix));
        starts_like_code && (line.contains(';') || line.ends_with('{') || line.ends_with('}'))
    })
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec!["// explain the value\nconst value = 1;", "/* public API */\nconst value = 1;"];
    let fail = vec!["// const obsolete = 1;", "/* function obsolete() {} */"];

    Tester::new(NoCommentedCode::NAME, NoCommentedCode::PLUGIN, pass, fail).test_and_snapshot();
}
