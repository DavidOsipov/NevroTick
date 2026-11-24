"""Source-change report for the selected fixture and Slint preview seams."""

from collections.abc import Iterator
from dataclasses import dataclass
import hashlib
import json
from pathlib import Path
import re
from typing import NotRequired, TypedDict

from ui_source_io import read_regular


UI = "crates/nevrotick-ui/src/"
MODEL = UI + "model.rs"
DRAFTS = UI + "source_drafts.rs"
FIXTURE = UI + "presentation.rs"
SLINT = UI + "slint_host/source_workflow.rs"
SCOPE = "UI preview source policy"
CONTEXT = "fixture-and-slint-source"
MARKER = "preview-contract-expect"

# The scanner supports the ordinary tokens in these handwritten carriers.
# Raw strings and token-consuming macros can need new selection logic.
LEXEME = re.compile(
    r'\s+|//[^\n]*|/\*|(?:br|cr|r)\#*"|(?:b|c)?"(?:\\[\s\S]|[^"\\])*"'
    r"|b?'(?:\\[^\n]|[^'\\\n])'|r#\w+|'?\w+|::|->|=>|\.\.=|\.\.|&&|\|\||==|!=|>=|<=|[^\s]"
)
BLOCK = re.compile(r"/\*|\*/")
ALLOW_UNVALIDATED = re.compile(r"\bgarde\s*\([^\]]*\ballow_unvalidated\b[^\]]*\)")
EXPECTATION = re.compile(
    rf"// {MARKER} (CR-0[13]) (\S+) {CONTEXT} ([a-f0-9]{{64}})"
    r" \| rationale=(.+?) \| owner=(.+?) \| consumer=(.+?)"
    r" \| evidence=(.+?) \| recheck=(.+)"
)
SYMBOL = re.compile(r"[A-Za-z_]\w*(?:::[A-Za-z_]\w*)+")
OUTCOME_LINTS = {
    "warnings", "unused", "unused_must_use", "let_underscore", "let_underscore_drop",
    "clippy::all", "clippy::default", "clippy::suspicious", "clippy::restriction",
    "clippy::unused_result_ok", "clippy::let_underscore_must_use", "clippy::let_underscore_future",
    "clippy::let_underscore_drop",
    # Rust 1.100 still applies these levels when the renamed-lint diagnostic is allowed.
    "clippy_all", "clippy_suspicious", "clippy_restriction",
}


@dataclass(frozen=True)
class Token:
    value: str
    offset: int
    end: int


@dataclass(frozen=True)
class Seam:
    path: str
    owner: str
    method: str
    signature: str
    body: str
    review_with: tuple[str, ...]

    @property
    def symbol(self) -> str:
        return f"{self.owner}::{self.method}" if self.owner else self.method

    @property
    def qualified_symbol(self) -> str:
        return self.symbol if self.owner else f"{Path(self.path).stem}::{self.method}"


class Finding(TypedDict):
    rule: str
    path: str
    line: int
    end_line: int
    occurrence: str
    context: str
    guarantee: str
    basis: str
    owner: NotRequired[str]
    review_with: NotRequired[tuple[str, ...]]
    changes: NotRequired[dict[str, tuple[str, str]]]
    binding: NotRequired[str]
    exception: NotRequired[dict[str, str]]


