"""Handling publication alone must not silently omit successful check outcomes."""
from metadata.pipeline.generate import CommandReport

def consume(report: CommandReport) -> int:
    match report["status"]:
        case "generated-source":
            return report["file_count"]
    return 0
