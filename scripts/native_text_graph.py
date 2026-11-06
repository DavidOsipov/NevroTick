"""Closed native text consumer witnesses, independent of locale declarations.

This is bounded lexical source evidence for the selected maintained pipeline,
not Rust name resolution, native execution or a semantic approval grant. Pure
formatters/enum definitions and test-only calls never create a native consumer.
"""
import re

BASE='crates/nevrotick-ui/src/'
MAIN=BASE+'main.rs'
HOST=BASE+'slint_host.rs'
FLOW=BASE+'slint_host/native_workflow.rs'
COPY=BASE+'slint_host/catalog_workflow.rs'
SHELL=BASE+'slint_host/shell_workflow.rs'
TRAY=BASE+'native_tray.rs'
TRAY_OS=BASE+'native_tray/windows.rs'
TRAY_MODEL=BASE+'tray_ui.rs'
NOTICE=BASE+'notifications_ui/native_delivery.rs'
NOTICE_OS=BASE+'native_notifications_windows.rs'
NOTICE_FLOW=BASE+'slint_host/native_notification_workflow.rs'
PRESENTER=BASE+'localization/presentation.rs'
TRAY_LABELS={
 'Open':'ui.tray.menu.open','SyncNow':'ui.fixture.sync-now',
 'CheckSources':'ui.status.check.sources','ManageBackgroundService':'ui.shell.manage-service',
 'Diagnostics':'ui.tray.menu.diagnostics','Settings':'ui.fixture.settings-tab',
 'About':'ui.fixture.about-heading','QuitUi':'ui.shell.quit-ui',
}
NOTICE_FIELDS={'TEST_TITLE':'ui.native-notification.test-title','TEST_BODY':'ui.native-notification.test-body','SETTINGS_LABEL':'ui.shell.notifications'}
NOTICE_FEEDBACK={'NATIVE_NOTIFICATION_TEST_ACCEPTED':'ui.native-notification.test-accepted','NATIVE_NOTIFICATION_TEST_INDETERMINATE':'ui.native-notification.test-indeterminate','NATIVE_NOTIFICATION_TEST_COALESCED':'ui.native-notification.test-coalesced'}
STOP_KEYS=('ui.native-observer.stop-timeout','ui.native-observer.stop-failed')


