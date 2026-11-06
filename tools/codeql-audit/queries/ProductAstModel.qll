/** Exact AST selectors for the immutable product-bd1f7c4 projection. */
import rust

predicate uiAllocationReceiver(Function owner, Expr receiver) {
  owner.getCanonicalPath() = "<nevrotick_ui::ui_drafts::RawText>::parse" and
  owner.getLocation().getFile().getRelativePath() = "crates/nevrotick-ui/src/ui_drafts.rs" and
  exists(MethodCallExpr call |
    call.getEnclosingCallable() = owner and
    call.getIdentifier().getText() = "to_owned" and
    receiver = call.getReceiver() and
    receiver.getLocation().getStartLine() = 183 and
    receiver.getLocation().getStartColumn() = 17
  )
}

predicate engineFramingArgument(Function owner, Expr argument) {
  owner.getCanonicalPath() = "nevrotick_engine::stage_reply" and
  owner.getLocation().getFile().getRelativePath() = "crates/nevrotick-engine/src/lib.rs" and
  exists(CallExpr call |
    call.getEnclosingCallable() = owner and
    argument = call.getSyntacticPositionalArgument(0) and
    argument.getLocation().getStartLine() = 1171 and
    argument.getLocation().getStartColumn() = 46
  )
}

predicate localCase(string caseId, Function owner, int parameterIndex, Expr sink) {
  uiAllocationReceiver(owner, sink) and
  (caseId = "ui_raw" and parameterIndex = 1 or caseId = "ui_field" and parameterIndex = 0)
  or
  engineFramingArgument(owner, sink) and
  (caseId = "engine_bytes" and parameterIndex = 1 or caseId = "engine_slot" and parameterIndex = 0)
}

predicate globalCase(string caseId, Function sourceOwner, int parameterIndex,
                     Function sinkOwner, Expr sink) {
  sourceOwner.getCanonicalPath() = "<nevrotick_ui::ui_drafts::UiDrafts>::replace" and
  sourceOwner.getLocation().getFile().getRelativePath() = "crates/nevrotick-ui/src/ui_drafts.rs" and
  uiAllocationReceiver(sinkOwner, sink) and
  (caseId = "replace_raw" and parameterIndex = 1 or caseId = "replace_field" and parameterIndex = 0)
}
