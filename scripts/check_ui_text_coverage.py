"""Independent source-to-protected-text coverage checks (never catalog codegen).

The expected contract is code-owned; actual sinks are read separately from Slint
expressions and handwritten Rust presenters. Generated specs/projections and tests
are excluded from actual-use discovery, so a declared but orphaned ID cannot
approve itself. This is bounded source evidence, not compiled UIA/sink security.
"""
from pathlib import Path
import json,re,hashlib
import protected_json
import slint_text_graph
import native_text_graph
from ui_text_bindings import binding_definition_errors

MAX_FILES=256
MAX_TOTAL=8_000_000
MAX_FILE_BYTES=1_000_000
GLOBALS={'FixtureCopy','PreferencesCopy','SourceCopy','RecoveryCopy','ShellCopy','PolicyCopy','InspectionCopy','DiagnosticCopy','RawCopy'}
FACT_PROPERTIES={'FixtureCopy.brand','FixtureCopy.author-value','FixtureCopy.platform-value','FixtureCopy.code-license-value'}
# Protected data/technical fields, not unchecked translated prose.
FACT_PROPERTIES|={'DiagnosticCopy.report-content'}
# The fixed initialization carrier has separate typed feedback-sink accounting.
FACT_PROPERTIES|={'LocalizationCopy.value'}
# Exact source-owned display facts, not translator-selected copy exemptions.
# Each exception binds file + component/property + complete literal value.
LITERAL_FACT_BINDINGS={
 ('crates/nevrotick-ui/ui/about.slint','BodyText','text','Slint 1.18.1; garde 0.23.0; Serde 1.0.229; serde_json 1.0.154.'):'Pinned admitted technology/version facts',
 ('crates/nevrotick-ui/ui/about.slint','FactRow','value','https://github.com/DavidOsipov/NevroTick'):'Fixed code-owned repository URI; catalogs cannot choose the target',
 ('crates/nevrotick-ui/ui/components.slint','ActionButton','text','?'):'Fixed help glyph; adjacent maintained text owns the accessible explanation',
 ('crates/nevrotick-ui/ui/settings.slint','FactRow','value','English'):'Initial adopted language identity; no translator chooses an installed locale',
 ('crates/nevrotick-ui/ui/app.slint','AppWindow','draft-custom-minutes','1'):'Checked initial positive whole-minute raw draft, not a translated label',
 ('crates/nevrotick-ui/ui/settings.slint','SettingsPage','draft-custom-minutes','1'):'Same checked initial positive whole-minute raw draft in standalone fixture',
 ('crates/nevrotick-ui/ui/app.slint','BodyText','text','The interface encountered an internal error. Close and reopen NevroTick.'):'Fixed bootstrap failure; private copy/raw terminal marker only, independent exact review required',
}
# Exact fixed bootstrap UI-only exit consent, reviewed independently from runtime
# catalogs in EMERGENCY-COPY.json. This exception is source ownership, not human
# semantic approval, and never authorizes a service/clock effect.
for owner,prop,literal in [
 ('QuitWarning','title','Close NevroTick interface?'),
 ('QuitWarning','safe-label','Keep NevroTick open'),
 ('QuitWarning','third-label','Close interface only'),
 ('SectionHeading','text','The interface encountered an internal error'),
 ('BodyText','text','The interface cannot safely continue its normal workflow. Closing affects only this app. It does not request a service or clock change. Background services may keep running.'),
 ('BodyText','text','Unsaved changes in this app will be lost. New local notifications require an active NevroTick observer.'),
 ('SecondaryText','text','If this window remains open, closing has not been confirmed. Use Close again.'),
]:
 LITERAL_FACT_BINDINGS[('crates/nevrotick-ui/ui/quit-warning.slint',owner,prop,literal)]='Fixed emergency bootstrap exact source; independent human review not established'
# Only these closed Option<UiText> slots admit an absent message. Their maintained
# fixture defaults are not delivered translation values. Required catalog text
# remains nonempty and nonblank, including the eventual labels in these slots.
OPTIONAL_EMPTY_FEEDBACK_BINDINGS=frozenset(
 ('crates/nevrotick-ui/ui/app.slint','AppWindow','fixture-'+field,'')
 for field in ['editor-error','editor-feedback','interval-error','interval-feedback','preference-error','preference-feedback','recovery-error','recovery-feedback','recovery-back-label','quit-headline','quit-consequence','shell-feedback','service-feedback','notification-feedback','diagnostic-feedback','policy-error','source-query-error','accepted-interval-label','status-feedback','tray-feedback','status-headline','status-consequence','status-profile-label','status-profile','status-authentication','status-agreement','status-freshness','status-check-label','status-remedy-label']
)
# Code-owned separators join separately checked facts/messages. They are allowed
# only inside a compound expression at these reviewed source sinks.
COMPOUND_FACT_SEPARATORS={
 ('crates/nevrotick-ui/ui/status.slint','BodyText','text',': '),
 ('crates/nevrotick-ui/ui/status.slint','SecondaryText','text',': '),
 ('crates/nevrotick-ui/ui/sources.slint','FactRow','value',' · '),
 ('crates/nevrotick-ui/ui/sources.slint','FactRow','value',' '),
}
GENERATED_RUST_PATHS=frozenset({
 'crates/nevrotick-ui/src/localization/contract.rs',
 'crates/nevrotick-ui/src/slint_host/catalog_projection.rs',
})
TEST_SOURCE_PATHS=frozenset({'crates/nevrotick-ui/src/localization/tests.rs'})
LEGACY_NATIVE='crates/nevrotick-ui/src/native.rs'
LEGACY_NATIVE_DIRECTORY='crates/nevrotick-ui/src/native/'


