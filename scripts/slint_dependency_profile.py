"""Source-only schema-3 Slint graph admission, shared by the product verifier.

Generator contract: keep schema-2's scope, lockfile_sha256, manifests_sha256,
packages, metadata_only_packages and rows; set schema_version=3 and add
first_party_build_scripts={"crates/nevrotick-ui/build.rs": SHA256}. Selected
package records have name, version, checksum, license, selected_license,
build_script, proc_macro, links (string or null), files_sha256 and
upstream_revision (40 hex digits or null when absent from the reviewed source).
Metadata-only records retain exactly name/version/checksum/license/files_sha256/
role="metadata-only" and confer no execution or licence grant.

Each target row contains features and local_features maps from name@version to
{target: [SORTED_FEATURE_SET, ...], host: [SORTED_FEATURE_SET, ...]}; omit absent
roles, sort/deduplicate all sets and their list. features contains only actually
selected registry packages, local_features all six roots. metadata_features maps
every registry package in the filtered Cargo metadata to its sorted feature list,
including inert metadata extras. allowed_build_scripts and allowed_proc_macros
are exact sorted selected identities (build scripts also include nevrotick-ui).
duplicate_exceptions is the sorted list of all selected identities whose names
have multiple selected versions. No wildcard or subtree exceptions are admitted.
Each row also has package_scopes={PACKAGE: {features, local_features}} for all six
packages. These are exact package selections with normal/build/dev edges; the
workspace row uses normal/build edges. Fixed manifests admit only the exchange
dev edge to time-core, which is already a feature-free workspace root. Runtime
Slint features are six on Windows; the host uses only std, renderer-software and
compat-1-18 for portable model/binding tests, without a desktop backend.

The optional patched_packages field admits dependency-free spin_on 0.1.1 at
third-party/spin_on, optionally followed by the exact Windows-only
accesskit_windows 0.34.0 derivative at third-party/accesskit_windows. Original
archive/file-map/revision and actual derivative/file-change pins are distinct;
no registry checksum is assigned to modified bytes. Spin product contexts remain
host-only, with its standalone target test scope. AccessKit has only the existing
Windows GUI normal target role and no standalone scope. features/metadata_features
include explicit non-first-party identities; local_features still contains the six product
owners. Optional archive_file_bypasses bind literal paths and SHA-256 values.
They grant no execution and do not replace separate build/proc-macro admission.

After source/licence review, run validate_policy_shape(policy), compute
canonical_policy_sha256(policy), and deliberately set REVIEWED_POLICY_SHA256 in
this code. validate_policy always requires that independent anchor; the policy
cannot supply its own expected digest. Shape validation alone never admits code.
Frozen Cargo metadata describes an overapproximation; a separate frozen normal/
build tree must match role-specific selected features before compilation. Only
then selected_metadata may produce cargo-deny's bounded selected projection.
"""
from __future__ import annotations

import copy
import hashlib
import json
import re

# Exact private build/test cohort. Source, role and primary-grant evidence is
# recorded in S0.5; supplemental conveyance obligations remain explicitly open.
REVIEWED_POLICY_SHA256: str | None = "a903246efd5377ce2e21798544f610df90294e72e8bfdffac799dc4decb9f184"
HOST_TARGET = "x86_64-unknown-linux-gnu"
WINDOWS_TARGET = "x86_64-pc-windows-msvc"
TARGETS = (HOST_TARGET, WINDOWS_TARGET)
MEMBERS = ("nevrotick-ui", "ntp-wire", "time-domain", "ntp-exchange", "time-core", "nevrotick-engine")
PATCH_NAME = "spin_on"
PATCH_ID = "spin_on@0.1.1"
PATCH_PATH = "third-party/spin_on"
PATCH_MODIFIED_FILES = {"Cargo.toml", "src/lib.rs", "NEVROTICK-PATCH.md", "tests/stack_pin.rs", "Cargo.lock", ".gitignore"}
PATCH_UPSTREAM = {"name": PATCH_NAME, "version": "0.1.1", "path": PATCH_PATH,
                  "license": "Apache-2.0 OR MIT", "selected_license": "MIT", "build_script": False,
                  "proc_macro": False, "links": None,
                  "upstream_checksum": "076e103ed41b9864aa838287efe5f4e3a7a0362dd00671ae62a212e5e4612da2",
                  "upstream_files_sha256": "a4c380640062932da27aaa676898ba8977e846e0e90b8100a0b1a03e6da82e2c",
                  "upstream_revision": "1605b63506316d89722c650e3fac6fdb643c3ef9"}
PATCH_PRESERVED_FILES = {"Cargo.toml.orig": "bca09dcf4b245bf5c27996374f6324da85b9ffb706a609e60d3407eb0f5e6c19",
                         "LICENSE-MIT": "23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3",
                         "LICENSE-APACHE": "a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2",
                         ".cargo_vcs_info.json": "8eb3c99e9b7fe4cd3bd4757137992ba7bd023d096a3a38b54ba6d9aa7dcd8f4e"}
# Second exact source substitution: Windows GUI-normal dependency only. It adds
# no standalone package scope, feature expansion or Linux compilation grant.
ACCESSKIT_NAME = "accesskit_windows"
ACCESSKIT_ID = "accesskit_windows@0.34.0"
ACCESSKIT_PATH = "third-party/accesskit_windows"
ACCESSKIT_UPSTREAM = {'name': 'accesskit_windows',
 'version': '0.34.0',
 'path': 'third-party/accesskit_windows',
 'license': 'MIT AND BSD-3-Clause AND GPL-3.0-only',
 'selected_license': 'MIT AND BSD-3-Clause AND GPL-3.0-only',
 'build_script': False,
 'proc_macro': False,
 'links': None,
 'upstream_checksum': '106c2b961215864d1c2e703ee63269c25c4e80a577ffb2c1017b9c17dcdf83a1',
 'upstream_files_sha256': '7b5aa35d98dbf1c4ba0d311f591de9486000b84f97190462de4266542d0306e7',
 'upstream_revision': 'c88605b96d04431f9c3c792464a0f2f253480e94'}
