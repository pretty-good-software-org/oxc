use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;

use crate::{context::LintContext, rule::Rule};

#[derive(Debug, Default, Clone)]
pub struct NoImplicitGlobal;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Reports writes to unresolved names that are not configured or standard globals.
    NoImplicitGlobal,
    oxc,
    correctness,
    none,
    version = "0.0.1",
    short_description = "Disallow implicit global declarations.",
);

impl Rule for NoImplicitGlobal {
    fn should_run(&self, ctx: &crate::context::ContextHost) -> bool {
        // The partial loader does not model Svelte's implicit reactive bindings.
        // Enable this rule once framework-aware scope analysis supplies those bindings.
        ctx.file_extension().is_none_or(|extension| extension != "svelte")
    }

    fn run_once(&self, ctx: &LintContext) {
        for (name, reference_ids) in ctx.scoping().root_unresolved_references() {
            if ctx.get_global_variable_value(name).is_some() {
                continue;
            }
            for reference_id in reference_ids {
                let reference = ctx.scoping().get_reference(*reference_id);
                if reference.is_write() {
                    ctx.diagnostic(
                        OxcDiagnostic::warn(format!(
                            "Add an explicit declaration for `{name}` to avoid creating an implicit global."
                        ))
                        .with_help("Declare the variable with `let`, `const`, or `var`.")
                        .with_label(ctx.semantic().reference_span(reference)),
                    );
                }
            }
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;
    Tester::new(
        NoImplicitGlobal::NAME,
        NoImplicitGlobal::PLUGIN,
        vec!["let value = 1; value = 2;", "window.value = 1;"],
        vec!["implicitValue = 1;"],
    )
    .test_and_snapshot();
}

#[test]
fn test_svelte_reactive_bindings() {
    use crate::tester::Tester;
    let pass = vec!["<script>$: ({ name } = person); name = 'updated';</script>"];
    Tester::new(NoImplicitGlobal::NAME, NoImplicitGlobal::PLUGIN, pass, vec![])
        .change_rule_path("test.svelte")
        .test();
}
