from core_combination_accounting import (
    LifecyclePartialObservation,
    ObservedLifecycleAttempts,
    complete_lifecycle_attempts,
)


def wrong(partial: LifecyclePartialObservation) -> ObservedLifecycleAttempts:
    return complete_lifecycle_attempts(partial)