ACCESSKIT_MODIFIED_FILES = {'Cargo.toml', 'NEVROTICK-PATCH.md', 'src/lib.rs', 'src/node.rs', 'src/text.rs', 'src/value_input.rs'}
ACCESSKIT_PRESERVED_FILES = {'.cargo_vcs_info.json': '7b556e30d8d54e863f20727b9547512b6955849fc5d6c1819b01301c8a0bd764',
 'CHANGELOG.md': 'a30775fb0f1a725f3e001db5182e8b88a7a39c6b4e4b6c0e07e02cf38432b1e3',
 'Cargo.lock': 'a45d3595200859d8a401238de180c1b2b6f749a7ec8c06ef44c95a5542663bb6',
 'Cargo.toml.orig': '71d45e050aab4e968e634c93e161f7fd3609d8b783374ca07e52e3f4b619bd97',
 'Cargo.toml.upstream': 'baad411ffb33b69d13765a538db985128def773bc93e6722183c34efcd5289b2',
 'LICENSE-APACHE': '62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a',
 'LICENSE-GPL-3.0-only': 'b6a8bf63ae2c95b0aa256ee774680d7bd18f21f457da75466f7610b3b16acc4e',
 'LICENSE-MIT': '23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3',
 'LICENSE.chromium': '845022e0c1db1abb41a6ba4cd3c4b674ec290f3359d9d3c78ae558d4c0ed9308',
 'README.md': '622db8e92d711b64279a91e25ae9345959c82bb4d47157dbf5c3a6a53f80f6d6',
 'examples/hello_world.rs': '5a7dc1b1856e08773367c9c5ec7a857e4ad60afda0cb460852fb0c4573132e4c',
 'src/adapter.rs': '6519a7e59a380c8afadca814fd998d0697e91ad8be94da8e11ac8e5580d47edf',
 'src/context.rs': 'd67198d243aafe1e8940a6642765d66ce17fbb13e23f98d9e09d99d9400c09ae',
 'src/filters.rs': '744374b547065b153123c80028f129f5c6bb53052660205e1a2f231c3e169f64',
 'src/subclass.rs': '69bb742d5451f2cff1c2110e77782948cb751aaad3302c5b1453ebf8b3d20a9e',
 'src/tests/mod.rs': '5ea818811560d2af26cade33f7ec162c5f718ee62b2c9b3dd3dc822677d30409',
 'src/tests/simple.rs': '8bd3eb20e258f2a9d5379671f5c3b4121623a66a95f6acfa9f70412323c87691',
 'src/tests/subclassed.rs': '0987f1d442d1b57657f17fac6a9623ae012603531f47afcb31ff6ca73cb8e932',
 'src/util.rs': 'dab7b1ffeccd6124cfc0dea6ff237e3ac9ba3661c0e23e04a1e433a596490656',
 'src/window_handle.rs': 'dec1d78c81784a18c50f066e89d57d0fe02a7c9199a11e47a0833f8b9113d8c2'}
PATCH_NAME_SETS = ((), (PATCH_NAME,), (PATCH_NAME, ACCESSKIT_NAME))


def patch_names(policy: dict) -> tuple[str, ...]:
    """Select only the closed ordered path-patch cohort; never arbitrary names."""
    patches = policy.get("patched_packages", [])
    if type(patches) is not list or len(patches) > 2 or any(type(item) is not dict for item in patches):
        raise Incomplete("Only the exact Spin and optional Windows AccessKit patches are admitted.")
    names = tuple(item.get("name") for item in patches)
    if names not in PATCH_NAME_SETS:
        raise Incomplete("Only the exact ordered Spin and optional Windows AccessKit patches are admitted.")
    return names


def patch_records(policy: dict) -> dict[str, dict]:
    """Map the already closed cohort by exact identity without changing its role."""
    patch_names(policy)
    return {identity(record): record for record in policy.get("patched_packages", [])}


LOCAL_IDENTITIES = {name + "@0.0.0" for name in MEMBERS}
LOCAL_SCOPE_MEMBERS = {"nevrotick-ui": {"nevrotick-ui"}, "ntp-wire": {"ntp-wire"},
                       "time-domain": {"time-domain"}, "time-core": {"time-core", "time-domain"},
                       "ntp-exchange": {"ntp-exchange", "ntp-wire", "time-domain", "time-core"},
                       "nevrotick-engine": {"nevrotick-engine", "ntp-wire"}}
UI_BUILD = "crates/nevrotick-ui/build.rs"
REGISTRY = "registry+https://github.com/rust-lang/crates.io-index"
MAX_POLICY_BYTES = 2 * 1024 * 1024
MAX_METADATA_BYTES = 8 * 1024 * 1024
MAX_PACKAGES = 768
MAX_TREE_BYTES = 1024 * 1024
MAX_TREE_LINES = 16384
SLINT_FEATURES = ["accessibility", "backend-winit", "compat-1-18", "renderer-software", "std", "unstable-winit-030"]
HOST_SLINT_FEATURES = ["compat-1-18", "renderer-software", "std"]
GRANTS = {"MIT", "Apache-2.0", "BSD-2-Clause", "BSD-3-Clause", "ISC", "Zlib", "BSL-1.0",
          "Unicode-3.0", "CC0-1.0", "MIT AND Unicode-3.0", "Apache-2.0 AND MIT", "GPL-3.0-only"}
