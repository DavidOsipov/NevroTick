"""Legal immediate consumer of the actual metadata implementation interfaces."""
from pathlib import Path
from typing import Protocol
from metadata.pipeline.contract import Metadata, load_metadata
from metadata.pipeline.generate import CommandReport, FieldCounts, check, field_counts, generate


class Publisher(Protocol):
    def __call__(self, root: Path, output: Path, source: Path | None = None, *,
                 stage: str = "source") -> CommandReport: ...


def count_result(report: CommandReport) -> int:
    match report["status"]:
        case "generated-source":
            return report["file_count"]
        case "checked-source":
            return report["file_count"]


def use_actual_interfaces(root: Path, output: Path) -> tuple[Metadata, FieldCounts, int]:
    publisher: Publisher = generate
    observer: Publisher = check
    model = load_metadata(root)
    counts = field_counts(model)
    return model, counts, count_result(publisher(root, output)) + count_result(observer(root, output))