def excluded_source(path):
    """Exclude only fixed generated/legacy/test owners, never a filename suffix."""
    return (path.startswith('crates/nevrotick-ui/tests/') or path in TEST_SOURCE_PATHS
            or path in GENERATED_RUST_PATHS or path==LEGACY_NATIVE
            or path.startswith(LEGACY_NATIVE_DIRECTORY))


def inert_source(source,keep_strings=False):
    """Retain positions/newlines; optionally keep strings while masking comments."""
    out=list(source);i=0;state=None;depth=0;escaped=False
    while i<len(source):
        if state=='line':
            if source[i]=='\n':state=None
            else:out[i]=' '
            i+=1;continue
        if state=='block':
            if source.startswith('/*',i):depth+=1;out[i:i+2]=[' ',' '];i+=2
            elif source.startswith('*/',i):depth-=1;out[i:i+2]=[' ',' '];i+=2;state='block' if depth else None
            else:
                if source[i]!='\n':out[i]=' '
                i+=1
            continue
        if state=='string':
            if not keep_strings and source[i]!='\n':out[i]=' '
            if source[i]=='"' and not escaped:state=None
            escaped=not escaped and source[i]=='\\'
            i+=1;continue
        if source.startswith('//',i):state='line';out[i:i+2]=[' ',' '];i+=2
        elif source.startswith('/*',i):state='block';depth=1;out[i:i+2]=[' ',' '];i+=2
        elif source[i]=='"':
            state='string';escaped=False
            if not keep_strings:out[i]=' '
            i+=1
        else:i+=1
    return ''.join(out)


def text_expressions(source,clean):
    """Locate complete text-property expressions outside strings and comments.

    Nested parentheses, arrays and expression blocks do not end a property.
    These are lexical source spans, not Slint name/type or visual resolution.
    """
    pattern=r'(?<![.\w-])(text|accessible-label|accessible-description|placeholder-text|title|value)\s*:'
    for found in slint_text_graph.outside_parameters(clean,re.finditer(pattern,clean)):
        start=found.end();at=start;parentheses=0;brackets=0;braces=0
        while at<len(clean):
            token=clean[at]
            if token==';' and not parentheses and not brackets and not braces:break
            if token=='}' and not parentheses and not brackets and not braces:break
            if token=='(':parentheses+=1
            elif token==')':parentheses-=1
            elif token=='[':brackets+=1
            elif token==']':brackets-=1
            elif token=='{':braces+=1
            elif token=='}':braces-=1
            if min(parentheses,brackets,braces)<0:raise ValueError('text expression nesting')
            at+=1
        yield found[1],start,at


def expression_literals(source,clean):
    """Read every literal in each UI text expression, not only its first token."""
    comments_masked=inert_source(source,keep_strings=True)
    token_pattern=r'"(?:[^"\\]|\\.)*"'
    for prop,start,end in text_expressions(source,clean):
        expression=comments_masked[start:end]
        direct_empty=expression.strip().strip('() \t\r\n')=='""'
        for found in re.finditer(token_pattern,expression):
            try:literal=json.loads(found[0])
            except json.JSONDecodeError:literal=found[0]
            yield {'line':source.count('\n',0,start+found.start())+1,'property':prop,'literal':literal,
                   'compound_empty':not literal and not direct_empty}


def without_test_items(source):
    """Mask exact cfg(test)/test items while retaining original source positions.

    This lexical check recognizes explicit test-only attributes; it does not
    resolve arbitrary cfg expressions, macros, feature graphs or Rust calls.
    Unbalanced selected items reject rather than silently becoming owners.
    """
    clean=inert_source(source);out=list(source)
    attributes=re.finditer(r'#\s*\[\s*(?:cfg\s*\(\s*test\s*\)|test)\s*\]',clean)
    for attribute in attributes:
        at=attribute.end()
        while at<len(clean):
            while at<len(clean) and clean[at].isspace():at+=1
            if not clean.startswith('#',at):break
            opening=clean.find('[',at)
            if opening<0 or clean[at+1:opening].strip():break
            depth=1;at=opening+1
            while at<len(clean) and depth:
                if clean[at]=='[':depth+=1
                elif clean[at]==']':depth-=1
                at+=1
            if depth:raise ValueError('incomplete selected test attribute')
        parentheses=0;brackets=0;end=None
        while at<len(clean):
            token=clean[at]
            if token=='(':parentheses+=1
            elif token==')':parentheses-=1
            elif token=='[':brackets+=1
            elif token==']':brackets-=1
            elif token==';' and not parentheses and not brackets:end=at+1;break
            elif token=='{' and not parentheses and not brackets:
                depth=1;at+=1
                while at<len(clean) and depth:
                    if clean[at]=='{':depth+=1
                    elif clean[at]=='}':depth-=1
                    at+=1
                if depth:raise ValueError('incomplete selected test item')
                end=at;break
            if parentheses<0 or brackets<0:raise ValueError('invalid selected test item nesting')
            at+=1
        if end is None:raise ValueError('missing selected test item')
        for index in range(attribute.start(),end):
            if out[index]!='\n':out[index]=' '
    return ''.join(out)


