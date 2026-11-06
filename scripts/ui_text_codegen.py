"""Generate protected UI identifiers/types into a new private review candidate.

Input is the fixed code-owned contract, never a locale catalog. Expected SHA-256
binds retained source bytes; it establishes no truthful wording or human approval.
The generator writes no checkout, baseline, locale CBOR or approval ledger.
"""
from __future__ import annotations

import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import stat
import tempfile

import protected_json
from ui_text_bindings import binding_definition_errors

CONTRACT_PATH = Path('crates/nevrotick-ui/src/localization/contract-source.json')
TEMPLATE_PATH = Path('scripts/ui-text-templates/projection-header.rs.in')
TEMPLATE_SHA256 = 'e2b669a3efc7b8c62d90afddd43e79af785b813e08b5316fd8f553aced3ad817'
MAX_CONTRACT_BYTES = 1_000_000
MAX_TEMPLATE_BYTES = 65_536
MAX_OUTPUT_BYTES = 4_194_304
MAX_OUTPUT_FILE_BYTES = 1_048_576


class Invalid(ValueError):
    """Fixed bounded engineering rejection; catalog or source data is not echoed."""


def _read_regular(path: Path, maximum: int) -> bytes:
    """Read one retained finite regular file; reject symlinks and changed custody."""
    before = path.lstat()
    if not stat.S_ISREG(before.st_mode) or before.st_size > maximum:
        raise Invalid('A fixed generator input is not a bounded regular file.')
    flags = os.O_RDONLY | getattr(os, 'O_NOFOLLOW', 0)
    descriptor = os.open(path, flags)
    with os.fdopen(descriptor, 'rb') as stream:
        opened = os.fstat(stream.fileno())
        if (opened.st_dev, opened.st_ino) != (before.st_dev, before.st_ino):
            raise Invalid('Generator input custody changed.')
        raw = stream.read(maximum + 1)
        after = os.fstat(stream.fileno())
    if (len(raw) > maximum or (opened.st_dev, opened.st_ino, opened.st_size,
        opened.st_mtime_ns, opened.st_ctime_ns) != (after.st_dev, after.st_ino,
        after.st_size, after.st_mtime_ns, after.st_ctime_ns)):
        raise Invalid('Generator input exceeded its bound or changed during reading.')
    return raw


def _fixed_input(root: Path, relative: Path, maximum: int) -> bytes:
    """Admit only a fixed nonsymlink path beneath the selected repository root."""
    current = root
    for part in relative.parts:
        current = current / part
        if current.is_symlink():
            raise Invalid('Generator input paths cannot contain symbolic links.')
    return _read_regular(current, maximum)


def _check_bindings(data: dict) -> None:
    """Reuse the single closed binding/action profile owner before code emission."""
    if binding_definition_errors(data['messages']):
        raise Invalid('Protected Slint binding/action profiles rejected.')


def _output_bounds(files: dict[str, bytes]) -> None:
    """Permit only the fixed generated basenames and finite encoded output."""
    names = {'contract.rs', 'catalog_projection.rs', 'localization-copy.slint',
             'contract-fingerprint-input.bin', 'registry-tombstones.json', 'output-slot-map.json', 'identity.json'}
    names.update(namespace + '-copy.slint' for namespace in
        ('fixture', 'preferences', 'source', 'recovery', 'shell', 'policy', 'inspection', 'diagnostic', 'raw'))
    if set(files) != names or any(type(value) is not bytes for value in files.values()):
        raise Invalid('Generated output identity set rejected.')
    if any(len(value) > MAX_OUTPUT_FILE_BYTES for value in files.values()) or sum(map(len, files.values())) > MAX_OUTPUT_BYTES:
        raise Invalid('Generated output exceeded its fixed byte budget.')