# Digests bind reviewed token sequences at each selected occurrence. Ordinary
# comments and formatting are excluded. Contract docs and literals are retained.
SEAMS = (
    Seam(MODEL, "SourcePreview", "parse",
         "d8cdbf9878bf09aab51ab7077ea266cbd299268d25c8535ed072817a67ac56a4",
         "44f258162894263056914088805e35434b01d8948c6c52997153c03e8307f9dd",
         ("PreviewModel::update_selected", "DraftSource::parse")),
    Seam(MODEL, "PreviewModel", "update_selected",
         "aa6b7aa229e9f1494726ad1c4c3416ba2b4c2a7a83fbf7632153f33a888f3217",
         "0b640d91ab37036f5037976877b57137a3457f247317b143a92751171cd8be61",
         ("SourcePreview::parse", "UiSession::update_preview")),
    Seam(FIXTURE, "UiSession", "update_preview",
         "a137346425ebc803e151b5f65a72150fde6e756ac163802039420956d1ac8d2e",
         "e83a7ba65e5c2690eea233a2c1d3621765cb08ac13efba4aaad40501e2a7cca8",
         ("PreviewModel::update_selected",)),
    Seam(DRAFTS, "DraftSource", "parse",
         "9e7ae0aecfba098717a2d32e8878955bcc7a570575cab2e8b0968469f507f2d7",
         "d08b35b26f0823b8d93632ca62ed90ee5af7cbe8d356c069ec9962ede2dc1930",
         ("SourcePreview::parse", "SourceDrafts::from_baseline", "SourceDrafts::add", "SourceDrafts::edit")),
    Seam(DRAFTS, "SourceDrafts", "from_baseline",
         "5db98856e2e8b9e2afc23185d2c8acda7235b48f785d0368990738043abef877",
         "98ebd4a9b6dd6901207d7f0d1625acaa3b6547d72704242f8d3b08ca16997707",
         ("DraftSource::parse", "SourceDrafts::fixture", "RecoveryPreview::parse")),
    Seam(DRAFTS, "SourceDrafts", "add",
         "fffecee5d3a046aff6b8798e6ddc6f5c60c75749006e877d567a7e687c805105",
         "f17446acf9ca9703326592019780ac79b3652cf43377249054229a3552781d7e",
         ("DraftSource::parse", "source_workflow::update_preview")),
    Seam(DRAFTS, "SourceDrafts", "edit",
         "ec2c4b2eb3bca7e65cf96981362f021a1dd31ca02e3258440cd125b227a67927",
         "f05adc5d9c11e1122a554c95917347b2066d009dae16fd8388cb0d71636e9afe",
         ("DraftSource::parse", "SourceDrafts::replace_change", "source_workflow::update_preview")),
    Seam(DRAFTS, "SourceDrafts", "replace_change",
         "7489d0b2f1cf001d8d34e907ba32b9ad4c2cd89f8f5ec5421bb2442116c97aa9",
         "823f1a490f2e6aaeb5f63425233a445abd86f122c2ceedb5d340b63f46623ccc",
         ("SourceDrafts::edit",)),
    Seam(SLINT, "", "install_source_editor_callbacks",
         "4c4b38442b935c820a610ac6514c2ab1bf56853f7ada7127252fc82f6528bd8e",
         "9a371ed9aad49e9f7868ac428fb7ffb5683e085cf666613def7374efdb47e5a4",
         ("source_workflow::update_preview",)),
    Seam(SLINT, "", "update_preview",
         "15487842b972d829d6787a32f6af1d2df7c305f7152c586652f6b62d6f7bac8f",
         "1174b5f035070eeead2c5e2f304c024ee6d70941e6339b747a73ae2e77d92c2d",
         ("SourceDrafts::add", "SourceDrafts::edit", "SourceProjection::apply")),
)
MODEL_SURFACE = "01d443e5bbb7510ef08f71e0cc43c1a7a2796c40ed8606b761fae298243ea7ec"


def digest(values: list[str]) -> str:
    """Bind whole lexemes, including their order and boundaries."""
    return hashlib.sha256(json.dumps(values, ensure_ascii=False).encode()).hexdigest()


def tokenize(source: str) -> tuple[list[Token], list[Token], str]:
    """Collect selected lexemes and real waiver comments, masking inert braces."""
    tokens: list[Token] = []
    expectations: list[Token] = []
    clean = list(source)
    offset = 0
    while offset < len(source):
        match = LEXEME.match(source, offset)
        assert match is not None
        value = match[0]
        end = match.end()
        is_doc = value.startswith(("//!", "///")) and not value.startswith("////")
        if value == "/*":
            depth = 1
            for delimiter in BLOCK.finditer(source, end):
                depth += 1 if delimiter[0] == "/*" else -1
                if depth == 0:
                    end = delimiter.end()
                    break
            else:
                raise ValueError("Unterminated block comment in selected preview source.")
            value = source[offset:end]
            is_doc = value.startswith(("/*!", "/**")) and not value.startswith("/***")
        if re.match(rf"(?://|/\*)\s*{MARKER}\b", value):
            expectations.append(Token(value, offset, end))
        is_comment = value.startswith(("//", "/*"))
        is_literal = '"' in value or (value.endswith("'") and len(value) > 1)
        if re.match(r'(?:br|cr|r)\#*"', value) or value in {'"', "'"}:
            raise ValueError("Unsupported literal syntax in selected preview source.")
        if is_comment or is_literal:
            clean[offset:end] = ["\n" if ch == "\n" else " " for ch in value]
        if is_doc or (not is_comment and not value.isspace()):
            tokens.append(Token(value, offset, end))
        offset = end
    return tokens, expectations, "".join(clean)


