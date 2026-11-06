/** Global value-flow configuration; validation is deliberately not a barrier. */
import ProductAstModel
import codeql.rust.dataflow.DataFlow

module ProductConfig implements DataFlow::ConfigSig {
  predicate isSource(DataFlow::Node node) {
    exists(string caseId, Function sourceOwner, int parameterIndex, Function sinkOwner, Expr sink |
      globalCase(caseId, sourceOwner, parameterIndex, sinkOwner, sink) and
      node instanceof DataFlow::ParameterNode and
      node.asParameter() = sourceOwner.getParam(parameterIndex)
    )
  }

  predicate isSink(DataFlow::Node node) {
    exists(string caseId, Function sourceOwner, int parameterIndex, Function sinkOwner, Expr sink |
      globalCase(caseId, sourceOwner, parameterIndex, sinkOwner, sink) and
      node instanceof DataFlow::ExprNode and node.asExpr() = sink
    )
  }
}
