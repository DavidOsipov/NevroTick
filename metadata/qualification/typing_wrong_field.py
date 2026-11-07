"""An invented report field must fail at this immediate consumer."""
from metadata.pipeline.generate import CommandReport

def use(report: CommandReport) -> object:
    return report["files_count"]