def group_end(tokens: list[Token], start: int) -> int:
    """Select the end of a balanced brace, bracket or parenthesis group."""
    expected = {"(": ")", "[": "]", "{": "}"}
    nesting: list[str] = []
    for index in range(start, len(tokens)):
        value = tokens[index].value
        if value in expected:
            nesting.append(expected[value])
        elif value in expected.values():
            if not nesting or nesting.pop() != value:
                break
            if not nesting:
                return index
    raise ValueError("Unbalanced delimiters in selected preview source.")


def attribute_arguments(tokens: list[Token], start: int, end: int) -> list[tuple[int, int]]:
    """Split a token range at its own commas, leaving nested groups and literals intact."""
    arguments = []
    first = index = start
    while index < end:
        value = tokens[index].value
        if value in {"(", "[", "{"}:
            index = group_end(tokens, index)
        elif value == ",":
            arguments.append((first, index))
            first = index + 1
        index += 1
    if first < end:
        arguments.append((first, end))
    return arguments


def inner_outcome_suppressions(tokens: list[Token]) -> Iterator[tuple[Token, Token, list[str]]]:
    """Find broad inner lint overrides, including inactive conditional branches."""
    values = [token.value.removeprefix("r#") for token in tokens]
    for start in range(len(tokens) - 3):
        if values[start:start + 3] != ["#", "!", "["]:
            continue
        end = group_end(tokens, start + 2)
        pending = [(start + 3, end)]
        while pending:
            first, last = pending.pop()
            if last - first < 3 or values[first + 1] != "(" or values[last - 1] != ")":
                continue
            head = values[first]
            if head not in {"cfg_attr", "allow", "expect", "warn"}:
                continue
            arguments = attribute_arguments(tokens, first + 2, last - 1)
            if head == "cfg_attr":
                pending.extend(reversed(arguments[1:]))
            else:
                lints = {"".join(values[begin:finish]) for begin, finish in arguments}
                # Clippy also applies levels from bare names through its tool namespace.
                matched = sorted(
                    lint for lint in lints
                    if lint in OUTCOME_LINTS or f"clippy::{lint}" in OUTCOME_LINTS
                )
                if matched:
                    yield tokens[start], tokens[end], matched


def select(tokens: list[Token], seam: Seam) -> tuple[int, int, int, list[str]]:
    """Select a direct method or file-scope function, with its attribute/docs prefix."""
    values = [token.value for token in tokens]
    start, end = 0, len(tokens)
    owner_prefix: list[str] = []
    if seam.owner:
        owners = [i for i in range(len(values) - 2)
                  if values[i:i + 3] == ["impl", seam.owner, "{"]]
        if len(owners) != 1 or values[:owners[0]].count("{") != values[:owners[0]].count("}"):
            raise ValueError(f"Expected one file-scope impl for {seam.owner}.")
        # Retain cfg and other outer attributes on the containing impl.
        first = owners[0]
        while first > 0:
            if values[first - 1].startswith(("//!", "///", "/*!", "/**")):
                first -= 1
                continue
            if values[first - 1] != "]":
                break
            depth = 1
            opening = first - 2
            while opening >= 0 and depth:
                depth += int(values[opening] == "]") - int(values[opening] == "[")
                opening -= 1
            if opening < 0 or values[opening] != "#":
                break
            first = opening
        owner_prefix = values[first:owners[0]]
        start = owners[0] + 3
        end = group_end(tokens, start - 1)
    matches = []
    item = start
    index = start
    while index < end:
        value = values[index]
        if values[index:index + 2] == ["fn", seam.method]:
            # Function syntax is deliberately limited to the supported carriers.
            body = index + 2
            while body < end and values[body] not in {"{", ";"}:
                body = (group_end(tokens, body) + 1 if values[body] in {"(", "["}
                        else body + 1)
            if body == end or values[body] != "{":
                raise ValueError(f"Expected a body for {seam.symbol}.")
            matches.append((item, body, group_end(tokens, body) + 1))
        if value in {"(", "[", "{"}:
            index = group_end(tokens, index)
            if value == "{":
                item = index + 1
        elif value == ";":
            item = index + 1
        index += 1
    if len(matches) != 1:
        raise ValueError(f"Expected one direct occurrence of {seam.symbol} in {seam.path}.")
    return (*matches[0], owner_prefix)


