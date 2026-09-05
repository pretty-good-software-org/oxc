// Port of sonarjs/non-existent-operator (S2757).
// https://github.com/SonarSource/eslint-plugin-sonarjs/blob/v3.0.7/packages/jsts/src/rules/S2757

use oxc_ast::{AstKind, ast::Expression};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::{GetSpan, Span};
use oxc_syntax::operator::{AssignmentOperator, UnaryOperator};

use crate::{AstNode, context::LintContext, rule::Rule};

fn non_existent_operator_diagnostic(span: Span, operator: &str) -> OxcDiagnostic {
    OxcDiagnostic::warn(format!("Was \"{operator}=\" meant instead?")).with_label(span)
}

#[derive(Debug, Default, Clone)]
pub struct NonExistentOperator;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Reports assignments and initializations that look like they were meant to use a
    /// compound assignment operator (`+=`, `-=`, `!=`) but instead accidentally use `=`
    /// immediately followed by a unary operator (`=+`, `=-`, `=!`), e.g. `x =- 1;`.
    ///
    /// ### Why is this bad?
    ///
    /// `=+`, `=-` and `=!` are not real operators. `x =- 1;` parses as `x = (-1);`, which is
    /// almost always a typo for the compound assignment `x -= 1;`. The rule only fires when
    /// there is no space between `=` and the unary operator, but there is space (or nothing
    /// adjacent) between the unary operator and its operand, since that whitespace pattern is
    /// what makes the code misleading to a human reader.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```javascript
    /// let x = 1;
    /// x =- 1;
    /// x =+ 1;
    /// x =! 1;
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```javascript
    /// let x = 1;
    /// x -= 1;
    /// x = -1;
    /// x = -x;
    /// ```
    NonExistentOperator,
    oxc,
    suspicious,
    suggestion,
    version = "0.0.1",
    short_description = "Reports non-existent operators such as `=+`, `=-` and `=!`.",
);

impl Rule for NonExistentOperator {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        match node.kind() {
            AstKind::AssignmentExpression(assignment) => {
                if assignment.operator == AssignmentOperator::Assign {
                    check_operator(&assignment.right, true, ctx);
                }
            }
            AstKind::VariableDeclarator(declarator) => {
                if let Some(init) = &declarator.init {
                    check_operator(init, false, ctx);
                }
            }
            _ => {}
        }
    }
}

fn check_operator<'a>(unary_node: &Expression<'a>, is_assignment: bool, ctx: &LintContext<'a>) {
    let Expression::UnaryExpression(unary) = unary_node else { return };
    if !is_unary_operator_of_interest(unary.operator) {
        return;
    }

    let source = ctx.source_text();
    let unary_start = unary.span.start;
    let argument_start = unary.argument.span().start;

    // The unary operator must NOT be adjacent to its operand (there is whitespace between
    // them), otherwise this is a normal, unambiguous unary expression.
    if unary_start + 1 == argument_start {
        return;
    }

    // The assignment operator `=` must be immediately adjacent to the unary operator (no
    // whitespace between `=` and e.g. `-`), which is what makes `=- ` look like `-=`.
    if unary_start == 0 {
        return;
    }
    let assign_op_start = unary_start - 1;
    if source.as_bytes().get(assign_op_start as usize) != Some(&b'=') {
        return;
    }

    let operator = unary.operator.as_str();
    let span = Span::new(assign_op_start, unary_start + 1);

    if is_assignment {
        let suggestion = format!("{operator}=");
        ctx.diagnostic_with_suggestion(
            non_existent_operator_diagnostic(span, operator),
            move |fixer| fixer.replace(span, suggestion),
        );
    } else {
        ctx.diagnostic(non_existent_operator_diagnostic(span, operator));
    }
}

fn is_unary_operator_of_interest(operator: UnaryOperator) -> bool {
    matches!(
        operator,
        UnaryOperator::UnaryNegation | UnaryOperator::UnaryPlus | UnaryOperator::LogicalNot
    )
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec![
        "let x = 1; x -= 1;",
        "let x = 1; x = -1;",
        "let x = 1; x = - 1;",
        "let x = 1; x = -x;",
        "let x = -1;",
        "let x = - 1;",
        "let x = 1; x = !x;",
        "let x = 1; x =-x;",
        "let x = 1; x = !!x;",
    ];

    let fail = vec![
        "let x = 1; x =- 1;",
        "let x = 1; x =+ 1;",
        "let x = 1; x =! x;",
        "let x =- 1;",
        "let x =+ 1;",
        "let x =! x;",
    ];

    let fix = vec![
        ("let x = 1; x =- 1;", "let x = 1; x -= 1;", None),
        ("let x = 1; x =+ 1;", "let x = 1; x += 1;", None),
        ("let x = 1; x =! x;", "let x = 1; x != x;", None),
        // No suggestion for VariableDeclarator initializers.
        ("let x =- 1;", "let x =- 1;", None),
    ];

    Tester::new(NonExistentOperator::NAME, NonExistentOperator::PLUGIN, pass, fail)
        .expect_fix(fix)
        .test_and_snapshot();
}