RUNTIME_SOURCE_OWNERS={
 'crates/nevrotick-ui/src/slint_host.rs':'checked preference/navigation UI callbacks',
 'crates/nevrotick-ui/src/slint_host/source_workflow.rs':'guarded source editor/review projection',
 'crates/nevrotick-ui/src/slint_host/recovery_workflow.rs':'guarded recovery/back/leave projection',
 'crates/nevrotick-ui/src/slint_host/shell_workflow.rs':'guarded quit/service/notification projection',
 'crates/nevrotick-ui/src/slint_host/policy_workflow.rs':'guarded policy preview projection',
 'crates/nevrotick-ui/src/slint_host/inspection_workflow.rs':'guarded source query/detail projection',
 'crates/nevrotick-ui/src/slint_host/diagnostic_workflow.rs':'guarded diagnostics projection',
 'crates/nevrotick-ui/src/slint_host/status_workflow.rs':'guarded status projection',
 'crates/nevrotick-ui/src/slint_host/raw_workflow.rs':'guarded untrusted raw editor ingress',
 'crates/nevrotick-ui/src/slint_host/tray_workflow.rs':'guarded tray text projection',
}
# Explicit reviewed presenter entry/format functions, not every function in a
# registered file. Changing this list requires source call-site reconciliation;
# lexical names are not compiler resolution or proof of runtime invocation.
EMITTER_FUNCTIONS={
 'crates/nevrotick-ui/src/slint_host.rs':{'create','install_callbacks','project_preferences','reset_interval','save_interval','cancel_interval','change_preference','navigate','change_mode'},
 'crates/nevrotick-ui/src/slint_host/source_workflow.rs':{'from_source','for_add','apply','source_row','review_value','change_row','selected_editor','select_source','open_editor','update_preview','cancel_editor','change_protocol','source_request','sources_reached'},
 'crates/nevrotick-ui/src/slint_host/recovery_workflow.rs':{'apply','initialize','open','navigation_request','update','local_request','resolve_leave','navigate','change_mode','request_shell_route','publish_shell_route','ready','navigation'},
 'crates/nevrotick-ui/src/slint_host/shell_workflow.rs':{'from_owner','apply','initialize','shell_request','show_keep_reason','unavailable_service','unavailable_notification','complete_exit_request','close_allowed','diagnose','select_section','project_route','project_section','install_callbacks'},
 'crates/nevrotick-ui/src/slint_host/policy_workflow.rs':{'apply','initialize','request'},
 'crates/nevrotick-ui/src/slint_host/inspection_workflow.rs':{'apply','request','show_error'},
 'crates/nevrotick-ui/src/slint_host/diagnostic_workflow.rs':{'project','install_callbacks'},
 'crates/nevrotick-ui/src/slint_host/status_workflow.rs':{'project','install_callbacks'},
 # Raw/tray formatter owners remain unregistered until their actual receiving
 # function and text sink chain are reconciled. Reserved tray text is not a use.
 'crates/nevrotick-ui/src/slint_host/raw_workflow.rs':set(),
 'crates/nevrotick-ui/src/slint_host/tray_workflow.rs':set(),
}
# New guarded receiving helpers are named explicitly after call-site tracing of
# the frozen raw/typed integration. Mere membership of a file never grants use.
EMITTER_FUNCTIONS['crates/nevrotick-ui/src/slint_host/inspection_workflow.rs'].add('show_context_error')
EMITTER_FUNCTIONS['crates/nevrotick-ui/src/slint_host/raw_workflow.rs'].update({'reset','show_error','reject_stale'})
EMITTER_FUNCTIONS['crates/nevrotick-ui/src/slint_host/shell_workflow.rs'].add('show_quit_confirmation')
EMITTER_FUNCTIONS['crates/nevrotick-ui/src/slint_host/source_workflow.rs'].update({'source_feedback','source_request_ready','show_hidden_source'})
PRESENTER='crates/nevrotick-ui/src/localization/presentation.rs'
# A pure enum formatter is not a sink. Only this reviewed receiving bridge may
# reach these finite IDs; tray enums remain reserved while no renderer is owned.
RUNTIME_ENUM_SOURCE_OWNERS={
 'crates/nevrotick-ui/src/status_ui.rs':[
     ('crates/nevrotick-ui/src/slint_host/status_workflow.rs','project'),
 ],
}