FORBIDDEN_SELECTED = {"ring", "aws-lc-rs", "aws-lc-sys", "aws-lc-fips-sys", "skia-bindings",
                      "i-slint-renderer-skia", "i-slint-renderer-femtovg", "i-slint-backend-qt",
                      "wgpu", "wgpu-core", "wgpu-hal", "clang-sys", "bindgen"}
NAME = r"[A-Za-z0-9_-]{1,128}"
VERSION = r"(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)(?:\+[0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*)?"


class Incomplete(ValueError):
    """An unadmitted graph, source-only observation or policy failed closed."""


def canonical_policy_sha256(policy: dict) -> str:
    """Hash the complete policy using canonical UTF-8 JSON, without a self-pin."""
    try:
        payload = json.dumps(policy, sort_keys=True, separators=(",", ":"),
                             ensure_ascii=False, allow_nan=False).encode("utf-8")
    except (TypeError, ValueError, UnicodeError, RecursionError) as error:
        raise Incomplete("Policy cannot be canonically encoded.") from error
    if len(payload) > MAX_POLICY_BYTES:
        raise Incomplete("Dependency policy exceeds its canonical byte bound.")
    return hashlib.sha256(payload).hexdigest()


def identity(record: dict) -> str:
    """Return a validated exact name@version identity without name-only merging."""
    name, version = record.get("name"), record.get("version")
    if (type(name) is not str or not re.fullmatch(NAME, name) or type(version) is not str
            or len(version) > 128 or not re.fullmatch(VERSION, version)):
        raise Incomplete("Invalid exact dependency identity.")
    return name + "@" + version


def _hash(value) -> bool:
    """Recognize a canonical SHA-256 string, never a caller-selected digest."""
    return type(value) is str and re.fullmatch(r"[0-9a-f]{64}", value) is not None


def _strings(values, *, maximum=256) -> bool:
    """Require a bounded canonical set of simple nonempty strings."""
    return (type(values) is list and len(values) <= maximum
            and all(type(item) is str and 0 < len(item) <= 128 for item in values)
            and values == sorted(set(values)))


def _features(values) -> bool:
    """Admit sorted feature names, including Cargo's valid dependency features."""
    return _strings(values) and all(re.fullmatch(r"[A-Za-z0-9_+./?-]+", item) for item in values)


def _roles(value) -> bool:
    """Require nonempty target/host contexts, each preserving distinct feature sets."""
    return (type(value) is dict and bool(value) and set(value) <= {"target", "host"}
            and all(type(sets) is list and 0 < len(sets) <= 32
                    and all(_features(features) for features in sets)
                    and sets == sorted(sets) and len({tuple(features) for features in sets}) == len(sets)
                    for sets in value.values()))


def duplicate_identities(features: dict) -> list[str]:
    """Select exact versions requiring duplicate exceptions, never whole subtrees."""
    names = [item.split("@", 1)[0] for item in features]
    return sorted(item for item in features if names.count(item.split("@", 1)[0]) > 1)


def package_scopes(policy: dict) -> tuple[str, ...]:
    """Keep six product scopes and Spin's standalone scope; AccessKit has none."""
    return (*MEMBERS, PATCH_NAME) if PATCH_NAME in patch_names(policy) else MEMBERS


def selected_records(policy: dict) -> dict[str, dict]:
    """Keep reviewed registry and patched source roles distinct under exact IDs."""
    return {identity(record): record for record in policy["packages"] + policy.get("patched_packages", [])}


def validate_path_patch(patch: dict) -> None:
    """Bind each of the two exact derivatives to separate immutable provenance."""
    if type(patch) is not dict:
        raise Incomplete("Path patch provenance or exact modified-file inventory changed.")
    if patch.get("name") == PATCH_NAME:
        upstream, modified = PATCH_UPSTREAM, PATCH_MODIFIED_FILES
    elif patch.get("name") == ACCESSKIT_NAME:
        upstream, modified = ACCESSKIT_UPSTREAM, ACCESSKIT_MODIFIED_FILES
    else:
        raise Incomplete("Unknown local path-patch identity.")
    if (set(patch) != set(upstream) | {"files_sha256", "modified_files_sha256"}
            or any(type(patch.get(key)) is not type(value) or patch[key] != value for key, value in upstream.items())
            or not _hash(patch["files_sha256"]) or type(patch["modified_files_sha256"]) is not dict
            or set(patch["modified_files_sha256"]) != modified
            or not all(_hash(value) for value in patch["modified_files_sha256"].values())):
        raise Incomplete("Path patch provenance or exact modified-file inventory changed.")


