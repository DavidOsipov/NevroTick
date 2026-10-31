from typing import Literal


def wrong(status: Literal["complete", "partial", "missing", "interrupted"]) -> None:
    match status:
        case "complete" | "partial" | "missing":
            return