def functions(source):
    """Bounded lexical function spans; this is trace evidence, not Rust resolution."""
    clean=inert_source(source);out=[]
    for found in re.finditer(r'\bfn\s+([A-Za-z_][A-Za-z0-9_]*)\s*(?:<[^\n]*>)?\s*\(',clean):
        start=clean.find('{',found.end())
        if start<0:continue
        depth=1;end=start+1
        while end<len(clean) and depth:
            if clean[end]=='{':depth+=1
            elif clean[end]=='}':depth-=1
            end+=1
        if depth:raise ValueError('incomplete source function')
        out.append((found[1],found.start(),end))
    return out


def extract(files,trace=None,native_emitters=None):
    """Extract finite actual GUI/runtime references independently of a catalog."""
    if len(files)>MAX_FILES or sum(len(x.encode()) for x in files.values())>MAX_TOTAL:raise ValueError('source inventory bound')
    files={path:without_test_items(source) if path.endswith('.rs') else source for path,source in files.items()}
    if trace is None:trace=slint_text_graph.build(files,inert_source)
    runtime_owners=dict(RUNTIME_SOURCE_OWNERS)
    emitters={path:set(names)for path,names in EMITTER_FUNCTIONS.items()}
    for path,names in(native_emitters or {}).items():
        runtime_owners[path]='actual native receiving owner'
        emitters.setdefault(path,set()).update(names)
    gui={};symbols={};runtime_keys={};literals=[];unregistered=[]
    visible_bindings={}
    for binding in trace.bindings:
        if trace.sinks(binding['target']):visible_bindings.setdefault(binding['path'],[]).append(binding)
    provider=files.get(PRESENTER,'');provider_functions=functions(provider)
    reachable=set()
    for path,source in files.items():
        if path in runtime_owners:
            clean=inert_source(source)
            spans=functions(source)
            for found in re.finditer(r'\bUiText::([a-z][a-z0-9_]*)\s*\(',clean):
                enclosing=next((name for name,start,end in reversed(spans) if start<=found.start()<end),None)
                if enclosing in emitters.get(path,set()):reachable.add(found[1])
    changed=True
    while changed:
        prior=len(reachable)
        for name,start,end in provider_functions:
            if name in reachable:reachable.update(re.findall(r'\b(?:Self|UiText)::([a-z][a-z0-9_]*)\s*\(',inert_source(provider[start:end])))
        changed=len(reachable)!=prior
    for path,source in files.items():
        if len(source.encode())>MAX_FILE_BYTES:raise ValueError('source-file bound')
        if excluded_source(path):continue
        clean=inert_source(source)
        if path.endswith('.slint') and path not in slint_text_graph.COPY_SOURCE_PATHS:
            literal_source=inert_source(source,keep_strings=True)
            for binding in visible_bindings.get(path,[]):
                expression=literal_source[binding['start']:binding['end']]
                direct_empty=expression.strip().strip('() \t\r\n')=='\"\"'
                for token in re.finditer(r'\"(?:[^\"\\]|\\.)*\"',expression):
                    try:literal=json.loads(token[0])
                    except json.JSONDecodeError:literal=token[0]
                    literals.append({'path':path,'line':source.count('\n',0,binding['start']+token.start())+1,'owner':binding['owner'],'property':binding['property'],'literal':literal,'compound_empty':not literal and not direct_empty,'compound':expression.strip()!=token[0]})
            for m in re.finditer(r'\b([A-Z][A-Za-z0-9_]*Copy)\.([a-z][a-z0-9-]*)\b',clean):
                if not trace.active_source(path,m.start()):continue
                key=m[1]+'.'+m[2];gui.setdefault(key,[]).append({'path':path,'line':source.count('\n',0,m.start())+1})
        if path.endswith('.rs'):
            spans=functions(source) if re.search(r'\bMessageId::[A-Z]',clean) else []
            for m in re.finditer(r'\bMessageId::([A-Z][A-Z0-9_]*)\b',clean):
                enclosing=next((name for name,start,end in reversed(spans) if start<=m.start()<end),None)
                if path==PRESENTER and enclosing in reachable:
                    symbols.setdefault(m[1],[]).append({'path':path,'line':source.count('\n',0,m.start())+1,'owner':'reachable typed UiText::'+enclosing})
                elif path in runtime_owners and enclosing in emitters.get(path,set()):
                    symbols.setdefault(m[1],[]).append({'path':path,'line':source.count('\n',0,m.start())+1,'owner':runtime_owners[path]+':'+enclosing})
                elif path in runtime_owners:
                    declarations=list(re.finditer(r'\bconst\s+([A-Z][A-Z0-9_]*)\s*:[^;]*;',clean))
                    declaration=next((declaration for declaration in declarations if declaration.start()<=m.start()<declaration.end()),None)
                    consumers=[] if declaration is None else [name for name,start,end in spans if name in emitters.get(path,set()) and re.search(r'\b'+re.escape(declaration[1])+r'\b',clean[start:end])]
                    if consumers:
                        symbols.setdefault(m[1],[]).append({'path':path,'line':source.count('\n',0,m.start())+1,'owner':'typed guard constant '+declaration[1],'consumers':consumers})
                    else:unregistered.append({'path':path,'line':source.count('\n',0,m.start())+1,'symbol':m[1]})
                elif path!=PRESENTER:
                    unregistered.append({'path':path,'line':source.count('\n',0,m.start())+1,'symbol':m[1]})
            # Actual finite enum-to-ID branches, never all strings/IDs in files.
            for function in re.finditer(r'\b(?:pub(?:\([^)]*\))?\s+)?(?:const\s+)?fn\s+catalog_id\s*\([^)]*\)\s*->\s*&\s*\x27static\s+str\s*\{',source):
                start=function.end();depth=1;i=start
                while i<len(clean) and depth:
                    if clean[i]=='{':depth+=1
                    elif clean[i]=='}':depth-=1
                    i+=1
                consumers=[]
                for consumer_path,consumer_name in RUNTIME_ENUM_SOURCE_OWNERS.get(path,[]):
                    consumer=files.get(consumer_path,'')
                    if consumer_name not in emitters.get(consumer_path,set()):continue
                    for name,begin,end in functions(consumer):
                        body=inert_source(consumer[begin:end])
                        if name==consumer_name and re.search(r'\bUiText::key\s*\(',body) and re.search(r'\.catalog_id\s*\(',body):
                            consumers.append({'path':consumer_path,'function':consumer_name})
                if not consumers:continue
                for m in re.finditer(r'"(ui\.[a-z0-9.-]+)"',source[start:i]):
                    runtime_keys.setdefault(m[1],[]).append({'path':path,'line':source.count('\n',0,start+m.start())+1,'owner':'closed catalog_id enum formatter','receiving_consumers':consumers})
    return {'gui':gui,'symbols':symbols,'runtime_keys':runtime_keys,'literals':literals,'unregistered_runtime_refs':unregistered}


