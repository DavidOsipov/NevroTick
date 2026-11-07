"""The checked actual generator cannot implement an incompatible caller contract."""
from pathlib import Path
from typing import Protocol
from metadata.pipeline.generate import CommandReport, generate

class IncompatiblePublisher(Protocol):
    def __call__(self, root: Path, output: int, source: Path | None = None, *,
                 stage: str = "source") -> CommandReport: ...

publisher: IncompatiblePublisher = generate
