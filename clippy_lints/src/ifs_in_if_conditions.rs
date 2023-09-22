use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::higher;
use hir::intravisit::walk_fn;
use hir::{intravisit, Body, Expr, FnDecl};
use intravisit::{walk_expr, FnKind, Visitor};
use rustc_hir as hir;
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::hir::nested_filter;
use rustc_middle::lint::in_external_macro;
use rustc_session::{declare_lint_pass, declare_tool_lint};
use rustc_span::def_id::LocalDefId;
use rustc_span::Span;

declare_clippy_lint! {
    /// ### What it does
    /// Checks for `if` expressions in the conditions of `if`/`else-if` expressions
    ///
    /// ### Why is this bad?
    /// Doing so makes the code difficult to read.
    ///
    /// ### Example
    /// ```rust
    /// if if a == 13 { 10 } else { 0 } > 5 {
    ///     println!("nested if");
    /// }
    /// ```
    /// Use instead:
    /// ```rust
    /// let n = if a == 13 { 10 } else { 0 };
    /// if n > 5 {
    ///     println!("regular if");
    /// }
    /// ```
    #[clippy::version = "1.74.0"]
    pub IFS_IN_IF_CONDITIONS,
    style,
    "checks for usage of `if` expressions in the conditions of `if`/`else-if` expressions"
}
declare_lint_pass!(IfInIfCondition => [IFS_IN_IF_CONDITIONS]);

impl<'tcx> LateLintPass<'tcx> for IfInIfCondition {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        kind: FnKind<'tcx>,
        decl: &'tcx FnDecl<'_>,
        body: &'tcx Body<'_>,
        span: Span,
        id: LocalDefId,
    ) {
        if span.from_expansion() {
            return;
        }

        let mut vis = IfVisitor::new(cx);
        walk_fn(&mut vis, kind, decl, body.id(), id);
    }
}

struct IfVisitor<'a, 'tcx> {
    cx: &'a LateContext<'tcx>,
    in_outer_if: bool,
}

impl<'a, 'tcx> IfVisitor<'a, 'tcx> {
    fn new(cx: &'a LateContext<'tcx>) -> Self {
        Self { cx, in_outer_if: true }
    }
}

impl<'a, 'tcx> Visitor<'tcx> for IfVisitor<'a, 'tcx> {
    type NestedFilter = nested_filter::OnlyBodies;

    fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
        // Don't lint `expr`s in macros
        if in_external_macro(self.cx.tcx.sess, expr.span) {
            return;
        }

        if let Some(higher::If { cond, then: _, r#else }) = higher::If::hir(expr) {
            if !self.in_outer_if {
                span_lint_and_help(
                    self.cx,
                    IFS_IN_IF_CONDITIONS,
                    expr.span,
                    "`if` expr in `if` condition",
                    None,
                    "consider assigning the result of the `if` to a variable and using the variable in the condition instead",
                );
            }
            self.in_outer_if = false;

            walk_expr(self, cond);
            if let Some(r#else) = r#else {
                self.in_outer_if = true;
                self.visit_expr(r#else);
            }
        } else {
            self.in_outer_if = true;
            walk_expr(self, expr);
        }

        self.in_outer_if = true;
    }

    fn nested_visit_map(&mut self) -> Self::Map {
        self.cx.tcx.hir()
    }
}