def generate(root: Path, expected_sha256: str, output_parent: Path | None = None) -> Path:
    """Prepare a new private directory; never integrate generated source.

    Rejects malformed/changed fixed inputs before creating output. Parent output
    must be outside the repository; fixed files are exclusively created with mode
    0600 under a new 0700 directory. Failure returns no apparent complete result.
    """
    root = Path(root).resolve(strict=True)
    raw = _fixed_input(root, CONTRACT_PATH, MAX_CONTRACT_BYTES)
    template = _fixed_input(root, TEMPLATE_PATH, MAX_TEMPLATE_BYTES)
    files = _emit_files(raw, expected_sha256, template)
    _output_bounds(files)
    parent = Path(output_parent if output_parent is not None else tempfile.gettempdir())
    if parent.is_symlink():
        raise Invalid('Output parent cannot be a symbolic link.')
    parent = parent.resolve(strict=True)
    if not parent.is_dir() or parent.is_relative_to(root):
        raise Invalid('Generated review output must be outside the repository.')
    output = Path(tempfile.mkdtemp(prefix='nevrotick-ui-text-', dir=parent))
    try:
        for name, value in sorted(files.items()):
            descriptor = os.open(output / name, os.O_WRONLY | os.O_CREAT | os.O_EXCL | getattr(os, 'O_NOFOLLOW', 0), 0o600)
            with os.fdopen(descriptor, 'wb') as stream:
                stream.write(value)
        return output
    except BaseException:
        shutil.rmtree(output)
        raise


_FEEDBACK_SLOTS = (('EditorError', 'editor-error'), ('EditorFeedback', 'editor-feedback'), ('IntervalError', 'interval-error'), ('IntervalFeedback', 'interval-feedback'), ('PreferenceError', 'preference-error'), ('PreferenceFeedback', 'preference-feedback'), ('RecoveryError', 'recovery-error'), ('RecoveryFeedback', 'recovery-feedback'), ('RecoveryBackLabel', 'recovery-back-label'), ('QuitHeadline', 'quit-headline'), ('QuitConsequence', 'quit-consequence'), ('ShellFeedback', 'shell-feedback'), ('ServiceFeedback', 'service-feedback'), ('NotificationFeedback', 'notification-feedback'), ('DiagnosticFeedback', 'diagnostic-feedback'), ('PolicyError', 'policy-error'), ('SourceQueryError', 'source-query-error'), ('AcceptedIntervalLabel', 'accepted-interval-label'), ('StatusFeedback', 'status-feedback'), ('TrayFeedback', 'tray-feedback'), ('StatusHeadline', 'status-headline'), ('StatusConsequence', 'status-consequence'), ('StatusProfileLabel', 'status-profile-label'), ('StatusProfile', 'status-profile'), ('StatusAuthentication', 'status-authentication'), ('StatusAgreement', 'status-agreement'), ('StatusFreshness', 'status-freshness'), ('StatusCheckLabel', 'status-check-label'), ('StatusRemedyLabel', 'status-remedy-label'))