PRODUCT_BUILD_SOURCE='crates/nevrotick-ui/build.rs'
PRODUCT_HOST_SOURCE='crates/nevrotick-ui/src/slint_host.rs'


def product_root_errors(files,trace):
    """Bind product composition to the selected build entry and Rust receiver.

    Small isolated source-control fixtures need no Rust/build scaffolding. The
    real product snapshot includes the protected host source, so it must also
    retain the exact build entry and checked constructor. This is lexical source
    custody, not arbitrary Rust cfg, macro or control-flow resolution.
    """
    if PRODUCT_HOST_SOURCE not in files:return []
    build=without_test_items(files.get(PRODUCT_BUILD_SOURCE,''))
    host=without_test_items(files[PRODUCT_HOST_SOURCE])
    build_main=[(start,end)for name,start,end in functions(build)if name=='main']
    host_create=[(start,end)for name,start,end in functions(host)if name=='create']
    paths=[]
    for start,end in build_main:
        source=build[start:end];clean=inert_source(source);comments=inert_source(source,keep_strings=True)
        for call in re.finditer(r'\bslint_build::compile_with_config\s*\(',clean):
            literal=re.match(r'\s*"([^"\\]*)"',comments[call.end():])
            paths.append(literal[1]if literal else None)
    constructors=sum(len(re.findall(r'\bAppWindow::new\s*\(',inert_source(host[start:end])))for start,end in host_create)
    other_window_constructor=False
    for path,source in files.items():
        if not path.endswith('.rs') or excluded_source(path):continue
        source=inert_source(without_test_items(source))
        if any(call[1]in trace.window_components-{'AppWindow'}for call in re.finditer(r'\b([A-Z][A-Za-z0-9_]*)::new\s*\(',source)):
            other_window_constructor=True
            break
    if len(build_main)!=1 or paths!=['ui/app.slint'] or len(host_create)!=1 or constructors!=1 or trace.roots!={'AppWindow'} or other_window_constructor:
        return [{'kind':'product-ui-entrypoint-mismatch','build_source':PRODUCT_BUILD_SOURCE,'host_source':PRODUCT_HOST_SOURCE,'expected_component':'AppWindow','expected_entry':'ui/app.slint'}]
    return []


def semantic_fingerprint(contract):
    """Independently bind text meaning/types/actions, excluding GUI topology.

    This identity is compatibility evidence, not semantic approval or runtime
    catalog admission. Full protected metadata shape is admitted by its owner.
    """
    semantic={key:value for key,value in contract.items() if key not in {'schema_version','gui_build_version','status','contract_revision','messages'}}
    semantic['messages']=sorted([
        ({key:row.get(key) for key in ['key','kind','group','meaning_revision','meaning_context','active','reference']} if row['active'] else
         {'key':row['key'],'active':False,'status':'reserved' if 'reservation' in row else 'retired'})
        for row in contract['messages']],key=lambda row:row['key'])
    data=b'NevroTick UI protected semantic text-contract v2\0'+json.dumps(semantic,ensure_ascii=False,sort_keys=True,separators=(',',':')).encode()
    return hashlib.sha256(data).hexdigest()