def validate_policy_shape(policy: dict) -> None:
    """Check closed bounded schema and role consistency; this grants no admission."""
    required = {"schema_version", "scope", "lockfile_sha256", "manifests_sha256", "packages",
                "metadata_only_packages", "rows", "first_party_build_scripts"}
    optional = {"patched_packages", "archive_file_bypasses"}
    if (type(policy) is not dict or not required <= set(policy) or set(policy) - required - optional
            or type(policy["schema_version"]) is not int
            or policy["schema_version"] != 3 or type(policy["scope"]) is not str
            or not 0 < len(policy["scope"]) <= 8192 or not _hash(policy["lockfile_sha256"])):
        raise Incomplete("Unadmitted schema-3 dependency policy fields.")
    manifests = policy["manifests_sha256"]
    expected = {"Cargo.toml", *("crates/" + name + "/Cargo.toml" for name in MEMBERS)}
    if (type(manifests) is not dict or set(manifests) != expected
            or not all(_hash(value) for value in manifests.values())
            or type(policy["first_party_build_scripts"]) is not dict
            or set(policy["first_party_build_scripts"]) != {UI_BUILD}
            or not _hash(policy["first_party_build_scripts"][UI_BUILD])):
        raise Incomplete("Manifest or first-party build-script identity differs from the profile.")
    selected, inert = {}, {}
    base_keys = {"name", "version", "checksum", "license", "files_sha256"}
    for field, destination, extra in (
        ("packages", selected, {"selected_license", "build_script", "proc_macro", "links", "upstream_revision"}),
        ("metadata_only_packages", inert, {"role"}),
    ):
        records = policy[field]
        if type(records) is not list or not 0 < len(records) <= MAX_PACKAGES:
            raise Incomplete("Dependency package inventory is empty or exceeds its bound.")
        for record in records:
            if type(record) is not dict or set(record) != base_keys | extra:
                raise Incomplete("Unadmitted dependency fields or metadata-only role promotion.")
            key = identity(record)
            if key in selected or key in inert:
                raise Incomplete("Duplicate exact dependency identity or role promotion.")
            if (not _hash(record["checksum"]) or not _hash(record["files_sha256"])
                    or type(record["license"]) is not str or not 0 < len(record["license"]) <= 256):
                raise Incomplete("Dependency source or declared licence identity is malformed.")
            if field == "metadata_only_packages":
                if record["role"] != "metadata-only":
                    raise Incomplete("Unadmitted metadata-only dependency role.")
            else:
                revision = record["upstream_revision"]
                grant = record["selected_license"]
                if (type(grant) is not str or grant not in GRANTS
                        or type(record["build_script"]) is not bool or type(record["proc_macro"]) is not bool
                        or not (record["links"] is None or type(record["links"]) is str
                                and 0 < len(record["links"]) <= 128)
                        or not (revision is None or type(revision) is str and re.fullmatch(r"[0-9a-f]{40}", revision))):
                    raise Incomplete("Unreviewed selected licence or dependency execution role.")
                name = record["name"]
                if name in FORBIDDEN_SELECTED or (grant == "GPL-3.0-only" and not (
                    (name in {"slint", "slint-build", "slint-macros"} or name.startswith("i-slint-"))
                    and record["version"] == "1.18.1"
                )):
                    raise Incomplete("A held provider or unselected renderer/framework grant was promoted.")
            destination[key] = record
        if [identity(record) for record in records] != sorted(destination):
            raise Incomplete("Dependency records must use canonical exact-identity order.")
    if len(selected) + len(inert) > MAX_PACKAGES:
        raise Incomplete("Combined dependency inventory exceeds its bound.")
    patches = policy.get("patched_packages", [])
    names = patch_names(policy)
    patched_ids = set()
    for patch in patches:
        validate_path_patch(patch)
        key = identity(patch)
        if key in selected or key in inert:
            raise Incomplete("Path patch cannot also retain a registry or metadata-only source role.")
        patched_ids.add(key)
        selected[key] = patch
    if PATCH_NAME in names and any(key.split("@", 1)[0] == "pin-utils" for key in selected):
        raise Incomplete("The dependency-free Spin patch cannot restore pin-utils.")
    bypasses = policy.get("archive_file_bypasses", [])
    if type(bypasses) is not list or len(bypasses) > 64:
        raise Incomplete("Archive-file bypass inventory exceeds its bound.")
    seen_bypasses = set()
    for bypass in bypasses:
        if (type(bypass) is not dict or set(bypass) != {"package", "path", "sha256", "reason"}
                or type(bypass["package"]) is not str or bypass["package"] not in selected
                or bypass["package"] in patched_ids or type(bypass["path"]) is not str
                or not 0 < len(bypass["path"]) <= 256
                or not re.fullmatch(r"[A-Za-z0-9_.-]+(?:/[A-Za-z0-9_.-]+)*", bypass["path"])
                or any(part in {".", ".."} for part in bypass["path"].split("/"))
                or not _hash(bypass["sha256"]) or type(bypass["reason"]) is not str
                or not 0 < len(bypass["reason"]) <= 2048):
            raise Incomplete("Unreviewed archive path, bytes or blanket execution bypass.")
        key = (bypass["package"], bypass["path"])
        if key in seen_bypasses:
            raise Incomplete("Duplicate exact archive-file bypass.")
        seen_bypasses.add(key)
    rows = policy["rows"]
    if type(rows) is not dict or set(rows) != set(TARGETS):
        raise Incomplete("Missing or unadmitted target/host dependency row.")
    reached = set()
    for target, row in rows.items():
        keys = {"features", "local_features", "metadata_features", "allowed_build_scripts",
                "allowed_proc_macros", "duplicate_exceptions", "package_scopes"}
        if type(row) is not dict or set(row) != keys:
            raise Incomplete("Unadmitted dependency row fields.")
        features, locals_, metadata = row["features"], row["local_features"], row["metadata_features"]
        if (type(features) is not dict or not set(features) <= set(selected)
                or not features or not all(_roles(value) for value in features.values())
                or type(locals_) is not dict or set(locals_) != LOCAL_IDENTITIES
                or any(value != {"target": [[]]} for value in locals_.values())
                or type(metadata) is not dict or not set(features) <= set(metadata)
                or not set(metadata) <= selected.keys() | inert.keys()
                or not all(_features(value) for value in metadata.values())):
            raise Incomplete("Selected target/host features or inert metadata features are malformed.")
        if any(not {feature for sets in roles.values() for values in sets for feature in values}
                   <= set(metadata[key]) for key, roles in features.items()):
            raise Incomplete("Selected features cannot exceed their reviewed metadata overapproximation.")
        runtime = SLINT_FEATURES if target == WINDOWS_TARGET else HOST_SLINT_FEATURES
        if (features.get("slint@1.18.1") != {"target": [runtime]}
                or features.get("slint-build@1.18.1") != {"host": [["compat-1-18"]]}):
            raise Incomplete("Slint runtime/build features differ from the reviewed software profile.")
        scripts = sorted([key for key in features if selected[key]["build_script"]] + ["nevrotick-ui@0.0.0"])
        macros = sorted(key for key in features if selected[key]["proc_macro"])
        if (row["allowed_build_scripts"] != scripts or row["allowed_proc_macros"] != macros
                or row["duplicate_exceptions"] != duplicate_identities(features)
                or any(set(features[key]) != {"host"} for key in macros)):
            raise Incomplete("Build/proc-macro/duplicate roles differ from the selected graph.")
        scopes = row["package_scopes"]
        if PATCH_NAME in names and features.get(PATCH_ID) != {"host": [[]]}:
            raise Incomplete("The spin_on patch may only enter the product's host-tool graph.")
        if ACCESSKIT_NAME in names:
            if target == WINDOWS_TARGET:
                if features.get(ACCESSKIT_ID) != {"target": [[]]} or metadata.get(ACCESSKIT_ID) != []:
                    raise Incomplete("The AccessKit patch is only an empty-feature Windows GUI target dependency.")
            elif ACCESSKIT_ID in features or ACCESSKIT_ID in metadata:
                raise Incomplete("The Windows AccessKit patch has no Linux selected or metadata role.")
        if type(scopes) is not dict or set(scopes) != set(package_scopes(policy)):
            raise Incomplete("Every package selection needs its own reviewed feature contexts.")
        for name, scope in scopes.items():
            if name == PATCH_NAME:
                if scope != {"features": {PATCH_ID: {"target": [[]]}}, "local_features": {}}:
                    raise Incomplete("The standalone patched package scope must remain dependency/feature-free.")
                continue
            local = {member + "@0.0.0": {"target": [[]]} for member in LOCAL_SCOPE_MEMBERS[name]}
            if (type(scope) is not dict or set(scope) != {"features", "local_features"}
                    or scope["local_features"] != local or type(scope["features"]) is not dict
                    or not set(scope["features"]) <= set(features)
                    or not all(_roles(value) for value in scope["features"].values())):
                raise Incomplete("Package scope has unreviewed roots, roles or selected sources.")
            for key, roles in scope["features"].items():
                for role, values in roles.items():
                    admitted = {feature for feature_set in features[key].get(role, []) for feature in feature_set}
                    if (role not in features[key] or any(not set(value) <= admitted for value in values)
                            or selected[key]["proc_macro"] and role != "host"):
                        raise Incomplete("Package scope exceeds the reviewed workspace role/features.")
            if name == "nevrotick-ui":
                if (scope["features"].get("slint@1.18.1") != {"target": [runtime]}
                        or scope["features"].get("slint-build@1.18.1") != {"host": [["compat-1-18"]]}):
                    raise Incomplete("GUI package scope lacks its exact runtime/build profile.")
                if ACCESSKIT_NAME in names and target == WINDOWS_TARGET and scope["features"].get(ACCESSKIT_ID) != {"target": [[]]}:
                    raise Incomplete("Windows GUI scope lacks its exact AccessKit target dependency.")
            elif any(key.split("@", 1)[0].startswith(("slint", "i-slint-")) for key in scope["features"]):
                raise Incomplete("Slint dependencies must remain isolated from pure/engine package selections.")
            elif patched_ids & set(scope["features"]):
                raise Incomplete("Path patches belong only to their exact GUI host/target graph roles.")
        reached.update(features)
    if reached != set(selected):
        raise Incomplete("Selected package inventory contains an unreachable execution grant.")
    canonical_policy_sha256(policy)


