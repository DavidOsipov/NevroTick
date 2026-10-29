#!/usr/bin/env python3
"""Check this pack's metadata, discovery files, links and skill inventories.

Uses only Python's standard library. Pack metadata deliberately uses a small YAML
subset: unquoted keys and JSON-quoted string values. This is a package check,
not a Rust program test or a claim that an agent will select the correct skill.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
from collections import Counter
from pathlib import Path
from urllib.parse import unquote, urlsplit


def read_metadata(path: Path) -> dict[str, str]:
    text = path.read_text(encoding="utf-8")
    match = re.match(r"\A---\n(.*?)\n---(?:\n|$)", text, re.S)
    if not match:
        raise ValueError("missing YAML frontmatter")
    fields: dict[str, str] = {}
    for line in match[1].splitlines():
        if not line.strip():
            continue
        key, sep, value = line.partition(":")
        if not sep or key not in {"name", "description", "license"} or key in fields:
            raise ValueError(f"unsupported or duplicate metadata key: {key!r}")
        parsed = json.loads(value.strip())
        if not isinstance(parsed, str) or not parsed.strip():
            raise ValueError(f"{key} must be a nonempty quoted string")
        fields[key] = parsed
    if not {"name", "description"} <= fields.keys():
        raise ValueError("name and description are required")
    return fields


def upstream_targets(root: Path, errors: list[str]) -> list[str]:
    """Check pinned imported leaves without changing the website donor inventory."""
    manifest = root / "UPSTREAM.json"
    if not manifest.exists():
        return []
    targets: list[str] = []
    try:
        entries = json.loads(manifest.read_text(encoding="utf-8"))["skills"]
        if not isinstance(entries, list) or not entries:
            raise ValueError("upstream inventory must contain skill entries")
        for entry in entries:
            name = entry["name"]
            target = entry["target"]
            if not isinstance(name, str) or not re.fullmatch(r"[a-z0-9]+(?:-[a-z0-9]+)*", name):
                raise ValueError("invalid upstream skill name")
            if target != f"upstream/{name}/SKILL.md":
                raise ValueError(f"invalid upstream skill target: {target}")
            if target in targets:
                raise ValueError(f"duplicate upstream skill target: {target}")
            targets.append(target)
            if not re.fullmatch(r"[a-f0-9]{40}", entry["commit"]):
                raise ValueError(f"invalid upstream commit for {name}")
            if not re.fullmatch(r"[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+", entry["repository"]):
                raise ValueError(f"invalid upstream repository for {name}")
            source = Path(entry["source_path"])
            if source.is_absolute() or ".." in source.parts or not source.parts:
                raise ValueError(f"invalid upstream source path for {name}")
            sources = entry["source_files"]
            if not isinstance(sources, dict) or f"{source}/SKILL.md" not in sources:
                raise ValueError(f"missing upstream source inventory for {name}")
            for item, digest in sources.items():
                if Path(item).is_absolute() or ".." in Path(item).parts or not Path(item).parts:
                    raise ValueError(f"invalid upstream source file path: {item}")
                if not re.fullmatch(r"[a-f0-9]{64}", digest):
                    raise ValueError(f"invalid upstream source digest: {item}")
            leaf = (root / target).parent.resolve()
            if not leaf.is_relative_to(root.resolve()):
                raise ValueError(f"upstream leaf escapes skill root: {name}")
            license_file = Path(entry["license_file"])
            if license_file.is_absolute() or ".." in license_file.parts or not (root / license_file).resolve().is_relative_to(leaf):
                raise ValueError(f"invalid upstream license file for {name}")
            if not (root / license_file).is_file():
                raise ValueError(f"missing upstream license file for {name}")
            if not isinstance(entry["license"], str) or not entry["license"].strip():
                raise ValueError(f"missing upstream license identifier for {name}")
            if read_metadata(root / target).get("license") != entry["license"]:
                raise ValueError(f"upstream license metadata mismatch for {name}")
            files = entry["files"]
            if not isinstance(files, dict) or target not in files or str(license_file) not in files:
                raise ValueError(f"incomplete upstream file inventory for {name}")
            for item, digest in files.items():
                path = Path(item)
                if path.is_absolute() or ".." in path.parts or not (root / path).resolve().is_relative_to(leaf):
                    raise ValueError(f"invalid upstream file path: {item}")
                if not re.fullmatch(r"[a-f0-9]{64}", digest):
                    raise ValueError(f"invalid upstream installed digest: {item}")
                if hashlib.sha256((root / path).read_bytes()).hexdigest() != digest:
                    errors.append(f"upstream file changed: {item}")
            actual = {str(p.relative_to(root)) for p in leaf.rglob("*") if p.is_file()}
            if actual != set(files):
                errors.append(f"upstream file inventory mismatch for {name}")
    except (ValueError, OSError, KeyError, TypeError) as exc:
        errors.append(f"UPSTREAM.json: {exc}")
    return targets


def local_targets(root: Path, errors: list[str]) -> list[str]:
    """Register original leaves without inventing a donor or imported hashes."""
    manifest = root / "LOCAL.json"
    if not manifest.exists():
        return []
    targets: list[str] = []
    try:
        inventory = json.loads(manifest.read_text(encoding="utf-8"))
        if type(inventory["schema_version"]) is not int or inventory["schema_version"] != 1:
            raise ValueError("unsupported local inventory schema")
        entries = inventory["skills"]
        if not isinstance(entries, list) or not entries:
            raise ValueError("local inventory must contain skill entries")
        for entry in entries:
            if not isinstance(entry, dict) or set(entry) != {"name", "target", "origin"}:
                raise ValueError("unsupported local inventory fields")
            name, target = entry["name"], entry["target"]
            if (not isinstance(name, str) or len(name) > 64
                    or not re.fullmatch(r"[a-z0-9]+(?:-[a-z0-9]+)*", name)):
                raise ValueError("invalid local skill name")
            if (not isinstance(target, str)
                    or not re.fullmatch(r"(?:environment|implementation|planning|security|testing)/"
                                        + re.escape(name) + r"/SKILL\.md", target)):
                raise ValueError(f"invalid local skill target: {target}")
            if entry["origin"] != "locally-authored":
                raise ValueError(f"invalid local skill origin: {name}")
            if target in targets:
                raise ValueError(f"duplicate local skill target: {target}")
            targets.append(target)
            if not (root / target).resolve().is_relative_to(root.resolve()):
                raise ValueError(f"local skill target escapes root: {target}")
    except (ValueError, OSError, KeyError, TypeError) as exc:
        errors.append(f"LOCAL.json: {exc}")
    return targets


def prose(text: str) -> str:
    """Remove fenced examples before checking prose links."""
    result = []
    active = None
    for line in text.splitlines():
        fence = re.match(r"^\s*(`{3,}|~{3,})(.*)$", line)
        if fence:
            marker = fence[1]
            if active is None:
                active = marker
            elif marker[0] == active[0] and len(marker) >= len(active) and not fence[2].strip():
                active = None
            result.append("")
        else:
            result.append(line if active is None else "")
    return "\n".join(result)


def anchors(path: Path) -> set[str]:
    found: set[str] = set()
    counts: Counter[str] = Counter()
    for line in prose(path.read_text(encoding="utf-8")).splitlines():
        heading = re.match(r"^#{1,6}\s+(.+?)(?:\s+#+)?$", line)
        if heading:
            label = re.sub(r"\[([^]]+)\]\([^)]*\)", r"\1", heading[1])
            slug = re.sub(r"[^\w\- ]", "", label.lower()).replace(" ", "-")
            found.add(slug if not counts[slug] else f"{slug}-{counts[slug]}")
            counts[slug] += 1
    return found


def check(root: Path, source_root: Path | None = None) -> tuple[list[str], int]:
    errors: list[str] = []
    repo = root.parent.parent.resolve()
    skills = sorted(root.rglob("SKILL.md"))
    names: set[str] = set()
    for path in skills:
        label = str(path.relative_to(root))
        try:
            data = read_metadata(path)
            name = data["name"]
            if not re.fullmatch(r"[a-z0-9]+(?:-[a-z0-9]+)*", name) or len(name) > 64:
                errors.append(f"{label}: invalid name {name!r}")
            if name != path.parent.name:
                errors.append(f"{label}: name differs from directory")
            if name in names:
                errors.append(f"{label}: duplicate skill name {name!r}")
            names.add(name)
            if len(data["description"]) > 1024 or any(c in data["description"] for c in "<>"):
                errors.append(f"{label}: invalid description")
            ui = path.parent / "agents/openai.yaml"
            lines = ui.read_text(encoding="utf-8").splitlines()
            if not lines or lines[0] != "interface:":
                raise ValueError("UI metadata needs interface mapping")
            values = {}
            for line in lines[1:]:
                match = re.fullmatch(r"  ([a-z_]+): (.+)", line)
                if not match or match[1] in values:
                    raise ValueError("invalid or duplicate UI metadata field")
                values[match[1]] = json.loads(match[2])
            if set(values) != {"display_name", "short_description", "default_prompt"}:
                raise ValueError("UI metadata fields are incomplete or unsupported")
            if any(not isinstance(v, str) or not v.strip() for v in values.values()):
                raise ValueError("UI metadata values must be nonempty strings")
            if not 25 <= len(values["short_description"]) <= 64:
                errors.append(f"{label}: UI short description must be 25-64 characters")
            invocations = re.findall(r"\$([\w-]+)", values["default_prompt"])
            if name not in invocations:
                errors.append(f"{label}: UI prompt does not invoke ${name}")
        except (ValueError, OSError) as exc:
            errors.append(f"{label}: {exc}")

    link_pattern = re.compile(r"!?\[[^\]\n]*\]\((<[^>\n]+>|[^\s)]+)(?:\s+\"[^\"]*\")?\)")
    for path in sorted(root.rglob("*.md")):
        for match in link_pattern.finditer(prose(path.read_text(encoding="utf-8"))):
            href = match[1].strip("<>")
            parsed = urlsplit(href)
            if parsed.scheme or parsed.netloc:
                continue
            target = (path.parent / unquote(parsed.path)).resolve() if parsed.path else path
            if not target.is_relative_to(repo):
                errors.append(f"{path.relative_to(root)}: link escapes repository: {href}")
            elif not target.exists():
                errors.append(f"{path.relative_to(root)}: missing link target: {href}")
            elif parsed.fragment and target.suffix == ".md" and unquote(parsed.fragment) not in anchors(target):
                errors.append(f"{path.relative_to(root)}: missing heading: {href}")

    try:
        inventory = json.loads((root / "ADAPTATION.json").read_text(encoding="utf-8"))
        entries = inventory["skills"]
        if not isinstance(entries, list) or not entries:
            raise ValueError("adaptation inventory must contain skill entries")
        sources, targets = [], []
        for entry in entries:
            source, target = entry["source"], entry["target"]
            sources.append(source)
            targets.append(target)
            if not re.fullmatch(r"[a-f0-9]{64}", entry["source_sha256"]):
                raise ValueError(f"invalid source digest for {source}")
            for item in (source, target):
                if Path(item).is_absolute() or ".." in Path(item).parts or not item.endswith("/SKILL.md"):
                    raise ValueError(f"invalid inventory path: {item}")
            if source_root is not None:
                digest = hashlib.sha256((source_root / source).read_bytes()).hexdigest()
                if digest != entry["source_sha256"]:
                    errors.append(f"source changed since adaptation: {source}")
        if len(sources) != len(set(sources)) or len(targets) != len(set(targets)):
            errors.append("duplicate source or target in adaptation inventory")
        imported = upstream_targets(root, errors)
        authored = local_targets(root, errors)
        if set(targets) & set(imported):
            errors.append("adaptation and upstream skill targets overlap")
        if set(authored) & (set(targets) | set(imported)):
            errors.append("local and adapted/upstream skill targets overlap")
        expected = set(targets) | set(imported) | set(authored)
        actual = {str(p.relative_to(root)) for p in skills}
        if actual != expected:
            errors.append(f"inventory mismatch: unlisted={sorted(actual-expected)}; missing={sorted(expected-actual)}")
        if source_root is not None:
            source_skills = {str(p.relative_to(source_root)) for p in source_root.rglob("SKILL.md")}
            if source_skills != set(sources):
                errors.append("source inventory differs from the recorded adaptation")
    except (ValueError, OSError, KeyError, TypeError) as exc:
        errors.append(f"ADAPTATION.json: {exc}")
    return errors, len(skills)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument("--source-root", type=Path, help="optionally verify the original website .agents tree")
    args = parser.parse_args()
    errors, count = check(args.root.resolve(), args.source_root)
    for error in errors:
        print(f"FAIL: {error}")
    if errors:
        print(f"Failed with {len(errors)} error(s) across {count} skills.")
        return 1
    print(f"PASS: {count} skills; metadata, UI prompts, local links and skill inventories.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