def missing_required_bindings(files,trace):
    """Require explicit text/name custody for reached native text controls.

    Field values may legitimately start empty; their accessible label must
    still be bound. A native checkbox has its own visible state glyph. A
    textless button needs both an icon and an accessible name. The top Menu
    under ContextMenuArea is a container; displayed menu entries need titles.
    Only real product-driver snapshots require the application window title;
    isolated lexical controls need not invent an unrelated title resource.
    """
    required={'Text':'text','Button':'text','CheckBox':'text','MenuItem':'title',
              'Menu':'title','Window':'title','LineEdit':'accessible-label',
              'TextEdit':'accessible-label','ComboBox':'model'}
    errors=[]
    for identity,instance in trace.instance_details.items():
        native=next((base for base in instance['ancestry']if base in required),None)
        if native is None:continue
        if native=='Window'and PRODUCT_HOST_SOURCE not in files:continue
        bindings=instance['bindings'];prop=required[native]
        if prop in bindings:continue
        if native=='Button'and 'icon'in bindings and 'accessible-label'in bindings:continue
        if native=='CheckBox'and 'accessible-label'in bindings:continue
        if native=='Menu':
            parent=trace.instance_details.get(instance['parent'],{})
            if 'ContextMenuArea'in parent.get('ancestry',()):continue
        errors.append({'kind':'missing-required-ui-binding','path':instance['path'],
                       'line':instance['line'],'owner':instance['type'],
                       'instance':identity,'instance_path':trace.instance_path(identity),'property':prop})
    return errors


# These are checked presentation data, never translator-selected label slots.
# Each exception binds its exact view, lexical component, control/property and
# expression, together with the reached handwritten receiving method below.
CHECKED_DATA_BINDINGS={
 ('sources.slint','SourcesPage','FactRow','value','root.accepted-query'):'query',
 ('source-details.slint','SourceDetailsPanel','FactRow','value','root.source.label'):'details',
 ('source-details.slint','SourceDetailsPanel','FactRow','value','root.source.host'):'details',
 ('source-details.slint','SourceDetailsPanel','FactRow','value','root.source.port'):'details',
 ('sources.slint','SourcesPage','SectionHeading','text','source.label'):'sources',
 ('sources.slint','SourcesPage','BodyText','text','source.host'):'sources',
 ('sources.slint','SourcesPage','FactRow','value','source.port'):'sources',
 ('statistics.slint','StatisticsPage','SectionHeading','text','source.label'):'sources',
 ('statistics.slint','StatisticsPage','SecondaryText','text','source.host'):'sources',
 ('recovery.slint','RecoveryPage','BodyText','text','identity'):'helpers',
 ('about.slint','AboutPage','FactRow','value','GuiFacts.build-version'):'gui-build-version',
 ('about.slint','AboutPage','FactRow','value','GuiFacts.text-contract-version'):'text-contract-version',
}
CHECKED_DATA_RECEIVERS={
 'query':('inspection_workflow.rs','apply','set_accepted_source_query'),
 'details':('inspection_workflow.rs','apply','set_inspection_detail_source'),
 'sources':('source_workflow.rs','apply','set_sources'),
 'helpers':('recovery_workflow.rs','apply','set_recovery_preview_hosts'),
}
OPTIONAL_COMPONENT_TEXT={
 ('PageHeading','description'),('PageHeading','kicker'),('FactRow','note'),
}