def validate_policy(policy: dict) -> None:
    """Require the complete code-owned policy digest after structural validation."""
    validate_policy_shape(policy)
    if (not _hash(REVIEWED_POLICY_SHA256)
            or canonical_policy_sha256(policy) != REVIEWED_POLICY_SHA256):
        raise Incomplete("Schema-3 Slint dependency policy lacks its reviewed code-owned pin.")


def _decode_metadata(output: str) -> dict:
    """Decode bounded Cargo JSON without duplicate fields or non-JSON numbers."""
    if type(output) is not str or len(output.encode("utf-8")) > MAX_METADATA_BYTES:
        raise Incomplete("Cargo metadata exceeds its schema-3 byte bound.")

    def unique(pairs):
        result = {}
        for key, value in pairs:
            if key in result:
                raise Incomplete("Duplicate Cargo metadata field.")
            result[key] = value
        return result

    def invalid(_value):
        raise Incomplete("Non-JSON number in Cargo metadata.")

    try:
        data = json.loads(output, object_pairs_hook=unique, parse_constant=invalid)
    except (ValueError, UnicodeError, RecursionError) as error:
        raise Incomplete("Invalid Cargo metadata JSON.") from error
    if (type(data) is not dict or type(data.get("packages")) is not list
            or not 0 < len(data["packages"]) <= MAX_PACKAGES + len(MEMBERS)
            or type(data.get("resolve")) is not dict
            or type(data["resolve"].get("nodes")) is not list
            or len(data["resolve"]["nodes"]) != len(data["packages"])):
        raise Incomplete("Cargo metadata lacks its bounded complete package/node graph.")
    return data


