from typing import Literal, TypedDict, assert_never

from core_combination_accounting import CombinationObservation


class InterruptedObservation(TypedDict):
    combination_observation_status: Literal["interrupted"]


def wrong(record: CombinationObservation | InterruptedObservation) -> None:
    if record["combination_observation_status"] == "complete-source-recipe":
        return
    if record["combination_observation_status"] == "partial-source-recipe":
        return
    if record["combination_observation_status"] == "missing":
        return
    assert_never(record)
