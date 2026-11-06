# SPDX-License-Identifier: GPL-3.0-only
"""Causal source-custody controls; no compiler, toolkit or native execution."""
import copy
from pathlib import Path
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import native_input_source as n

ROOT = Path(__file__).resolve().parents[2]


class NativeInputSourceControls(unittest.TestCase):
    def setUp(self):
        self.files = n.read_sources(ROOT)

    def change(self, files, path, old, new):
        self.assertEqual(files[path].count(old), 1, "unique source mutation witness")
        files[path] = files[path].replace(old, new, 1)

    def rejected(self, path, old, new):
        files = copy.deepcopy(self.files)
        self.change(files, path, old, new)
        self.assertTrue(n.check(files), "causal source mutation must reject")

    def test_current_maintained_main_host_and_filter_are_healthy(self):
        self.assertEqual(n.check(self.files), [])
        self.assertIn(n.INPUT, self.files)
        self.assertLessEqual(len(self.files[n.INPUT].encode()), 32768)

    def test_whitespace_and_comment_changes_preserve_contract(self):
        files = copy.deepcopy(self.files)
        self.change(files, n.HOST, '        .select()', '        /* reviewed */ .select ( )')
        self.change(files, n.INPUT, '    if allowed {', '    // no executable change\n    if allowed {')
        self.assertEqual(n.check(files), [])

    def test_each_required_current_source_is_independently_required(self):
        for path in (n.MAIN, n.HOST, n.INPUT, n.RAW):
            files = copy.deepcopy(self.files)
            del files[path]
            with self.subTest(path=path):
                self.assertTrue(n.check(files))

    def test_main_route_cannot_be_replaced_or_supplied_by_test_comment_or_literal(self):
        for old, new in (
            ('nevrotick_ui::slint_host::run_native()', 'nevrotick_ui::slint_host::run()'),
            ('#[cfg(windows)]', '#[cfg(not(windows))]'),
            ('nevrotick_ui::slint_host::run_native()',
             'if false { nevrotick_ui::slint_host::run_native() } else { nevrotick_ui::slint_host::run() }'),
        ):
            with self.subTest(new=new):
                self.rejected(n.MAIN, old, new)
        for decoy in (
            '// fn main() { nevrotick_ui::slint_host::run_native() }',
            '#[cfg(test)] fn main() { nevrotick_ui::slint_host::run_native() }',
            'const DECOY: &str = r#"#[cfg(windows)] fn main() { nevrotick_ui::slint_host::run_native() }"#;',
        ):
            files = copy.deepcopy(self.files)
            files[n.MAIN] = decoy
            with self.subTest(decoy=decoy):
                self.assertTrue(n.check(files))

    def test_native_run_cannot_call_fixture_or_ignore_startup_failure(self):
        for new in (
            'create()?.run().map_err(HostError::Platform)',
            'let _ = create_native(); create()?.run().map_err(HostError::Platform)',
            'let unused = || create_native(); create()?.run().map_err(HostError::Platform)',
        ):
            with self.subTest(new=new):
                self.rejected(n.HOST, 'create_native()?.run().map_err(HostError::Platform)', new)

    def test_explicit_backend_renderer_and_selection_success_precede_create(self):
        for old, new in (
            ('.backend_name("winit".into())', '.backend_name("qt".into())'),
            ('.renderer_name("software".into())', '.renderer_name("skia".into())'),
            ('.map_err(HostError::Platform)?;\n    let host = create()?;',
             '.ok();\n    let host = create()?;'),
            ('    slint::BackendSelector::new()', '    let unused = || slint::BackendSelector::new()'),
            ('    let host = create()?;\n    native_input::install',
             '    let host = create()?;\n    if false { slint::BackendSelector::new().select().map_err(HostError::Platform)?; }\n    native_input::install'),
        ):
            with self.subTest(new=new):
                self.rejected(n.HOST, old, new)
        files = copy.deepcopy(self.files)
        selector = '''    slint::BackendSelector::new()
        .backend_name("winit".into())
        .renderer_name("software".into())
        .select()
        .map_err(HostError::Platform)?;
'''
        self.change(files, n.HOST, selector, '')
        self.change(files, n.HOST, '    let host = create()?;', '    let host = create()?;\n' + selector)
        self.assertTrue(n.check(files))

    def test_hook_must_install_on_same_host_before_observers_and_show(self):
        hook = '    native_input::install(host.window.window());'
        observer = '    native_workflow::install(&host.window, &host.state).map_err(HostError::Native)?;'
        for new in (
            '',
            '    native_input::install(other.window.window());',
            '    let unused = || native_input::install(host.window.window());',
            '    if false { native_input::install(host.window.window()); }',
            '    host.window.show().map_err(HostError::Platform)?;\n' + hook,
        ):
            with self.subTest(new=new):
                self.rejected(n.HOST, hook, new)
        files = copy.deepcopy(self.files)
        self.change(files, n.HOST, hook + '\n' + observer, observer + '\n' + hook)
        self.assertTrue(n.check(files))
        self.rejected(n.HOST, '#[cfg(any(windows, test))]\nmod native_input;',
                      '#[cfg(test)]\nmod native_input;')

    def test_fixture_constructor_installs_no_native_hook_or_observer(self):
        anchor = '    let sources = SourceDrafts::fixture().map_err(HostError::Fixture)?;'
        for call in ('native_input::install(window.window());', 'native_workflow::install(&window, &state)?;',
                     'slint::BackendSelector::new().select()?;', 'window.show()?;'):
            with self.subTest(call=call):
                self.rejected(n.HOST, anchor, '    ' + call + '\n' + anchor)

    def test_one_hook_cannot_be_replaced_by_test_or_duplicated_by_method_or_ufcs(self):
        self.rejected(n.INPUT, 'window.on_winit_window_event(filter_window_event);',
                      '// window.on_winit_window_event(filter_window_event);')
        self.rejected(n.INPUT, 'window.on_winit_window_event(filter_window_event);',
                      'window.on_winit_window_event(|_, _| slint::winit_030::EventResult::Propagate);')
        for extra in (
            'fn replace(window: &slint::Window) { window.on_winit_window_event(|_, _| EventResult::Propagate); }',
            'fn replace(window: &slint::Window) { WinitWindowAccessor::on_winit_window_event(window, |_, _| EventResult::Propagate); }',
        ):
            files = copy.deepcopy(self.files)
            files[n.BASE + 'other.rs'] = extra
            with self.subTest(extra=extra):
                self.assertTrue(n.check(files))
        files = copy.deepcopy(self.files)
        self.change(files, n.INPUT, 'window.on_winit_window_event(filter_window_event);', '')
        files[n.BASE + 'other.rs'] = '#[cfg(test)] ' + extra
        self.assertTrue(n.check(files))

    def test_ime_policy_cursors_and_default_lifecycle_propagation_are_closed(self):
        for old, new in (
            ('WindowEvent::Ime(Ime::Preedit(text, cursor)) => ime_preedit_allowed(text, *cursor),',
             'WindowEvent::Ime(Ime::Preedit(text, cursor)) => true,'),
            ('WindowEvent::Ime(Ime::Commit(text)) => ime_commit_allowed(text),',
             'WindowEvent::Ime(Ime::Commit(text)) => keyboard_chunk_allowed(text),'),
            ('        _ => true,', '        _ => false,'),
            ('        EventResult::Propagate\n    } else {\n        EventResult::PreventDefault',
             '        EventResult::PreventDefault\n    } else {\n        EventResult::Propagate'),
            ('start <= text.len() && end <= text.len()', 'start <= text.len()'),
            ('start <= text.len() && end <= text.len()', 'start <= end && end <= text.len()'),
            ("(b' '..=b'~').contains(&byte)", 'byte.is_ascii()'),
            ('const MAX_NATIVE_INPUT_BYTES: usize = 1024;', 'const MAX_NATIVE_INPUT_BYTES: usize = 1025;'),
        ):
            with self.subTest(new=new):
                self.rejected(n.INPUT, old, new)
        self.rejected(n.RAW, 'pub const MAX_RAW_TEXT_BYTES: usize = 1024;',
                      'pub const MAX_RAW_TEXT_BYTES: usize = 2048;')

    def test_keyboard_guards_original_borrowed_chunks_before_one_derived_key(self):
        for old, new in (
            ('WindowEvent::KeyboardInput { event, .. } => keyboard_event_allowed(event),',
             'WindowEvent::KeyboardInput { event, .. } => true,'),
            ('text.len() <= MAX_NATIVE_INPUT_BYTES', 'text.chars().count() <= MAX_NATIVE_INPUT_BYTES'),
            ('.is_some_and(|text| !keyboard_chunk_allowed(text.as_str()))',
             '.is_some_and(|text| !ime_commit_allowed(text.as_str()))'),
            ('if let Key::Character(text) = &event.logical_key',
             'if let Key::Character(text) = &event.key_without_modifiers()'),
            ('!matches!(event.key_without_modifiers(), Key::Character(text) if !keyboard_chunk_allowed(text.as_str()))',
             'true'),
            ('    if event\n        .text', '    let _ = event.key_without_modifiers();\n    if event\n        .text'),
        ):
            with self.subTest(new=new):
                self.rejected(n.INPUT, old, new)

    def test_unsupported_capture_macro_cfg_and_source_bounds_fail_closed(self):
        for old, new in (
            ('#![forbid(unsafe_code)]', '#![forbid(unsafe_code)]\nmacro_rules! matches { ($($x:tt)*) => { true } }'),
            ('#[cfg(windows)]\npub(super) fn install', '#[cfg(test)]\npub(super) fn install'),
            ('window.on_winit_window_event(filter_window_event);',
             'window.on_winit_window_event(move |window, event| { let retained = window.clone(); filter_window_event(window, event) });'),
        ):
            with self.subTest(new=new):
                self.rejected(n.INPUT, old, new)
        files = copy.deepcopy(self.files)
        files[n.INPUT] += '\n//' + 'x' * 32768
        self.assertTrue(n.check(files))
        files = copy.deepcopy(self.files)
        files[n.MAIN] = ' ' * (n.MAX_FILE_BYTES + 1)
        self.assertTrue(n.check(files))
        files = copy.deepcopy(self.files)
        files[n.MAIN] = None
        self.assertTrue(n.check(files))


    def test_complete_critical_cfg_stacks_reject_additional_disabling_attributes(self):
        for path, signature in ((n.MAIN, 'fn main()'), (n.HOST, 'pub fn run_native()'),
                                (n.HOST, 'pub fn create_native()')):
            for attribute in ('#[cfg(any())]', '#[cfg_attr(windows, cfg(any()))]'):
                files = copy.deepcopy(self.files)
                old = '#[cfg(windows)]\n' + signature
                self.change(files, path, old, attribute + '\n' + old)
                with self.subTest(path=path, signature=signature, attribute=attribute):
                    self.assertIn('native input unique current function', n.check(files))
        for attribute in ('#[cfg(any())]', '#[cfg_attr(windows, cfg(any()))]'):
            files = copy.deepcopy(self.files)
            old = '#[cfg(any(windows, test))]\nmod native_input;'
            self.change(files, n.HOST, old, attribute + '\n' + old)
            with self.subTest(module_attribute=attribute):
                self.assertIn('native input private platform module', n.check(files))
        for attribute in ('#![cfg(any())]', '#![cfg_attr(windows, cfg(any()))]'):
            files = copy.deepcopy(self.files)
            files[n.MAIN] = attribute + '\n' + files[n.MAIN]
            with self.subTest(main_inner_attribute=attribute):
                self.assertIn('native input current main attribute prefix', n.check(files))

    def test_quote_char_and_raw_string_cannot_hide_a_live_additional_setter(self):
        literals = ["'\"'", "'\\\"'", 'r#"quoted " text"#', 'cr#"""#', 'r"\\"']
        for literal in literals:
            files = copy.deepcopy(self.files)
            files[n.BASE + 'other.rs'] = ('fn replace(window: &slint::Window) { let quote = '
                + literal + '; window.on_winit_window_event(|_, _| EventResult::Propagate); }')
            with self.subTest(literal=literal):
                self.assertEqual(n.check(files), ['native input unsupported hook-owner literal syntax'])
            # Unsupported syntax in unrelated, hook-free sources contributes no
            # hook witness, while it still receives all inventory byte limits.
            files[n.BASE + 'other.rs'] = 'fn unrelated() { let quote = ' + literal + '; }'
            with self.subTest(hook_free_literal=literal):
                self.assertEqual(n.check(files), [])


if __name__ == '__main__':
    unittest.main()