def validate_metadata(target: str, output: str, policy: dict) -> dict:
    """Validate the frozen metadata overapproximation without selecting its extras."""
    validate_policy(policy)
    if target not in TARGETS:
        raise Incomplete("Unadmitted metadata target.")
    data, row = _decode_metadata(output), policy["rows"][target]
    records = {identity(item): item for item in policy["packages"] + policy["metadata_only_packages"] + policy.get("patched_packages", [])}
    selected = selected_records(policy)
    patches = patch_records(policy)
    nodes = {}
    for node in data["resolve"]["nodes"]:
        if (type(node) is not dict or type(node.get("id")) is not str or node["id"] in nodes
                or not _features(node.get("features"))):
            raise Incomplete("Cargo metadata has duplicate nodes or malformed features.")
        nodes[node["id"]] = node
    registry, locals_, ids = {}, {}, set()
    for package in data["packages"]:
        if type(package) is not dict or type(package.get("id")) is not str:
            raise Incomplete("Cargo metadata package identity is malformed.")
        package_id, key = package["id"], identity(package)
        if package_id in ids or package_id not in nodes or "source" not in package:
            raise Incomplete("Cargo metadata package/node identities differ.")
        ids.add(package_id)
        if package["source"] is None and key in patches:
            workspace_root = data.get("workspace_root")
            if (type(workspace_root) is not str or not workspace_root.startswith("/")
                    or package.get("manifest_path") != workspace_root.rstrip("/") + "/" + patches[key]["path"] + "/Cargo.toml"
                    or key in registry or package.get("license") != patches[key]["license"]):
                raise Incomplete("The patched package is outside its exact source/licence path.")
            registry[key] = nodes[package_id]["features"]
        elif package["source"] is None:
            if (key not in LOCAL_IDENTITIES or key in locals_
                    or package.get("license") != ("GPL-3.0-only" if key == "nevrotick-ui@0.0.0" else "GPL-2.0-only")):
                raise Incomplete("Unreviewed local owner or first-party licence in Cargo metadata.")
            locals_[key] = nodes[package_id]["features"]
        else:
            if (package["source"] != REGISTRY or key not in records or key in registry or key in patches
                    or package.get("license") != records[key]["license"]):
                raise Incomplete("Unadmitted Cargo metadata source, licence or exact identity.")
            registry[key] = nodes[package_id]["features"]
        if key in row["features"]:
            targets = package.get("targets")
            if (type(targets) is not list or any(type(item) is not dict
                                                or type(item.get("kind")) is not list for item in targets)):
                raise Incomplete("Cargo metadata lacks selected dependency execution targets.")
            build = any("custom-build" in item["kind"] for item in targets)
            macro = any("proc-macro" in item["kind"] for item in targets)
            if (build != selected[key]["build_script"] or macro != selected[key]["proc_macro"]
                    or package.get("links") != selected[key]["links"]):
                raise Incomplete("Selected Cargo build/proc-macro/native role drifted.")
    if (ids != set(nodes) or registry != row["metadata_features"]
            or locals_ != {key: [] for key in LOCAL_IDENTITIES}
            or type(data.get("workspace_members")) is not list
            or not all(type(value) is str for value in data["workspace_members"])
            or len(set(data["workspace_members"])) != len(MEMBERS)
            or set(data["workspace_members"]) != {package["id"] for package in data["packages"]
                                                 if package["source"] is None and identity(package) in LOCAL_IDENTITIES}):
        raise Incomplete("Actual Cargo metadata differs from its reviewed overapproximation.")
    for node in nodes.values():
        if (type(node.get("dependencies")) is not list
                or any(type(item) is not str or item not in ids for item in node["dependencies"])
                or type(node.get("deps")) is not list
                or any(type(item) is not dict or type(item.get("pkg")) is not str
                       or item["pkg"] not in ids for item in node["deps"])):
            raise Incomplete("Cargo metadata contains a dangling or malformed dependency edge.")
    return {"target": target, "metadata_packages": sorted(registry),
            "selected_packages": sorted(row["features"]),
            "inert_metadata_extras": sorted(set(registry) - set(row["features"]))}