def required_source_errors(contract,files,trace,actual,runtime_sinks):
    """Reject required text bindings with no independently owned source flow.

    This is bounded source may-flow, not value/branch evaluation. It admits only
    catalog bindings, exact source-owned facts, observed typed feedback slots,
    the enumerated checked-data receiving paths and guarded optional components.
    Arbitrary uninitialized root properties are never text ownership.
    """
    active={row['key']:row for row in contract['messages']if row['active']}
    symbols={row['symbol']:row['key']for row in active.values()}
    owned={('@copy',row['slint']['global']+'.'+row['slint']['property'])for row in active.values()if row.get('slint')}
    owned|={('@copy',prop)for prop in FACT_PROPERTIES if prop!='LocalizationCopy.value'}
    if 'ui.interval.every'in active:owned.add(('@copy','PreferencesCopy.interval-options'))
    if 'ui.duration.minutes'in active:owned.add(('@copy','DiagnosticCopy.durations'))
    observed={symbols[symbol]for symbol in actual['symbols']if symbol in symbols}|set(actual['runtime_keys'])
    slots={prop.removeprefix('fixture-')for _path,_owner,prop,_empty in OPTIONAL_EMPTY_FEEDBACK_BINDINGS}
    for key in observed:
        for component,prop in (runtime_sinks or {}).get(key,()):
            if component=='AppWindow'and prop in slots:
                owned.update((identity,prop)for identity,types in trace.instances.items()if component in types)
    receivers={}
    for name,(filename,function,call)in CHECKED_DATA_RECEIVERS.items():
        source=without_test_items(files.get('crates/nevrotick-ui/src/slint_host/'+filename,''))
        receivers[name]=any(found==function and re.search(r'\b'+call+r'\s*\(',inert_source(source[start:end]))for found,start,end in functions(source))
    literal_sources={path:inert_source(source,keep_strings=True)for path,source in files.items()if path.endswith('.slint')}
    facts_path='crates/nevrotick-ui/ui/localization-copy.slint'
    facts=literal_sources.get(facts_path,'');clean_facts=inert_source(files.get(facts_path,''))
    declarations=list(re.finditer(r'\bexport\s+global\s+GuiFacts\s*\{',clean_facts))
    facts_body=''
    if len(declarations)==1:
        start=declarations[0].end();end=start;depth=1
        while end<len(clean_facts)and depth:
            if clean_facts[end]=='{':depth+=1
            elif clean_facts[end]=='}':depth-=1
            if depth>slint_text_graph.MAX_BRACE_DEPTH:raise ValueError('Slint fact depth bound')
            end+=1
        if depth==0:facts_body=facts[start:end-1]
    for name,version in [('gui-build-version',contract.get('gui_build_version')),('text-contract-version',contract.get('text_contract_version'))]:
        prop='build-version'if name=='gui-build-version'else name
        receivers[name]=type(version)is int and bool(re.search(r'\bout\s+property\s*<\s*string\s*>\s*'+prop+r'\s*:\s*"'+str(version)+r'"\s*;',facts_body))
    for identity,instance in trace.instance_details.items():
        for prop,binding in instance['bindings'].items():
            raw=literal_sources[binding['path']][binding['start']:binding['end']].strip()
            try:literal=json.loads(raw)
            except (ValueError,RecursionError):literal=None
            if type(literal)is str and (binding['path'],binding['owner'],prop,literal)in LITERAL_FACT_BINDINGS:
                owned.add((identity,prop))
            expression=re.sub(r'\s+',' ',binding['expression'].strip())
            data=CHECKED_DATA_BINDINGS.get((Path(binding['path']).name,binding.get('component'),binding['owner'],prop,expression))
            if data is not None and binding['path']=='crates/nevrotick-ui/ui/'+Path(binding['path']).name and receivers.get(data):
                owned.add((identity,prop))
            optional=next((field for component,field in OPTIONAL_COMPONENT_TEXT if binding.get('component')==component and expression=='root.'+field),None)
            if optional is not None and binding['path']=='crates/nevrotick-ui/ui/components.slint'and binding['owner']=='SecondaryText'and prop=='text':
                prefix=literal_sources[instance['path']][max(0,instance['start']-256):instance['start']]
                if re.search(r'\bif\s+root\.'+optional+r'\s*!=\s*""\s*:\s*SecondaryText\s*$',prefix):owned.add((identity,prop))
    # One bounded forward closure qualifies sources; no per-leaf cross product.
    pending=list(owned);qualified=set()
    while pending:
        node=pending.pop()
        if node in qualified:continue
        qualified.add(node)
        if len(qualified)>slint_text_graph.MAX_EDGES+slint_text_graph.MAX_NODES:raise ValueError('Slint owned-source work bound')
        pending.extend(trace.edges.get(node,()))
    errors=[]
    required={'Text':'text','Button':'text','CheckBox':'text','MenuItem':'title','Menu':'title',
              'Window':'title','LineEdit':'accessible-label','TextEdit':'accessible-label','ComboBox':'model'}
    for identity,instance in trace.instance_details.items():
        native=next((base for base in instance['ancestry']if base in required),None)
        if native is None or(native=='Window'and PRODUCT_HOST_SOURCE not in files):continue
        prop=required[native];bindings=instance['bindings']
        if native=='Menu'and 'ContextMenuArea'in trace.instance_details.get(instance['parent'],{}).get('ancestry',()):continue
        if prop not in bindings:
            if native=='Button'and 'icon'in bindings and 'accessible-label'in bindings:prop='accessible-label'
            elif native=='CheckBox'and 'accessible-label'in bindings:prop='accessible-label'
            else:continue  # The separate missing-binding check owns absence.
        if(identity,prop)not in qualified:
            errors.append({'kind':'unowned-required-ui-source','path':instance['path'],'line':instance['line'],
                           'owner':instance['type'],'property':prop,'instance':identity,
                           'instance_path':trace.instance_path(identity)})
    return errors