def report(root: Path) -> dict:
    """Report drift for the selected source snapshot and exact adjacent closures."""
    if (len(SEAMS) != 10 or len({(seam.path, seam.symbol) for seam in SEAMS}) != 10
            or not re.fullmatch(r"[a-f0-9]{64}", MODEL_SURFACE)
            or any(not seam.review_with or any(SYMBOL.fullmatch(name) is None for name in seam.review_with)
                   for seam in SEAMS)
            or {seam.path for seam in SEAMS} != {MODEL, DRAFTS, FIXTURE, SLINT}):
        raise ValueError("Missing preview contract rule selection or reviewed baseline.")
    raw = {name: read_regular(root / name, 65536) for name in (MODEL, DRAFTS, FIXTURE, SLINT)}
    sources = {name: data.decode("utf-8") for name, data in raw.items()}
    lexed = {name: tokenize(source) for name, source in sources.items()}
    findings: list[Finding] = []

    def finding(rule: str, path: str, offset: int, occurrence: str, guarantee: str,
                through: int | None = None) -> Finding:
        end = offset if through is None else through
        row: Finding = {
            "rule": rule,
            "path": path,
            "line": sources[path].count("\n", 0, offset) + 1,
            "end_line": sources[path].count("\n", 0, end) + 1,
            "occurrence": occurrence,
            "context": CONTEXT,
            "guarantee": guarantee,
            "basis": "selected source syntax",
        }
        findings.append(row)
        return row

    for path, (tokens, _, clean) in lexed.items():
        for attribute in ALLOW_UNVALIDATED.finditer(clean):
            finding("preview-no-allow-unvalidated", path, attribute.start(), "garde attribute",
                    "Require a field rule or an independently justified field exemption.")
        for start, end, lints in inner_outcome_suppressions(tokens):
            finding("preview-inner-outcome-suppression", path, start.offset, ", ".join(lints),
                    "Preserve the selected outcome-handling diagnostics across the preview module.",
                    end.end)

    model, _, clean = lexed[MODEL]
    declarations = list(re.finditer(r"\bstruct\s+PreviewInput\b", clean))
    if len(declarations) != 1:
        raise ValueError("Expected one selected PreviewInput declaration.")
    declaration = declarations[0].start()
    prefix = clean[:declaration].rstrip()
    derive = re.search(r"#\[\s*derive\s*\(\s*Validate\s*\)\s*\]$", prefix)
    if (derive is None or prefix.count("{") != prefix.count("}")
            or prefix[:derive.start()].rstrip().endswith("]")
            or not re.match(r"struct\s+PreviewInput\s*<\s*'a\s*>\s*\{", clean[declaration:])):
        finding("preview-input-admission", MODEL, declaration, "PreviewInput",
                "Keep the borrowed candidate private at file scope with a direct Validate derive.")
    body = re.match(r"[^{}]*\{([^{}]*)\}", clean[declaration:])
    if body is None:
        raise ValueError("Unsupported PreviewInput field declaration.")
    field_rules = "".join(
        rf"#\s*\[\s*garde\s*\(\s*custom\s*\(\s*{field}_rule\s*\)\s*\)\s*\]"
        rf"\s*{field}\s*:\s*&\s*'a\s+str\s*,\s*"
        for field in ("label", "host", "port")
    )
    if re.fullmatch(r"\s*" + field_rules, body[1]) is None:
        finding("preview-input-field-rules", MODEL, declaration, "PreviewInput fields",
                "Keep the explicit private label, host and port rules.")

    model_selected: set[int] = set()
    advisory: dict[tuple[str, str, str], tuple[Finding, int]] = {}
    observed: dict[str, str] = {}
    for seam in SEAMS:
        tokens = lexed[seam.path][0]
        start, body_start, end, owner_prefix = select(tokens, seam)
        if seam.path == MODEL:
            model_selected.update(range(start, end))
        changes: dict[str, tuple[str, str]] = {}
        for part, first, last, baseline, prefix_tokens in (
            ("interface", start, body_start, seam.signature, owner_prefix),
            ("body", body_start, end, seam.body, []),
        ):
            occurrence = f"{seam.symbol}/{part}"
            actual = digest(prefix_tokens + [token.value for token in tokens[first:last]])
            observed[f"{seam.path}:{occurrence}"] = f"{baseline}:{actual}"
            if not re.fullmatch(r"[a-f0-9]{64}", baseline):
                raise ValueError(f"Missing reviewed baseline for {occurrence}.")
            if actual == baseline:
                continue
            if part == "body" and seam.path == MODEL:
                requirement = ("Review length bounds, validation of the same fields, error propagation and publication."
                               if seam.method == "parse"
                               else "Preserve checked construction before modifying the selected slot.")
                row = finding("CR-03", seam.path, tokens[first].offset, occurrence, requirement,
                              tokens[last - 1].end)
                row["changes"] = {part: (baseline, actual)}
                row["owner"] = seam.qualified_symbol
                row["review_with"] = seam.review_with
                advisory[(seam.path, "CR-03", occurrence)] = (row, tokens[first].end)
            else:
                changes[part] = (baseline, actual)
        if changes:
            row = finding("CR-01", seam.path, tokens[start].offset, seam.symbol,
                          "Review the interface, failure behavior and commit with the checked owner and consumer.",
                          tokens[end - 1].end)
            row["changes"] = changes
            row["owner"] = seam.qualified_symbol
            row["review_with"] = seam.review_with
            advisory[(seam.path, "CR-01", seam.symbol)] = (row, tokens[start].offset)

    surface = digest([token.value for index, token in enumerate(model) if index not in model_selected])
    if surface != MODEL_SURFACE:
        row = finding("CR-03", MODEL, 0, "model admission surface",
                      "Review changes to construction and mutation routes, bounds and field-rule implementations.",
                      model[-1].end)
        row["changes"] = {"surface": (MODEL_SURFACE, surface)}
        row["review_with"] = ("SourcePreview::parse", "PreviewModel::update_selected")
    # A closure is local to one occurrence and the selected chain it can affect.
    # Draft/Slint changes do not invalidate a fixture-only closure and vice versa.
    for (path, _, _), (row, _) in advisory.items():
        chain = {MODEL, FIXTURE} if path == FIXTURE else {MODEL, DRAFTS, SLINT}
        if path == MODEL:
            chain.add(FIXTURE)
        binding = [row["rule"], row["occurrence"], row["context"], row["owner"],
                   MODEL_SURFACE, surface, *row["review_with"]]
        binding.extend(f"{key}={value}" for key, value in observed.items()
                       if key.split(":", 1)[0] in chain)
        row["binding"] = digest(binding)

    for path, (_, comments, _) in lexed.items():
        seen: set[tuple[str, str]] = set()
        for comment in comments:
            match = EXPECTATION.fullmatch(comment.value)
            if match is None:
                finding("invalid-expectation", path, comment.offset, MARKER,
                        "Supply a selected rule, exact occurrence, context, binding and review rationale.")
                continue
            rule, occurrence, binding, rationale, owner, consumer, evidence, recheck = match.groups()
            target = advisory.get((path, rule, occurrence))
            attachment = ""
            if target is not None:
                _, anchor = target
                # The interface marker precedes the item, and the body marker
                # immediately follows the opening brace of its selected method.
                attachment = (sources[path][comment.end:anchor] if rule == "CR-01"
                              else sources[path][anchor:comment.offset])
            if (target is None or (rule, occurrence) in seen or target[0]["binding"] != binding
                    or attachment.count("\n") != 1 or attachment.strip()
                    or owner != target[0]["owner"] or consumer not in target[0]["review_with"]
                    or len(evidence.split()) < 3
                    or any(len(value.split()) < 4 for value in (rationale, recheck))):
                finding("invalid-expectation", path, comment.offset, occurrence,
                        "Use a current, unique, adjacent expectation naming the selected owner and related symbol.")
            else:
                target[0]["exception"] = {"rationale": rationale, "owner": owner, "consumer": consumer,
                                          "evidence": evidence, "recheck": recheck}
            seen.add((rule, occurrence))

    for path, data in raw.items():
        if read_regular(root / path, 65536) != data:
            raise ValueError("Selected preview source changed during inspection.")
    return {"scope": SCOPE, "sha256": hashlib.sha256(raw[MODEL]).hexdigest(),
            "source_sha256": {name: hashlib.sha256(data).hexdigest() for name, data in raw.items()},
            "findings": findings,
            "status": "findings" if any("exception" not in row for row in findings) else "passed"}
