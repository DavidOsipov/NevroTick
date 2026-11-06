/** AST endpoint membership only; this query does not establish data flow. */
import ProductAstModel

predicate endpoint(string question, string caseId, string role, int parameterIndex, AstNode node) {
  exists(Function owner, Expr sink |
    question = "local" and localCase(caseId, owner, parameterIndex, sink) and
    (role = "source" and node = owner.getParam(parameterIndex) or role = "sink" and node = sink)
  )
  or
  exists(Function sourceOwner, Function sinkOwner, Expr sink |
    question = "global" and globalCase(caseId, sourceOwner, parameterIndex, sinkOwner, sink) and
    (role = "source" and node = sourceOwner.getParam(parameterIndex) or role = "sink" and node = sink)
  )
}

from string question, string caseId, string role, int parameterIndex, AstNode node
where endpoint(question, caseId, role, parameterIndex, node)
select question, caseId, role, parameterIndex,
       node.getEnclosingCallable().(Function).getCanonicalPath(),
       node.getLocation().getFile().getRelativePath(),
       node.getLocation().getStartLine(), node.getLocation().getStartColumn()
