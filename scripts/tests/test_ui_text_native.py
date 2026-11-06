"""Causal controls for actual native source witnesses and both coverage directions."""
from pathlib import Path
import base64,copy,hashlib,sys,unittest
from unittest.mock import patch
sys.path.insert(0,str(Path(__file__).resolve().parents[1]))
import check_ui_text_coverage as c
import native_text_graph as n
import slint_text_graph
import protected_json

FIXTURE=Path(__file__).with_name('fixtures')/'ui-text-native-source-v2.json'

def fixture():
    with FIXTURE.open('rb')as stream:raw=stream.read(1_000_001)
    if len(raw)>1_000_000:raise ValueError('source fixture byte bound')
    value=protected_json.read(raw)
    if set(value)!={'schema_version','scope','encoding','source_sha256','files'}or type(value['schema_version'])is not int or value['schema_version']!=1:raise ValueError('closed source fixture')
    if value['encoding']!='base64-chunks-v1':raise ValueError('closed source encoding')
    files={path:base64.b64decode(''.join(chunks),validate=True).decode('utf-8','strict')for path,chunks in value['files'].items()}
    if len(files)>c.MAX_FILES or sum(len(x.encode())for x in files.values())>c.MAX_TOTAL:raise ValueError('source fixture bounds')
    if{path:hashlib.sha256(text.encode()).hexdigest()for path,text in files.items()}!=value['source_sha256']:raise ValueError('frozen source fixture identity')
    return files


def result(files):
    trace=slint_text_graph.build(files,c.inert_source)
    return n.native_uses(files,c.inert_source,c.without_test_items,c.functions,trace)


def change(files,path,old,new,count=1):
    if files[path].count(old)<count:raise ValueError('causal mutation target missing')
    files[path]=files[path].replace(old,new,count)


def selected_notice_let_else(files,names=('document','toast')):
    for name,call in [('document','document(prepared, token)'),
                      ('toast','ToastNotification::CreateToastNotification(&document)')]:
        if name not in names:continue
        old=('let '+name+' = match '+call+' {\n'
             '            Ok(value) => value,\n'
             '            Err(_) => return RequestOutcome::NotSubmitted(NativeFailure::Preparation),\n'
             '        };')
        new=('let Ok('+name+') = '+call+' else {\n'
             '            return RequestOutcome::NotSubmitted(NativeFailure::Preparation);\n'
             '        };')
        change(files,n.NOTICE_OS,old,new)


