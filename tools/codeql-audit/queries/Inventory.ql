/** AST-only function inventory, including retained source/test/example files. */
import rust
from Function f, string file
where
  file = f.getLocation().getFile().getRelativePath() and
  (file.matches("crates/%") or file.matches("third-party/%"))
select f.getCanonicalPath(), file, f.getLocation().getStartLine(),
       f.getLocation().getStartColumn(), f.getNumberOfParamsInclSelf()