def check(contract,files,baseline=None,runtime_sinks=None):
    """Return forward/reverse violations; neither side is inferred from catalogtext."""
    rows=contract['messages'];active={r['key']:r for r in rows if r['active']};symbols={r['symbol']:r['key'] for r in rows if r['active']}
    maps={}
    for row in active.values():
        target=row.get('slint')
        if target:maps.setdefault(target['global']+'.'+target['property'],set()).add(row['key'])
    # Fixed preset formatting is a protected typed-argument use at this GUI array.
    if 'ui.interval.every' in active:maps.setdefault('PreferencesCopy.interval-options',set()).add('ui.interval.every')
    if 'ui.duration.minutes' in active:maps.setdefault('DiagnosticCopy.durations',set()).add('ui.duration.minutes')
    trace=slint_text_graph.build(files,inert_source)
    native=native_text_graph.native_uses(files,inert_source,without_test_items,functions,trace)
    actual=extract(files,trace,native['emitters']);used={};errors=binding_definition_errors(rows)
    errors.extend(native['errors'])
    errors.extend(product_root_errors(files,trace))
    errors.extend(missing_required_bindings(files,trace))
    errors.extend(required_source_errors(contract,files,trace,actual,runtime_sinks))
    def runtime_rendered(key):
        return runtime_sinks is None or any(trace.reaches(*sink) for sink in runtime_sinks.get(key,()))
    for ref in actual['unregistered_runtime_refs']:errors.append({'kind':'unregistered-runtime-owner',**ref})
    for literal in actual['literals']:
        if literal.get('compound_empty',False):continue
        fact=(literal['path'],literal['owner'],literal['property'],literal['literal'])
        if fact not in LITERAL_FACT_BINDINGS and fact not in OPTIONAL_EMPTY_FEEDBACK_BINDINGS and not(literal['compound'] and fact in COMPOUND_FACT_SEPARATORS):
            errors.append({'kind':'unowned-ui-literal' if literal['literal'] else 'empty-required-ui-text',**literal})
    for prop,locations in actual['gui'].items():
        if prop in FACT_PROPERTIES:continue
        if prop not in maps:errors.append({'kind':'missing-gui-text','property':prop,'locations':locations});continue
        sinks=trace.sinks(('@copy',prop))
        if sinks:
            for key in maps[prop]:used.setdefault(key,[]).extend({'kind':'GUI','property':prop,'text_endpoints':sinks,**loc} for loc in locations)
    for symbol,locations in actual['symbols'].items():
        if symbol not in symbols:errors.append({'kind':'missing-runtime-text','symbol':symbol,'locations':locations});continue
        if runtime_rendered(symbols[symbol]):used.setdefault(symbols[symbol],[]).extend({'kind':'typed Rust formatter','symbol':symbol,**loc}for loc in locations)
    for key,locations in actual['runtime_keys'].items():
        if key not in active:errors.append({'kind':'missing-runtime-text','key':key,'locations':locations});continue
        if runtime_rendered(key):used.setdefault(key,[]).extend(locations)
    # Native renderers have independent host/API witnesses, not synthetic
    # Slint component leaves or credit for a pure catalog_id formatter.
    for key,locations in native['uses'].items():
        if key not in active:
            errors.append({'kind':'missing-native-text','key':key,'locations':locations})
        else:used.setdefault(key,[]).extend(locations)
    for key in active:
        if key not in used:errors.append({'kind':'orphan-resource-id','key':key})
    for row in active.values():
        ref=row['reference'];texts=[ref['text']] if row['kind']=='Text' else list(ref['forms'].values())
        if not texts or any(type(x)is not str or not x for x in texts):errors.append({'kind':'empty-required-text','key':row['key']})
    candidate_identity=semantic_fingerprint(contract)
    prior_identity=None
    if baseline is not None:
        prior_identity=semantic_fingerprint(baseline)
        prior={row['key'] for row in baseline['messages'] if row['active']}
        if candidate_identity!=prior_identity and contract.get('text_contract_version',0)<=baseline.get('text_contract_version',0):
            errors.append({'kind':'text-contract-version-not-incremented','new_required_ids':sorted(set(active)-prior),
                           'previous_semantic_fingerprint':prior_identity,'candidate_semantic_fingerprint':candidate_identity})
    return {'ok':not errors,'errors':errors,'active_ids':len(active),'used_ids':len(used),'uses':used,'actual':actual,
            'runtime_sink_map_provided':runtime_sinks is not None,'baseline_provided':baseline is not None,
            'semantic_fingerprint':candidate_identity,'baseline_semantic_fingerprint':prior_identity}


def snapshot(root):
    """Read only finite first-party source files; no symlink or out-of-root input."""
    root=Path(root).resolve();base=root/'crates/nevrotick-ui';files={};scanned=0;total=0
    def source_paths():
        build=root/PRODUCT_BUILD_SOURCE
        if build.exists():yield build
        for folder,suffix in [('ui','.slint'),('src','.rs')]:
            yield from (base/folder).rglob('*'+suffix)
    for path in source_paths():
        scanned+=1
        if scanned>MAX_FILES:raise ValueError('source inventory bound')
        if path.is_symlink() or not path.is_file() or not path.resolve().is_relative_to(root):raise ValueError('regular source required')
        relative=str(path.relative_to(root))
        if excluded_source(relative):continue
        budget=min(MAX_FILE_BYTES,MAX_TOTAL-total)
        if path.stat().st_size>budget:raise ValueError('source byte bound')
        # The limited read also handles growth after stat without copying
        # an unbounded repository file before the inventory admission.
        with path.open('rb') as stream:payload=stream.read(budget+1)
        if len(payload)>budget:raise ValueError('source byte bound')
        total+=len(payload)
        files[relative]=payload.decode('utf-8',errors='strict')
    return dict(sorted(files.items()))
