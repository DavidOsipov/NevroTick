/** Sensitivity control: comparison with healthy expected rows must fail. */
import ProductAstModel
import codeql.rust.dataflow.DataFlow
from string caseId, Function owner, int parameterIndex, Expr sinkExpr,
     DataFlow::ParameterNode source, DataFlow::ExprNode sink
where
  localCase(caseId, owner, parameterIndex, sinkExpr) and
  source.asParameter() = owner.getParam(parameterIndex) and sink.asExpr() = sinkExpr and
  owner.getCanonicalPath() = "nevrotick::deliberately_missing_product_owner" and
  DataFlow::localFlow(source, sink)
select caseId, parameterIndex, owner.getCanonicalPath(),
       sinkExpr.getLocation().getFile().getRelativePath(),
       sinkExpr.getLocation().getStartLine(), sinkExpr.getLocation().getStartColumn()
