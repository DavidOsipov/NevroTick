/** Sensitivity control: comparison with healthy expected rows must fail. */
import ProductFlowModel
module ProductFlow = DataFlow::Global<ProductConfig>;
from string caseId, Function sourceOwner, int parameterIndex, Function sinkOwner, Expr sinkExpr,
     DataFlow::ParameterNode source, DataFlow::ExprNode sink
where
  globalCase(caseId, sourceOwner, parameterIndex, sinkOwner, sinkExpr) and
  source.asParameter() = sourceOwner.getParam(parameterIndex) and sink.asExpr() = sinkExpr and
  sourceOwner.getCanonicalPath() = "nevrotick::deliberately_missing_product_owner" and
  ProductFlow::flow(source, sink)
select caseId, parameterIndex, sourceOwner.getCanonicalPath(), sinkOwner.getCanonicalPath(),
       sinkExpr.getLocation().getFile().getRelativePath(),
       sinkExpr.getLocation().getStartLine(), sinkExpr.getLocation().getStartColumn()
