/**
 * @name UI raw input reaches its allocation receiver
 * @description Inspect value propagation across replace and parse without modeling validation as a barrier.
 * @kind path-problem
 * @id rust/nevrotick/product-ui-raw-allocation
 * @problem.severity recommendation
 * @precision high
 */
import ProductFlowModel
module ProductFlow = DataFlow::Global<ProductConfig>;
import ProductFlow::PathGraph
from ProductFlow::PathNode source, ProductFlow::PathNode sink
where ProductFlow::flowPath(source, sink)
select sink.getNode(), source, sink,
       "Raw UI text reaches the allocation receiver through replace and parse; inspect validation separately."
