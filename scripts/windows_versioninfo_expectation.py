"""Expected source-mode VERSIONINFO facts for the two linked MSVC images."""

from __future__ import annotations

import json
from pathlib import Path
import sys

def expected(root: Path, role: str) -> dict:
    """Project one fixed component's facts directly from source for readback."""
    from metadata.pipeline.contract import load_metadata

    binaries = {"ui": "nevrotick-ui", "observe": "nevrotick-observe"}
    if role not in binaries:
        raise ValueError("Unsupported version resource role.")
    model = load_metadata(root)
    component = next(item for item in model.components if item.id == role)
    if component.file_version is None or component.version is None or component.binary_name != binaries[role]:
        raise ValueError("Missing selected component version facts.")
    major, minor, patch, revision = component.file_version
    high = major << 16 | minor
    low = patch << 16 | revision
    strings = {"CompanyName": model.product.publisher, "FileDescription": component.description,
               "FileVersion": component.version, "InternalName": component.binary_name,
               "OriginalFilename": component.binary_name + ".exe", "ProductName": model.product.name,
               "ProductVersion": component.version}
    optional = {f"component.{role}.pe.{key}": key for key in ("LegalCopyright", "LegalTrademarks", "Comments")}
    strings.update({optional[field.key]: field.value for field in model.fields
                    if field.key in optional and field.value and field.status in {"populated", "derived"}})
    return {"fixed": [0xFEEF04BD, 0x10000, high, low, high, low, 0x3F, 2, 0x40004, 1, 0, 0, 0],
            "strings": strings, "translation": [1033, 1200]}


if __name__ == "__main__":
    # Import the fixed read-only source snapshot under isolated Python.
    root = Path(__file__).resolve().parents[1]
    sys.path.insert(0, str(root))
    print(json.dumps(expected(root, sys.argv[1]), ensure_ascii=True))