def validate_tree(target: str, output: str, policy: dict, *, scope: str = "workspace",
                  include_edges: bool = False) -> dict:
    """Check frozen Cargo's selected role/feature tree, rejecting ambiguous dedupes."""
    validate_policy(policy)
    if (target not in TARGETS or type(output) is not str or len(output.encode("utf-8")) > MAX_TREE_BYTES
            or type(include_edges) is not bool):
        raise Incomplete("Unadmitted or oversized selected Cargo tree.")
    full_row = policy["rows"][target]
    if scope != "workspace" and scope not in package_scopes(policy):
        raise Incomplete("Unknown Cargo package selection scope.")
    row = full_row if scope == "workspace" else full_row["package_scopes"][scope]
    expected = {**row["features"], **row["local_features"]}
    expected_roots = LOCAL_IDENTITIES if scope == "workspace" else {PATCH_ID if scope == PATCH_NAME else scope + "@0.0.0"}
    selected = selected_records(policy)
    patches = patch_records(policy)
    stack, builds, templates, roots, occurrences = {}, {}, {}, set(), set()
    pending_build = None
    lines = output.splitlines()
    if not lines or len(lines) > MAX_TREE_LINES:
        raise Incomplete("Selected Cargo tree is empty or exceeds its line bound.")
    for line in lines:
        if not line:
            continue
        marker = re.fullmatch(r"((?:\|   |    )*)\[(build|dev)-dependencies\]", line)
        if marker:
            depth = len(marker[1]) // 4
            section = marker[2]
            if (depth not in stack or stack[depth]["duplicate"] or pending_build is not None
                    or section in stack[depth]["sections"]):
                raise Incomplete("Invalid Cargo tree dependency-section marker.")
            stack[depth]["sections"].add(section)
            builds[depth], pending_build = section, depth
            continue
        match = re.fullmatch(r"((?:(?:\|   |    )*(?:\|-- |`-- ))?)(" + NAME + r") v(" + VERSION +
                             r")([^|]*)\|([^\r\n]*?)( \(\*\))?", line)
        if match is None:
            raise Incomplete("Malformed selected Cargo tree line.")
        prefix, name, version, annotation, feature_text, duplicate = match.groups()
        depth, key = len(prefix) // 4, name + "@" + version
        if depth > 128 or key not in expected:
            raise Incomplete("Cargo tree promoted an unselected dependency or depth.")
        if key in LOCAL_IDENTITIES or key in patches:
            path = patches[key]["path"] if key in patches else "crates/" + name
            if not re.fullmatch(r" \(/[^()\r\n]*/" + re.escape(path) + r"\)", annotation):
                raise Incomplete("Unexpected local Cargo tree source annotation.")
            macro = False
        else:
            macro = selected[key]["proc_macro"]
            if annotation != (" (proc-macro)" if macro else ""):
                raise Incomplete("Cargo tree source/proc-macro annotation differs from admission.")
        features = feature_text.split(",") if feature_text else []
        if not _features(features):
            raise Incomplete("Cargo tree features are malformed or duplicated.")
        if pending_build is not None and depth != pending_build + 1:
            raise Incomplete("Cargo tree dependency section has no direct dependency.")
        pending_build = None
        if depth and (depth - 1 not in stack or stack[depth - 1]["duplicate"]):
            raise Incomplete("Cargo tree has an incomplete or deduplicated parent.")
        for previous in list(stack):
            if previous >= depth:
                del stack[previous]
        for previous in list(builds):
            if previous >= depth:
                del builds[previous]
        kind = builds.get(depth - 1, "normal") if depth else "normal"
        role = "host" if macro or depth and (stack[depth - 1]["role"] == "host" or kind == "build") else "target"
        if key == ACCESSKIT_ID and key in patches and (role != "target" or kind != "normal" or not depth):
            raise Incomplete("The patched Windows AccessKit occurrence must retain its normal target edge.")
        context = (key, tuple(features), role)
        occurrence = {"context": context, "role": role, "children": [], "duplicate": bool(duplicate), "sections": set()}
        if depth:
            stack[depth - 1]["children"].append((key, tuple(features), kind))
        else:
            if role != "target" or key not in expected_roots or context in roots:
                raise Incomplete("Unexpected or repeated Cargo workspace root.")
            roots.add(context)
        if not duplicate:
            templates.setdefault(context, []).append(occurrence["children"])
        stack[depth] = occurrence
        occurrences.add(context)
    if pending_build is not None or {item[0] for item in roots} != expected_roots:
        raise Incomplete("Selected Cargo tree is missing workspace roots or dependencies.")
    for context, children in templates.items():
        if any(value != children[0] for value in children[1:]):
            raise Incomplete("Conflicting full Cargo tree templates for the same role/features.")
    reached, edges, pending = set(), set(), list(roots)
    while pending:
        context = pending.pop()
        if context in reached:
            continue
        if context not in templates:
            raise Incomplete("Deduplicated Cargo tree has no full template for the same host/target role.")
        reached.add(context)
        for key, features, kind in templates[context][0]:
            macro = selected[key]["proc_macro"] if key in selected else False
            role = "host" if context[2] == "host" or kind == "build" or macro else "target"
            child = (key, features, role)
            edges.add((context, child, kind))
            pending.append(child)
    actual = {}
    for key, features, role in reached:
        actual.setdefault(key, {}).setdefault(role, []).append(list(features))
    actual = {key: {role: sorted(values) for role, values in roles.items()} for key, roles in actual.items()}
    if reached != occurrences or actual != expected:
        raise Incomplete("Selected Cargo tree target/host features differ from the reviewed execution graph.")
    result = {"target": target, "scope": scope, "selected_packages": sorted(row["features"]),
              "role_feature_contexts": len(reached), "inert_metadata_extras_selected": 0}
    if include_edges:
        result["dependency_edges"] = [
            {"parent": parent[0], "parent_features": list(parent[1]), "parent_role": parent[2],
             "child": child[0], "child_features": list(child[1]), "child_role": child[2], "kind": kind}
            for parent, child, kind in sorted(edges)
        ]
    return result


def _alias(value: str) -> str:
    """Use Cargo's resolved dependency spelling for manifest aliases."""
    if type(value) is not str or not re.fullmatch(NAME, value):
        raise Incomplete("Malformed selected dependency alias.")
    return value.replace("-", "_")


def _dependency_matches(declaration: dict, edge: dict, kind: dict, child: dict) -> bool:
    """Match a resolved alias/kind/target to its unchanged Cargo declaration."""
    declared_kind = declaration.get("kind") or "normal"
    resolved_kind = kind.get("kind") or "normal"
    if declared_kind != resolved_kind or declaration.get("target") != kind.get("target"):
        return False
    rename = declaration.get("rename")
    return (_alias(rename or declaration.get("name")) == _alias(edge.get("name"))
            or rename is None and declaration.get("name") == child["name"])


def _project_feature_refs(definitions: dict, active: set[str], dependencies: dict[str, set[str]]) -> dict:
    """Retain only observed local/dependency feature arcs in the scanner view.

    Full declarations remain in the archived Cargo metadata and admitted source.
    Inactive weak references must not address a same-named dependency selected
    elsewhere: krates 0.21.2 requires every feature arc to exist in its node list.
    """
    projected = {}
    for feature in sorted(active):
        refs = definitions.get(feature)
        if type(refs) is not list or any(type(ref) is not str or len(ref) > 256 for ref in refs):
            raise Incomplete("Selected feature lacks its bounded Cargo definition.")
        kept = []
        for ref in refs:
            if ref.startswith("dep:"):
                present = _alias(ref[4:]) in dependencies
            elif "/" in ref:
                alias, requested = ref.split("/", 1)
                alias = alias.removesuffix("?")
                present = requested in dependencies.get(_alias(alias), set())
            else:
                present = ref in active
            if present:
                kept.append(ref)
        projected[feature] = kept
    return projected


