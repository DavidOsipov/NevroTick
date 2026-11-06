"""Independent healthy/fault controls for source-to-text coverage bookkeeping."""
import copy,unittest,tempfile,importlib.util,io,json
from contextlib import redirect_stdout
from unittest.mock import patch
from pathlib import Path
import sys
import os
import subprocess
sys.path.insert(0,str(Path(__file__).resolve().parents[1]))
import check_ui_text_coverage as checker


def fixture():
    contract={'text_contract_version':1,'messages':[{'key':'ui.fixture.heading','symbol':'SAMPLE_HEADING','active':True,'kind':'Text','reference':{'text':'Heading'},'slint':{'global':'FixtureCopy','property':'heading'}}]}
    files={'crates/nevrotick-ui/ui/app.slint':'export component AppWindow inherits Window { Text { text: FixtureCopy.heading; accessible-label: FixtureCopy.heading; } }'}
    return contract,files


class Coverage(unittest.TestCase):
    def test_literal_false_native_control_cannot_consume_the_only_id(self):
        for control in ['Text','Button']:
            with self.subTest(control=control):
                contract,files=fixture();path=next(iter(files))
                files[path]='export component AppWindow inherits Window { if false: '+control+' { text: FixtureCopy.heading; } }'
                self.assertIn({'kind':'orphan-resource-id','key':'ui.fixture.heading'},checker.check(contract,files)['errors'])

    def test_literal_false_container_prunes_its_complete_children_subtree(self):
        for before,element in [('', 'Rectangle'),('component Pane inherits Rectangle { @children }','Pane')]:
            with self.subTest(element=element):
                contract,files=fixture();path=next(iter(files))
                files[path]=before+' export component AppWindow inherits Window { if false: '+element+' { Text { text: FixtureCopy.heading; } } }'
                self.assertIn({'kind':'orphan-resource-id','key':'ui.fixture.heading'},checker.check(contract,files)['errors'])

    def test_literal_false_custom_instance_and_definition_do_not_create_a_sink(self):
        for source in [
            'component Pane inherits Rectangle { Text { text: FixtureCopy.heading; } } export component AppWindow inherits Window { if false: Pane {} }',
            'component Pane inherits Rectangle { in property<string> caption; Text { text: root.caption; } } export component AppWindow inherits Window { if false: Pane { caption: FixtureCopy.heading; } }',
            'component Pane inherits Rectangle { in property<string> caption: FixtureCopy.heading; if false: Text { text: root.caption; } } export component AppWindow inherits Window { Pane {} }',
            'component Pane inherits Rectangle { in property<string> caption: FixtureCopy.heading; Text { text: root.caption; } } export component AppWindow inherits Window { if false: Pane {} }',
            'component Base inherits Rectangle { if false: Text { text: FixtureCopy.heading; } } component Derived inherits Base {} export component AppWindow inherits Window { Derived {} }',
        ]:
            with self.subTest(source=source):
                contract,files=fixture();files[next(iter(files))]=source
                self.assertIn({'kind':'orphan-resource-id','key':'ui.fixture.heading'},checker.check(contract,files)['errors'])

    def test_true_and_dynamic_conditions_keep_their_actual_possible_consumers(self):
        for condition in ['true','root.show','!false','false || root.show']:
            with self.subTest(condition=condition):
                contract,files=fixture();path=next(iter(files))
                files[path]='component Pane inherits Rectangle { in property<string> caption; Text { text: root.caption; } } export component AppWindow inherits Window { in property<bool> show; if '+condition+': Pane { caption: FixtureCopy.heading; } }'
                self.assertTrue(checker.check(contract,files)['ok'])

    def test_literal_false_recognition_survives_comments_spacing_and_parentheses(self):
        for condition,between in [('false',' '),('(false)',' '),('( ( false ) )',' '),('false',' '*500),('false',' hidden-name := '+' '*500),('false /* fixed source comment */','\n'*300)]:
            with self.subTest(condition=condition,spacing=len(between)):
                contract,files=fixture();path=next(iter(files))
                header=between+'Text'if ':='in between else between+'hidden := Text'
                files[path]='export component AppWindow inherits Window { if '+condition+':'+header+' { text: FixtureCopy.heading; } }'
                self.assertIn({'kind':'orphan-resource-id','key':'ui.fixture.heading'},checker.check(contract,files)['errors'])

    def test_literal_false_condition_pruning_does_not_inherit_the_type_lookback_limit(self):
        for name,gap in [('Rectangle',' '*500),('A'+'x'*500,' ')]:
            with self.subTest(name_length=len(name),gap=len(gap)):
                contract,files=fixture();path=next(iter(files))
                files[path]='export component AppWindow inherits Window { if false: '+name+gap+'{ Text { text: FixtureCopy.heading; } } }'
                self.assertIn({'kind':'orphan-resource-id','key':'ui.fixture.heading'},checker.check(contract,files)['errors'])

    def test_dead_subtree_creates_no_required_binding_or_unknown_copy_errors(self):
        contract,files=fixture();path=next(iter(files))
        files[path]=files[path][:-1]+' if false: Rectangle { Button {} Text { text: MissingCopy.future; } } }'
        self.assertTrue(checker.check(contract,files)['ok'])

    def test_literal_false_subtree_still_consumes_the_existing_parser_budget(self):
        contract,files=fixture();path=next(iter(files))
        files[path]='export component AppWindow inherits Window { if false: Rectangle { Text {} Button {} } }'
        with patch.object(checker.slint_text_graph,'MAX_NODES',3):
            with self.assertRaisesRegex(ValueError,'Slint node bound'):checker.check(contract,files)

    def test_literal_false_header_cannot_fabricate_an_enclosing_property_binding(self):
        contract,files=fixture();path=next(iter(files))
        files[path]='export component AppWindow inherits Window {\n            in property<string> false;\n            in property<string> unused: FixtureCopy.heading;\n            Text { text: root.false; }\n            if false: Text { text: FixtureCopy.heading; }\n        }'
        report=checker.check(contract,files)
        self.assertIn({'kind':'orphan-resource-id','key':'ui.fixture.heading'},report['errors'])
        self.assertTrue(any(error['kind']=='unowned-required-ui-source'for error in report['errors']))

    def test_literal_false_menu_subtree_cannot_consume_an_id(self):
        contract,files=fixture();path=next(iter(files))
        files[path]='export component AppWindow inherits Window { if false: ContextMenuArea { Menu { title: FixtureCopy.heading; MenuItem { title: FixtureCopy.heading; } } } }'
        self.assertIn({'kind':'orphan-resource-id','key':'ui.fixture.heading'},checker.check(contract,files)['errors'])

    def test_bound_but_unowned_blank_alias_cannot_name_a_control(self):
        for declaration,control in [('', 'Button { text: root.missing; }'),
                                    ('', 'Text { text: root.missing; }'),
                                    ('component ActionButton inherits Button { in property<string> caption; text: root.caption; }',
                                     'ActionButton { caption: root.missing; }')]:
            with self.subTest(control=control):
                contract,files=fixture();key=next(iter(files))
                files[key]=declaration+files[key].replace('inherits Window {','inherits Window { in property<string> missing;',1)[:-1]+control+' }'
                self.assertTrue(any(error['kind']=='unowned-required-ui-source'for error in checker.check(contract,files)['errors']))

    def test_required_runtime_label_needs_its_observed_typed_owner(self):
        contract,files=fixture();key=next(iter(files))
        files[key]=files[key].replace('inherits Window {','inherits Window { in property<string> status-check-label;',1)[:-1]+'Button { text: root.status-check-label; } }'
        sinks={'ui.fixture.heading':[('AppWindow','status-check-label')]}
        owner='crates/nevrotick-ui/src/slint_host/source_workflow.rs'
        files[owner]='fn source_request() { write(MessageId::SAMPLE_HEADING); }'
        self.assertTrue(checker.check(contract,files,runtime_sinks=sinks)['ok'])
        files[owner]='// fn source_request() { write(MessageId::SAMPLE_HEADING); }'
        self.assertTrue(any(error['kind']=='unowned-required-ui-source'for error in checker.check(contract,files,runtime_sinks=sinks)['errors']))

    def test_new_text_controls_cannot_omit_their_required_effective_binding(self):
        for declaration,control in [('', 'Button {}'),('', 'Text {}'),
                                    ('component ActionButton inherits Button {}','ActionButton {}'),
                                    ('component Label inherits Text {}','Label {}'),
                                    ('','CheckBox {}'),('','ComboBox {}'),
                                    ('','ContextMenuArea { Menu { MenuItem {} } }')]:
            with self.subTest(control=control):
                contract,files=fixture();key=next(iter(files))
                files[key]=declaration+files[key][:-1]+control+' }'
                self.assertTrue(any(error['kind']=='missing-required-ui-binding'for error in checker.check(contract,files)['errors']))

    def test_real_product_window_requires_title_independently_of_existing_text(self):
        contract,files=fixture();key=next(iter(files))
        files[checker.PRODUCT_BUILD_SOURCE]='fn main() { slint_build::compile_with_config("ui/app.slint", config); }'
        files[checker.PRODUCT_HOST_SOURCE]='fn create() { AppWindow::new(); }'
        self.assertTrue(any(error['kind']=='missing-required-ui-binding'and error['property']=='title'for error in checker.check(contract,files)['errors']))
        files[key]=files[key].replace('inherits Window {','inherits Window { title: FixtureCopy.heading;',1)
        self.assertTrue(checker.check(contract,files)['ok'])

    def test_icon_buttons_and_blank_raw_fields_keep_explicit_accessible_meaning(self):
        contract,files=fixture();key=next(iter(files))
        files[key]=files[key][:-1]+'''Button {
            icon: @image-url("fixed-icon.png"); accessible-label: FixtureCopy.heading;
        } LineEdit { accessible-label: FixtureCopy.heading; } }'''
        self.assertTrue(checker.check(contract,files)['ok'])
        files[key]=files[key].replace('icon: @image-url("fixed-icon.png");','')
        self.assertTrue(any(error['kind']=='missing-required-ui-binding'for error in checker.check(contract,files)['errors']))

    def test_separate_component_instances_cannot_supply_each_others_text_path(self):
        contract,files=fixture()
        files[next(iter(files))]='''component Pane inherits Rectangle {
            in property<string> heading;
            in property<string> body;
            Text { text: root.body; }
        }
        export component AppWindow inherits Window {
            Pane { heading: FixtureCopy.heading; }
            Pane { body: self.heading; }
        }'''
        result=checker.check(contract,files)
        self.assertIn({'kind':'orphan-resource-id','key':'ui.fixture.heading'},result['errors'])
        # Both bindings on the same concrete instance are a real healthy chain.
        files[next(iter(files))]=files[next(iter(files))].replace(
            'Pane { heading: FixtureCopy.heading; }\n            Pane { body: self.heading; }',
            'Pane { heading: FixtureCopy.heading; body: self.heading; }')
        self.assertTrue(checker.check(contract,files)['ok'])

    def test_named_instance_references_are_real_cross_instance_flows(self):
        contract,files=fixture()
        files[next(iter(files))]='''component Pane inherits Rectangle {
            in property<string> heading;
            in property<string> body;
            Text { text: root.body; }
        }
        export component AppWindow inherits Window {
            source := Pane { heading: FixtureCopy.heading; body: self.heading; }
            Pane { body: source.heading; }
        }'''
        self.assertTrue(checker.check(contract,files)['ok'])

    def test_instance_expansion_obeys_existing_node_binding_and_edge_budgets(self):
        contract,files=fixture();key=next(iter(files))
        files[key]='''component Label inherits Text { text: FixtureCopy.heading; }
        component Scene inherits Rectangle { Label {} Label {} }
        export component AppWindow inherits Window { Scene {} Scene {} Scene {} Scene {} }'''
        self.assertTrue(checker.check(contract,files)['ok'])
        for limit,maximum,message in [('MAX_NODES',10,'instance bound'),
                                      ('MAX_BINDINGS',4,'binding count'),
                                      ('MAX_EDGES',6,'binding edge bound')]:
            with self.subTest(limit=limit),patch.object(checker.slint_text_graph,limit,maximum):
                with self.assertRaisesRegex(ValueError,message):checker.check(contract,files)

    def test_only_closed_guarded_optional_component_text_can_be_absent(self):
        contract,files=fixture()
        files['crates/nevrotick-ui/ui/components.slint']='''component SecondaryText inherits Text {}
        component PageHeading inherits Rectangle {
            in property<string> kicker;
            if root.kicker != "": SecondaryText { text: root.kicker; }
        }'''
        key=next(iter(files));files[key]=files[key][:-1]+'PageHeading {} }'
        self.assertTrue(checker.check(contract,files)['ok'])
        files['crates/nevrotick-ui/ui/components.slint']=files['crates/nevrotick-ui/ui/components.slint'].replace('if root.kicker != ""','if true')
        self.assertTrue(any(error['kind']=='unowned-required-ui-source'for error in checker.check(contract,files)['errors']))

    def test_composition_and_inheritance_depth_are_bounded_before_expansion(self):
        contract,files=fixture();key=next(iter(files))
        views=''.join('component C'+str(i)+' inherits Rectangle { C'+str(i+1)+' {} }'for i in range(10))
        views+='component C10 inherits Text { text: FixtureCopy.heading; } export component AppWindow inherits Window { C0 {} }'
        files[key]=views
        with patch.object(checker.slint_text_graph,'MAX_BRACE_DEPTH',8):
            with self.assertRaisesRegex(ValueError,'composition depth bound'):checker.check(contract,files)
        files[key]='component C0 inherits Text {} '+''.join('component C'+str(i)+' inherits C'+str(i-1)+' {}'for i in range(1,10))+'export component AppWindow inherits Window { C9 { text: FixtureCopy.heading; } }'
        with patch.object(checker.slint_text_graph,'MAX_BRACE_DEPTH',8):
            with self.assertRaisesRegex(ValueError,'inheritance depth bound'):checker.check(contract,files)

    def test_inherited_component_instances_keep_separate_property_state(self):
        contract,files=fixture()
        files[next(iter(files))]='''component BasePane inherits Rectangle {
            in property<string> heading;
            in property<string> body;
            Text { text: root.body; }
        }
        component Pane inherits BasePane {}
        export component AppWindow inherits Window {
            Pane { heading: FixtureCopy.heading; }
            Pane { body: self.heading; }
        }'''
        result=checker.check(contract,files)
        self.assertIn({'kind':'orphan-resource-id','key':'ui.fixture.heading'},result['errors'])

    def test_overridden_default_cannot_preserve_an_unrendered_copy_chain(self):
        contract,files=fixture()
        files[next(iter(files))]='''component Pane inherits Rectangle {
            in property<string> heading;
            in property<string> body: root.heading;
            Text { text: root.body; }
        }
        export component AppWindow inherits Window {
            in property<string> absent;
            Pane { heading: FixtureCopy.heading; body: root.absent; }
        }'''
        self.assertIn({'kind':'orphan-resource-id','key':'ui.fixture.heading'},checker.check(contract,files)['errors'])
        files[next(iter(files))]=files[next(iter(files))].replace('body: root.absent;','')
        self.assertTrue(checker.check(contract,files)['ok'])

    def test_children_and_inherited_native_text_remain_reached_per_instance(self):
        contract,files=fixture()
        files[next(iter(files))]='''component Container inherits Rectangle {
            VerticalLayout { @children }
        }
        component Label inherits Text {}
        export component AppWindow inherits Window {
            Container { Label { text: FixtureCopy.heading; } }
        }'''
        self.assertTrue(checker.check(contract,files)['ok'])

    def test_product_wrapper_cannot_fall_back_to_driverless_unit_fixture_scope(self):
        specification=importlib.util.spec_from_file_location('ui_text_wrapper_roots',Path(__file__).resolve().parents[1]/'check-ui-text.py')
        wrapper=importlib.util.module_from_spec(specification);specification.loader.exec_module(wrapper)
        contract,files=fixture();contract.update(catalog_schema_version=2,gui_build_version=1)
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory);metadata=root/'crates/nevrotick-ui/src/localization';metadata.mkdir(parents=True)
            for filename in ['contract-source.json','text-contract-v1-baseline.json']:(metadata/filename).write_text(json.dumps(contract),encoding='utf-8')
            manifest={'schema_version':1,'scope':'Isolated fixture','optional_empty_slots':[],'runtime_sinks':{}}
            (metadata/'runtime-text-sinks.json').write_text(json.dumps(manifest),encoding='utf-8')
            for name,source in files.items():
                target=root/name;target.parent.mkdir(parents=True,exist_ok=True);target.write_text(source,encoding='utf-8')
            self.assertTrue(checker.check(contract,files,contract,{})['ok'])
            output=io.StringIO()
            with patch.object(sys,'argv',['check-ui-text.py','--root',str(root)]),redirect_stdout(output):code=wrapper.main()
            self.assertEqual(code,1)
            self.assertEqual(json.loads(output.getvalue())['status'],'failed')

    def test_uninstantiated_component_cannot_credit_a_resource(self):
        contract,files=fixture()
        files[next(iter(files))]='export component AppWindow inherits Window {} export component UnusedPage inherits Rectangle { Text { text: FixtureCopy.heading; } }'
        result=checker.check(contract,files)
        self.assertIn({'kind':'orphan-resource-id','key':'ui.fixture.heading'},result['errors'])

    def test_dormant_component_literals_and_copy_refs_are_not_ui_sinks(self):
        contract,files=fixture()
        files['crates/nevrotick-ui/ui/unused.slint']='export component UnusedPage inherits Rectangle { in property<string> warning: "Unowned dormant text"; Text { text: root.warning; } Text { text: MissingCopy.dormant; } }'
        result=checker.check(contract,files)
        self.assertTrue(result['ok'],result['errors'])
        self.assertNotIn('MissingCopy.dormant',result['actual']['gui'])

    def test_inheritance_and_instantiation_preserve_children_tooltip_and_menu(self):
        contract,files=fixture()
        files[next(iter(files))]='export component AppWindow inherits Window { ContentPanel { Tooltip { Text { text: FixtureCopy.heading; } } ContextMenuArea { Menu { MenuItem { title: FixtureCopy.heading; } } } } }'
        files['crates/nevrotick-ui/ui/components.slint']='export component BasePanel inherits Rectangle { VerticalLayout { @children } } export component ContentPanel inherits BasePanel {}'
        result=checker.check(contract,files)
        self.assertTrue(result['ok'],result['errors'])
        self.assertEqual(len(result['uses']['ui.fixture.heading']),2)

    def test_uninstantiated_runtime_sink_does_not_credit_a_resource(self):
        contract,files=fixture()
        files[next(iter(files))]='export component AppWindow inherits Window {} export component UnusedPage inherits Rectangle { in property<string> error; Text { text: root.error; } }'
        files['crates/nevrotick-ui/src/slint_host/source_workflow.rs']='fn source_request() { write(MessageId::SAMPLE_HEADING); }'
        result=checker.check(contract,files,runtime_sinks={'ui.fixture.heading':[('UnusedPage','error')]})
        self.assertIn({'kind':'orphan-resource-id','key':'ui.fixture.heading'},result['errors'])

    def test_product_root_cannot_be_supplied_by_an_uncompiled_file(self):
        contract,files=fixture();source=files.pop(next(iter(files)))
        files['crates/nevrotick-ui/ui/uncompiled.slint']=source
        result=checker.check(contract,files)
        self.assertIn({'kind':'orphan-resource-id','key':'ui.fixture.heading'},result['errors'])

    def test_commented_or_quoted_product_root_cannot_activate_a_component(self):
        contract,files=fixture()
        for prefix in [
            '// export component AppWindow inherits Window {}\n',
            'export global Spoof { in property<string> text: "export component AppWindow inherits Window {}"; } ',
        ]:
            with self.subTest(prefix=prefix):
                files[next(iter(files))]=prefix+'export component UnusedPage inherits Text { text: FixtureCopy.heading; }'
                self.assertIn({'kind':'orphan-resource-id','key':'ui.fixture.heading'},checker.check(contract,files)['errors'])

    def test_product_host_and_build_entry_must_match_the_protected_root(self):
        contract,files=fixture()
        # Real driver scope now also requires a named native window.
        key=next(iter(files));files[key]=files[key].replace('inherits Window {','inherits Window { title: FixtureCopy.heading;',1)
        build='crates/nevrotick-ui/build.rs';host='crates/nevrotick-ui/src/slint_host.rs'
        files[build]='fn main() { slint_build::compile_with_config("ui/app.slint", configuration); }'
        files[host]='fn create() { let window = AppWindow::new(); }'
        self.assertTrue(checker.check(contract,files)['ok'])
        for target,source in [
            (build,'fn main() { slint_build::compile_with_config("ui/unused.slint", configuration); }'),
            (host,'fn create() { let window = UnusedWindow::new(); }'),
            (host,'fn create() { /* AppWindow::new(); */ } #[cfg(test)] fn fake() { AppWindow::new(); }'),
            (build,'fn main() {} #[cfg(test)] fn fake() { slint_build::compile_with_config("ui/app.slint", configuration); }'),
        ]:
            with self.subTest(target=target,source=source):
                changed=dict(files);changed[target]=source
                self.assertTrue(any(error['kind']=='product-ui-entrypoint-mismatch'for error in checker.check(contract,changed)['errors']))
        changed=dict(files);del changed[build]
        self.assertTrue(any(error['kind']=='product-ui-entrypoint-mismatch'for error in checker.check(contract,changed)['errors']))

    def test_new_window_constructor_requires_reviewed_product_root_custody(self):
        contract,files=fixture()
        key=next(iter(files));files[key]=files[key].replace('inherits Window {','inherits Window { title: FixtureCopy.heading;',1)
        files['crates/nevrotick-ui/build.rs']='fn main() { slint_build::compile_with_config("ui/app.slint", configuration); }'
        host='crates/nevrotick-ui/src/slint_host.rs'
        files[host]='fn create() { let window = AppWindow::new(); }'
        files[next(iter(files))]+='export component DormantWindow inherits Window {}'
        self.assertTrue(checker.check(contract,files)['ok'])
        files[host]+='fn open_side_window() { DormantWindow::new(); }'
        self.assertTrue(any(error['kind']=='product-ui-entrypoint-mismatch'for error in checker.check(contract,files)['errors']))
        files[host]='fn create() { AppWindow::new(); } #[cfg(test)] fn fake() { DormantWindow::new(); } // DormantWindow::new();'
        self.assertTrue(checker.check(contract,files)['ok'])

    def test_snapshot_includes_only_the_one_bounded_build_entry(self):
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory);folder=root/'crates/nevrotick-ui';folder.mkdir(parents=True)
            source=folder/'build.rs';source.write_text('fn main() {}',encoding='utf-8')
            self.assertEqual(checker.snapshot(root),{'crates/nevrotick-ui/build.rs':'fn main() {}'})
            source.write_bytes(b'x'*1_000_001)
            with patch.object(Path,'open',side_effect=AssertionError('oversized build entry must not be read')):
                with self.assertRaisesRegex(ValueError,'source byte bound'):checker.snapshot(root)

    def test_wrapper_bounds_metadata_read_even_after_file_growth(self):
        specification=importlib.util.spec_from_file_location('ui_text_wrapper',Path(__file__).resolve().parents[1]/'check-ui-text.py')
        wrapper=importlib.util.module_from_spec(specification);specification.loader.exec_module(wrapper)
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory);target=root/'metadata.json';target.write_text('{}',encoding='utf-8')
            real_open=os.open
            def grow_before_open(path,flags,*args,**kwargs):
                target.write_bytes(b'x'*1_000_001)
                return real_open(path,flags,*args,**kwargs)
            with patch.object(os,'open',side_effect=grow_before_open), \
                    patch.object(wrapper.protected_json,'read',side_effect=AssertionError('oversized source must not be decoded')):
                with self.assertRaisesRegex(ValueError,'source.*(bound|custody)'):
                    wrapper.regular(root,'metadata.json')

    @unittest.skipUnless(hasattr(os,'mkfifo'),'POSIX file replacement controls')
    def test_source_readers_reject_fifo_replacement_before_waiting(self):
        """Exercise each real reader with a healthy neighbor and a raced FIFO."""
        program='''
import importlib.util,os,sys
from pathlib import Path
from unittest.mock import patch
sys.path.insert(0,sys.argv[1])
import check_ui_text_coverage as coverage
import native_input_source as native
spec=importlib.util.spec_from_file_location('gate',Path(sys.argv[1])/'check-ui-text.py')
gate=importlib.util.module_from_spec(spec);spec.loader.exec_module(gate)
root=Path(sys.argv[2]);mode=sys.argv[3]
relative={'metadata':'metadata.json','snapshot':'crates/nevrotick-ui/build.rs',
          'native':'crates/nevrotick-ui/src/main.rs'}[mode]
target=root/relative;target.parent.mkdir(parents=True,exist_ok=True)
target.write_text('{}' if mode=='metadata' else 'fn main() {}')
reader={'metadata':lambda:gate.regular(root,relative),
        'snapshot':lambda:coverage.snapshot(root),
        'native':lambda:native.read_sources(root)}[mode]
assert reader() == ({} if mode=='metadata' else {relative:'fn main() {}'})
real_os_open=os.open;real_path_open=Path.open
replaced=False
def swap(path):
    global replaced
    if Path(path)==target and not replaced:
        replaced=True;target.unlink();os.mkfifo(target)
def os_open(path,*args,**kwargs):
    swap(path);return real_os_open(path,*args,**kwargs)
def path_open(path,*args,**kwargs):
    swap(path);return real_path_open(path,*args,**kwargs)
with patch.object(os,'open',side_effect=os_open),patch.object(Path,'open',path_open):
    try:reader()
    except ValueError:
        assert replaced
        raise SystemExit(0)
raise SystemExit('FIFO was accepted')
'''
        for mode in ['metadata','snapshot','native']:
            with self.subTest(reader=mode),tempfile.TemporaryDirectory() as directory:
                try:
                    result=subprocess.run([sys.executable,'-B','-c',program,
                                           str(Path(__file__).resolve().parents[1]),directory,mode],
                                          capture_output=True,timeout=3,check=False)
                except subprocess.TimeoutExpired:
                    self.fail(mode+' reader waited for a FIFO writer')
                self.assertEqual(result.returncode,0,result.stderr.decode())

    def test_source_snapshot_bounds_bytes_before_reading_or_decoding(self):
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory);folder=root/'crates/nevrotick-ui/ui';folder.mkdir(parents=True)
            source=folder/'page.slint';source.write_text('Text {}',encoding='utf-8')
            self.assertEqual(checker.snapshot(root),{'crates/nevrotick-ui/ui/page.slint':'Text {}'})
            with source.open('wb') as stream:stream.truncate(1_000_001)
            with patch.object(Path,'open',side_effect=AssertionError('oversized source must not be opened')):
                with self.assertRaisesRegex(ValueError,'source.*bound'):checker.snapshot(root)

    def test_source_snapshot_bounds_count_and_aggregate_during_discovery(self):
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory);folder=root/'crates/nevrotick-ui/ui';folder.mkdir(parents=True)
            for name in ['a.slint','b.slint']:(folder/name).write_text('ab',encoding='utf-8')
            with patch.object(checker,'MAX_FILES',1):
                with self.assertRaisesRegex(ValueError,'source.*bound'):checker.snapshot(root)
            with patch.object(checker,'MAX_TOTAL',3):
                with self.assertRaisesRegex(ValueError,'source.*bound'):checker.snapshot(root)

    def test_healthy_visible_and_a11y_are_real_sinks(self):
        contract,files=fixture();result=checker.check(contract,files)
        self.assertTrue(result['ok'],result['errors']);self.assertEqual(len(result['uses']['ui.fixture.heading']),2)

    def test_added_control_missing_text_fails_forward(self):
        contract,files=fixture();files[next(iter(files))]=files[next(iter(files))][:-1]+'Button { text: FixtureCopy.new-control; } }'
        result=checker.check(contract,files);self.assertFalse(result['ok'])
        self.assertTrue(any(x['kind']=='missing-gui-text'for x in result['errors']))

    def test_removed_sink_leaves_orphan_fails_reverse(self):
        contract,files=fixture();files[next(iter(files))]='Rectangle {}'
        result=checker.check(contract,files);self.assertFalse(result['ok'])
        self.assertIn({'kind':'orphan-resource-id','key':'ui.fixture.heading'},result['errors'])

    def test_contract_generated_text_does_not_approve_its_own_orphan(self):
        contract,files=fixture();files[next(iter(files))]='Rectangle {}'
        files['crates/nevrotick-ui/src/localization/contract.rs']='MessageId::SAMPLE_HEADING'
        files['crates/nevrotick-ui/src/slint_host/catalog_projection.rs']='MessageId::SAMPLE_HEADING'
        files['crates/nevrotick-ui/ui/fixture-copy.slint']='export global FixtureCopy { out property<string> heading: "Heading"; }'
        self.assertTrue(any(x['kind']=='orphan-resource-id'for x in checker.check(contract,files)['errors']))

    def test_empty_required_resource_and_unowned_static_control_fail(self):
        contract,files=fixture();contract['messages'][0]['reference']['text']=''
        self.assertTrue(any(x['kind']=='empty-required-text'for x in checker.check(contract,files)['errors']))
        contract,files=fixture();files[next(iter(files))]=files[next(iter(files))][:-1]+'Button { text: "Unowned launch"; } }'
        self.assertTrue(any(x['kind']=='unowned-ui-literal'for x in checker.check(contract,files)['errors']))
        files[next(iter(files))]='export component AppWindow inherits Window { Button { text: ""; } }'
        self.assertTrue(any(x['kind']=='empty-required-ui-text'for x in checker.check(contract,files)['errors']))

    def test_comments_strings_and_tests_are_not_binding_owners(self):
        contract,files=fixture();files[next(iter(files))]='// text: FixtureCopy.heading\nText { text: "FixtureCopy.heading"; }'
        files['crates/nevrotick-ui/tests/fake.rs']='MessageId::SAMPLE_HEADING'
        self.assertTrue(any(x['kind']=='orphan-resource-id'for x in checker.check(contract,files)['errors']))

    def test_new_required_ids_require_contract_version_change(self):
        baseline,files=fixture();contract=copy.deepcopy(baseline)
        contract['messages'].append({'key':'ui.fixture.new-heading','symbol':'SAMPLE_NEW_HEADING','active':True,'kind':'Text','reference':{'text':'New heading'},'slint':{'global':'FixtureCopy','property':'new-heading'}})
        files[next(iter(files))]=files[next(iter(files))][:-1]+'Text { text: FixtureCopy.new-heading; } }'
        self.assertTrue(any(x['kind']=='text-contract-version-not-incremented'for x in checker.check(contract,files,baseline)['errors']))
        contract['text_contract_version']=2
        self.assertTrue(checker.check(contract,files,baseline)['ok'])

    def test_runtime_only_ids_need_an_actual_handwritten_owner(self):
        contract,files=fixture();files[next(iter(files))]='Rectangle {}'
        files['crates/nevrotick-ui/src/slint_host/source_workflow.rs']='fn source_request() { write(MessageId::SAMPLE_HEADING); }'
        self.assertTrue(checker.check(contract,files)['ok'])
        files['crates/nevrotick-ui/src/slint_host/source_workflow.rs']='use MessageId::SAMPLE_HEADING;'
        self.assertFalse(checker.check(contract,files)['ok'])
        del files['crates/nevrotick-ui/src/slint_host/source_workflow.rs']
        self.assertFalse(checker.check(contract,files)['ok'])


    def test_inline_cfg_test_modules_do_not_approve_runtime_orphans(self):
        contract,files=fixture();files[next(iter(files))]='Rectangle {}'
        owner='crates/nevrotick-ui/src/slint_host/source_workflow.rs'
        files[owner]='#[cfg(test)] mod tests { fn test_only() { write(MessageId::SAMPLE_HEADING); } }'
        result=checker.check(contract,files)
        self.assertIn({'kind':'orphan-resource-id','key':'ui.fixture.heading'},result['errors'])

    def test_inline_test_functions_do_not_approve_runtime_orphans(self):
        contract,files=fixture();files[next(iter(files))]='Rectangle {}'
        owner='crates/nevrotick-ui/src/slint_host/source_workflow.rs'
        files[owner]='#[test] fn test_only() { write(MessageId::SAMPLE_HEADING); }'
        self.assertIn({'kind':'orphan-resource-id','key':'ui.fixture.heading'},checker.check(contract,files)['errors'])

    def test_test_only_provider_calls_do_not_make_presenter_ids_reachable(self):
        contract,files=fixture();files[next(iter(files))]='Rectangle {}'
        files[checker.PRESENTER]='impl UiText { fn sample() { write(MessageId::SAMPLE_HEADING); } }'
        files['crates/nevrotick-ui/src/slint_host/source_workflow.rs']='#[cfg(test)] mod tests { fn test_only() { UiText::sample(); } }'
        self.assertIn({'kind':'orphan-resource-id','key':'ui.fixture.heading'},checker.check(contract,files)['errors'])

    def test_non_test_owner_beside_ignored_tests_remains_a_use(self):
        contract,files=fixture();files[next(iter(files))]='Rectangle {}'
        files['crates/nevrotick-ui/src/slint_host/source_workflow.rs']='#[cfg(test)] mod tests { fn helper() { ignored(); } } #[cfg(not(test))] fn source_request() { write(MessageId::SAMPLE_HEADING); }'
        self.assertTrue(checker.check(contract,files)['ok'])



    def test_unknown_copy_global_is_a_missing_gui_text(self):
        contract,files=fixture()
        files[next(iter(files))]=files[next(iter(files))][:-1]+'Button { text: AlienCopy.unregistered-control; } }'
        result=checker.check(contract,files)
        self.assertTrue(any(x['kind']=='missing-gui-text' and x['property']=='AlienCopy.unregistered-control' for x in result['errors']))

    def test_conditional_and_concatenated_text_literals_cannot_hide(self):
        for expression in ['ready ? "Unowned launch" : FixtureCopy.heading', 'FixtureCopy.heading + "Unowned suffix"']:
            with self.subTest(expression=expression):
                contract,files=fixture();files[next(iter(files))]='export component AppWindow inherits Window { Text { text: '+expression+'; } }'
                self.assertTrue(any(x['kind']=='unowned-ui-literal'for x in checker.check(contract,files)['errors']))

    def test_allowed_prefix_cannot_whitelist_unowned_suffix(self):
        contract,files=fixture()
        files[next(iter(files))]=files[next(iter(files))][:-1]+'UnownedHelp {} }'
        files['crates/nevrotick-ui/ui/components.slint']='export component UnownedHelp inherits Text { text: "?" + "Unowned suffix"; }'
        self.assertTrue(any(x['kind']=='unowned-ui-literal' and x['literal']=='Unowned suffix'for x in checker.check(contract,files)['errors']))

    def test_compound_absence_and_comment_strings_are_healthy_neighbors(self):
        contract,files=fixture()
        files[next(iter(files))]='export component AppWindow inherits Window { Text { text: ready ? FixtureCopy.heading : ""; /* text: "Not a control"; */ } }'
        self.assertTrue(checker.check(contract,files)['ok'])



    def test_new_dead_runtime_function_is_not_an_emitter(self):
        contract,files=fixture();files[next(iter(files))]='Rectangle {}'
        files['crates/nevrotick-ui/src/slint_host/source_workflow.rs']='fn unused_unregistered() { write(MessageId::SAMPLE_HEADING); }'
        self.assertIn({'kind':'orphan-resource-id','key':'ui.fixture.heading'},checker.check(contract,files)['errors'])

    def test_dead_runtime_provider_call_cannot_approve_an_id(self):
        contract,files=fixture();files[next(iter(files))]='Rectangle {}'
        files[checker.PRESENTER]='impl UiText { fn sample() { write(MessageId::SAMPLE_HEADING); } }'
        files['crates/nevrotick-ui/src/slint_host/source_workflow.rs']='fn unused_unregistered() { UiText::sample(); }'
        self.assertIn({'kind':'orphan-resource-id','key':'ui.fixture.heading'},checker.check(contract,files)['errors'])

    def test_copy_reference_without_a_text_endpoint_is_orphaned(self):
        contract,files=fixture()
        files[next(iter(files))]='export component AppWindow inherits Window { in property<string> unused: FixtureCopy.heading; Rectangle {} }'
        self.assertIn({'kind':'orphan-resource-id','key':'ui.fixture.heading'},checker.check(contract,files)['errors'])

    def test_runtime_sink_forwarding_reaches_derived_text(self):
        contract,files=fixture();files[next(iter(files))]='export component AppWindow inherits Window { in property<string> editor-error; SourcesPage { editor-error: root.editor-error; } }'
        files['crates/nevrotick-ui/ui/sources.slint']='export component SourcesPage inherits Rectangle { in property<string> editor-error; SourceEditor { error: root.editor-error; } }'
        files['crates/nevrotick-ui/ui/source-editor.slint']='export component SourceEditor inherits Rectangle { in property<string> error; SecondaryText { text: root.error; } }'
        files['crates/nevrotick-ui/ui/components.slint']='export component BodyText inherits Text {} export component SecondaryText inherits BodyText {}'
        files['crates/nevrotick-ui/src/slint_host/source_workflow.rs']='fn source_request() { write(MessageId::SAMPLE_HEADING); }'
        sinks={'ui.fixture.heading':[('AppWindow','editor-error')]}
        self.assertTrue(checker.check(contract,files,runtime_sinks=sinks)['ok'])
        # Leave the Rust formatter and all component forwarding intact; remove
        # only the final actual text binding, which must orphan the resource.
        files['crates/nevrotick-ui/ui/source-editor.slint']='export component SourceEditor inherits Rectangle { in property<string> error; SecondaryText {} }'
        self.assertIn({'kind':'orphan-resource-id','key':'ui.fixture.heading'},checker.check(contract,files,runtime_sinks=sinks)['errors'])

    def test_runtime_sink_inheritance_and_cycles_are_bounded(self):
        contract,files=fixture();files[next(iter(files))]='export component AppWindow inherits Window { in property<string> editor-error; Alert { body: root.editor-error; } }'
        files['crates/nevrotick-ui/ui/alert.slint']='export component BaseAlert inherits Rectangle { in property<string> body; Text { text: root.body; } } export component Alert inherits BaseAlert {}'
        files['crates/nevrotick-ui/src/slint_host/source_workflow.rs']='fn source_request() { write(MessageId::SAMPLE_HEADING); }'
        sinks={'ui.fixture.heading':[('AppWindow','editor-error')]}
        self.assertTrue(checker.check(contract,files,runtime_sinks=sinks)['ok'])
        files['crates/nevrotick-ui/ui/alert.slint']='export component Alert inherits Rectangle { in property<string> body; in property<string> echo: root.body; body: root.echo; }'
        self.assertIn({'kind':'orphan-resource-id','key':'ui.fixture.heading'},checker.check(contract,files,runtime_sinks=sinks)['errors'])



    def test_pure_enum_catalog_ids_without_receiving_owner_do_not_count(self):
        contract,files=fixture();files[next(iter(files))]='Rectangle {}'
        files['crates/nevrotick-ui/src/status_ui.rs']="impl StatusText { pub const fn catalog_id(self) -> &'static str { \"ui.fixture.heading\" } }"
        self.assertIn({'kind':'orphan-resource-id','key':'ui.fixture.heading'},checker.check(contract,files)['errors'])

    def test_registered_status_formatter_consumes_its_finite_enum_text(self):
        contract,files=fixture();files[next(iter(files))]='Rectangle {}'
        files['crates/nevrotick-ui/src/status_ui.rs']="impl StatusText { pub const fn catalog_id(self) -> &'static str { \"ui.fixture.heading\" } }"
        files['crates/nevrotick-ui/src/slint_host/status_workflow.rs']='fn project() { let text = UiText::key(value.catalog_id()); emit(text); }'
        self.assertTrue(checker.check(contract,files)['ok'])

    def test_reserved_tray_enum_and_comment_cannot_approve_live_text(self):
        contract,files=fixture();files[next(iter(files))]='Rectangle {}'
        files['crates/nevrotick-ui/src/tray_ui.rs']="impl TrayText { pub const fn catalog_id(self) -> &'static str { \"ui.fixture.heading\" } }"
        files['crates/nevrotick-ui/src/slint_host/tray_workflow.rs']='// UiText::key( is not an admitted native renderer or an executable call.'
        self.assertIn({'kind':'orphan-resource-id','key':'ui.fixture.heading'},checker.check(contract,files)['errors'])



    def test_hyphenated_copy_property_in_ternary_is_not_a_new_binding(self):
        contract,files=fixture();contract['messages'][0]['slint']['property']='heading-title';contract['messages'][0]['key']='ui.fixture.heading-title'
        files[next(iter(files))]='export component AppWindow inherits Window { Text { text: flag ? (true ? FixtureCopy.heading-title : FixtureCopy.heading-title) : FixtureCopy.heading-title; } }'
        self.assertTrue(checker.check(contract,files)['ok'])



    def test_function_and_callback_parameter_names_are_not_text_bindings(self):
        contract,files=fixture()
        files[next(iter(files))]='export component AppWindow inherits Window { function confirm(value: bool) -> bool { return value; } callback changed(text: string); Text { text: FixtureCopy.heading; } }'
        self.assertTrue(checker.check(contract,files)['ok'])



    def test_upstream_default_reaching_text_requires_catalog_ownership(self):
        contract,files=fixture()
        files[next(iter(files))]='export component AppWindow inherits Window { in property<string> warning: "Unowned warning"; Text { text: FixtureCopy.heading; } Text { text: root.warning; } }'
        self.assertTrue(any(x['kind']=='unowned-ui-literal' and x['literal']=='Unowned warning'for x in checker.check(contract,files)['errors']))

    def test_custom_component_default_is_traced_to_text(self):
        contract,files=fixture()
        files[next(iter(files))]='export component AppWindow inherits Window { Text { text: FixtureCopy.heading; } Alert {} }'
        files['crates/nevrotick-ui/ui/alert.slint']='export component Alert inherits Rectangle { in property<string> warning: "Unowned warning"; Text { text: root.warning; } }'
        self.assertTrue(any(x['kind']=='unowned-ui-literal'for x in checker.check(contract,files)['errors']))

    def test_exact_checked_draft_fact_does_not_exempt_another_control(self):
        contract,files=fixture();del files[next(iter(files))]
        files['crates/nevrotick-ui/ui/app.slint']='export component AppWindow inherits Window { in property<string> draft-custom-minutes: "1"; Text { text: FixtureCopy.heading; } LineEdit { accessible-label: FixtureCopy.heading; text: root.draft-custom-minutes; } }'
        self.assertTrue(checker.check(contract,files)['ok'])
        files['crates/nevrotick-ui/ui/app.slint']=files['crates/nevrotick-ui/ui/app.slint'].replace('draft-custom-minutes','unowned-label')
        self.assertTrue(any(x['kind']=='unowned-ui-literal'for x in checker.check(contract,files)['errors']))

    def test_unknown_copy_suffixed_view_is_not_a_generated_exception(self):
        contract,files=fixture()
        files[next(iter(files))]=files[next(iter(files))][:-1]+'UnexpectedCopyPage {} }'
        files['crates/nevrotick-ui/ui/unregistered-copy.slint']='export component UnexpectedCopyPage inherits Text { text: "Unowned control"; }'
        self.assertTrue(any(x['kind']=='unowned-ui-literal'for x in checker.check(contract,files)['errors']))

    def test_unknown_contract_named_rust_file_is_not_generated(self):
        contract,files=fixture()
        files['crates/nevrotick-ui/src/slint_host/extra/contract.rs']='fn unregistered() { emit(MessageId::SAMPLE_HEADING); }'
        self.assertTrue(any(x['kind']=='unregistered-runtime-owner'for x in checker.check(contract,files)['errors']))

    def test_context_menu_title_is_an_actual_text_endpoint(self):
        contract,files=fixture();contract['messages'][0]['key']='ui.raw.action-copy';contract['messages'][0]['slint']={'global':'RawCopy','property':'action-copy'}
        files[next(iter(files))]='export component AppWindow inherits Window { ContextMenuArea { Menu { MenuItem { title: RawCopy.action-copy; } } } }'
        self.assertTrue(checker.check(contract,files)['ok'])

    def test_source_change_array_ref_is_still_actual_text(self):
        contract,files=fixture()
        contract['messages']=[{'key':'ui.rust.change-'+name,'symbol':'CHANGE_'+name.upper(),'active':True,'kind':'Text','reference':{'text':name},'slint':{'global':'SourceCopy','property':'change-labels','slot':slot}}for slot,name in enumerate(['baseline','add','edit','remove'])]
        files[next(iter(files))]='export component AppWindow inherits Window { Text { text: SourceCopy.change-labels[0]; } }'
        self.assertTrue(checker.check(contract,files)['ok'])



    def test_same_version_rejects_bidirectional_semantic_changes(self):
        baseline,files=fixture()
        baseline.update(catalog_schema_version=2,gui_build_version=1,domain='nevrotick.ui',locale='en',plural_profile='english-u64-cardinal-v1',unicode_profile='pinned-v1',unicode_policy_sha256='a'*64,bounds={'MAX_MAP':512,'MAX_WORK':4000000},action_contract={'quit':'fixed-ui-only'})
        baseline['messages'][0].update(group='Notices',meaning_revision=1,meaning_context='Source-only fixture meaning')
        baseline['messages'].append({'key':'ui.reserved.tray','symbol':'RESERVED_TRAY','active':False,'kind':'Text','reference':{'text':'Reserved only'},'reservation':'No native renderer'})
        changes=[
          ('English wording',lambda c:c['messages'][0]['reference'].__setitem__('text','Revised heading')),
          ('kind and argument requirement',lambda c:c['messages'][0].update(kind='Minutes',reference={'forms':{'one':'{minutes}','other':'{minutes}'}})),
          ('critical group',lambda c:c['messages'][0].__setitem__('group','Quit')),
          ('meaning revision',lambda c:c['messages'][0].__setitem__('meaning_revision',2)),
          ('meaning context',lambda c:c['messages'][0].__setitem__('meaning_context','Changed authority wording')),
          ('action contract',lambda c:c['action_contract'].__setitem__('quit','different-action')),
          ('Unicode policy',lambda c:c.__setitem__('unicode_policy_sha256','b'*64)),
          ('bounds',lambda c:c['bounds'].__setitem__('MAX_MAP',768)),
          ('plural typed contract',lambda c:c.__setitem__('plural_profile','changed-profile')),
          ('catalog representation',lambda c:c.__setitem__('catalog_schema_version',3)),
          ('active removal',lambda c:c['messages'].__delitem__(0)),
          ('active retirement',lambda c:c['messages'][0].__setitem__('active',False)),
          ('reserved activation',lambda c:c['messages'][1].__setitem__('active',True)),
          ('reserved disposition',lambda c:c['messages'][1].pop('reservation')),
          ('unrendered new active ID',lambda c:c['messages'].append({'key':'ui.fixture.unrendered','symbol':'UNRENDERED','active':True,'kind':'Text','reference':{'text':'Unrendered'},'slint':None})),
        ]
        for label,change in changes:
            with self.subTest(label=label):
                contract=copy.deepcopy(baseline);change(contract)
                self.assertTrue(any(e['kind']=='text-contract-version-not-incremented'for e in checker.check(contract,files,baseline)['errors']))

    def test_gui_build_and_source_topology_do_not_change_text_identity(self):
        baseline,files=fixture();baseline.update(gui_build_version=1,status='review baseline',contract_revision='first')
        contract=copy.deepcopy(baseline);contract.update(gui_build_version=2,status='new build',contract_revision='second')
        contract['messages'][0].update(sources=['/inert/changed/source'],ordinal=99,symbol='RENAMED_SYMBOL')
        self.assertEqual(checker.semantic_fingerprint(contract),checker.semantic_fingerprint(baseline))
        self.assertTrue(checker.check(contract,files,baseline)['ok'])

    def test_changed_text_with_incremented_contract_version_has_a_valid_neighbor(self):
        baseline,files=fixture();contract=copy.deepcopy(baseline)
        contract['messages'][0]['reference']['text']='Revised heading'
        contract['text_contract_version']=2
        result=checker.check(contract,files,baseline)
        self.assertTrue(result['ok'],result['errors'])
        self.assertTrue(result['baseline_provided'])
        self.assertNotEqual(result['semantic_fingerprint'],result['baseline_semantic_fingerprint'])


    def test_duplicate_scalar_destination_cannot_approve_two_resources(self):
        contract,files=fixture();other=copy.deepcopy(contract['messages'][0]);other.update(key='ui.fixture.other-heading',symbol='OTHER_HEADING');contract['messages'].append(other)
        self.assertTrue(any(e['kind']=='duplicate-binding-definition'for e in checker.check(contract,files)['errors']))


    def test_closed_array_profiles_reject_aliasing_and_shape_mutations(self):
        def profile():
            contract,files=fixture()
            contract['messages']=[{'key':'ui.source.protocol-option.'+name,'symbol':'PROTOCOL_'+name.upper(),'active':True,'kind':'Text','reference':{'text':name.upper()},'slint':{'global':'SourceCopy','property':'protocols','slot':slot}}for slot,name in enumerate(['nts','ntp'])]
            files[next(iter(files))]='export component AppWindow inherits Window { ComboBox { model: SourceCopy.protocols; } }'
            return contract,files
        healthy,files=profile();self.assertTrue(checker.check(healthy,files)['ok'])
        changes=[
          ('duplicate index',lambda c:c['messages'][1]['slint'].__setitem__('slot',0)),
          ('scalar array mix',lambda c:c['messages'][0]['slint'].__setitem__('slot',None)),
          ('missing slot',lambda c:c['messages'].pop()),
          ('extra slot',lambda c:c['messages'].append({'key':'ui.source.protocol-option.extra','symbol':'EXTRA_PROTOCOL','active':True,'kind':'Text','reference':{'text':'extra'},'slint':{'global':'SourceCopy','property':'protocols','slot':2}})),
          ('unexpected slot',lambda c:c['messages'][1]['slint'].__setitem__('slot',2)),
          ('bool index',lambda c:c['messages'][0]['slint'].__setitem__('slot',False)),
          ('unknown array property',lambda c:[r['slint'].__setitem__('property','unknown-options')for r in c['messages']]),
          ('swapped action labels',lambda c:[r['slint'].__setitem__('slot',1-r['slint']['slot'])for r in c['messages']]),
          ('wrong text kind',lambda c:c['messages'][0].update(kind='Minutes',reference={'forms':{'one':'{minutes}','other':'{minutes}'}})),
        ]
        for label,change in changes:
            with self.subTest(label=label):
                candidate=copy.deepcopy(healthy);change(candidate)
                if label=='unknown array property':
                    current_files={path:body.replace('SourceCopy.protocols','SourceCopy.unknown-options')for path,body in files.items()}
                else:current_files=files
                self.assertTrue(any(e['kind']in{'duplicate-binding-definition','unsupported-array-binding','array-binding-profile-mismatch','unsupported-binding-type'}for e in checker.check(candidate,current_files)['errors']))


    def test_scalar_target_swapping_cannot_change_fixed_message_meaning(self):
        baseline,files=fixture();candidate=copy.deepcopy(baseline)
        candidate['messages'][0]['slint']['property']='another-heading'
        current_files={path:body.replace('FixtureCopy.heading','FixtureCopy.another-heading')for path,body in files.items()}
        self.assertEqual(checker.semantic_fingerprint(baseline),checker.semantic_fingerprint(candidate))
        self.assertTrue(any(e['kind']=='unsupported-scalar-binding'for e in checker.check(candidate,current_files)['errors']))


    def test_typed_presets_use_only_fixed_text_slots_and_required_minute_kind(self):
        contract,files=fixture()
        contract['messages']=[{'key':'ui.preferences.interval-option.'+name,'symbol':'INTERVAL_'+name.upper(),'active':True,'kind':'Text','reference':{'text':name},'slint':{'global':'PreferencesCopy','property':'interval-options','slot':slot}}for slot,name in [(0,'automatic'),(4,'custom')]]
        contract['messages'].append({'key':'ui.interval.every','symbol':'INTERVAL_EVERY','active':True,'kind':'Minutes','reference':{'forms':{'one':'{minutes}','other':'{minutes}'}},'slint':None})
        files[next(iter(files))]='export component AppWindow inherits Window { ComboBox { model: PreferencesCopy.interval-options; } }'
        self.assertTrue(checker.check(contract,files)['ok'])
        missing=copy.deepcopy(contract);missing['messages'].pop()
        self.assertTrue(any(e['kind']=='typed-array-binding-profile-mismatch'for e in checker.check(missing,files)['errors']))
        ordinary=copy.deepcopy(contract);ordinary['messages'][2]['slint']={'global':'PreferencesCopy','property':'interval-options','slot':1}
        self.assertTrue(any(e['kind']in{'unsupported-binding-type','array-binding-profile-mismatch','typed-array-binding-profile-mismatch'}for e in checker.check(ordinary,files)['errors']))


    def test_scalar_minutes_and_ordinary_duration_rows_cannot_bypass_typed_bridges(self):
        contract,files=fixture();contract['messages'][0].update(kind='Minutes',reference={'forms':{'one':'{minutes}','other':'{minutes}'}})
        self.assertTrue(any(e['kind']=='unsupported-binding-type'for e in checker.check(contract,files)['errors']))
        contract['messages']=[{'key':'ui.duration.minutes','symbol':'DURATION_MINUTES','active':True,'kind':'Minutes','reference':{'forms':{'one':'{minutes}','other':'{minutes}'}},'slint':None}]
        files[next(iter(files))]='export component AppWindow inherits Window { ComboBox { model: DiagnosticCopy.durations; } }'
        self.assertTrue(checker.check(contract,files)['ok'])
        contract['messages'].append({'key':'ui.diagnostic.duration-choice.five','symbol':'DURATION_FIVE','active':True,'kind':'Text','reference':{'text':'Five minutes'},'slint':{'global':'DiagnosticCopy','property':'durations','slot':0}})
        self.assertTrue(any(e['kind']in{'unsupported-array-binding','typed-array-binding-profile-mismatch'}for e in checker.check(contract,files)['errors']))


if __name__=='__main__':unittest.main()
