/** Borrowed-value propagation from UiDrafts::replace across RawText::parse. */
import ProductFlowModel
module ProductFlow = DataFlow::Global<ProductConfig>;
from string caseId, Function sourceOwner, int parameterIndex, Function sinkOwner, Expr sinkExpr,
     DataFlow::ParameterNode source, DataFlow::ExprNode sink
where
  globalCase(caseId, sourceOwner, parameterIndex, sinkOwner, sinkExpr) and
  source.asParameter() = sourceOwner.getParam(parameterIndex) and sink.asExpr() = sinkExpr and
  ProductFlow::flow(source, sink)
select caseId, parameterIndex, sourceOwner.getCanonicalPath(), sinkOwner.getCanonicalPath(),
       sinkExpr.getLocation().getFile().getRelativePath(),
       sinkExpr.getLocation().getStartLine(), sinkExpr.getLocation().getStartColumn()
