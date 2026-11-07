"""A direct consumer cannot mutate an admitted immutable model."""
from metadata.pipeline.contract import Metadata

def mutate(model: Metadata) -> None:
    model.target = "other-target"