class NativeTextCoverage(unittest.TestCase):
    def setUp(self):self.files=fixture()

    def test_actual_current_host_and_native_api_arguments_are_healthy(self):
        value=result(self.files)
        self.assertEqual(value['errors'],[])
        self.assertEqual(len(value['uses']),16)
        self.assertIn('ui.tray.menu.open',value['uses'])
        self.assertIn('ui.native-notification.test-title',value['uses'])
        self.assertIn('ui.native-notification.test-coalesced',value['uses'])
        self.assertIn('ui.native-observer.stop-failed',value['uses'])
        self.assertNotIn('ui.tray.check.none',value['uses'])
        self.assertEqual(value['emitters'][n.SHELL],{'finish_exit_request','hide_to_tray'})
        self.assertEqual(value['emitters'][n.NOTICE_FLOW],{'blocked','attempt_feedback'})

    def test_unused_pure_native_modules_do_not_create_consumers(self):
        del self.files[n.MAIN];del self.files[n.HOST]
        self.assertEqual(result(self.files),{'uses':{},'errors':[],'emitters':{}})

    def test_missing_real_entry_fails_even_when_formatter_definitions_remain(self):
        change(self.files,n.MAIN,'nevrotick_ui::slint_host::run_native','unused::run_native')
        value=result(self.files)
        self.assertEqual(value['uses'],{})
        self.assertTrue(value['errors'])

    def test_test_only_and_comment_entry_cannot_supply_product_root(self):
        self.files[n.MAIN]='// nevrotick_ui::slint_host::run_native();\n#[cfg(test)] fn main() { nevrotick_ui::slint_host::run_native(); }'
        self.assertEqual(result(self.files)['uses'],{})

    def test_host_install_is_required(self):
        change(self.files,n.HOST,'native_workflow::install','unused::install')
        self.assertEqual(result(self.files)['uses'],{})

    def test_menu_must_pass_actual_prepared_label_to_windows_api(self):
        change(self.files,n.TRAY_OS,'item.text().units().as_ptr()','other.text().units().as_ptr()')
        value=result(self.files)
        self.assertNotIn('ui.tray.menu.open',value['uses'])
        self.assertIn('ui.native-notification.test-title',value['uses'])
        self.assertTrue(value['errors'])

    def test_all_six_actual_menu_rows_must_reach_selection(self):
        change(self.files,n.TRAY_MODEL,'TrayItemId::Diagnostics,','TrayItemId::Settings,')
        self.assertNotIn('ui.tray.menu.diagnostics',result(self.files)['uses'])

    def test_catalog_id_without_actual_row_cannot_activate_a_label(self):
        change(self.files,n.TRAY_MODEL,'TrayItemId::Diagnostics => TrayLabel::Diagnostics','TrayItemId::Diagnostics => TrayLabel::Settings')
        self.assertNotIn('ui.tray.menu.diagnostics',result(self.files)['uses'])

    def test_checked_native_item_must_keep_rendered_text(self):
        change(self.files,n.TRAY,'NativeText::prepare(text, true)','NativeText::prepare(other, true)')
        self.assertNotIn('ui.tray.menu.open',result(self.files)['uses'])

    def test_label_identity_swap_is_not_hidden_by_equal_row_count(self):
        change(self.files,n.TRAY_MODEL,'Self::Diagnostics => "ui.tray.menu.diagnostics"','Self::Diagnostics => "ui.fixture.settings-tab"')
        self.assertNotIn('ui.tray.menu.diagnostics',result(self.files)['uses'])

    def test_label_result_permutation_cannot_swap_open_and_quit(self):
        change(self.files,n.TRAY,'let labels = [a?, b?, c?, d?, e?, f?];',
               'let labels = [f?, b?, c?, d?, e?, a?];')
        self.assertNotIn('ui.tray.menu.open',result(self.files)['uses'])

    def test_label_map_cannot_replace_prepared_text_with_a_literal(self):
        change(self.files,n.TRAY,'labels.each_ref().map(|text| text.as_str())',
               'labels.each_ref().map(|_text| "Fixed label")')
        self.assertNotIn('ui.tray.menu.open',result(self.files)['uses'])

    def test_rustfmt_method_chain_and_known_plain_text_view_are_healthy(self):
        change(self.files,n.TRAY,'labels.each_ref().map(|text| text.as_str())',
               'labels\n .each_ref()\n .map(crate::localization::PlainText::as_str)')
        self.assertEqual(result(self.files)['errors'],[])

    def test_toast_requires_actual_show_not_a_comment(self):
        change(self.files,n.NOTICE_OS,'.notifier.Show(','.notifier.UnusedShow(')
        self.files[n.NOTICE_OS]+='\n// .notifier.Show(&active.toast)\n'
        value=result(self.files)
        self.assertNotIn('ui.native-notification.test-title',value['uses'])
        self.assertIn('ui.tray.menu.open',value['uses'])

    def test_title_and_body_roles_cannot_be_swapped(self):
        change(self.files,n.NOTICE,'let title = render(TEST_TITLE)','let title = render(TEST_BODY)')
        self.assertNotIn('ui.native-notification.test-title',result(self.files)['uses'])

    def test_title_body_values_must_reach_dom_text_node_argument(self):
        change(self.files,n.NOTICE_OS,'CreateTextNode(&HSTRING::from(content))','CreateTextNode(&HSTRING::from(other))')
        self.assertNotIn('ui.native-notification.test-body',result(self.files)['uses'])

    def test_action_label_must_reach_fixed_content_attribute(self):
        change(self.files,n.NOTICE_OS,'HSTRING::from(prepared.action_label())','HSTRING::from(other.action_label())')
        self.assertNotIn('ui.shell.notifications',result(self.files)['uses'])

    def test_only_code_owned_notice_keys_can_fill_native_fields(self):
        change(self.files,n.NOTICE,'ui.native-notification.test-title','ui.native-notification.unowned-title')
        self.assertNotIn('ui.native-notification.test-title',result(self.files)['uses'])

    def test_sticky_feedback_needs_real_constructor(self):
        change(self.files,n.NOTICE_FLOW,'UiText::notification_test_feedback(','UiText::unused_feedback(')
        self.assertNotIn('ui.native-notification.test-accepted',result(self.files)['uses'])

    def test_sticky_feedback_needs_an_actual_window_text_leaf(self):
        path='crates/nevrotick-ui/ui/settings.slint'
        change(self.files,path,'text: root.notification-feedback','text: PreferencesCopy.notifications-help')
        self.assertNotIn('ui.native-notification.test-accepted',result(self.files)['uses'])

    def test_stop_feedback_needs_real_timer_caller(self):
        source=self.files[n.FLOW];spans=c.functions(source)
        name,start,end=next(x for x in spans if x[0]=='tick')
        self.files[n.FLOW]=source[:start]+source[start:end].replace('native_stop_feedback','unused_feedback')+source[end:]
        self.assertNotIn('ui.native-observer.stop-timeout',result(self.files)['uses'])

    def test_notice_getter_cannot_replace_catalog_body_with_fixed_words(self):
        change(self.files,n.NOTICE,'        &self.body','        "fixed body"')
        self.assertNotIn('ui.native-notification.test-body',result(self.files)['uses'])

    def test_notice_storage_cannot_put_title_into_body(self):
        change(self.files,n.NOTICE,'body: body.to_owned()','body: title.to_owned()')
        self.assertNotIn('ui.native-notification.test-body',result(self.files)['uses'])

    def test_toast_constructor_must_receive_selected_document(self):
        change(self.files,n.NOTICE_OS,'CreateToastNotification(&document)','CreateToastNotification(&XmlDocument::new()?)')
        self.assertNotIn('ui.native-notification.test-body',result(self.files)['uses'])

    def test_selected_preparation_let_else_forms_are_healthy(self):
        for names in [('document',),('toast',),('document','toast')]:
            with self.subTest(bindings=names):
                files=copy.deepcopy(self.files);selected_notice_let_else(files,names)
                self.assertEqual(result(files)['errors'],[])
                self.assertEqual(len(result(files)['uses']),16)
        files=copy.deepcopy(self.files);selected_notice_let_else(files)
        change(files,n.NOTICE_OS,'let Ok(document) = document(prepared, token) else {',
               'let Ok(\n document\n) = document(\n prepared, token\n) else {')
        change(files,n.NOTICE_OS,'let Ok(toast) = ToastNotification::CreateToastNotification(&document) else {',
               'let Ok(toast) =\n ToastNotification::CreateToastNotification(\n &document\n) else {')
        self.assertEqual(result(files)['errors'],[])

    def test_let_else_document_requires_current_prepared_arguments(self):
        for call in ['document(other, token)','document(prepared, other)']:
            with self.subTest(call=call):
                files=copy.deepcopy(self.files);selected_notice_let_else(files)
                change(files,n.NOTICE_OS,'document(prepared, token)',call)
                self.assertNotIn('ui.native-notification.test-body',result(files)['uses'])
                self.assertTrue(result(files)['errors'])

    def test_let_else_toast_requires_selected_document(self):
        selected_notice_let_else(self.files)
        change(self.files,n.NOTICE_OS,'CreateToastNotification(&document)',
               'CreateToastNotification(&XmlDocument::new()?)')
        self.assertNotIn('ui.native-notification.test-body',result(self.files)['uses'])
        self.assertTrue(result(self.files)['errors'])

    def test_let_else_preparation_failure_must_diverge_with_fixed_outcome(self):
        failure='return RequestOutcome::NotSubmitted(NativeFailure::Preparation);'
        for replacement in [failure.removeprefix('return '),
                            'return RequestOutcome::NotSubmitted(NativeFailure::CleanupPending);',
                            'return RequestOutcome::Attempted(Submission::AcceptedByOs);']:
            for name in ['document','toast']:
                with self.subTest(binding=name,replacement=replacement):
                    files=copy.deepcopy(self.files);selected_notice_let_else(files,(name,))
                    change(files,n.NOTICE_OS,failure,replacement)
                    self.assertNotIn('ui.native-notification.test-body',result(files)['uses'])
                    self.assertTrue(result(files)['errors'])

    def test_let_else_selected_binding_cannot_be_substituted(self):
        for name in ['document','toast']:
            with self.subTest(binding=name):
                files=copy.deepcopy(self.files);selected_notice_let_else(files)
                change(files,n.NOTICE_OS,'let Ok('+name+') =','let Ok(other) =')
                self.assertNotIn('ui.native-notification.test-body',result(files)['uses'])
                self.assertTrue(result(files)['errors'])

    def test_legacy_preparation_failure_cannot_be_replaced_with_success(self):
        change(self.files,n.NOTICE_OS,
               'Err(_) => return RequestOutcome::NotSubmitted(NativeFailure::Preparation),',
               'Err(_) => return RequestOutcome::Attempted(Submission::AcceptedByOs),')
        self.assertNotIn('ui.native-notification.test-body',result(self.files)['uses'])
        self.assertTrue(result(self.files)['errors'])

    def test_native_bridge_must_start_with_current_prepared_catalog_labels(self):
        change(self.files,n.FLOW,'TrayBridge::new(prepared.clone(), wake)','TrayBridge::new(PreparedTray::fixture(menu, interaction).unwrap(), wake)')
        self.assertNotIn('ui.tray.menu.open',result(self.files)['uses'])

    def test_native_bridge_getter_cannot_return_unrelated_projection(self):
        change(self.files,n.TRAY,'map(|slot| slot.projection.clone())','map(|slot| other.clone())')
        self.assertNotIn('ui.tray.menu.open',result(self.files)['uses'])

    def test_native_row_getter_cannot_hide_rendered_label(self):
        change(self.files,n.TRAY,'        &self.text','        &self.other')
        self.assertNotIn('ui.tray.menu.open',result(self.files)['uses'])

    def test_dom_text_nodes_must_join_returned_document(self):
        change(self.files,n.NOTICE_OS,'binding.AppendChild(&element)','binding.AppendChild(&other)')
        self.assertNotIn('ui.native-notification.test-body',result(self.files)['uses'])

    def test_toast_active_record_must_keep_selected_toast(self):
        change(self.files,n.NOTICE_OS,'            toast,','            toast: other,')
        self.assertNotIn('ui.native-notification.test-body',result(self.files)['uses'])

    def test_feedback_constructor_cannot_be_discarded_before_write(self):
        for path,name,key in [(n.NOTICE_FLOW,'attempt_feedback','ui.native-notification.test-accepted'),
                              (n.SHELL,'native_stop_feedback','ui.native-observer.stop-failed')]:
            with self.subTest(owner=name):
                files=copy.deepcopy(self.files);source=files[path]
                _name,start,end=next(x for x in c.functions(source)if x[0]==name)
                selected=source[start:end]
                self.assertIn('Some(message)',selected)
                files[path]=source[:start]+selected.replace('Some(message)','None')+source[end:]
                self.assertNotIn(key,result(files)['uses'])

    def test_present_active_catalog_cannot_be_replaced_with_engineering_example(self):
        change(self.files,n.NOTICE_FLOW,'Some(prepared) => Ok(prepared)','Some(_prepared) => PreparedTestNotice::engineering_example(notice)')
        self.assertNotIn('ui.native-notification.test-body',result(self.files)['uses'])

    def test_active_catalog_helper_cannot_collapse_checked_notice_to_absent(self):
        change(self.files,n.COPY,'PreparedTestNotice::from_snapshot(&active.snapshot(), notice)','PreparedTestNotice::engineering_example(notice)')
        self.assertNotIn('ui.native-notification.test-body',result(self.files)['uses'])

    def test_native_hook_preserves_forward_and_reverse_gates(self):
        key='ui.native-notification.test-title'
        contract={'text_contract_version':1,'messages':[{'key':key,'symbol':'NOTICE','active':True,'kind':'Text','reference':{'text':'Notice'}}]}
        files={'crates/nevrotick-ui/ui/app.slint':'export component AppWindow inherits Window {}'}
        observed={'uses':{key:[{'kind':'unit test native merge witness'}]},'errors':[],'emitters':{}}
        # These controls isolate the merge, independent of the actual-pipeline
        # fixture controls above. No mocked result is admitted by product code.
        with patch.object(n,'native_uses',return_value=observed):
            self.assertTrue(c.check(contract,files)['ok'])
            missing=copy.deepcopy(contract);missing['messages']=[]
            self.assertTrue(any(x['kind']=='missing-native-text'for x in c.check(missing,files)['errors']))
        with patch.object(n,'native_uses',return_value={'uses':{},'errors':[],'emitters':{}}):
            self.assertIn({'kind':'orphan-resource-id','key':key},c.check(contract,files)['errors'])


if __name__=='__main__':unittest.main()
