from core_combination_accounting import ObservedAttempts, PartialObservation, complete_attempts


def wrong(partial: PartialObservation) -> ObservedAttempts:
    return complete_attempts(partial)