def native_uses(files,inert_source,without_tests,functions,trace):
    """Return actual native keys and closed pipe faults; no catalog is an input."""
    source={path:without_tests(value)for path,value in files.items()if path.endswith('.rs')}
    cache={};errors=[];uses={};emitters={}
    def bodies(path,name):
        if path not in cache:
            text=source.get(path,'');cache[path]=(text,functions(text))
        text,spans=cache[path]
        owner,method=name.split('::',1)if '::'in name else (None,name)
        if(path,name)==(TRAY,'prepare'):owner='PreparedTray'
        selected=[(text[start:end],start)for function,start,end in spans if function==method]
        if owner is not None:
            # Shared method names require their maintained concrete impl.
            clean=inert_source(text);owners=list(re.finditer(r'\bimpl\s+'+re.escape(owner)+r'\s*\{',clean))
            if len(owners)!=1:return []
            lower=owners[0].end();upper=lower;depth=1
            while upper<len(clean)and depth:
                depth+=int(clean[upper]=='{')-int(clean[upper]=='}');upper+=1
            selected=[(value,start)for value,start in selected if lower<=start<upper]
        return selected
    def body(path,name):
        found=bodies(path,name)
        return found[0][0]if len(found)==1 else ''
    def code(path,name):return inert_source(body(path,name))
    def require(path,name,pattern,role,literals=False):
        text=body(path,name)
        selected=inert_source(text,keep_strings=True)if literals else inert_source(text)
        if not text or re.search(pattern,selected,re.S)is None:
            errors.append({'kind':'native-text-pipeline-mismatch','source':path,'function':name,'role':role});return False
        return True
    def exact_body(path,name,expected,role):
        value=body(path,name);clean=inert_source(value,keep_strings=True)
        start=inert_source(value).find('{');end=inert_source(value).rfind('}')
        tokens=lambda text:re.findall(r'"(?:[^"\\]|\\.)*"|[A-Za-z_][A-Za-z0-9_]*|[0-9]+|[^\s]',text)
        if start<0 or end<start or tokens(clean[start+1:end])!=tokens(expected):
            errors.append({'kind':'native-text-pipeline-mismatch','source':path,'function':name,'role':role})
    def preparation_binding(name,call):
        # Admit only the two selected success bindings and a diverging fixed
        # pre-Show failure. No wildcard may substitute an unrelated document,
        # toast, fallback value or success in the rejected preparation branch.
        failure=r'return\s+RequestOutcome::NotSubmitted\s*\(\s*NativeFailure::Preparation\s*\)'
        legacy=(r'let\s+'+name+r'\s*=\s*match\s+'+call+
                r'\s*\{\s*Ok\s*\(\s*value\s*\)\s*=>\s*value\s*,\s*'
                r'Err\s*\(\s*_\s*\)\s*=>\s*'+failure+r'\s*,\s*\}\s*;')
        let_else=(r'let\s+Ok\s*\(\s*'+name+r'\s*\)\s*=\s*'+call+
                  r'\s*else\s*\{\s*'+failure+r'\s*;?\s*\}\s*;')
        return r'(?:'+legacy+'|'+let_else+')'
    def getter(path,name,expression):
        value=code(path,name);start=value.find('{');end=value.rfind('}')
        if start<0 or end<start or re.fullmatch(r'\s*'+re.escape(expression)+r'\s*',value[start+1:end])is None:
            errors.append({'kind':'native-text-pipeline-mismatch','source':path,'function':name,'role':'exact retained field getter '+expression})
    def location(path,name,sink):
        selected=bodies(path,name);at=selected[0][1]if len(selected)==1 else 0
        return {'kind':'native text sink','path':path,'line':source.get(path,'').count('\n',0,at)+1,'function':name,'native_sink':sink}
    def record(key,path,name,sink):uses.setdefault(key,[]).append(location(path,name,sink))
    # Mere presence of unused native modules must not approve their resources.
    main_native=any(re.search(r'\bnevrotick_ui::slint_host::run_native\s*\(',inert_source(text))for text,_ in bodies(MAIN,'main'))
    install_native=re.search(r'\bnative_workflow::install\s*\(',code(HOST,'create_native'))is not None
    if not main_native and not install_native:return {'uses':{},'errors':[],'emitters':{}}
    common=[main_native,
      require(HOST,'run_native',r'\bcreate_native\s*\(','native entry constructs the selected owner'),
      require(HOST,'create_native',r'\bnative_workflow::install\s*\(','native owner is actually installed'),
      require(FLOW,'install',r'\btimer\.start\s*\(.*\btick\s*\(','owned timer reaches current native dispatcher')]
    if not main_native:errors.append({'kind':'native-text-pipeline-mismatch','source':MAIN,'function':'main','role':'native entry is not called'})
    if not all(common):return {'uses':{},'errors':errors,'emitters':{}}
    # The native lifecycle split moved already-owned recovery feedback into
    # two receiving helpers. Admit those symbols only through actual callbacks.
    lifecycle_start=len(errors)
    for path,name,pattern,role in [
      (HOST,'create',r'\binstall_callbacks\s*\(','ordinary window installs its callbacks'),
      (HOST,'install_callbacks',r'\bshell_workflow::install_callbacks\s*\(','actual shell callback owner is installed'),
      (SHELL,'install_callbacks',r'\.on_window_close_requested\s*\(.*\bclose_allowed\s*\(','real close event reaches owner'),
      (SHELL,'close_allowed',r'\bcomplete_exit_request\s*\(','real close reaches explicit exit completion'),
      (SHELL,'close_allowed',r'\bhide_to_tray\s*\(','real close reaches the actual hide operation'),
      (SHELL,'complete_exit_request',r'\bfinish_exit_request\s*\(','current exit reaches recovery helper'),
      (SHELL,'finish_exit_request',r'\bwrite\s*\(\s*window\s*,\s*state\s*,\s*FeedbackSlot::ShellFeedback\s*,\s*Some\s*\(\s*UiText::fixed\s*\(\s*message\s*\)\s*\)','exit failure has a visible typed destination'),
      (SHELL,'hide_to_tray',r'\bwrite\s*\(\s*window\s*,\s*state\s*,\s*FeedbackSlot::ShellFeedback\s*,\s*Some\s*\(\s*UiText::fixed\s*\(\s*message\s*\)\s*\)','hide failure has a visible typed destination'),
    ]:require(path,name,pattern,role)
    if len(errors)==lifecycle_start:emitters[SHELL]={'finish_exit_request','hide_to_tray'}
    # Tray: actual timer -> private snapshot -> immutable rows -> worker callback
    # -> popup -> AppendMenuW(text pointer). No pure TrayCheckBlock formatter counts.
    tray_start=len(errors)
    for path,name,pattern,role in [
      (FLOW,'tick',r'\bpump_tray\s*\(','actual owner pumps tray'),
      (FLOW,'pump_tray',r'\.copy\s*\.prepare_native_tray\s*\(','actual owner prepares from private catalog'),
      (FLOW,'pump_tray',r'\bNativeTray::start\s*\(','prepared tray has a real worker'),
      (COPY,'prepare_native_tray',r'\bPreparedTray::catalog\s*\(.*&active\.snapshot\s*\(','one active snapshot supplies all labels'),
      (TRAY,'catalog',r'\bUiText::key\s*\(\s*entry\.label\s*\(\s*\)\.catalog_id\s*\(','closed label enum is rendered'),
      (TRAY,'catalog',r'\.render\s*\(\s*snapshot\s*\)','label renderer uses the same snapshot'),
      (TRAY_OS,'start',r'\.spawn\s*\(.*\bworker\s*\(','actual worker is started'),
      (TRAY_OS,'worker',r'lpfnWndProc\s*:\s*Some\s*\(\s*window_proc\s*\)','registered native callback'),
      (TRAY_OS,'window_proc',r'\bdispatch\s*\(','callback reaches current dispatcher'),
      (TRAY_OS,'dispatch',r'\bicon_event\s*\(','icon event is dispatched'),
      (TRAY_OS,'icon_event',r'\bpopup\s*\(','real native menu can be opened'),
      (TRAY_OS,'popup',r'\bpopup_inner\s*\(','popup reaches real menu renderer'),
      (TRAY_OS,'popup_inner',r'for\s+item\s+in\s+projection\.items\s*\(\s*\).*\bAppendMenuW\s*\(\s*menu\s*,\s*flags\s*,\s*id\s*,\s*item\.text\s*\(\s*\)\.units\s*\(\s*\)\.as_ptr\s*\(\s*\)','actual menu text API argument'),
    ]:require(path,name,pattern,role)
    for path,name,pattern,role in [
      (FLOW,'pump_tray',r'let\s+bridge\s*=\s*TrayBridge::new\s*\(\s*prepared\.clone\s*\(\s*\)\s*,\s*wake\s*\)','active prepared labels initialize the actual bridge'),
      (FLOW,'pump_tray',r'NativeTray::start\s*\(\s*Arc::clone\s*\(\s*&bridge\s*\)','selected worker receives that bridge'),
      (FLOW,'pump_tray',r'tray\.port\.update\s*\(\s*prepared\.clone\s*\(\s*\)\s*\)','updates retain current prepared labels'),
      (TRAY,'new',r'Mailbox\s*\{[^}]*\bprojection\s*,','bridge stores the received complete projection'),
      (TRAY,'replace',r'\.projection\s*=\s*projection\s*;','updates replace the retained projection'),
      (TRAY,'prepared_projection',r'\.map\s*\(\s*\|slot\|\s*slot\.projection\.clone\s*\(\s*\)\s*\)','popup reads the retained immutable projection'),
      (TRAY_OS,'update',r'self\.bridge\.replace\s*\(\s*projection\s*\)','actual native update retains its argument'),
      (TRAY_OS,'start',r'let\s+owner\s*=\s*Arc::clone\s*\(\s*&bridge\s*\).*worker\s*\(\s*&owner\s*,','worker owns the selected bridge'),
      (TRAY_OS,'worker',r'CURRENT\.with\s*\(\s*\|slot\|\s*\{\s*\*slot\.borrow_mut\s*\(\s*\)\s*=\s*Some\s*\(\s*Arc::clone\s*\(\s*bridge\s*\)\s*\)','callback association retains the selected bridge'),
      (TRAY_OS,'dispatch',r'let\s+bridge\s*=\s*CURRENT\.with\s*\(\s*\|slot\|\s*slot\.try_borrow\s*\(\s*\)\.ok\s*\(\s*\)\.and_then\s*\(\s*\|slot\|\s*slot\.clone\s*\(\s*\)\s*\)\s*\)','native dispatcher reads that association'),
      (TRAY_OS,'dispatch',r'icon_event\s*\(\s*&bridge\s*,','selected bridge reaches icon callback'),
      (TRAY_OS,'icon_event',r'popup\s*\(\s*bridge\s*,','selected bridge reaches popup'),
      (TRAY_OS,'popup',r'popup_inner\s*\(\s*bridge\s*,','selected bridge reaches actual popup renderer'),
      (TRAY_OS,'popup_inner',r'let\s+projection\s*=\s*bridge\s*\.prepared_projection\s*\(\s*\)','actual rows come from that bridge'),
      (TRAY,'NativeText::prepare',r'for\s+unit\s+in\s+value\.encode_utf16\s*\(\s*\).*result\[used\]\s*=\s*unit\s*;.*Ok\s*\(\s*Self\s*\(\s*result\s*\)\s*\)','bounded native units retain the supplied label'),
    ]:require(path,name,pattern,role)
    for name,expression in [('items','&self.items'),('text','&self.text'),('units','&self.0')]:getter(TRAY,name,expression)
    exact_body(COPY,'prepare_native_tray',"""
        if self.failed { return Err(crate::native_tray::TrayError::Catalog); }
        if let Some(active) = &self.active {
            crate::native_tray::PreparedTray::catalog(menu, interaction, &active.snapshot(), tooltip,)
        } else { crate::native_tray::PreparedTray::fixture(menu, interaction) }
    """,'current catalog result is returned unchanged; engineering only when absent')
    entries=code(TRAY_MODEL,'entries')
    found=re.findall(r'\bTrayItemId::([A-Za-z]+)',entries)
    if found!=['Open','ContextualCheck','Diagnostics','Settings','About','QuitUi']:
        errors.append({'kind':'native-text-pipeline-mismatch','source':TRAY_MODEL,'function':'entries','role':'exact six actual menu rows'})
    if re.search(r'\.map\s*\(\s*\|id\|\s*self\.entry\s*\(\s*id\s*,',entries)is None:
        errors.append({'kind':'native-text-pipeline-mismatch','source':TRAY_MODEL,'function':'entries','role':'actual six row identities reach label selection'})
    expected_labels=['Open','SyncNow','CheckSources','ManageBackgroundService','Diagnostics','Settings','About','QuitUi']
    if re.findall(r'\bTrayLabel::([A-Za-z]+)',code(TRAY_MODEL,'entry'))!=expected_labels:
        errors.append({'kind':'native-text-pipeline-mismatch','source':TRAY_MODEL,'function':'entry','role':'every rendered code-owned menu label has an actual row'})
    require(TRAY,'catalog',r'let\s+entries\s*=\s*menu\.entries\s*\(\s*interaction\s*\)','actual rows feed catalog labels')
    require(TRAY,'catalog',r'let\s+labels\s*=\s*entries\.map\s*\(','every actual row is rendered')
    require(TRAY,'catalog',r'Self::prepare\s*\(\s*entries\s*,\s*labels\s*\.\s*each_ref\s*\(\s*\)\s*\.\s*map\s*\(\s*(?:crate::localization::PlainText::as_str|\|(?P<label_view>[a-z][A-Za-z0-9_]*)\|\s*(?P=label_view)\s*\.\s*as_str\s*\(\s*\))\s*\)\s*,\s*tip\s*\.\s*as_str\s*\(\s*\)','rendered label views feed checked native rows')
    unpack=re.search(r'let\s*\[([^\[\];]+)\]\s*=\s*labels\s*;\s*let\s+labels\s*=\s*\[([^\[\];]+)\]\s*;',code(TRAY,'catalog'))
    names=[];resolved=[]
    if unpack:
        names=[part.strip()for part in unpack[1].split(',')if part.strip()]
        for part in unpack[2].split(','):
            if not part.strip():continue
            found=re.fullmatch(r'\s*([a-z][A-Za-z0-9_]*)\s*\?\s*',part)
            resolved.append(found[1]if found else None)
    if len(names)!=6 or len(set(names))!=6 or any(re.fullmatch(r'[a-z][A-Za-z0-9_]*',name)is None for name in names)or resolved!=names:
        errors.append({'kind':'native-text-pipeline-mismatch','source':TRAY,'function':'catalog','role':'all six rendered label results retain their original row order'})
    require(TRAY,'prepare',r'entries\s*\.into_iter\s*\(\s*\)\s*\.zip\s*\(\s*labels\s*\).*NativeText::prepare\s*\(\s*text\s*,\s*true\s*\).*NativeItem\s*\{[^}]*\btext\s*,','native rows retain the prepared label')
    # Read only the named TrayLabel implementation, never all catalog_id methods.
    model=source.get(TRAY_MODEL,'');clean=inert_source(model);impl=re.search(r'\bimpl\s+TrayLabel\s*\{',clean)
    label_ids={}
    if impl:
        depth=1;end=impl.end()
        while end<len(clean)and depth:
            depth+=int(clean[end]=='{')-int(clean[end]=='}');end+=1
        segment=inert_source(model[impl.end():end-1],keep_strings=True)
        label_ids=dict(re.findall(r'\bSelf::([A-Za-z]+)\s*=>\s*"(ui\.[a-z0-9.-]+)"',segment))
    if label_ids!=TRAY_LABELS:
        errors.append({'kind':'native-text-pipeline-mismatch','source':TRAY_MODEL,'function':'TrayLabel::catalog_id','role':'fixed native label/action meanings'})
    if len(errors)==tray_start:
        for key in TRAY_LABELS.values():record(key,TRAY_OS,'popup_inner','AppendMenuW item.text().units()')
    # Toast: real callback and actual current snapshot preparation precede the
    # DOM title/body/action values, then the guarded OS Show operation.
    notice_start=len(errors)
    for path,name,pattern,role in [
      (FLOW,'install',r'\bnative_notification_workflow::install\s*\(','actual native notification owner is installed'),
      (SHELL,'install_settings_callbacks',r'\.on_notification_test_requested\s*\(.*\bnative_notification_workflow::request_test\s*\(','real button callback reaches native receiver'),
      (NOTICE_FLOW,'request_test',r'\brequest_native\s*\(','guarded receiver reaches native test request'),
      (NOTICE_FLOW,'request_native',r'\.copy\s*\.prepare_native_notification\s*\(','request uses current private catalog'),
      (NOTICE_FLOW,'request_native',r'\bport\.request_test\s*\(\s*&prepared\s*,','prepared fields reach actual adapter'),
      (COPY,'prepare_native_notification',r'\bPreparedTestNotice::from_snapshot\s*\(\s*&active\.snapshot\s*\(','active snapshot supplies every native field'),
      (NOTICE_OS,'request_test',r'\bdocument\s*\(\s*prepared\s*,','actual request builds document from checked fields'),
      (NOTICE_OS,'request_test',r'\bToastNotification::CreateToastNotification\s*\(','real notification object is constructed'),
      (NOTICE_OS,'request_test',r'\.notifier\.Show\s*\(\s*&active\.toast','real guarded native submission sink'),
      (NOTICE_OS,'document',r'for\s+content\s+in\s*\[\s*prepared\.title\s*\(\s*\)\s*,\s*prepared\.body\s*\(\s*\)\s*\].*\.CreateTextNode\s*\(\s*&HSTRING::from\s*\(\s*content\s*\)','actual title/body text node arguments'),
    ]:require(path,name,pattern,role)
    for path,name,pattern,role in [
      (NOTICE,'from_snapshot',r'let\s+render\s*=\s*\|key\|\s*\{\s*snapshot\s*\.text\s*\(\s*MessageId::from_key\s*\(\s*key\s*\)','closed keys are looked up in the selected snapshot'),
      (NOTICE,'checked_parts',r'Ok\s*\(\s*Self\s*\{\s*notice\s*,\s*title\s*:\s*title\.to_owned\s*\(\s*\)\s*,\s*body\s*:\s*body\.to_owned\s*\(\s*\)\s*,\s*action\s*:\s*action\.to_owned\s*\(\s*\)\s*,?\s*\}\s*\)','checked fields retain their exact roles'),
      (NOTICE_OS,'request_test',preparation_binding('document',r'document\s*\(\s*prepared\s*,\s*token\s*\)'), 'current prepared fields supply the selected document'),
      (NOTICE_OS,'request_test',preparation_binding('toast',r'ToastNotification::CreateToastNotification\s*\(\s*&document\s*\)'), 'that document supplies the selected toast'),
      (NOTICE_OS,'request_test',r'let\s+mut\s+active\s*=\s*ActiveToast\s*\{\s*toast\s*,','actual active object retains the selected toast'),
      (NOTICE_OS,'document',r'let\s+doc\s*=\s*XmlDocument::new\s*\(\s*\)','one document owns the selected nodes'),
      (NOTICE_OS,'document',r'let\s+text\s*=\s*doc\.CreateTextNode\s*\(\s*&HSTRING::from\s*\(\s*content\s*\)\s*\).*element\.AppendChild\s*\(\s*&text\s*\).*binding\.AppendChild\s*\(\s*&element\s*\)','title/body nodes join the binding'),
      (NOTICE_OS,'document',r'visual\.AppendChild\s*\(\s*&binding\s*\).*toast\.AppendChild\s*\(\s*&visual\s*\)','binding joins the toast'),
      (NOTICE_OS,'document',r'actions\.AppendChild\s*\(\s*&action\s*\).*toast\.AppendChild\s*\(\s*&actions\s*\).*doc\.AppendChild\s*\(\s*&toast\s*\).*Ok\s*\(\s*doc\s*\)','localized action joins the returned document'),
    ]:require(path,name,pattern,role)
    for name,expression in [('title','&self.title'),('body','&self.body'),('action_label','&self.action')]:getter(NOTICE,name,expression)
    exact_body(COPY,'prepare_native_notification',"""
        use crate::notifications_ui::native_delivery::{NoticeError, PreparedTestNotice};
        if self.failed { return Err(NoticeError::Catalog(Error::Schema)); }
        self.active.as_ref()
            .map(|active| PreparedTestNotice::from_snapshot(&active.snapshot(), notice))
            .transpose()
    """,'active preparation is returned, never collapsed into absent catalog')
    exact_body(NOTICE_FLOW,'choose_prepared',"""
        match active? {
            Some(prepared) => Ok(prepared),
            None if content == NoticeContent::EngineeringExample => {
                PreparedTestNotice::engineering_example(notice)
            }
            None => Err(NoticeError::Catalog(crate::localization::Error::Schema)),
        }
    """,'active checked notice retained; fixture admitted only when explicitly absent')
    require(NOTICE_FLOW,'request_native',r'let\s+prepared\s*=\s*\{.*choose_prepared\s*\(\s*owner\.copy\.prepare_native_notification\s*\(\s*notice\s*\)\s*,\s*notice\s*,\s*owner\.notifications\.platform\.content\s*,?\s*\)\s*\}\s*;\s*let\s+Ok\s*\(\s*prepared\s*\)\s*=\s*prepared','actual port argument retains selected catalog preparation')
    comments=inert_source(source.get(NOTICE,''),keep_strings=True)
    constants=dict(re.findall(r'\bconst\s+(TEST_TITLE|TEST_BODY|SETTINGS_LABEL)\s*:\s*&str\s*=\s*"(ui\.[a-z0-9.-]+)"',comments))
    if constants!=NOTICE_FIELDS:errors.append({'kind':'native-text-pipeline-mismatch','source':NOTICE,'function':'from_snapshot','role':'closed native field identities'})
    for variable,constant in [('title','TEST_TITLE'),('body','TEST_BODY'),('action','SETTINGS_LABEL')]:
        require(NOTICE,'from_snapshot',r'let\s+'+variable+r'\s*=\s*render\s*\(\s*'+constant+r'\s*\)','same-snapshot '+variable+' identity')
    require(NOTICE,'from_snapshot',r'Self::checked_parts\s*\(\s*notice\s*,\s*title\.as_str\s*\(\s*\)\s*,\s*body\.as_str\s*\(\s*\)\s*,\s*action\.as_str\s*\(\s*\)','native fields retain their roles')
    require(NOTICE_OS,'document',r'\.SetAttribute\s*\(\s*&HSTRING::from\s*\(\s*"content"\s*\)\s*,\s*&HSTRING::from\s*\(\s*prepared\.action_label\s*\(\s*\)','actual fixed action-label attribute',True)
    if len(errors)==notice_start:
        for constant,key in NOTICE_FIELDS.items():record(key,NOTICE_OS,'document','CreateTextNode '+constant if constant!='SETTINGS_LABEL'else'SetAttribute content/action_label')
    if len(errors)==notice_start:
        rejection_ok=require(NOTICE_FLOW,'request_test',r'\bblocked\s*\(','actual unavailable/rejected request reaches typed feedback')
        rejection_ok=require(NOTICE_FLOW,'blocked',r'UiText::notification_feedback\s*\(\s*block\s*\)','closed rejection semantics')and rejection_ok
        rejection_ok=require(NOTICE_FLOW,'blocked',r'\bwrite\s*\(\s*window\s*,\s*state\s*,\s*FeedbackSlot::NotificationFeedback\s*,\s*Some\s*\(\s*UiText::notification_feedback\s*\(\s*block\s*\)\s*\)','real rejection destination')and rejection_ok
        if rejection_ok:emitters[NOTICE_FLOW]={'blocked','attempt_feedback'}
    # Native-owned feedback is a real Slint sink, requiring the admitted host
    # callback/caller chain as well as its closed constructor and slot.
    if len(errors)==notice_start:
        feedback_ok=require(NOTICE_FLOW,'request_native',r'\battempt_feedback\s*\(','actual request projects sticky feedback')
        feedback_ok=require(NOTICE_FLOW,'attempt_feedback',r'UiText::notification_test_feedback\s*\(','closed native feedback factory')and feedback_ok
        feedback_ok=require(NOTICE_FLOW,'attempt_feedback',r'\bwrite\s*\(\s*window\s*,\s*state\s*,\s*FeedbackSlot::NotificationFeedback\s*,\s*Some\s*\(\s*message\s*\)','existing typed feedback destination')and feedback_ok
        factory=code(PRESENTER,'notification_test_feedback')
        for symbol in NOTICE_FEEDBACK:
            feedback_ok=bool(re.search(r'\bMessageId::'+symbol+r'\b',factory))and feedback_ok
        if feedback_ok and trace.reaches('AppWindow','notification-feedback'):
            for key in NOTICE_FEEDBACK.values():record(key,NOTICE_FLOW,'attempt_feedback','AppWindow.notification-feedback actual text leaf')
    stop_ok=require(FLOW,'tick',r'\bnative_stop_feedback\s*\(','actual stop outcome projects feedback')
    stop_ok=require(SHELL,'native_stop_feedback',r'\bUiText::key\s*\(\s*key\s*\)','closed stop keys are retained')and stop_ok
    stop_ok=require(SHELL,'native_stop_feedback',r'\bwrite\s*\(\s*window\s*,\s*state\s*,\s*FeedbackSlot::ShellFeedback\s*,\s*Some\s*\(\s*message\s*\)','existing stop feedback destination')and stop_ok
    stop_source=inert_source(body(SHELL,'native_stop_feedback'),keep_strings=True)
    stop_keys=set(re.findall(r'"(ui\.native-observer\.[a-z-]+)"',stop_source))
    if stop_keys!=set(STOP_KEYS):
        errors.append({'kind':'native-text-pipeline-mismatch','source':SHELL,'function':'native_stop_feedback','role':'closed stop outcome keys'});stop_ok=False
    if stop_ok and trace.reaches('AppWindow','shell-feedback'):
        for key in STOP_KEYS:record(key,SHELL,'native_stop_feedback','AppWindow.shell-feedback actual text leaf')
    return {'uses':uses,'errors':errors,'emitters':emitters}