def _core_emit(data, feedback_slots, projection_header, expected_sha256):
    """Render only checked identifiers/types; English references remain inert fingerprint data."""
    outputs = {}
    if set(data) != {'schema_version', 'domain', 'contract_revision', 'plural_profile', 'locale', 'unicode_profile', 'catalog_schema_version', 'gui_build_version', 'text_contract_version', 'status', 'bounds', 'unicode_policy_sha256', 'action_contract', 'messages'}:
        raise Invalid('closed protected contract fields')
    if any((type(data[key]) is not int for key in ['schema_version', 'catalog_schema_version', 'gui_build_version', 'text_contract_version'])):
        raise Invalid('closed integer versions')
    if data['catalog_schema_version'] != 2 or not 1 <= data['gui_build_version'] <= 4294967295 or (not 1 <= data['text_contract_version'] <= 4294967295):
        raise Invalid('closed GUI/text/catalog versions')
    if data['schema_version'] != 1 or data['domain'] != 'nevrotick.ui' or data['locale'] != 'en' or (data['plural_profile'] != 'english-u64-cardinal-v1'):
        raise Invalid('closed protected profile')
    NAMES = {'fixture': 'FixtureCopy', 'preferences': 'PreferencesCopy', 'source': 'SourceCopy', 'recovery': 'RecoveryCopy', 'shell': 'ShellCopy', 'policy': 'PolicyCopy', 'inspection': 'InspectionCopy', 'diagnostic': 'DiagnosticCopy', 'raw': 'RawCopy'}
    GROUPS = ['PreviewFacts', 'SourceEdit', 'Intervals', 'Lifecycle', 'Notifications', 'Policy', 'Notices', 'Recovery', 'Quit']
    expected_bounds = {'JSON_BYTES': 131072, 'CBOR_BYTES': 65536, 'STRING_BYTES': 1024, 'STRING_SCALARS': 512, 'KEY_BYTES': 96, 'DECODED_BYTES': 65536, 'MAX_DEPTH': 4, 'MAX_FIELDS': 2048, 'MAX_NODES': 4096, 'MAX_MAP': 512, 'MAX_WORK': 4000000, 'OUTPUT_BYTES': 4096, 'OUTPUT_UTF16': 2048, 'PROJECTION_BYTES': 131072, 'PROJECTION_UTF16': 65536}
    if not (data['bounds'] == expected_bounds and type(data['bounds']['MAX_MAP']) is int and data['bounds']['MAX_MAP'] == 512):
        raise Invalid('Protected contract shape rejected.')
    if not (type(data['unicode_profile']) is str and data['unicode_profile'] == 'unicode-15.1-english-printable-ascii-middle-dot-en-em-dash-v1'):
        raise Invalid('Protected contract shape rejected.')
    if not re.fullmatch('[a-f0-9]{64}', data['unicode_policy_sha256']):
        raise Invalid('Protected contract shape rejected.')
    if not type(data['action_contract']) is dict:
        raise Invalid('Protected contract shape rejected.')
    allrows = data['messages']
    if not (type(allrows) is list and 1 <= len(allrows) <= 768):
        raise Invalid('Protected contract shape rejected.')
    seenkeys = set()
    seensymbols = set()
    rows = []
    for i, row in enumerate(allrows, 1):
        if not set(row) <= {'key', 'ordinal', 'kind', 'group', 'meaning_revision', 'meaning_context', 'active', 'symbol', 'reference', 'sources', 'slint', 'retirement', 'reservation'}:
            raise Invalid('Protected contract shape rejected.')
        if not (type(row['ordinal']) is int and row['ordinal'] == i and (type(row['active']) is bool) and (type(row['meaning_revision']) is int) and (row['meaning_revision'] >= 1)):
            raise Invalid('Protected contract shape rejected.')
        key = row['key']
        symbol = row['symbol']
        if not (re.fullmatch('ui\\.[a-z][a-z0-9-]*(?:\\.[a-z][a-z0-9-]*)+', key) and len(key) <= 96):
            raise Invalid('Protected contract shape rejected.')
        if not (re.fullmatch('[A-Z][A-Z0-9_]*', symbol) and len(symbol) <= 120):
            raise Invalid('Protected contract shape rejected.')
        if not (key not in seenkeys and symbol not in seensymbols):
            raise Invalid('Protected contract shape rejected.')
        seenkeys.add(key)
        seensymbols.add(symbol)
        if not (row['kind'] in {'Text', 'Minutes'} and row['group'] in [None, *GROUPS]):
            raise Invalid('Protected contract shape rejected.')
        ref = row['reference']
        if not type(ref) is dict:
            raise Invalid('Protected contract shape rejected.')
        if row['kind'] == 'Text':
            if not (set(ref) == {'text'} and type(ref['text']) is str):
                raise Invalid('Protected contract shape rejected.')
        elif not (set(ref) == {'forms'} and set(ref['forms']) == {'one', 'other'} and all((type(t) is str for t in ref['forms'].values()))):
            raise Invalid('Protected contract shape rejected.')
        slint = row['slint']
        if slint is not None:
            if not set(slint) == {'global', 'namespace', 'property', 'slot'}:
                raise Invalid('Protected contract shape rejected.')
            if not NAMES.get(slint['namespace']) == slint['global']:
                raise Invalid('Protected contract shape rejected.')
            if not (re.fullmatch('[a-z][a-z0-9-]*', slint['property']) and len(slint['property']) <= 80):
                raise Invalid('Protected contract shape rejected.')
            if not (slint['slot'] is None or (type(slint['slot']) is int and 0 <= slint['slot'] < 5)):
                raise Invalid('Protected contract shape rejected.')
        if row['active']:
            rows.append(row)
    if not len(rows) <= data['bounds']['MAX_MAP']:
        raise Invalid('Protected contract shape rejected.')
    _check_bindings(data)
    semantic = {key: value for key, value in data.items() if key not in {'schema_version', 'gui_build_version', 'status', 'contract_revision', 'messages'}}
    semantic['messages'] = sorted([{key: row[key] for key in ['key', 'kind', 'group', 'meaning_revision', 'meaning_context', 'active', 'reference']} if row['active'] else {'key': row['key'], 'active': False, 'status': 'reserved' if 'reservation' in row else 'retired'} for row in allrows], key=lambda row: row['key'])
    fingerprint_input = b'NevroTick UI protected semantic text-contract v2\x00' + json.dumps(semantic, ensure_ascii=False, sort_keys=True, separators=(',', ':')).encode()
    fp = hashlib.sha256(fingerprint_input).hexdigest()
    outputs['contract-fingerprint-input.bin'] = fingerprint_input
    header = '//! Generated only from a protected source contract; never from catalog data.\n'
    rust = header + 'use super::Error;\n\n/// Code-owned coherent critical-copy group.\n#[derive(Clone, Copy, Debug, Eq, PartialEq)]\npub enum Group {\n'
    for g in GROUPS:
        rust += f'    /// Protected {g.lower()} meaning/action group.\n    {g},\n'
    rust += "}\n\n/// Independently selected argument and plural representation.\n#[derive(Clone, Copy, Debug, Eq, PartialEq)]\npub enum Kind {\n    /// Argument-free inert text.\n    Text,\n    /// Exactly one checked positive whole-minute argument in both English forms.\n    Minutes,\n}\n\n/// Private dense storage identity, not a wire ordinal or action.\n#[derive(Clone, Copy, Debug, Eq, PartialEq)]\npub struct MessageId(usize);\n\n/// Protected active definition; stable ordinals are never reused.\n#[derive(Clone, Copy, Debug)]\npub struct Spec {\n    /// Private checked storage identity.\n    pub id: MessageId,\n    /// Exact stable ASCII wire key.\n    pub key: &'static str,\n    /// Original append-only identity ordinal, including retired slots.\n    pub ordinal: u16,\n    /// Exact selected argument representation.\n    pub kind: Kind,\n    /// Coherent protected critical group, if any.\n    pub group: Option<Group>,\n}\n"
    rust += f'/// Compatibility fingerprint; grants no semantic approval.\npub const FINGERPRINT: &str = "{fp}";\n'
    rust += '/// Fixed product catalog domain, never a path.\npub const DOMAIN: &str = "nevrotick.ui";\n/// Fixed English cardinal rule and checked integer domain.\npub const PLURAL: &str = "english-u64-cardinal-v1";\n'
    rust += '/// Exact selected JSON/CBOR file representation version.\npub const CATALOG_SCHEMA_VERSION: u32 = 2;\n/// Code-owned engineering GUI build version, distinct from file schema.\npub const GUI_BUILD_VERSION: u32 = ' + str(data['gui_build_version']) + ';\n/// Code-owned text meaning/type contract version, distinct from build/schema.\npub const TEXT_CONTRACT_VERSION: u32 = ' + str(data['text_contract_version']) + ';\n'
    rust += f'/// Number of active messages, not registry ordinals.\npub const COUNT: usize = {len(rows)};\n'
    rust += "impl MessageId {\n    /// Resolve one exact active ID without aliases or normalization.\n    ///\n    /// # Errors\n    /// Rejects unknown and retired identities with a fixed schema error.\n    pub fn from_key(key: &str) -> Result<Self, Error> {\n        SPECS.iter().find(|spec| spec.key == key).map(|spec| spec.id).ok_or(Error::Schema)\n    }\n    /// Return the private dense slot, never the stable external ordinal.\n    #[must_use]\n    pub const fn index(self) -> usize { self.0 }\n    /// Borrow the independently protected definition.\n    ///\n    /// # Errors\n    /// Rejects an impossible absent storage identity instead of indexing unchecked.\n    pub fn spec(self) -> Result<&'static Spec, Error> { SPECS.get(self.0).ok_or(Error::Schema) }\n"
    for i, row in enumerate(rows):
        rust += f"    /// {row['key']}; stable ordinal {row['ordinal']}.\n    pub const {row['symbol']}: Self = Self({i});\n"
    rust += '}\n\n/// Active protected definitions in private dense storage order.\npub const SPECS: &[Spec] = &[\n'
    for row in rows:
        group = 'None' if row['group'] is None else 'Some(Group::' + row['group'] + ')'
        rust += f'''    Spec {{ id: MessageId::{row['symbol']}, key: "{row['key']}", ordinal: {row['ordinal']}, kind: Kind::{row['kind']}, group: {group} }},\n'''
    rust += '];\n'
    outputs['contract.rs'] = rust.encode('utf-8')
    outputs['registry-tombstones.json'] = (json.dumps([{'key': r['key'], 'ordinal': r['ordinal'], 'status': 'reserved' if 'reservation' in r else 'retired', 'reason': r.get('reservation', r.get('retirement', 'retired protected definition'))} for r in allrows if not r['active']], indent=2) + '\n').encode('utf-8')
    fields = {ns: {} for ns in NAMES}
    for row in rows:
        slint = row['slint']
        if slint:
            ns, prop, slot = (slint['namespace'], slint['property'], slint['slot'])
            if slot is None:
                fields[ns][prop] = [row]
            else:
                fields[ns].setdefault(prop, []).append(row)
    fields['preferences']['interval-options'] = 'preset-intervals'
    fields['diagnostic']['durations'] = 'example-durations'
    if not (len(feedback_slots) == 29 and len(set((row[0] for row in feedback_slots))) == 29):
        raise Invalid('Protected contract shape rejected.')
    for variant, field in feedback_slots:
        if not (re.fullmatch('[A-Z][A-Za-z0-9]*', variant) and re.fullmatch('[a-z][a-z0-9-]*', field)):
            raise Invalid('Protected contract shape rejected.')
    slot_fields = {}
    static_entries = []
    for ns, props in fields.items():
        slot_fields[ns] = {}
        for prop, value in props.items():
            if value == 'preset-intervals':
                entries = [('text', 'PREFERENCES_INTERVAL_OPTION_AUTOMATIC', None), ('minutes', 'INTERVAL_EVERY', 2), ('minutes', 'INTERVAL_EVERY', 5), ('minutes', 'INTERVAL_EVERY', 15), ('text', 'PREFERENCES_INTERVAL_OPTION_CUSTOM', None)]
            elif value == 'example-durations':
                entries = [('minutes', 'DURATION_MINUTES', number) for number in [5, 15, 30]]
            elif value[0]['slint']['slot'] is not None:
                selected = sorted(value, key=lambda row: row['slint']['slot'])
                if not [row['slint']['slot'] for row in selected] == list(range(len(selected))):
                    raise Invalid('Protected contract shape rejected.')
                entries = [('text', row['symbol'], None) for row in selected]
            else:
                entries = [('text', value[0]['symbol'], None)]
            indices = []
            for index, entry in enumerate(entries):
                constant = (ns + '_' + prop + ('_' + str(index) if len(entries) > 1 else '')).replace('-', '_').upper()
                indices.append(len(static_entries))
                static_entries.append((constant, *entry))
            slot_fields[ns][prop] = {'indices': indices, 'array': isinstance(value, str) or value[0]['slint']['slot'] is not None}
    static_count = len(static_entries)
    output_count = static_count + len(feedback_slots)
    feedback_indices = {field: static_count + index for index, (_, field) in enumerate(feedback_slots)}
    SLINT = '// Generated protected identifiers only; one immutable complete publication.\nexport struct UiCopy {\n    catalog-ready: bool,\n    revision: string,\n    texts: [string],\n}\n\nexport global LocalizationCopy {\n    in property <UiCopy> value;\n}\n'
    SLINT += '\nexport global GuiFacts {\n    out property <string> build-version: "' + str(data['gui_build_version']) + '";\n    out property <string> text-contract-version: "' + str(data['text_contract_version']) + '";\n}\n'
    outputs['localization-copy.slint'] = SLINT.encode('utf-8')
    facts = {'brand': '"NevroTick"', 'author-value': '"David Osipov"', 'platform-value': '"Windows 11 x64"', 'code-license-value': '"GUI: GPL-3.0-only. Timekeeping Core: GPL-2.0-only."'}
    for ns, name in NAMES.items():
        alias = 'import { LocalizationCopy } from "localization-copy.slint";\nexport global ' + name + ' {\n'
        if ns == 'fixture':
            for prop, expression in facts.items():
                alias += f'    out property <string> {prop}: {expression};\n'
        for prop, entry in slot_fields[ns].items():
            values = ['LocalizationCopy.value.texts[' + str(index) + ']' for index in entry['indices']]
            expression = '[' + ', '.join(values) + ']' if entry['array'] else values[0]
            alias += f'    out property <' + ('[string]' if entry['array'] else 'string') + f'> {prop}: {expression};\n'
        outputs[ns + '-copy.slint'] = (alias + '}\n').encode('utf-8')
    R = projection_header + '\nuse super::generated::UiCopy;\n'
    R += f'/// Number of fixed static output slots.\npub(super) const STATIC_SLOT_COUNT: usize = {static_count};\n/// Complete fixed static and typed-feedback shape.\npub(super) const OUTPUT_SLOT_COUNT: usize = {output_count};\n'
    R += '/// Protected output position; no catalog key or action can select it.\n#[derive(Clone, Copy, Debug, Eq, PartialEq)]\npub(super) struct CopySlot(usize);\nimpl CopySlot {\n    /// Return this code-owned model position.\n    #[must_use]\n    pub(super) const fn index(self) -> usize { self.0 }\n'
    for index, (constant, *_) in enumerate(static_entries):
        R += f'    /// Fixed output for `{constant.lower()}`.\n    pub(super) const {constant}: Self = Self({index});\n'
    for index, (_, field) in enumerate(feedback_slots, static_count):
        R += f"    /// Typed feedback output for {field}.\n    pub(super) const FEEDBACK_{field.replace('-', '_').upper()}: Self = Self({index});\n"
    R += '}\n\n/// Closed typed-feedback order, independent of translated words.\nconst FEEDBACK_SLOTS: &[(FeedbackSlot, CopySlot)] = &[\n'
    for variant, field in feedback_slots:
        R += f"    (FeedbackSlot::{variant}, CopySlot::FEEDBACK_{field.replace('-', '_').upper()}),\n"
    R += '];\n'
    R += "\n/// Finish one complete private model; no property setter is called here.\n///\n/// # Errors\n/// Rejects wrong shape or aggregate output before any model can be published.\nfn finish(revision: SharedString, rows: Vec<SharedString>) -> Result<UiCopy, Error> {\n    if rows.len() != OUTPUT_SLOT_COUNT { return Err(Error::Schema); }\n    let value = UiCopy { catalog_ready: true, revision,\n        texts: ModelRc::new(ImmutableCopyModel { rows: rows.into_boxed_slice() }) };\n    check(&value)?;\n    Ok(value)\n}\n\n/// Retain only the next code-owned output position.\n///\n/// # Errors\n/// Rejects a programming mismatch before it can produce a published row table.\nfn push(rows: &mut Vec<SharedString>, slot: CopySlot, text: SharedString) -> Result<(), Error> {\n    if rows.len() != slot.index() { return Err(Error::Schema); }\n    rows.push(text);\n    Ok(())\n}\n\n/// Append every closed feedback slot through the same complete output budget.\n///\n/// # Errors\n/// Rejects aggregate exhaustion before retaining another toolkit string.\nfn append_feedback(rows: &mut Vec<SharedString>, budget: &mut OutputBudget, feedback: &PreparedFeedback) -> Result<(), Error> {\n    for (slot, output) in FEEDBACK_SLOTS {\n        let value = match feedback.get(*slot) {\n            Some(text) => budget.retain(text)?,\n            None => SharedString::default(),\n        };\n        push(rows, *output, value)?;\n    }\n    Ok(())\n}\n\n/// Prepare one immutable fixed-slot generation before the workflow commits it.\n///\n/// # Errors\n/// Rejects any argument, model shape or complete-output failure without publication.\npub(super) fn prepare(snapshot: &Snapshot<'_>, feedback: &PreparedFeedback) -> Result<UiCopy, Error> {\n    let revision: SharedString = snapshot.revision().to_string().into();\n    let mut budget = OutputBudget::default();\n    budget.charge(&revision)?;\n    let mut copy = CopyPreparation { snapshot, budget };\n    let mut rows = Vec::with_capacity(OUTPUT_SLOT_COUNT);\n"
    parts = [static_entries[start:start + 12] for start in range(0, static_count, 12)]
    for index in range(len(parts)):
        R += f'    fill_{index + 1}(&mut copy, &mut rows)?;\n'
    R += '    append_feedback(&mut rows, &mut copy.budget, feedback)?;\n    finish(revision, rows)\n}\n'
    for index, part in enumerate(parts, 1):
        R += f"\n/// Render one closed output cohort without resetting the preparation budget.\n///\n/// # Errors\n/// Propagates protected rendering or aggregate exhaustion before publication.\nfn fill_{index}(copy: &mut CopyPreparation<'_, '_>, rows: &mut Vec<SharedString>) -> Result<(), Error> {{\n"
        for constant, method, symbol, argument in part:
            expression = 'copy.' + method + '(MessageId::' + symbol + ('' if argument is None else ', ' + str(argument)) + ')?'
            R += f'    push(rows, CopySlot::{constant}, {expression})?;\n'
        R += '    Ok(())\n}\n'
    R += '\n/// Replace feedback by constructing a new immutable model; old readers retain theirs.\n///\n/// # Errors\n/// Rejects a malformed prior shape or complete-output failure before publication.\npub(super) fn replace_feedback(value: &UiCopy, feedback: &PreparedFeedback) -> Result<UiCopy, Error> {\n    check(value)?;\n    let mut budget = OutputBudget::default();\n    budget.charge(&value.revision)?;\n    let mut rows = Vec::with_capacity(OUTPUT_SLOT_COUNT);\n    for index in 0..STATIC_SLOT_COUNT {\n        let text = value.texts.row_data(index).ok_or(Error::Schema)?;\n        budget.charge(&text)?;\n        rows.push(text);\n    }\n    append_feedback(&mut rows, &mut budget, feedback)?;\n    finish(value.revision.clone(), rows)\n}\n\n/// Check the complete static/feedback shape and both aggregate output limits.\n///\n/// # Errors\n/// Rejects missing rows, empty required static text, overflow or output exhaustion.\npub(super) fn check(value: &UiCopy) -> Result<(), Error> {\n    if !value.catalog_ready || value.texts.row_count() != OUTPUT_SLOT_COUNT { return Err(Error::Schema); }\n    let mut budget = OutputBudget::default();\n    budget.charge(&value.revision)?;\n    for index in 0..OUTPUT_SLOT_COUNT {\n        let text = value.texts.row_data(index).ok_or(Error::Schema)?;\n        if index < STATIC_SLOT_COUNT && text.is_empty() { return Err(Error::Schema); }\n        budget.charge(&text)?;\n    }\n    Ok(())\n}\n'
    outputs['catalog_projection.rs'] = R.encode('utf-8')
    outputs['output-slot-map.json'] = (json.dumps({'static_count': static_count, 'output_count': output_count, 'fields': slot_fields, 'feedback': feedback_indices}, indent=2) + '\n').encode('utf-8')
    receipt = {'protected_input_sha256': expected_sha256, 'compatibility_fingerprint': fp, 'active_messages': len(rows), 'registry_ordinals': len(allrows), 'files': [{'path': name, 'sha256': hashlib.sha256(value).hexdigest(), 'bytes': len(value)} for name, value in sorted(outputs.items())]}
    outputs['identity.json'] = (json.dumps(receipt, indent=2) + '\n').encode('utf-8')
    return outputs


