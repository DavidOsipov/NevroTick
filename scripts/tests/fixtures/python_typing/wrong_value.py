from core_combination_accounting import PublicCalls


def wrong(calls: PublicCalls) -> None:
    calls["terms"] = "one"
