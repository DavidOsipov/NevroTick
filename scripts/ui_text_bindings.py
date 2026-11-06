"""Closed source-owned UI binding/action profiles shared by coverage and codegen.

Metadata may describe these existing destinations, never select new slot order,
argument kinds, source trust choices, interval facts or executable actions.
"""
# These profiles are source-owned action/field contracts, never inferred from
# translator metadata or an arbitrary set of observed array indexes.
BINDING_NAMESPACES = {
    'FixtureCopy': 'fixture', 'PreferencesCopy': 'preferences',
    'SourceCopy': 'source', 'RecoveryCopy': 'recovery', 'ShellCopy': 'shell',
    'PolicyCopy': 'policy', 'InspectionCopy': 'inspection',
    'DiagnosticCopy': 'diagnostic', 'RawCopy': 'raw',
}
ARRAY_BINDING_KEYS = {
    ('SourceCopy', 'protocols'): {0: 'ui.source.protocol-option.nts', 1: 'ui.source.protocol-option.ntp'},
    ('SourceCopy', 'change-labels'): {0: 'ui.rust.change-baseline', 1: 'ui.rust.change-add', 2: 'ui.rust.change-edit', 3: 'ui.rust.change-remove'},
    ('RecoveryCopy', 'counts'): {0: 'ui.recovery.count-option.off', 1: 'ui.recovery.count-option.one', 2: 'ui.recovery.count-option.two'},
    ('DiagnosticCopy', 'scopes'): {0: 'ui.diagnostic.scope-option.ui', 1: 'ui.diagnostic.scope-option.service', 2: 'ui.diagnostic.scope-option.combined'},
    ('PreferencesCopy', 'interval-options'): {0: 'ui.preferences.interval-option.automatic', 4: 'ui.preferences.interval-option.custom'},
}
TYPED_ARRAY_BINDINGS = {
    ('PreferencesCopy', 'interval-options'): ('ui.interval.every', ((1, 2), (2, 5), (3, 15))),
    ('DiagnosticCopy', 'durations'): ('ui.duration.minutes', ((0, 5), (1, 15), (2, 30))),
}
SCALAR_BINDING_ALIASES = {('SourceCopy', 'review-none'): 'ui.rust.review-none'}


def binding_definition_errors(rows):
    """Reject overlapping or unselected message-to-action field definitions.

    Ordinary targets contain Text only. Fixed array keys/indexes are selected
    above, including the deliberate 0/4 interval text slots; positive minute
    facts have their separate closed bridge and cannot become ordinary labels.
    """
    errors=[];seen={};arrays={}
    active={row['key']:row for row in rows if row['active']}
    for row in rows:
        if not row['active'] or row.get('slint') is None:continue
        target=row['slint'];global_name=target['global'];prop=target['property']
        destination=(global_name,prop);label=global_name+'.'+prop
        slot=target.get('slot')
        if slot is not None and type(slot) is not int:
            errors.append({'kind':'unsupported-array-binding','property':label,'key':row['key']});continue
        prior=seen.setdefault(destination,set())
        if slot in prior or (prior and (slot is None or None in prior)):
            errors.append({'kind':'duplicate-binding-definition','property':label,'key':row['key']})
        prior.add(slot)
        namespace=BINDING_NAMESPACES.get(global_name)
        if namespace is None or ('namespace' in target and target['namespace']!=namespace):
            errors.append({'kind':'unsupported-scalar-binding' if slot is None else 'unsupported-array-binding','property':label,'key':row['key']})
        if row['kind']!='Text':
            errors.append({'kind':'unsupported-binding-type','property':label,'key':row['key']})
        if slot is None:
            if destination in ARRAY_BINDING_KEYS or destination in TYPED_ARRAY_BINDINGS:
                errors.append({'kind':'array-binding-profile-mismatch','property':label,'key':row['key']})
            expected=SCALAR_BINDING_ALIASES.get(destination,'ui.'+str(namespace)+'.'+prop)
            if row['key']!=expected:
                errors.append({'kind':'unsupported-scalar-binding','property':label,'key':row['key']})
        else:
            arrays.setdefault(destination,[]).append((slot,row['key']))
            profile=ARRAY_BINDING_KEYS.get(destination)
            if profile is None:
                errors.append({'kind':'unsupported-array-binding','property':label,'key':row['key']})
            elif profile.get(slot)!=row['key']:
                errors.append({'kind':'array-binding-profile-mismatch','property':label,'key':row['key']})
    for destination,expected in ARRAY_BINDING_KEYS.items():
        if destination not in seen:continue
        found=arrays.get(destination,[])
        if len(found)!=len(expected) or dict(found)!=expected:
            errors.append({'kind':'array-binding-profile-mismatch','property':'.'.join(destination)})
    for destination,(key,_facts) in TYPED_ARRAY_BINDINGS.items():
        row=active.get(key)
        if destination in seen or row is not None:
            if row is None or row['kind']!='Minutes' or row.get('slint') is not None:
                errors.append({'kind':'typed-array-binding-profile-mismatch','property':'.'.join(destination),'key':key})
        if destination not in ARRAY_BINDING_KEYS and destination in seen:
            errors.append({'kind':'typed-array-binding-profile-mismatch','property':'.'.join(destination),'key':key})
    return errors