def selected_metadata(target: str, metadata_output: str, tree_output: str, policy: dict,
                      *, package_trees: dict[str, str] | None = None) -> dict:
    """Build a closed scanner view from the six product and optional Spin edge graphs.

    This scratch projection preserves package identity, licences and all source
    provenance. It prunes inactive packages, edges and feature arcs/declaration
    requests, rather than pretending resolver-overapproximation features execute.
    Node features stay exactly the reviewed workspace union. Source manifests,
    grants, compiler inputs and archived observations are never rewritten.
    """
    validate_metadata(target, metadata_output, policy)
    scopes = package_scopes(policy)
    if type(package_trees) is not dict or set(package_trees) != set(scopes):
        raise Incomplete("Selected projection requires all six product and the admitted standalone Spin package-tree observations.")
    graphs = [validate_tree(target, tree_output, policy, include_edges=True)]
    graphs.extend(validate_tree(target, package_trees[name], policy, scope=name, include_edges=True)
                  for name in scopes)
    selected_edges = {(edge["parent"], edge["child"], edge["kind"])
                      for graph in graphs for edge in graph["dependency_edges"]}
    data = copy.deepcopy(_decode_metadata(metadata_output))
    row = policy["rows"][target]
    wanted = set(row["features"]) | LOCAL_IDENTITIES
    data["packages"] = [package for package in data["packages"] if identity(package) in wanted]
    ids = {package["id"] for package in data["packages"]}
    packages = {package["id"]: package for package in data["packages"]}
    identities = {package["id"]: identity(package) for package in data["packages"]}
    data["resolve"]["nodes"] = [node for node in data["resolve"]["nodes"] if node["id"] in ids]
    nodes = {node["id"]: node for node in data["resolve"]["nodes"]}
    for node in data["resolve"]["nodes"]:
        roles = row["features"].get(identities[node["id"]], {"target": [[]]})
        node["features"] = sorted({feature for sets in roles.values() for features in sets for feature in features})
    observed_edges = set()
    for node in data["resolve"]["nodes"]:
        package = packages[node["id"]]
        declarations, definitions = package.get("dependencies"), package.get("features")
        if type(declarations) is not list or type(definitions) is not dict:
            raise Incomplete("Selected package lacks its Cargo dependency/feature declarations.")
        active = set(node["features"])
        optional_aliases = {_alias(declaration.get("rename") or declaration.get("name"))
                            for declaration in declarations if declaration.get("optional")
                            and (declaration.get("rename") or declaration.get("name")) in active}
        for feature in active:
            refs = definitions.get(feature)
            if type(refs) is not list or any(type(ref) is not str for ref in refs):
                raise Incomplete("Selected feature lacks its Cargo definition.")
            for ref in refs:
                if ref.startswith("dep:"):
                    optional_aliases.add(_alias(ref[4:]))
                elif "/" in ref and "?/" not in ref:
                    optional_aliases.add(_alias(ref.split("/", 1)[0]))
        retained, resolved = {}, []
        for edge in node["deps"]:
            if edge["pkg"] not in ids:
                continue
            child = packages[edge["pkg"]]
            kinds = []
            if type(edge.get("dep_kinds")) is not list:
                raise Incomplete("Selected dependency lacks its Cargo kinds.")
            for kind in edge["dep_kinds"]:
                if type(kind) is not dict or kind.get("kind") not in (None, "normal", "build", "dev"):
                    raise Incomplete("Selected dependency has an unreviewed kind.")
                key = (identity(package), identity(child), kind.get("kind") or "normal")
                if key not in selected_edges:
                    continue
                matches = []
                for index, declaration in enumerate(declarations):
                    if not _dependency_matches(declaration, edge, kind, child):
                        continue
                    alias = _alias(declaration.get("rename") or declaration.get("name"))
                    if declaration.get("optional") and alias not in optional_aliases:
                        continue
                    matches.append(index)
                if len(matches) != 1:
                    raise Incomplete("Observed selected edge lacks one exact Cargo declaration.")
                retained.setdefault(matches[0], set()).add(edge["pkg"])
                observed_edges.add(key)
                kinds.append(kind)
            if kinds:
                edge["dep_kinds"] = kinds
                resolved.append(edge)
        node["deps"] = resolved
        node["dependencies"] = sorted({edge["pkg"] for edge in resolved})
        package["dependencies"] = []
        for index, children in sorted(retained.items()):
            declaration = declarations[index]
            requested = declaration.get("features")
            if (type(requested) is not list or any(type(feature) is not str for feature in requested)
                    or type(declaration.get("uses_default_features")) is not bool):
                raise Incomplete("Selected Cargo dependency feature request is malformed.")
            enabled = set.intersection(*(set(nodes[child]["features"]) for child in children))
            declaration["features"] = [feature for feature in requested if feature in enabled]
            declaration["uses_default_features"] &= "default" in enabled
            package["dependencies"].append(declaration)
        dependency_features = {}
        for edge in resolved:
            child = packages[edge["pkg"]]
            enabled = set(nodes[edge["pkg"]]["features"])
            for alias in (edge["name"], child["name"]):
                dependency_features.setdefault(_alias(alias), set()).update(enabled)
        package["features"] = _project_feature_refs(definitions, active, dependency_features)
    if observed_edges != selected_edges:
        raise Incomplete("Selected scanner projection lost an observed dependency edge.")
    return data
