use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{AstNode, context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoEmptyTestFile;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Reports test/spec files that contain no recognized test declaration.
    NoEmptyTestFile,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow empty test files.",
);

impl Rule for NoEmptyTestFile {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::Program(program) = node.kind() else { return };
        let Some(filename) = ctx.file_path().file_name().and_then(|name| name.to_str()) else {
            return;
        };
        if !filename.contains(".test.") && !filename.contains(".spec.") {
            return;
        }
        let source = ctx.source_range(program.span);
        let has_test = ["it(", "test(", "describe(", "suite(", "specify("]
            .iter()
            .any(|api| source.contains(api));
        if !has_test {
            ctx.diagnostic(
                OxcDiagnostic::warn("Add some tests to this file or delete it.")
                    .with_label(program.span),
            );
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoEmptyTestFile::NAME,
        NoEmptyTestFile::PLUGIN,
        vec!["it('works', testFn);"],
        vec!["const helper = true;"],
    )
    .change_rule_path("example.test.ts")
    .test_and_snapshot();
}