def _emit_files(raw: bytes, expected_sha256: str, template: bytes) -> dict[str, bytes]:
    """Require exact bounded metadata/template identity before any code emission.

    The source owner reviews the expected identity separately. Neither a catalog
    fingerprint nor this function can establish human semantic approval.
    """
    if type(raw) is not bytes or len(raw) > MAX_CONTRACT_BYTES:
        raise Invalid('Protected contract byte bound rejected.')
    if type(expected_sha256) is not str or re.fullmatch('[a-f0-9]{64}', expected_sha256) is None:
        raise Invalid('An exact reviewed protected contract SHA-256 is required.')
    if hashlib.sha256(raw).hexdigest() != expected_sha256:
        raise Invalid('Protected contract identity mismatch.')
    if type(template) is not bytes or len(template) > MAX_TEMPLATE_BYTES or hashlib.sha256(template).hexdigest() != TEMPLATE_SHA256:
        raise Invalid('Protected generator template identity mismatch.')
    try:
        data = protected_json.read(raw)
        files = _core_emit(data, _FEEDBACK_SLOTS, template.decode('utf-8'), expected_sha256)
        _output_bounds(files)
        return files
    except Invalid:
        raise
    except (KeyError, IndexError, TypeError, ValueError, OverflowError, RecursionError) as error:
        raise Invalid('Protected contract or generated shape rejected.') from error
