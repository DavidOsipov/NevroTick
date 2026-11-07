"""Independent inert controls for core admission/accounting; no Cargo/product runs.

All fake source files, reports and command observations are explicitly synthetic.
These controls exercise actual Python readers/parsers/loops, not Rust behavior or
actual tool admission. The coordinator owns execution and receipt collection.
"""
from __future__ import annotations

import contextlib
import copy
import hashlib
import importlib.util
import io
import json
from pathlib import Path
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import Mock, patch

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("core_analysis_controls", ROOT / "scripts/verify-core-analysis.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)

# Reuse independently authored literal transport rows, never production recipes.
LIFECYCLE_FIXTURES_SPEC = importlib.util.spec_from_file_location(
    "core_lifecycle_transport_fixtures", ROOT / "scripts/tests/test_core_combination_accounting.py")
LIFECYCLE_FIXTURES = importlib.util.module_from_spec(LIFECYCLE_FIXTURES_SPEC)
LIFECYCLE_FIXTURES_SPEC.loader.exec_module(LIFECYCLE_FIXTURES)

ADAPTIVE_FIXTURES_SPEC = importlib.util.spec_from_file_location(
    "core_adaptive_transport_fixtures", ROOT / "scripts/tests/test_core_adaptive_accounting.py")
ADAPTIVE_FIXTURES = importlib.util.module_from_spec(ADAPTIVE_FIXTURES_SPEC)
ADAPTIVE_FIXTURES_SPEC.loader.exec_module(ADAPTIVE_FIXTURES)


class ExactRegistryProjectionTests(unittest.TestCase):
    """Lock parsing controls use inert registry blocks and never execute Cargo."""

    REGISTRY = 'registry+https://github.com/rust-lang/crates.io-index'
    LOCAL = ('version = 4\n\n[[package]]\nname = "time-core"\nversion = "0.0.0"\n'
             'dependencies = ["garde", "time-domain"]\n\n[[package]]\n'
             'name = "time-domain"\nversion = "0.0.0"\ndependencies = ["garde"]\n')

    @classmethod
    def block(cls, name, version, dependencies=(), source=None):
        """Construct independent literal machine-style package bytes."""
        text = ('[[package]]\nname = "' + name + '"\nversion = "' + version + '"\n'
                'source = "' + (source or cls.REGISTRY) + '"\nchecksum = "' + 'a' * 64 + '"\n')
        if dependencies:
            text += 'dependencies = [\n' + ''.join(' "' + value + '",\n' for value in dependencies) + ']\n'
        return text

    def product(self, qualified=False):
        """A garde-only closure shares syn's name with an unrelated GUI version."""
        return (self.LOCAL + self.block('garde', '0.23.0', ['syn 2.0.119' if qualified else 'syn'])
                + self.block('syn', '2.0.119'))

    def test_exact_map_accepts_distinct_versions_without_merging_or_byte_changes(self):
        old = self.block('syn', '2.0.119')
        new = self.block('syn', '3.0.9')
        result = MODULE.registry_blocks(old + new)
        self.assertEqual(set(result), {('syn', '2.0.119'), ('syn', '3.0.9')})
        self.assertEqual(result[('syn', '2.0.119')][1], old)
        self.assertEqual(result[('syn', '3.0.9')][1], new)

    def test_exact_edges_resolve_qualifiers_and_reject_ambiguous_missing_or_foreign_sources(self):
        blocks = MODULE.registry_blocks(self.block('syn', '2.0.119') + self.block('syn', '3.0.9'))
        self.assertEqual(MODULE.resolve_registry_dependency('syn 2.0.119', blocks), ('syn', '2.0.119'))
        self.assertEqual(MODULE.resolve_registry_dependency('syn 2.0.119 (' + self.REGISTRY + ')', blocks),
                         ('syn', '2.0.119'))
        for reference in ('syn', 'syn 9.0.0', 'absent', 'syn 2.0.119 (registry+https://wrong.invalid)',
                          'syn 2.0.119 extra', '', 1):
            with self.subTest(reference=reference), self.assertRaises(MODULE.Incomplete):
                MODULE.resolve_registry_dependency(reference, blocks)
        self.assertEqual(MODULE.resolve_registry_dependency('syn', MODULE.registry_blocks(self.block('syn', '2.0.119'))),
                         ('syn', '2.0.119'))

    def test_duplicate_exact_records_and_unreviewed_origins_reject(self):
        for text in (self.block('syn', '2.0.119') * 2,
                     self.block('syn', '2.0.119', source='git+https://wrong.invalid'),
                     self.block('syn', '2.0.119') + self.block('unreached', '1.0.0', source='registry+https://wrong.invalid')):
            with self.subTest(text=text), self.assertRaises(MODULE.Incomplete):
                MODULE.registry_blocks(text)

    def test_irrelevant_gui_duplicates_and_qualified_selected_labels_preserve_fuzz_bytes(self):
        control = self.block('libfuzzer-sys', '0.4.13')
        expected = MODULE.reviewed_fuzz_lock(self.product(), control)
        candidate = (self.product(qualified=True) + self.block('syn', '3.0.9')
                     + self.block('windows-sys', '0.52.0') + self.block('windows-sys', '0.61.2'))
        self.assertEqual(MODULE.reviewed_fuzz_lock(candidate, control), expected)
        self.assertIn(control, expected)
        self.assertNotIn('3.0.9', expected)
        self.assertNotIn('windows-sys', expected)

    def test_checked_in_standalone_lock_bytes_remain_the_current_pure_projection(self):
        control = (ROOT / 'policy/time-core-fuzz.lock').read_text()
        self.assertEqual(MODULE.reviewed_fuzz_lock((ROOT / 'Cargo.lock').read_text(), control), control)

    def test_selected_label_projection_changes_no_source_checksum_or_other_field(self):
        control = self.block('libfuzzer-sys', '0.4.13')
        product = self.product(qualified=True) + self.block('syn', '3.0.9')
        before = MODULE.registry_blocks(product)[('garde', '0.23.0')][0]
        result = MODULE.reviewed_fuzz_lock(product, control)
        after = MODULE.registry_blocks(result)[('garde', '0.23.0')][0]
        self.assertEqual(after, dict(before, dependencies=['syn']))
        for change in ('source', 'version', 'checksum'):
            baseline = MODULE.reviewed_fuzz_lock(self.product(), control)
            if change == 'source':
                changed = product.replace(self.REGISTRY, 'registry+https://wrong.invalid', 1)
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.reviewed_fuzz_lock(changed, control)
            else:
                changed = product.replace('version = "0.23.0"', 'version = "0.23.1"', 1) if change == 'version' else product.replace('checksum = "' + 'a' * 64, 'checksum = "' + 'b' * 64, 1)
                self.assertNotEqual(MODULE.reviewed_fuzz_lock(changed, control), baseline)

    def test_reached_multi_versions_and_original_fuzz_conflicts_stay_rejected(self):
        cases = [
            (self.LOCAL + self.block('garde', '0.23.0', ['syn']) + self.block('syn', '2.0.119')
             + self.block('syn', '3.0.9'), self.block('libfuzzer-sys', '0.4.13')),
            (self.LOCAL + self.block('garde', '0.23.0', ['syn 2.0.119', 'syn 3.0.9'])
             + self.block('syn', '2.0.119') + self.block('syn', '3.0.9'), self.block('libfuzzer-sys', '0.4.13')),
            (self.product(), self.block('syn', '3.0.9')),
            (self.product(), self.block('syn', '2.0.119', ['changed-edge'])),
            (self.product(), self.block('original-only', '1.0.0') + self.block('original-only', '2.0.0')),
            (self.product().replace('"syn"', '"missing"', 1), self.block('libfuzzer-sys', '0.4.13')),
        ]
        for product, control in cases:
            with self.subTest(product=product, control=control), self.assertRaises(MODULE.Incomplete):
                MODULE.reviewed_fuzz_lock(product, control)

    def test_fuzz_vendor_admission_uses_reached_version_among_gui_duplicates(self):
        class ReachedOwnedConfiguration(Exception):
            pass

        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            work, vendor = root / 'work', root / 'vendor'
            work.mkdir()
            source = vendor / 'syn-2.0.119'
            source.mkdir(parents=True)
            (source / 'lib.rs').write_text('// Inert source custody fixture; never compiled.\n')
            files = MODULE.PRODUCT.inventory(source)
            checksum = {'package': 'a' * 64, 'files': files}
            (source / '.cargo-checksum.json').write_text(json.dumps(checksum))
            package = {'name': 'syn', 'version': '2.0.119', 'checksum': checksum['package']}
            selected = {'manifest': 'policy/time-core-fuzz-manifest.toml',
                        'lockfile': 'policy/time-core-fuzz.lock',
                        'target_source': 'policy/time-core-fuzz-target.rs',
                        'locked_registry_packages': [package]}
            session = SimpleNamespace(work=work, vendor=vendor, dependency_policy={'packages': [
                {'name': 'syn', 'version': '3.0.9', 'files_sha256': '0' * 64},
                {'name': 'syn', 'version': '2.0.119', 'files_sha256': MODULE.PREPARE.files_digest(files)},
            ]}, configure=Mock(side_effect=ReachedOwnedConfiguration))
            with patch.object(MODULE, 'admit_fuzz_sources', return_value={'locked_registry_packages': []}):
                with self.assertRaises(ReachedOwnedConfiguration):
                    MODULE.fuzz(session, selected, {})
            session.configure.assert_called_once_with(MODULE.COMMON.FUZZ_CARGO_CONFIG)
            self.assertEqual((work / 'fuzz-vendor/syn-2.0.119/lib.rs').read_bytes(), (source / 'lib.rs').read_bytes())


class HistoricalS14cTestCase(unittest.TestCase):
    """Run predecessor parser controls under their explicit qualified profile.

    These fixtures admit no current Rust bytes or campaign. Current four-owner
    admission is checked separately by CheckedInCorePolicy/SettledRate controls.
    """

    ALLOWANCE_MAP_NAMES = (
        'GLOBAL_LIMITS', 'SELECTION_GLOBAL_LIMITS', 'SELECTION_HELPER_GLOBAL_LIMITS',
        'SELECTION_HELPER_CANONICAL_GLOBAL_LIMITS', 'SOURCE_RAW_PRIVATE_GLOBAL_LIMITS',
        'PROJECTION_TELEMETRY', 'ENCLOSURE_TELEMETRY', 'PROJECTED_BATCH_TELEMETRY',
        'PRINCIPAL_TELEMETRY', 'COMBINATION_TELEMETRY', 'LIFECYCLE_TELEMETRY',
        'ADAPTIVE_TELEMETRY', 'PROJECTION_ADAPTER_TELEMETRY', 'STANDARD_ERROR_TELEMETRY',
        'TRACKING_TELEMETRY', 'CHART_TELEMETRY', 'CHART_BATCH_TELEMETRY',
        'CHART_PRINCIPAL_TELEMETRY', 'CHART_COMBINATION_TELEMETRY',
        'CHART_COMPOSITION_TELEMETRY', 'ACHIEVED_PHASE_TELEMETRY',
    )

    def run(self, result=None):
        names = ('SUBJECTS', 'SCOPE', 'COORDINATOR_COHORTS', 'MUTATION_INVENTORIES',
                 'MUTATION_ERROR_VALUES', 'FINAL_PACKAGE_TEST_COUNTS',
                 'INVENTORY_FIXED', 'INVENTORY_ADMISSION', 'INVENTORY_BOUND_INPUTS',
                 'INVENTORY_SOURCE_FILE_COUNT', 'INVENTORY_SUBJECTS')
        fixed = {**MODULE.FIXED, **{
            key: MODULE.S14C_FIXED[key] for key in
            ('schema_version', 'source_file_count', 'final_package_test_counts', 'critical_files',
             'mutation_inventories', 'mutation_error_values', 'coordinator_cohorts')}}
        fixed.pop('rate_delivery_telemetry')
        with contextlib.ExitStack() as stack:
            for name in self.ALLOWANCE_MAP_NAMES:
                mapping = getattr(MODULE, name)
                stack.enter_context(patch.dict(mapping, copy.deepcopy(mapping), clear=True))
            for name in names:
                stack.enter_context(patch.object(MODULE, name, copy.deepcopy(getattr(MODULE, 'S14C_' + name))))
            stack.enter_context(patch.object(MODULE, 'FIXED', fixed))
            stack.enter_context(patch.object(MODULE, 'BOUND_INPUTS', MODULE.S14C_INVENTORY_BOUND_INPUTS))
            stack.enter_context(patch.object(MODULE, 'SOURCE_FILE_COUNT', 156))
            stack.enter_context(patch.object(MODULE, 'COHORT_PLAN', 'tasks/s1.4c-achieved-phase-cohort-plan.json'))
            critical = {key: value for key, value in MODULE.COMPLETE_CRITICAL_SUBJECTS.items() if key != 'rate_delivery'}
            stack.enter_context(patch.object(MODULE, 'COMPLETE_CRITICAL_SUBJECTS', critical))
            reservation = MODULE.bind_checkpoint_source_allowances()
            stack.enter_context(patch.object(MODULE, 'CHECKPOINT_SOURCE_RESERVATION', reservation))
            return super().run(result)


# Independent checkpoint facts; historical parser funding stays below.
S14C_SUBJECTS = {
    'reference_input': 'crates/time-core/src/reference_input.rs',
}
S14B_SUBJECTS = {
    'chart': 'crates/time-core/src/regression/chart.rs',
    'chart_roundoff': 'crates/time-core/src/regression/chart/roundoff.rs',
    'chart_batch': 'crates/time-core/src/chart_batch.rs',
}
S158_SUBJECTS = {
    'combination': 'crates/time-core/src/combination.rs',
    'combination_admission': 'crates/time-core/src/combination/admission.rs',
    'combination_lifecycle': 'crates/time-core/src/combination/lifecycle.rs',
    'chart': 'crates/time-core/src/regression/chart.rs',
    'chart_roundoff': 'crates/time-core/src/regression/chart/roundoff.rs',
    'chart_combination_roundoff': 'crates/time-core/src/combination/chart/roundoff.rs',
    'chart_combination_receiving': 'crates/time-core/src/combination/chart/receiving.rs',
    'chart_combination_reduction': 'crates/time-core/src/combination/chart/reduction.rs',
    'chart_combination_lifecycle': 'crates/time-core/src/combination/lifecycle/chart.rs',
}
S158_COUNTS = {'ordinary': 282, 'doctests': 24, 'baseline': 306}

# Frozen metadata from original S158 policy63549c87; no numerical budget copy.
# This literal preserves historical validator controls after the current policy
# moves to empty E. It is never current product admission or score evidence.
S158_POLICY_FIXTURE = json.loads(r'''{
  "inputs_sha256": {
    ".cargo/audit.toml": "00980fdea89e771a1c60b4495a713f2a3f50880b257020f890a22a723682a813",
    ".cargo/config.toml": "3950d427469851b5965a18eaaefce065c862ccec9c9861637c3e360f6a429f78",
    ".cargo/mutants.toml": "af0e3b1d7774464fb6d3b8566b61472a853ce2e6b0ba26a1c4fbe82f676b5f5b",
    ".config/nextest.toml": "138a3584caddb823362272ed590954f58e228913bddc4ce1fe2e3b84e8ee9f35",
    "Cargo.lock": "416b349d87cc0e53562d79d494316fd20c210ede2e45c2568f5973652753ec8f",
    "Cargo.toml": "2410193ccead664712fe69ee36c3ba0f88fe5b0d61aedcba65ad668a1844083b",
    "clippy.toml": "6e6800cb3d293f1ec076d5f3efa33cfecd93850e497a952cc058d7fc6503af50",
    "crates/nevrotick-engine/Cargo.toml": "8399d79a18b2746d2a93aa5299df631fd4297f146b880831a84b3b6b2abd42e1",
    "crates/nevrotick-engine/src/bin/nevrotick-observe.rs": "6f51aafe69426edd03d276014e58bb7bd5140f11e9ba1c5f724eff5f2f174ecf",
    "crates/nevrotick-engine/src/lib.rs": "71cb5a0d994a7f0aa895922f95a0302dcc3152988162a43ccc9aa7d57acdf5fe",
    "crates/nevrotick-engine/tests/execute_entered.rs": "a8fea0f70cf2bc147900e2e639a3252c0c7dea0b9a10f2d3f25794273f9ba97a",
    "crates/nevrotick-engine/tests/prepare_entered.rs": "84a38779f863697a58f36e7ebf8d4acf80d0bab653fe2ae0ac488fa2ece96666",
    "crates/nevrotick-engine/tests/prepare_release.rs": "f089024cd0bea7c84bbba360c55cf78683978983966267573c1ae6b9f198991d",
    "crates/nevrotick-engine/tests/private/argument_guards.rs": "e7f122097eb04677484641b33f8682da6950e66e67f7558f2ba6c1b06aab653a",
    "crates/nevrotick-engine/tests/private/processing_guards.rs": "35e0f34f0a1ada129d10558e10d747f0a9f12b449b4c82ee81b61777553baabe",
    "crates/nevrotick-engine/tests/runtime_boundary.rs": "f056bfe1b515e84f885c7630b79d254913692635653cae86e6dbaad61cd2e11e",
    "crates/nevrotick-ui/Cargo.toml": "cf4d4ac8953f22c2060e3d99550bea9db6a394dd94c924123a40f68b2276475a",
    "crates/nevrotick-ui/src/lib.rs": "b03120485dc7cf1f2608887b3f90334270a4f5a87a612c2946618d8b11973930",
    "crates/nevrotick-ui/src/main.rs": "fb76561d571a6b3de3c3e9e44c5dadd5e04e8552a216143cfd9035ab71498e31",
    "crates/nevrotick-ui/src/model.rs": "e433cba003ba7dff0c20e3259f5815ea6962182f85b999ef09739622aa63a49f",
    "crates/nevrotick-ui/src/native.rs": "39d1a94900cc943bf19ae940ddc303cffa5f2f29da2e03eee986c62c4a44799b",
    "crates/nevrotick-ui/src/native/controls.rs": "338416ea51cf74ad6d89ba1f84928dd76104974037edee55bd0170c01de1c48f",
    "crates/nevrotick-ui/src/native/view.rs": "d5f95514dfc29f8c1db6b9f8049cba61e008a4c9482efb17c9762fa63bfb27ac",
    "crates/nevrotick-ui/tests/preview.rs": "34e717874498935dee0aebdda0b88306fbf9d32ae0846bb3c70f688d6fcd1a21",
    "crates/ntp-exchange/COPYING": "ab15fd526bd8dd18a9e77ebc139656bf4d33e97fc7238cd11bf60e2b9b8666c6",
    "crates/ntp-exchange/Cargo.toml": "ddcfb2276783c3583a5c69b1e9027a7f342612fe15ad177a8c05339e6333e18f",
    "crates/ntp-exchange/PROVENANCE.md": "9fd2700c4f15511237d8ac91abf729bb712a9ff1f99444354273094eeae0c951",
    "crates/ntp-exchange/clippy.toml": "9ed31945c67b7d684aaf9e5980d9fd7e1b0f352af4dbc275f6e7dea53bb3da0c",
    "crates/ntp-exchange/src/association.rs": "627a342203a7bd3c9c540fdc5490b4e2701bb955cc5413ae3569b5e6c17dce39",
    "crates/ntp-exchange/src/context.rs": "3d84e94e9d930315a0bbc9983c21d97841cb17ed4e01c4f5e2a30e69037f10c7",
    "crates/ntp-exchange/src/lib.rs": "68811f208aff164ddc3ef911beb85bff470a1469bacaa7b5e5e5708db08b1143",
    "crates/ntp-exchange/tests/association.rs": "09a8b65f511b94549474c373ba76b6fcb69df8b6f8eac5565a7d9dbb4728ef05",
    "crates/ntp-exchange/tests/support/scenario_driver.rs": "6dc32362082b6e036687d35ab71184ed2f5f0cb8353c083e44160245d167b31e",
    "crates/ntp-exchange/tests/support/scenario_expectations.rs": "7e4603160857f8591429ba82b19d1dc67c6f42701761e3dd6b00894568bf8a20",
    "crates/ntp-exchange/tests/support/vectors.rs": "3977e5a998119ac851bef1d1bc48b449c1fb2e5396d33c5c0aa7ad380f07eded",
    "crates/ntp-wire/COPYING": "ab15fd526bd8dd18a9e77ebc139656bf4d33e97fc7238cd11bf60e2b9b8666c6",
    "crates/ntp-wire/Cargo.toml": "dc75f8a638a956217d3ecfa666a840b2ecef99c0a06dac7443ab5cf775032579",
    "crates/ntp-wire/PROVENANCE.md": "b40bdd9417b3c24d5268220325b2afe4ba477c0d4adc736e1548da12f49acd8d",
    "crates/ntp-wire/src/datagram.rs": "dd2930d03a1c3278a300a60e1cddc4420bf11ae3ce562fe15978a03bac183cf3",
    "crates/ntp-wire/src/header.rs": "daadf927a85636e261323eb38e613877e21d54dc5f64e5b5dd49b17b60af70da",
    "crates/ntp-wire/src/lib.rs": "b5615163e8f2bfade4e46b8e78319bcfbf2ae2dee83579b731d04587daaf695c",
    "crates/ntp-wire/src/nts_ke.rs": "330b62944f23d5c4fb0a507f4e9cd7ea2f29064536ece9d7837096b932461ab3",
    "crates/ntp-wire/src/nts_negotiation.rs": "0981341a360c8a93e92b6a309540b2409dac1e1fdf7dd5ca9c0a5bbc49da86b1",
    "crates/ntp-wire/src/nts_packet.rs": "effe5a07d2543fa993ebf66f61e76d3d255c422600bfc0b9d29a6231c513f0a8",
    "crates/ntp-wire/src/nts_plaintext.rs": "065f21a8a8b9e2ca7bfee8b80f78b2bd94fe5317cd988451536ea68da1bae0c6",
    "crates/ntp-wire/tests/datagram.rs": "957884ff9f7afcd17e8f4696e1330c2162834a81955bf44d45af1025ef018bce",
    "crates/ntp-wire/tests/header.rs": "bd6c57a35eb26c73af9d9d52262294e54d29ec3b71ae1cecbc908add963372e8",
    "crates/ntp-wire/tests/nts_export.rs": "4e667112b6f487e5c1cb81eafbfc6e58d33b576c659ed14811ddb460d0c54200",
    "crates/ntp-wire/tests/nts_ke.rs": "1047ce4e5d600a115242be3cd00cb0a06c3c1c21754d7fea98995368e5c80e0e",
    "crates/ntp-wire/tests/nts_negotiation.rs": "9be895f688b2b9f07ed063da5283eaa3a4dad75a92054338f476344955f631b0",
    "crates/ntp-wire/tests/nts_packet.rs": "ba10fb8870c07dcd457cf5d65e13388ab326f97fa8b228212f6c886dd9eb1cb6",
    "crates/ntp-wire/tests/nts_plaintext.rs": "103e2782e1dab3c72762d7802e78cddeaa90eb342641ba0131225d19f0a4e005",
    "crates/ntp-wire/tests/nts_request.rs": "b0c304bc0961bde5a3a6adba201997768cf7579232a8849f5dad06f967facf12",
    "crates/ntp-wire/tests/nts_stream.rs": "3d19462e28c19557fa641c94fc8dce4da18f4c41efc70df746d6c65cc490de7c",
    "crates/ntp-wire/tests/support/vectors.rs": "4878fea3db71ddd458abf11cd63a8ad7ef96b05ab61091eadd14cf732d5fb718",
    "crates/time-core/COPYING": "ab15fd526bd8dd18a9e77ebc139656bf4d33e97fc7238cd11bf60e2b9b8666c6",
    "crates/time-core/Cargo.toml": "8b51f9fdfb2b65c757a7ad9e9c592340e2873719db14a19fddeb6747b69bcea6",
    "crates/time-core/PROVENANCE.md": "3d79995e4743484bbc4b7a78ef8303f00d92e4ddf93d2ad11077c251fffbc1e6",
    "crates/time-core/clippy.toml": "9ed31945c67b7d684aaf9e5980d9fd7e1b0f352af4dbc275f6e7dea53bb3da0c",
    "crates/time-core/src/chart_batch.rs": "32269a5ce0cddc9db1c43382d142beaf80cdabe636ad2e3d7e4c1f2f49ec947c",
    "crates/time-core/src/chart_enclosure.rs": "6785e1caa3274d9f2561735ad611128f1ceded20c739337dd8f70de4e863c0c2",
    "crates/time-core/src/combination.rs": "d33493baf6505782e6c9061e04a6204d1dc65b1a05f0acf0120dc2831f698326",
    "crates/time-core/src/combination/admission.rs": "7519a3ffabcbe05e7b4adc24baff0bcbbdcc1be31768454b49708ab190350bc0",
    "crates/time-core/src/combination/chart.rs": "71520d44b91d321caea3bceaff043a42ae5ad6d3e65a23218ad072387a2f0eed",
    "crates/time-core/src/combination/chart/receiving.rs": "4e0687bd5b3dd1e1ada09af7e17a561f5c1ee322761aae76f8c0c2964ff1c553",
    "crates/time-core/src/combination/chart/receiving_tests.rs": "b4234295d21a3b5a0df40c99803ff7625f21143b44b9db87149eb2518919f132",
    "crates/time-core/src/combination/chart/reduction.rs": "3a9e98e64224ca0ad5f4c016a5c370dd305e52aef6dc8d38b4f2cc2db7eb74eb",
    "crates/time-core/src/combination/chart/roundoff.rs": "1001dffe6cac911207879962f39b3aeff5a0668916cf53b5e42986523ff1c5eb",
    "crates/time-core/src/combination/lifecycle.rs": "6e415208ccfd93ebe976c09c887c8c42090f85bfec313a17c19594a891dc5081",
    "crates/time-core/src/combination/lifecycle/chart.rs": "b2195651f756deed769f2b8183ae57c5fd622e38fd558060c49a44e2084b2ec7",
    "crates/time-core/src/lib.rs": "3be6021d5a889950f320ba16f16e2b652cd67e701cb2716e538df0da430ddb16",
    "crates/time-core/src/principal.rs": "cf4962c8d3536a528b7c783543379bdaeaf2e4070355fbeb7921fb5329b63d1d",
    "crates/time-core/src/principal/chart.rs": "397e5138ded2186527cc5a6f9dcb8526aaab3211da1e1d805ba2b4980fdd9f9c",
    "crates/time-core/src/projected_batch.rs": "e5566c764afab0b10414047894c29e99864b7ca373833f6f92aac10082412cab",
    "crates/time-core/src/projection_enclosure.rs": "6b7b079f7d820e611bd13bb96361c243973d231a7f65a07fb6d4350a63282023",
    "crates/time-core/src/regression.rs": "52223fcf848927b344673ec810ca0c784045c8e61835a01e9451de236d8f28d4",
    "crates/time-core/src/regression/adaptive_history.rs": "dfe76dd00208d6cfded982b5cb98a77ec1c21cc02b7c2cdf4c26a0f837f225f5",
    "crates/time-core/src/regression/chart.rs": "ed2c7d54b7a9a346327199b3b4d3b32fc8ed14f8d7c5a6bb61b867b820f9aed0",
    "crates/time-core/src/regression/chart/roundoff.rs": "99bb94b460a66401ea683440b3991ae75a11f3ca1de75dc4fceb71f7109e9169",
    "crates/time-core/src/regression/history.rs": "1ca8462c25de0e82f04fc3ede2ac05be9c3f972251da01dc5b1f66344828bbeb",
    "crates/time-core/src/regression/normalization.rs": "c1d23a6add923bb588bd6b9949f8151d3e7bd22b6c15f3c2123cb5056327ecc3",
    "crates/time-core/src/regression/projection.rs": "cb5af00a9dfb731b5d2b95fd2cd69f6e08b856097948e5b1c3719a6ee9b23dc5",
    "crates/time-core/src/regression/residuals.rs": "4494ae6e8fd3967d4f3e5f91774c7192b85be8cbe5bb9d90366eb7201db93686",
    "crates/time-core/src/regression/runs.rs": "275e850035bf0c381e2a0476e6eb6e942ba97fcc18b60cdf0e01c047f15e165a",
    "crates/time-core/src/selection.rs": "da8b87fe69215b71419ff370d4032f0dc49363131c75e70076cb2cce6305d19e",
    "crates/time-core/tests/adaptive_history.rs": "0b34520494b848709015a05d395a82c247474445feaa060a50eff257efaae342",
    "crates/time-core/tests/chart.rs": "838a8b05014564eb7dff44ede2d1d8e63cfea2b3c0c3be10a56d0e023f171228",
    "crates/time-core/tests/chart_combination_edges.rs": "ed0055ab15c9665c56b737c9958abf1f74bd7a718db7a4552c6207431421d360",
    "crates/time-core/tests/chart_combination_lifecycle.rs": "c90a40dfd8699c5a9fb7437b76aefb7f703446adc757b63b5ed914439c2b6da4",
    "crates/time-core/tests/combination_lifecycle.rs": "cb80c8e934b9a1dc6ef23023d52c378e8b70980613b6df6869e94ad4286e6ad9",
    "crates/time-core/tests/history.rs": "d50716c686b9bd8888b5324b203b5756e7d104af83291d21b7e0f12f3af5a095",
    "crates/time-core/tests/normalization.rs": "323be5e7b5f63913af12c624bfa4c1e4a2181d99ac4579db2314fdd1d53db5e1",
    "crates/time-core/tests/principal.rs": "cc912848779f75b2aee8591de424d6bb5d662e93c10bd2a3d62d3e4315bb3fe9",
    "crates/time-core/tests/private/adaptive_history_guards.rs": "06721a1264a45a9f8f697a7de014c9a26afe699a7c80978506d7f5ea38393567",
    "crates/time-core/tests/private/chart_batch_cases.rs": "6d7638728ee09df57522771ec76465603a9a2bb46f44d754ba35f6f0d4c905d1",
    "crates/time-core/tests/private/chart_batch_guards.rs": "8c16f05d4155a2653100f6cc9392ff78bc0f931fe2eb12d6710cbd9ff4e1591f",
    "crates/time-core/tests/private/chart_combination_lifecycle_guards.rs": "42bd34d892968439bd5852857e2c56ae980a42f9ad4594a4fb7a688e99ff3e45",
    "crates/time-core/tests/private/chart_combination_roundoff.rs": "aa2fa8275c1199f2595c0f2528ea4ed3cd2d1177514dc79266cebde5f3efc579",
    "crates/time-core/tests/private/chart_guards.rs": "6b04885fd9b96baeb7516581d04919cad828fe5d3e429cff364fbddd023034cf",
    "crates/time-core/tests/private/chart_principal_cases.rs": "e3a5f2a52a6e98ec21d02342a4333d6581ec00fa227500c2de1c481cb396d5f1",
    "crates/time-core/tests/private/chart_principal_pending.rs": "61b67722a25f79b128b737eec8fade441bf224a6ed502e7d42d2030b440ae1ad",
    "crates/time-core/tests/private/chart_roundoff.rs": "cc2e0cd8a5003fd1d65cb0156f7c26bea33ebc8e182bac00d44e8c65b83de68e",
    "crates/time-core/tests/private/combination_cases.rs": "1e1d97c3635ef6917c0b4b9627fe27e2c56d132d68543189b63293f3307446e2",
    "crates/time-core/tests/private/enclosure_guards.rs": "64e12d347f5f1628b06cee364b51d6950e50a4426c6ef982d84607576673a16d",
    "crates/time-core/tests/private/lifecycle_admission.rs": "09810f71269decd7b27ac6584642503465a2eea9c95bbb8e995905a858ba07cc",
    "crates/time-core/tests/private/lifecycle_counter.rs": "18b0bbe30a786cc96d18c164290ffa231f15c97daefb44d6cfb1f661656ab0ec",
    "crates/time-core/tests/private/normalization_guards.rs": "92bc46d8f467720426d4d60c7a47cdb25fb3aa12ddb588597b212f7f6cbca580",
    "crates/time-core/tests/private/principal_guards.rs": "acf0c6c39a98a3c019a83c1ce4465841d7f8113ac3c6cd108c7018b69f9fbff6",
    "crates/time-core/tests/private/projected_batch_guards.rs": "dc791ec46140e8f447462dbb90d0fe291d4359dd4a162aa18a1e50b145ba835f",
    "crates/time-core/tests/private/projection_guards.rs": "c8b293c1e4ce886bf4c909752063e00ed2c535a05768d4fda8ad926601665555",
    "crates/time-core/tests/private/residual_guards.rs": "ce353bc270656f35471d229b0d0e57fc0063fb1c8c8e65bb95106591ebe8107b",
    "crates/time-core/tests/private/runs_guards.rs": "026d9011f245c3c5649b10d30a33181afe89e97a6a9b8a08a793354fe32a0df6",
    "crates/time-core/tests/private/runs_sign_observer.rs": "72441136bc1b2f357d13d7c9382b721955ed0e670d8160b946fd5ee6bd2b25bb",
    "crates/time-core/tests/private/selection_guards.rs": "bd958290436a819c664c419bb26bc8a2b44c323c74858ee985a91aa9e41b397d",
    "crates/time-core/tests/private/unit_guards.rs": "bed0f4f635751f99232ff7df5fa22c1bf3c7cf8dbe97f63920cc0008fecb5552",
    "crates/time-core/tests/projected_batch.rs": "11a035013bb3eb19af0f112b401f71a649d11daa009e6868727d5f2111f12301",
    "crates/time-core/tests/projection.rs": "4a75cdc23d1d433821941ec7b5b3d288289e76abde8bd154c88b9aedec8858a1",
    "crates/time-core/tests/projection_enclosure.rs": "13d15d30cd3d9a22414a34ce573b7e67ed281dd121b1cf1e8e5b514e4852d741",
    "crates/time-core/tests/regression.rs": "d8718a3894540c9acf89980022462af8ca15215c0f1d55711b1b59491bc97dca",
    "crates/time-core/tests/residuals.rs": "01b56e1688f14b6cbc52eca013af3613196bb827eaa0be29db2c63f1e2f63244",
    "crates/time-core/tests/retained_runs.rs": "51e27cdc763e0f895cd5fbe48649df3df7e040ba454e35e657ba7b6158fda2b6",
    "crates/time-core/tests/runs.rs": "8e6298c0553dbc7e8e74e2af8146c68d637c43e2161c30fa6fa9f700e99e112f",
    "crates/time-core/tests/selection.rs": "65cf8fde0f8f19486446d889083367ccabdc6051f682fd965fc17ce511c0709b",
    "crates/time-core/tests/support/history_facts.rs": "0124afaa33eabeaf5ba83504b153eb760bf1d407da648a6c79ec87cc8514e722",
    "crates/time-core/tests/support/normalization_facts.rs": "f537b6f805f7eabbf689c49063eb422a66ed8adcbea7618f6885677b4a172d04",
    "crates/time-core/tests/support/principal_cases.rs": "6f4b02a4399efe81d07205f3078de588376570cde256005ff1b072189ec2774c",
    "crates/time-core/tests/support/residual_facts.rs": "264e368d9c4f817423f957e3c6c6d9a65bb4a1846fcd6d4611fbfd4693cf4ce9",
    "crates/time-domain/COPYING": "ab15fd526bd8dd18a9e77ebc139656bf4d33e97fc7238cd11bf60e2b9b8666c6",
    "crates/time-domain/Cargo.toml": "e0cb947e288754b7a85d3ca059f048d931bfca6d1264b7dd472fdb62569163ac",
    "crates/time-domain/PROVENANCE.md": "c91a2fd4ba3a361ee0fdd9e72a468b64a6c21b45c5595b16ce06972f8e3b8716",
    "crates/time-domain/clippy.toml": "9ed31945c67b7d684aaf9e5980d9fd7e1b0f352af4dbc275f6e7dea53bb3da0c",
    "crates/time-domain/src/clock.rs": "beca67da84fc7a4717dd6f35325ec1308d4a7e5b6c2ad30dfc65392f94bbde51",
    "crates/time-domain/src/coordinate.rs": "63f7230f915491973d4d682ee66afa5dc0c22bd8bc38fcc375d1840b3c046e80",
    "crates/time-domain/src/era.rs": "366397df0c6bcd260d566b5bac310c882796734887af886a5fdf99a023bb1dc8",
    "crates/time-domain/src/exchange.rs": "d7208286fb97303a64b719aa880ee07ec2411fef8c17abbda9acfb5e2b33e7b6",
    "crates/time-domain/src/lib.rs": "882d62dc51517698ecba1daf94c6697092fe29c57d59a0ca31e782e2571c33db",
    "crates/time-domain/src/rounding.rs": "387fa926a079aa7b87477c97dea8b79049595ac8a793ead449b19abb8b0b5519",
    "crates/time-domain/tests/clock.rs": "0bcb41861a1de5cbf8e27caa60d7026ea1a8b409c4b412a42646ba5a5060e27a",
    "crates/time-domain/tests/coordinate.rs": "42260b6613becf9bb843accf5687c7d29ba81decf355e7052c2136da0ae15b2c",
    "crates/time-domain/tests/era.rs": "fc68ae397d2d1abd90faae3c8fd645e0d554aa0e47983f5ee2aca581206289a8",
    "crates/time-domain/tests/exchange.rs": "1baad389cb7962ed37e2ca0afff7395621cdb2503dcb2a40be29065ea39a6bde",
    "crates/time-domain/tests/rounding.rs": "c87e80db880cd376ef59be6f697076fb5eb39e5b9f44b5b3609974fdf9479eb6",
    "deny.toml": "8e3ce1731c0327df4cd2d14d8fc4b296d3e31e152fa144bbb983e0835e247ff1",
    "policy/rust-workspace.toml": "2c5a7920c366c21bb8989bb1595c12b33a47ff4aee5927d3157498be014f7b3c",
    "policy/tool-manifest.json": "9daab43041ce4b0e798ffb7e4dab3f9d6ac89a0811815f8550c4667203e17b53",
    "policy/ui-m0-dependencies.json": "7fd786ea6e874c518405e4b7c2c0a64bea932f42dff52bd3e8a37801ed911f93",
    "policy/verification-manifest.toml": "1bdd82bd45f4d4dda9a409b1380ae323d9d68238961b59d850ad798f0f5132e4",
    "rust-toolchain.toml": "2b58ea5089dfdfacf78ae2a5f52dfce89a583339ccc2c7cc0b40e96230dc9ccc",
    "rustfmt.toml": "b71fbf7f2204a6c2d97aea258e7712da4d1405fe6a51fb7bb8a951a5991c2406",
    "scripts/check-environment.py": "8a6ca976dcc46dc7986472c72bde8d0be0596d873afd2ec88b0419df8239d880",
    "scripts/check-strict.py": "ea8f3910cbbd6663ea38c2753ce21affc64d3e65a284a6d4c76e66c51395d172",
    "scripts/verify.py": "776ad897d2ae35bb7c26c30a31f86728d06d4fb985303a4d7f071b15bf649824"
  },
  "mutation_equivalences": [
    {
      "candidate_sha256": "5f0a23c12bf5adeae4bacd47592c0b7ec24714b5e862d5d76115483f6f15f0a7",
      "id": "S158-R-E-001",
      "mutant": {
        "file": "crates/time-core/src/combination/chart/receiving.rs",
        "function": {
          "function_name": "ChartTrackingSupporters<'entries, 'chart>::from_decision",
          "return_type": "-> Result<Self, FixtureTrackingCombinationError>",
          "span": {
            "end": {
              "column": 6,
              "line": 111
            },
            "start": {
              "column": 5,
              "line": 61
            }
          }
        },
        "genre": "BinaryOperator",
        "name": "crates/time-core/src/combination/chart/receiving.rs:87:23: replace || with && in ChartTrackingSupporters<'entries, 'chart>::from_decision",
        "package": "time-core",
        "replacement": "&&",
        "span": {
          "end": {
            "column": 25,
            "line": 87
          },
          "start": {
            "column": 23,
            "line": 87
          }
        }
      },
      "proof": {
        "path": "tasks/s1.3-chart-combination-equivalence.md",
        "reviewer": "root; s157_gate_review",
        "section": "S158-R-E-001",
        "sha256": "055046256bf3fa5aca61fb36042f819d2d53407512e0da1c89a801ccd040f085",
        "verdict": "independently-justified-equivalent"
      },
      "rationale": "Checked witnessed selections contain1\u201316 supporters; both bound comparisons are false.",
      "requirement": "CHECK-7/8/14/25",
      "source_sha256": "4e0687bd5b3dd1e1ada09af7e17a561f5c1ee322761aae76f8c0c2964ff1c553"
    },
    {
      "candidate_sha256": "5f0a23c12bf5adeae4bacd47592c0b7ec24714b5e862d5d76115483f6f15f0a7",
      "id": "S158-R-E-002",
      "mutant": {
        "file": "crates/time-core/src/combination/chart/receiving.rs",
        "function": {
          "function_name": "admit_chart_records",
          "return_type": "-> Result<(), FixtureCombinationError>",
          "span": {
            "end": {
              "column": 2,
              "line": 279
            },
            "start": {
              "column": 1,
              "line": 199
            }
          }
        },
        "genre": "BinaryOperator",
        "name": "crates/time-core/src/combination/chart/receiving.rs:216:19: replace || with && in admit_chart_records",
        "package": "time-core",
        "replacement": "&&",
        "span": {
          "end": {
            "column": 21,
            "line": 216
          },
          "start": {
            "column": 19,
            "line": 216
          }
        }
      },
      "proof": {
        "path": "tasks/s1.3-chart-combination-equivalence.md",
        "reviewer": "root; s157_gate_review",
        "section": "S158-R-E-002",
        "sha256": "055046256bf3fa5aca61fb36042f819d2d53407512e0da1c89a801ccd040f085",
        "verdict": "independently-justified-equivalent"
      },
      "rationale": "Checked witness and batch bounds make every operand false; raw holders remain unrestricted.",
      "requirement": "CHECK-7/8/14/25",
      "source_sha256": "4e0687bd5b3dd1e1ada09af7e17a561f5c1ee322761aae76f8c0c2964ff1c553"
    },
    {
      "candidate_sha256": "5f0a23c12bf5adeae4bacd47592c0b7ec24714b5e862d5d76115483f6f15f0a7",
      "id": "S158-R-E-003",
      "mutant": {
        "file": "crates/time-core/src/combination/chart/receiving.rs",
        "function": {
          "function_name": "admit_chart_records",
          "return_type": "-> Result<(), FixtureCombinationError>",
          "span": {
            "end": {
              "column": 2,
              "line": 279
            },
            "start": {
              "column": 1,
              "line": 199
            }
          }
        },
        "genre": "BinaryOperator",
        "name": "crates/time-core/src/combination/chart/receiving.rs:216:45: replace || with && in admit_chart_records",
        "package": "time-core",
        "replacement": "&&",
        "span": {
          "end": {
            "column": 47,
            "line": 216
          },
          "start": {
            "column": 45,
            "line": 216
          }
        }
      },
      "proof": {
        "path": "tasks/s1.3-chart-combination-equivalence.md",
        "reviewer": "root; s157_gate_review",
        "section": "S158-R-E-003",
        "sha256": "055046256bf3fa5aca61fb36042f819d2d53407512e0da1c89a801ccd040f085",
        "verdict": "independently-justified-equivalent"
      },
      "rationale": "The changed precedence has the same false operands; all raw holder/principal errors retain precedence.",
      "requirement": "CHECK-7/8/14/25",
      "source_sha256": "4e0687bd5b3dd1e1ada09af7e17a561f5c1ee322761aae76f8c0c2964ff1c553"
    },
    {
      "candidate_sha256": "5f0a23c12bf5adeae4bacd47592c0b7ec24714b5e862d5d76115483f6f15f0a7",
      "id": "S158-R-E-004",
      "mutant": {
        "file": "crates/time-core/src/combination/chart/receiving.rs",
        "function": {
          "function_name": "admit_chart_records",
          "return_type": "-> Result<(), FixtureCombinationError>",
          "span": {
            "end": {
              "column": 2,
              "line": 279
            },
            "start": {
              "column": 1,
              "line": 199
            }
          }
        },
        "genre": "BinaryOperator",
        "name": "crates/time-core/src/combination/chart/receiving.rs:228:9: replace || with && in admit_chart_records",
        "package": "time-core",
        "replacement": "&&",
        "span": {
          "end": {
            "column": 11,
            "line": 228
          },
          "start": {
            "column": 9,
            "line": 228
          }
        }
      },
      "proof": {
        "path": "tasks/s1.3-chart-combination-equivalence.md",
        "reviewer": "root; s157_gate_review",
        "section": "S158-R-E-004",
        "sha256": "055046256bf3fa5aca61fb36042f819d2d53407512e0da1c89a801ccd040f085",
        "verdict": "independently-justified-equivalent"
      },
      "rationale": "A mismatched principal address rejects in both forms; matching address forces matching witnessed original label.",
      "requirement": "CHECK-7/8/14/25",
      "source_sha256": "4e0687bd5b3dd1e1ada09af7e17a561f5c1ee322761aae76f8c0c2964ff1c553"
    },
    {
      "candidate_sha256": "5f0a23c12bf5adeae4bacd47592c0b7ec24714b5e862d5d76115483f6f15f0a7",
      "id": "S158-R-E-005",
      "mutant": {
        "file": "crates/time-core/src/combination/chart/receiving.rs",
        "function": {
          "function_name": "admit_chart_source",
          "return_type": "-> Result<Option<AdmittedChartSupporters<'input, 'entries, 'chart>>, FixtureTrackingCombinationError, >",
          "span": {
            "end": {
              "column": 2,
              "line": 367
            },
            "start": {
              "column": 1,
              "line": 281
            }
          }
        },
        "genre": "BinaryOperator",
        "name": "crates/time-core/src/combination/chart/receiving.rs:310:25: replace || with && in admit_chart_source",
        "package": "time-core",
        "replacement": "&&",
        "span": {
          "end": {
            "column": 27,
            "line": 310
          },
          "start": {
            "column": 25,
            "line": 310
          }
        }
      },
      "proof": {
        "path": "tasks/s1.3-chart-combination-equivalence.md",
        "reviewer": "root; s157_gate_review",
        "section": "S158-R-E-005",
        "sha256": "055046256bf3fa5aca61fb36042f819d2d53407512e0da1c89a801ccd040f085",
        "verdict": "independently-justified-equivalent"
      },
      "rationale": "Absent actual principal implies both selected-kind and optional label are absent under checked decision construction.",
      "requirement": "CHECK-7/8/14/25",
      "source_sha256": "4e0687bd5b3dd1e1ada09af7e17a561f5c1ee322761aae76f8c0c2964ff1c553"
    },
    {
      "candidate_sha256": "5f0a23c12bf5adeae4bacd47592c0b7ec24714b5e862d5d76115483f6f15f0a7",
      "id": "S158-E-001",
      "mutant": {
        "file": "crates/time-core/src/combination/chart/reduction.rs",
        "function": {
          "function_name": "ChartSums::finish",
          "return_type": "-> Result<ChartQuantities, ChartReductionError>",
          "span": {
            "end": {
              "column": 6,
              "line": 234
            },
            "start": {
              "column": 5,
              "line": 207
            }
          }
        },
        "genre": "BinaryOperator",
        "name": "crates/time-core/src/combination/chart/reduction.rs:225:13: replace || with && in ChartSums::finish",
        "package": "time-core",
        "replacement": "&&",
        "span": {
          "end": {
            "column": 15,
            "line": 225
          },
          "start": {
            "column": 13,
            "line": 225
          }
        }
      },
      "proof": {
        "path": "tasks/s1.3-chart-combination-equivalence.md",
        "reviewer": "root; s157_gate_review",
        "section": "S158-E-001",
        "sha256": "055046256bf3fa5aca61fb36042f819d2d53407512e0da1c89a801ccd040f085",
        "verdict": "independently-justified-equivalent"
      },
      "rationale": "Every constructible immutable ChartSums preserves exact point bits between its four companion streams and Sums through identical checked primitives; both finish mismatch predicates are false. Complete-domain induction and failure ordering are in D01.",
      "requirement": "CHECK-7/8/14/25",
      "source_sha256": "3a9e98e64224ca0ad5f4c016a5c370dd305e52aef6dc8d38b4f2cc2db7eb74eb"
    },
    {
      "candidate_sha256": "5f0a23c12bf5adeae4bacd47592c0b7ec24714b5e862d5d76115483f6f15f0a7",
      "id": "S158-E-002",
      "mutant": {
        "file": "crates/time-core/src/combination.rs",
        "function": {
          "function_name": "TrackingSupporters<'entry>::from_decision",
          "return_type": "-> Result<Self, FixtureTrackingCombinationError>",
          "span": {
            "end": {
              "column": 6,
              "line": 537
            },
            "start": {
              "column": 5,
              "line": 482
            }
          }
        },
        "genre": "BinaryOperator",
        "name": "crates/time-core/src/combination.rs:503:23: replace || with && in TrackingSupporters<'entry>::from_decision",
        "package": "time-core",
        "replacement": "&&",
        "span": {
          "end": {
            "column": 25,
            "line": 503
          },
          "start": {
            "column": 23,
            "line": 503
          }
        }
      },
      "proof": {
        "path": "tasks/s1.3-chart-combination-equivalence.md",
        "reviewer": "root; s157_gate_review",
        "section": "S158-E-002",
        "sha256": "055046256bf3fa5aca61fb36042f819d2d53407512e0da1c89a801ccd040f085",
        "verdict": "independently-justified-equivalent"
      },
      "rationale": "Genuine immutable decision construction bounds witness population to1..16 and batch entries to16; the exact count predicates are false independently of arbitrary raw record holders, slots and supplied principal associations. D02-D05 preserve all later error precedence.",
      "requirement": "CHECK-7/8/14/25",
      "source_sha256": "d33493baf6505782e6c9061e04a6204d1dc65b1a05f0acf0120dc2831f698326"
    },
    {
      "candidate_sha256": "5f0a23c12bf5adeae4bacd47592c0b7ec24714b5e862d5d76115483f6f15f0a7",
      "id": "S158-E-003",
      "mutant": {
        "file": "crates/time-core/src/combination.rs",
        "function": {
          "function_name": "TrackingSupporters<'entry>::from_decision",
          "return_type": "-> Result<Self, FixtureTrackingCombinationError>",
          "span": {
            "end": {
              "column": 6,
              "line": 537
            },
            "start": {
              "column": 5,
              "line": 482
            }
          }
        },
        "genre": "BinaryOperator",
        "name": "crates/time-core/src/combination.rs:503:49: replace || with && in TrackingSupporters<'entry>::from_decision",
        "package": "time-core",
        "replacement": "&&",
        "span": {
          "end": {
            "column": 51,
            "line": 503
          },
          "start": {
            "column": 49,
            "line": 503
          }
        }
      },
      "proof": {
        "path": "tasks/s1.3-chart-combination-equivalence.md",
        "reviewer": "root; s157_gate_review",
        "section": "S158-E-003",
        "sha256": "055046256bf3fa5aca61fb36042f819d2d53407512e0da1c89a801ccd040f085",
        "verdict": "independently-justified-equivalent"
      },
      "rationale": "Genuine immutable decision construction bounds witness population to1..16 and batch entries to16; the exact count predicates are false independently of arbitrary raw record holders, slots and supplied principal associations. D02-D05 preserve all later error precedence.",
      "requirement": "CHECK-7/8/14/25",
      "source_sha256": "d33493baf6505782e6c9061e04a6204d1dc65b1a05f0acf0120dc2831f698326"
    },
    {
      "candidate_sha256": "5f0a23c12bf5adeae4bacd47592c0b7ec24714b5e862d5d76115483f6f15f0a7",
      "id": "S158-E-004",
      "mutant": {
        "file": "crates/time-core/src/combination.rs",
        "function": {
          "function_name": "admit_record_source",
          "return_type": "-> Result<(), FixtureCombinationError>",
          "span": {
            "end": {
              "column": 2,
              "line": 896
            },
            "start": {
              "column": 1,
              "line": 842
            }
          }
        },
        "genre": "BinaryOperator",
        "name": "crates/time-core/src/combination.rs:859:19: replace || with && in admit_record_source",
        "package": "time-core",
        "replacement": "&&",
        "span": {
          "end": {
            "column": 21,
            "line": 859
          },
          "start": {
            "column": 19,
            "line": 859
          }
        }
      },
      "proof": {
        "path": "tasks/s1.3-chart-combination-equivalence.md",
        "reviewer": "root; s157_gate_review",
        "section": "S158-E-004",
        "sha256": "055046256bf3fa5aca61fb36042f819d2d53407512e0da1c89a801ccd040f085",
        "verdict": "independently-justified-equivalent"
      },
      "rationale": "Genuine immutable decision construction bounds witness population to1..16 and batch entries to16; the exact count predicates are false independently of arbitrary raw record holders, slots and supplied principal associations. D02-D05 preserve all later error precedence.",
      "requirement": "CHECK-7/8/14/25",
      "source_sha256": "d33493baf6505782e6c9061e04a6204d1dc65b1a05f0acf0120dc2831f698326"
    },
    {
      "candidate_sha256": "5f0a23c12bf5adeae4bacd47592c0b7ec24714b5e862d5d76115483f6f15f0a7",
      "id": "S158-E-005",
      "mutant": {
        "file": "crates/time-core/src/combination.rs",
        "function": {
          "function_name": "admit_record_source",
          "return_type": "-> Result<(), FixtureCombinationError>",
          "span": {
            "end": {
              "column": 2,
              "line": 896
            },
            "start": {
              "column": 1,
              "line": 842
            }
          }
        },
        "genre": "BinaryOperator",
        "name": "crates/time-core/src/combination.rs:859:45: replace || with && in admit_record_source",
        "package": "time-core",
        "replacement": "&&",
        "span": {
          "end": {
            "column": 47,
            "line": 859
          },
          "start": {
            "column": 45,
            "line": 859
          }
        }
      },
      "proof": {
        "path": "tasks/s1.3-chart-combination-equivalence.md",
        "reviewer": "root; s157_gate_review",
        "section": "S158-E-005",
        "sha256": "055046256bf3fa5aca61fb36042f819d2d53407512e0da1c89a801ccd040f085",
        "verdict": "independently-justified-equivalent"
      },
      "rationale": "Genuine immutable decision construction bounds witness population to1..16 and batch entries to16; the exact count predicates are false independently of arbitrary raw record holders, slots and supplied principal associations. D02-D05 preserve all later error precedence.",
      "requirement": "CHECK-7/8/14/25",
      "source_sha256": "d33493baf6505782e6c9061e04a6204d1dc65b1a05f0acf0120dc2831f698326"
    },
    {
      "candidate_sha256": "5f0a23c12bf5adeae4bacd47592c0b7ec24714b5e862d5d76115483f6f15f0a7",
      "id": "S158-E-006",
      "mutant": {
        "file": "crates/time-core/src/combination.rs",
        "function": {
          "function_name": "combine_records",
          "return_type": "-> Result<FixtureCombinationKernelOutcome, FixtureCombinationError>",
          "span": {
            "end": {
              "column": 2,
              "line": 1339
            },
            "start": {
              "column": 1,
              "line": 1275
            }
          }
        },
        "genre": "BinaryOperator",
        "name": "crates/time-core/src/combination.rs:1299:26: replace || with && in combine_records",
        "package": "time-core",
        "replacement": "&&",
        "span": {
          "end": {
            "column": 28,
            "line": 1299
          },
          "start": {
            "column": 26,
            "line": 1299
          }
        }
      },
      "proof": {
        "path": "tasks/s1.3-chart-combination-equivalence.md",
        "reviewer": "root; s157_gate_review",
        "section": "S158-E-006",
        "sha256": "055046256bf3fa5aca61fb36042f819d2d53407512e0da1c89a801ccd040f085",
        "verdict": "independently-justified-equivalent"
      },
      "rationale": "An absent actual principal entails absent label and a nonselected kind under checked immutable decision construction. Arbitrary metadata retains capacity and mismatch errors before this unchanged no-estimate result; see D06/D07.",
      "requirement": "CHECK-7/8/14/25",
      "source_sha256": "d33493baf6505782e6c9061e04a6204d1dc65b1a05f0acf0120dc2831f698326"
    },
    {
      "candidate_sha256": "5f0a23c12bf5adeae4bacd47592c0b7ec24714b5e862d5d76115483f6f15f0a7",
      "id": "S158-E-007",
      "mutant": {
        "file": "crates/time-core/src/combination/admission.rs",
        "function": {
          "function_name": "admit_lifecycle_source",
          "return_type": "-> Result<Option<AdmittedFixtureSupporters<'input, 'entry>>, FixtureCombinationError>",
          "span": {
            "end": {
              "column": 2,
              "line": 286
            },
            "start": {
              "column": 1,
              "line": 214
            }
          }
        },
        "genre": "BinaryOperator",
        "name": "crates/time-core/src/combination/admission.rs:237:21: replace || with && in admit_lifecycle_source",
        "package": "time-core",
        "replacement": "&&",
        "span": {
          "end": {
            "column": 23,
            "line": 237
          },
          "start": {
            "column": 21,
            "line": 237
          }
        }
      },
      "proof": {
        "path": "tasks/s1.3-chart-combination-equivalence.md",
        "reviewer": "root; s157_gate_review",
        "section": "S158-E-007",
        "sha256": "055046256bf3fa5aca61fb36042f819d2d53407512e0da1c89a801ccd040f085",
        "verdict": "independently-justified-equivalent"
      },
      "rationale": "An absent actual principal entails absent label and a nonselected kind under checked immutable decision construction. Arbitrary metadata retains capacity and mismatch errors before this unchanged no-estimate result; see D06/D07.",
      "requirement": "CHECK-7/8/14/25",
      "source_sha256": "7519a3ffabcbe05e7b4adc24baff0bcbbdcc1be31768454b49708ab190350bc0"
    },
    {
      "candidate_sha256": "5f0a23c12bf5adeae4bacd47592c0b7ec24714b5e862d5d76115483f6f15f0a7",
      "id": "S158-E-008",
      "mutant": {
        "file": "crates/time-core/src/combination/admission.rs",
        "function": {
          "function_name": "contributor_count",
          "return_type": "-> Result<usize, FixtureCombinationError>",
          "span": {
            "end": {
              "column": 2,
              "line": 332
            },
            "start": {
              "column": 1,
              "line": 294
            }
          }
        },
        "genre": "BinaryOperator",
        "name": "crates/time-core/src/combination/admission.rs:301:30: replace || with && in contributor_count",
        "package": "time-core",
        "replacement": "&&",
        "span": {
          "end": {
            "column": 32,
            "line": 301
          },
          "start": {
            "column": 30,
            "line": 301
          }
        }
      },
      "proof": {
        "path": "tasks/s1.3-chart-combination-equivalence.md",
        "reviewer": "root; s157_gate_review",
        "section": "S158-E-008",
        "sha256": "055046256bf3fa5aca61fb36042f819d2d53407512e0da1c89a801ccd040f085",
        "verdict": "independently-justified-equivalent"
      },
      "rationale": "The sole checked admitted-token construction establishes immutable population1..16. Both changed count predicates are false for every raw16-bit mask; padding, membership, empty and principal checks remain unchanged. See D08/D09.",
      "requirement": "CHECK-7/8/14/25",
      "source_sha256": "7519a3ffabcbe05e7b4adc24baff0bcbbdcc1be31768454b49708ab190350bc0"
    },
    {
      "candidate_sha256": "5f0a23c12bf5adeae4bacd47592c0b7ec24714b5e862d5d76115483f6f15f0a7",
      "id": "S158-E-009",
      "mutant": {
        "file": "crates/time-core/src/combination/lifecycle/chart.rs",
        "function": {
          "function_name": "contributor_count",
          "return_type": "-> Result<usize, FixtureCombinationError>",
          "span": {
            "end": {
              "column": 2,
              "line": 434
            },
            "start": {
              "column": 1,
              "line": 402
            }
          }
        },
        "genre": "BinaryOperator",
        "name": "crates/time-core/src/combination/lifecycle/chart.rs:410:9: replace || with && in contributor_count",
        "package": "time-core",
        "replacement": "&&",
        "span": {
          "end": {
            "column": 11,
            "line": 410
          },
          "start": {
            "column": 9,
            "line": 410
          }
        }
      },
      "proof": {
        "path": "tasks/s1.3-chart-combination-equivalence.md",
        "reviewer": "root; s157_gate_review",
        "section": "S158-E-009",
        "sha256": "055046256bf3fa5aca61fb36042f819d2d53407512e0da1c89a801ccd040f085",
        "verdict": "independently-justified-equivalent"
      },
      "rationale": "The sole checked admitted-token construction establishes immutable population1..16. Both changed count predicates are false for every raw16-bit mask; padding, membership, empty and principal checks remain unchanged. See D08/D09.",
      "requirement": "CHECK-7/8/14/25",
      "source_sha256": "b2195651f756deed769f2b8183ae57c5fd622e38fd558060c49a44e2084b2ec7"
    },
    {
      "candidate_sha256": "5f0a23c12bf5adeae4bacd47592c0b7ec24714b5e862d5d76115483f6f15f0a7",
      "id": "S158-E-010",
      "mutant": {
        "file": "crates/time-core/src/combination/chart/roundoff.rs",
        "function": {
          "function_name": "lower_margin",
          "return_type": "-> Result<f64, CompanionError>",
          "span": {
            "end": {
              "column": 2,
              "line": 341
            },
            "start": {
              "column": 1,
              "line": 314
            }
          }
        },
        "genre": "BinaryOperator",
        "name": "crates/time-core/src/combination/chart/roundoff.rs:325:27: replace || with && in lower_margin",
        "package": "time-core",
        "replacement": "&&",
        "span": {
          "end": {
            "column": 29,
            "line": 325
          },
          "start": {
            "column": 27,
            "line": 325
          }
        }
      },
      "proof": {
        "path": "tasks/s1.3-chart-combination-equivalence.md",
        "reviewer": "root; s157_gate_review",
        "section": "S158-E-010",
        "sha256": "055046256bf3fa5aca61fb36042f819d2d53407512e0da1c89a801ccd040f085",
        "verdict": "independently-justified-equivalent"
      },
      "rationale": "Over all raw binary64 pairs, the exact changed initial guard only admits additional positive infinity/NaN values with valid errors; unchanged subtraction and later nonfinite rejection return the identical error. Default Rust floating-point assumptions and exception-status limits are explicit in D10.",
      "requirement": "CHECK-7/8/14/25",
      "source_sha256": "1001dffe6cac911207879962f39b3aeff5a0668916cf53b5e42986523ff1c5eb"
    },
    {
      "candidate_sha256": "5f0a23c12bf5adeae4bacd47592c0b7ec24714b5e862d5d76115483f6f15f0a7",
      "id": "S158-E-011",
      "mutant": {
        "file": "crates/time-core/src/combination/chart/roundoff.rs",
        "function": {
          "function_name": "lower_margin",
          "return_type": "-> Result<f64, CompanionError>",
          "span": {
            "end": {
              "column": 2,
              "line": 341
            },
            "start": {
              "column": 1,
              "line": 314
            }
          }
        },
        "genre": "BinaryOperator",
        "name": "crates/time-core/src/combination/chart/roundoff.rs:329:29: replace || with && in lower_margin",
        "package": "time-core",
        "replacement": "&&",
        "span": {
          "end": {
            "column": 31,
            "line": 329
          },
          "start": {
            "column": 29,
            "line": 329
          }
        }
      },
      "proof": {
        "path": "tasks/s1.3-chart-combination-equivalence.md",
        "reviewer": "root; s157_gate_review",
        "section": "S158-E-011",
        "sha256": "055046256bf3fa5aca61fb36042f819d2d53407512e0da1c89a801ccd040f085",
        "verdict": "independently-justified-equivalent"
      },
      "rationale": "The unchanged initial guard establishes finite positive value and finite0<=error<value. Its binary64 difference is finite positive, at least the smallest subnormal. Both changed rejection predicates are false; see complete raw-domain/default-environment argument D11.",
      "requirement": "CHECK-7/8/14/25",
      "source_sha256": "1001dffe6cac911207879962f39b3aeff5a0668916cf53b5e42986523ff1c5eb"
    },
    {
      "candidate_sha256": "5f0a23c12bf5adeae4bacd47592c0b7ec24714b5e862d5d76115483f6f15f0a7",
      "id": "S158-E-012",
      "mutant": {
        "file": "crates/time-core/src/combination/chart/roundoff.rs",
        "function": {
          "function_name": "lower_margin",
          "return_type": "-> Result<f64, CompanionError>",
          "span": {
            "end": {
              "column": 2,
              "line": 341
            },
            "start": {
              "column": 1,
              "line": 314
            }
          }
        },
        "genre": "BinaryOperator",
        "name": "crates/time-core/src/combination/chart/roundoff.rs:336:26: replace && with || in lower_margin",
        "package": "time-core",
        "replacement": "||",
        "span": {
          "end": {
            "column": 28,
            "line": 336
          },
          "start": {
            "column": 26,
            "line": 336
          }
        }
      },
      "proof": {
        "path": "tasks/s1.3-chart-combination-equivalence.md",
        "reviewer": "root; s157_gate_review",
        "section": "S158-E-012",
        "sha256": "055046256bf3fa5aca61fb36042f819d2d53407512e0da1c89a801ccd040f085",
        "verdict": "independently-justified-equivalent"
      },
      "rationale": "The unchanged subnormal branch leaves only positive normal rounded values. Their next_down predecessor is finite and strictly positive, including the smallest-normal endpoint. The exact final predicate change therefore preserves every return bit/error; see D12/D13.",
      "requirement": "CHECK-7/8/14/25",
      "source_sha256": "1001dffe6cac911207879962f39b3aeff5a0668916cf53b5e42986523ff1c5eb"
    },
    {
      "candidate_sha256": "5f0a23c12bf5adeae4bacd47592c0b7ec24714b5e862d5d76115483f6f15f0a7",
      "id": "S158-E-013",
      "mutant": {
        "file": "crates/time-core/src/combination/chart/roundoff.rs",
        "function": {
          "function_name": "lower_margin",
          "return_type": "-> Result<f64, CompanionError>",
          "span": {
            "end": {
              "column": 2,
              "line": 341
            },
            "start": {
              "column": 1,
              "line": 314
            }
          }
        },
        "genre": "BinaryOperator",
        "name": "crates/time-core/src/combination/chart/roundoff.rs:336:35: replace > with >= in lower_margin",
        "package": "time-core",
        "replacement": ">=",
        "span": {
          "end": {
            "column": 36,
            "line": 336
          },
          "start": {
            "column": 35,
            "line": 336
          }
        }
      },
      "proof": {
        "path": "tasks/s1.3-chart-combination-equivalence.md",
        "reviewer": "root; s157_gate_review",
        "section": "S158-E-013",
        "sha256": "055046256bf3fa5aca61fb36042f819d2d53407512e0da1c89a801ccd040f085",
        "verdict": "independently-justified-equivalent"
      },
      "rationale": "The unchanged subnormal branch leaves only positive normal rounded values. Their next_down predecessor is finite and strictly positive, including the smallest-normal endpoint. The exact final predicate change therefore preserves every return bit/error; see D12/D13.",
      "requirement": "CHECK-7/8/14/25",
      "source_sha256": "1001dffe6cac911207879962f39b3aeff5a0668916cf53b5e42986523ff1c5eb"
    },
    {
      "candidate_sha256": "5f0a23c12bf5adeae4bacd47592c0b7ec24714b5e862d5d76115483f6f15f0a7",
      "id": "S158-E-014",
      "mutant": {
        "file": "crates/time-core/src/combination/lifecycle/chart.rs",
        "function": {
          "function_name": "estimate_valid",
          "return_type": "-> bool",
          "span": {
            "end": {
              "column": 2,
              "line": 570
            },
            "start": {
              "column": 1,
              "line": 532
            }
          }
        },
        "genre": "BinaryOperator",
        "name": "crates/time-core/src/combination/lifecycle/chart.rs:549:9: replace || with && in estimate_valid",
        "package": "time-core",
        "replacement": "&&",
        "span": {
          "end": {
            "column": 11,
            "line": 549
          },
          "start": {
            "column": 9,
            "line": 549
          }
        }
      },
      "proof": {
        "path": "tasks/s1.3-chart-combination-equivalence.md",
        "reviewer": "root; s157_gate_review",
        "section": "S158-E-014",
        "sha256": "055046256bf3fa5aca61fb36042f819d2d53407512e0da1c89a801ccd040f085",
        "verdict": "independently-justified-equivalent"
      },
      "rationale": "Earlier unchanged OR atoms reject arbitrary batch/principal pointer mismatches. Otherwise actual checked batch/entry construction makes all descriptor/source tags/target tags/query mismatch atoms false. Only the exact adjacent middle pair changes; arbitrary raw counts/labels/scalars still reach unchanged guards. See D14-D18.",
      "requirement": "CHECK-7/8/14/25",
      "source_sha256": "b2195651f756deed769f2b8183ae57c5fd622e38fd558060c49a44e2084b2ec7"
    },
    {
      "candidate_sha256": "5f0a23c12bf5adeae4bacd47592c0b7ec24714b5e862d5d76115483f6f15f0a7",
      "id": "S158-E-015",
      "mutant": {
        "file": "crates/time-core/src/combination/lifecycle/chart.rs",
        "function": {
          "function_name": "estimate_valid",
          "return_type": "-> bool",
          "span": {
            "end": {
              "column": 2,
              "line": 570
            },
            "start": {
              "column": 1,
              "line": 532
            }
          }
        },
        "genre": "BinaryOperator",
        "name": "crates/time-core/src/combination/lifecycle/chart.rs:550:9: replace || with && in estimate_valid",
        "package": "time-core",
        "replacement": "&&",
        "span": {
          "end": {
            "column": 11,
            "line": 550
          },
          "start": {
            "column": 9,
            "line": 550
          }
        }
      },
      "proof": {
        "path": "tasks/s1.3-chart-combination-equivalence.md",
        "reviewer": "root; s157_gate_review",
        "section": "S158-E-015",
        "sha256": "055046256bf3fa5aca61fb36042f819d2d53407512e0da1c89a801ccd040f085",
        "verdict": "independently-justified-equivalent"
      },
      "rationale": "Earlier unchanged OR atoms reject arbitrary batch/principal pointer mismatches. Otherwise actual checked batch/entry construction makes all descriptor/source tags/target tags/query mismatch atoms false. Only the exact adjacent middle pair changes; arbitrary raw counts/labels/scalars still reach unchanged guards. See D14-D18.",
      "requirement": "CHECK-7/8/14/25",
      "source_sha256": "b2195651f756deed769f2b8183ae57c5fd622e38fd558060c49a44e2084b2ec7"
    },
    {
      "candidate_sha256": "5f0a23c12bf5adeae4bacd47592c0b7ec24714b5e862d5d76115483f6f15f0a7",
      "id": "S158-E-016",
      "mutant": {
        "file": "crates/time-core/src/combination/lifecycle/chart.rs",
        "function": {
          "function_name": "estimate_valid",
          "return_type": "-> bool",
          "span": {
            "end": {
              "column": 2,
              "line": 570
            },
            "start": {
              "column": 1,
              "line": 532
            }
          }
        },
        "genre": "BinaryOperator",
        "name": "crates/time-core/src/combination/lifecycle/chart.rs:551:9: replace || with && in estimate_valid",
        "package": "time-core",
        "replacement": "&&",
        "span": {
          "end": {
            "column": 11,
            "line": 551
          },
          "start": {
            "column": 9,
            "line": 551
          }
        }
      },
      "proof": {
        "path": "tasks/s1.3-chart-combination-equivalence.md",
        "reviewer": "root; s157_gate_review",
        "section": "S158-E-016",
        "sha256": "055046256bf3fa5aca61fb36042f819d2d53407512e0da1c89a801ccd040f085",
        "verdict": "independently-justified-equivalent"
      },
      "rationale": "Earlier unchanged OR atoms reject arbitrary batch/principal pointer mismatches. Otherwise actual checked batch/entry construction makes all descriptor/source tags/target tags/query mismatch atoms false. Only the exact adjacent middle pair changes; arbitrary raw counts/labels/scalars still reach unchanged guards. See D14-D18.",
      "requirement": "CHECK-7/8/14/25",
      "source_sha256": "b2195651f756deed769f2b8183ae57c5fd622e38fd558060c49a44e2084b2ec7"
    },
    {
      "candidate_sha256": "5f0a23c12bf5adeae4bacd47592c0b7ec24714b5e862d5d76115483f6f15f0a7",
      "id": "S158-E-017",
      "mutant": {
        "file": "crates/time-core/src/combination/lifecycle/chart.rs",
        "function": {
          "function_name": "estimate_valid",
          "return_type": "-> bool",
          "span": {
            "end": {
              "column": 2,
              "line": 570
            },
            "start": {
              "column": 1,
              "line": 532
            }
          }
        },
        "genre": "BinaryOperator",
        "name": "crates/time-core/src/combination/lifecycle/chart.rs:552:9: replace || with && in estimate_valid",
        "package": "time-core",
        "replacement": "&&",
        "span": {
          "end": {
            "column": 11,
            "line": 552
          },
          "start": {
            "column": 9,
            "line": 552
          }
        }
      },
      "proof": {
        "path": "tasks/s1.3-chart-combination-equivalence.md",
        "reviewer": "root; s157_gate_review",
        "section": "S158-E-017",
        "sha256": "055046256bf3fa5aca61fb36042f819d2d53407512e0da1c89a801ccd040f085",
        "verdict": "independently-justified-equivalent"
      },
      "rationale": "Earlier unchanged OR atoms reject arbitrary batch/principal pointer mismatches. Otherwise actual checked batch/entry construction makes all descriptor/source tags/target tags/query mismatch atoms false. Only the exact adjacent middle pair changes; arbitrary raw counts/labels/scalars still reach unchanged guards. See D14-D18.",
      "requirement": "CHECK-7/8/14/25",
      "source_sha256": "b2195651f756deed769f2b8183ae57c5fd622e38fd558060c49a44e2084b2ec7"
    },
    {
      "candidate_sha256": "5f0a23c12bf5adeae4bacd47592c0b7ec24714b5e862d5d76115483f6f15f0a7",
      "id": "S158-E-018",
      "mutant": {
        "file": "crates/time-core/src/combination/lifecycle/chart.rs",
        "function": {
          "function_name": "estimate_valid",
          "return_type": "-> bool",
          "span": {
            "end": {
              "column": 2,
              "line": 570
            },
            "start": {
              "column": 1,
              "line": 532
            }
          }
        },
        "genre": "BinaryOperator",
        "name": "crates/time-core/src/combination/lifecycle/chart.rs:553:9: replace || with && in estimate_valid",
        "package": "time-core",
        "replacement": "&&",
        "span": {
          "end": {
            "column": 11,
            "line": 553
          },
          "start": {
            "column": 9,
            "line": 553
          }
        }
      },
      "proof": {
        "path": "tasks/s1.3-chart-combination-equivalence.md",
        "reviewer": "root; s157_gate_review",
        "section": "S158-E-018",
        "sha256": "055046256bf3fa5aca61fb36042f819d2d53407512e0da1c89a801ccd040f085",
        "verdict": "independently-justified-equivalent"
      },
      "rationale": "Earlier unchanged OR atoms reject arbitrary batch/principal pointer mismatches. Otherwise actual checked batch/entry construction makes all descriptor/source tags/target tags/query mismatch atoms false. Only the exact adjacent middle pair changes; arbitrary raw counts/labels/scalars still reach unchanged guards. See D14-D18.",
      "requirement": "CHECK-7/8/14/25",
      "source_sha256": "b2195651f756deed769f2b8183ae57c5fd622e38fd558060c49a44e2084b2ec7"
    }
  ]
}''')


def synthetic_compiler_span(source, line, **changes):
    """Author a complete pinned rustc span for inert diagnostic controls."""
    return {'file_name': source, 'byte_start': 0, 'byte_end': 1,
            'line_start': line, 'line_end': line, 'column_start': 1,
            'column_end': 2, 'is_primary': True, 'text': [], 'label': None,
            'suggested_replacement': None, 'suggestion_applicability': None,
            'expansion': None, **changes}


def synthetic_compiler_diagnostic(code, message, source, line):
    """Construct synthetic schema-complete errors, never native evidence."""
    return {'$message_type': 'diagnostic', 'message': message,
            'code': {'code': code, 'explanation': None}, 'level': 'error',
            'spans': [synthetic_compiler_span(source, line)],
            'children': [], 'rendered': None}


def fund_synthetic_source_recipes(test):
    """Fund isolated parser fixtures explicitly while real discovery is pending.

    Only cumulative synthetic maps change; every per-invocation profile gate
    stays intact. These allowances admit no actual source, process or campaign.
    Individual threshold controls override their targeted map inside each test.
    """
    chart_combination_package = MODULE.chart_combination_budget_vector(include_doctests=True)
    chart_combination_external = MODULE.chart_combination_budget_vector(external_legal=True)
    composition_fixture = MODULE.chart_composition_fixture_budget()
    composition_external_fixture = MODULE.chart_composition_fixture_budget(external_legal=True)
    composition_package = MODULE.chart_composition_budget_vector(include_doctests=True)
    composition_external = MODULE.chart_composition_budget_vector(external_legal=True)
    phase_fixture = MODULE.achieved_phase_fixture_budget()
    phase_composition = MODULE.achieved_phase_composition_delta(include_doctests=True)
    rate_fixture = MODULE.rate_delivery_fixture_budget()
    rate_composition = MODULE.rate_delivery_composition_delta(include_doctests=True)
    for mapping, values in (
        (MODULE.FINAL_PACKAGE_TEST_COUNTS, {'ordinary': 258, 'doctests': 23, 'baseline': 281}),
        (MODULE.CHART_COMBINATION_TELEMETRY,
         {'global_limits': {key: (value + composition_fixture[key] + phase_fixture[key] + rate_fixture[key]) * 1000
                            + chart_combination_external[key] + composition_external_fixture[key]
                            for key, value in chart_combination_package.items()}}),
        (MODULE.CHART_COMPOSITION_TELEMETRY,
         {'global_limits': {key: (value + phase_composition[key] + rate_composition[key]) * 1000 + composition_external[key]
                            for key, value in composition_package.items()}}),
        (MODULE.ACHIEVED_PHASE_TELEMETRY,
         {'global_limits': {key: value * 1000 for key, value in
                            MODULE.CHART.achieved_phase_package_reservation(include_doctests=True).items()}}),
        (MODULE.RATE_DELIVERY_TELEMETRY,
         {'global_limits': {key: value * 1000 for key, value in
                            MODULE.RATE_DELIVERY.package_overlay(include_doctests=True).items()}}),
        (MODULE.CHART_PRINCIPAL_TELEMETRY,
         {'global_limits': {key: value * 1000 + MODULE.chart_principal_budget_vector(external_legal=True)[key]
                            for key, value in MODULE.chart_principal_budget_vector(include_doctests=True).items()}}),
        (MODULE.CHART_BATCH_TELEMETRY,
         {'global_limits': {key: value * 1000 + MODULE.chart_batch_budget_vector(external_legal=True)[key]
                            for key, value in MODULE.chart_batch_budget_vector(include_doctests=True).items()}}),
        (MODULE.CHART_TELEMETRY,
         {'global_limits': {key: value * 1000 + MODULE.chart_budget_vector(external_legal=True)[key]
                            for key, value in MODULE.chart_budget_vector(include_doctests=True).items()}}),
        (MODULE.TRACKING_TELEMETRY,
         {'global_limits': {key: value * 1000 + MODULE.tracking_budget_vector(package=False)[key]
                            for key, value in MODULE.tracking_budget_vector().items()}}),
        (MODULE.STANDARD_ERROR_TELEMETRY,
         {'global_limits': {key: value * 1000 + MODULE.standard_error_budget_vector(package=False, charged_fit_upper=313)[key]
                            for key, value in MODULE.standard_error_budget_vector().items()}}),
        (MODULE.PROJECTION_ADAPTER_TELEMETRY,
         {'global_limits': {key: value * 1000 + MODULE.projection_adapter_budget_vector(external=True)[key]
                            for key, value in MODULE.projection_adapter_budget_vector().items()}}),
        (MODULE.SOURCE_RAW_PRIVATE_GLOBAL_LIMITS,
         {key: amount * 1000 for key, amount in MODULE.SOURCE_RAW_PRIVATE_WORK_LIMITS.items()}),
        (MODULE.PRINCIPAL_TELEMETRY,
         {'global_limits': {key: amount * 1000 for key, amount in MODULE.principal_budget_vector().items()}}),
        (MODULE.COMBINATION_TELEMETRY,
         {'global_limits': {key: amount * 1000 for key, amount in MODULE.combination_budget_vector().items()}}),
        (MODULE.LIFECYCLE_TELEMETRY,
         {'global_limits': {key: amount * 1000 for key, amount in MODULE.lifecycle_budget_vector().items()}}),
        (MODULE.ADAPTIVE_TELEMETRY,
         {'global_limits': {key: amount * 1000 + MODULE.adaptive_external_budget_vector()[key]
                            for key, amount in MODULE.adaptive_budget_vector(include_doctests=True).items()}}),
    ):
        fixture = patch.dict(mapping, values)
        fixture.start()
        test.addCleanup(fixture.stop)


class ChartCheckpointTests(HistoricalS14cTestCase):
    """Preserved S154 recipes within the additive S157 source reservation."""

    PASSED = ''.join('test ' + name + ' ... ok\n' for name in MODULE.CHART.CURRENT_ORDINARY_TESTS)
    DOCS = ''.join('test ' + name + ' ... ok\n' for name in MODULE.CHART_ORIGINAL_DOCTESTS)

    def setUp(self):
        fund_synthetic_source_recipes(self)

    def test_current_scope_is_explicit_and_historical_e_cannot_transfer(self):
        self.assertEqual(MODULE.SUBJECTS, S14C_SUBJECTS)
        self.assertEqual((MODULE.SOURCE_FILE_COUNT, len(set(MODULE.BOUND_INPUTS))), (156, 156))
        self.assertEqual(len(MODULE.COMPLETE_CRITICAL_SUBJECTS), 24)
        self.assertEqual(MODULE.FIXED['schema_version'], 25)
        self.assertEqual(MODULE.COORDINATOR_COHORTS['schema_version'], 15)
        self.assertEqual(MODULE.COORDINATOR_COHORTS['checkpoint'], 'S1.4c')
        self.assertEqual(MODULE.INVENTORY_FIXED['schema_version'], 16)
        self.assertEqual(MODULE.INVENTORY_ADMISSION, 'tasks/s1.4c-achieved-phase-inventory-admission.json')
        self.assertIn(MODULE.FINAL_PACKAGE_TEST_COUNTS,
                      ({'ordinary': 0, 'doctests': 0, 'baseline': 0},
                       {'ordinary': 258, 'doctests': 23, 'baseline': 281}))
        self.assertEqual(MODULE.HISTORICAL_S152B_TEST_COUNTS, {'ordinary': 223, 'doctests': 9, 'baseline': 232})
        MODULE.admit_core_equivalence_scope([])
        for inventories in ({}, {'chart': {'count': 1, 'sha256': '1' * 64}}):
            with patch.dict(MODULE.MUTATION_INVENTORIES, inventories, clear=True):
                MODULE.admit_core_equivalence_scope([])
                for bad in (None, {}, S152BEquivalenceControls().records()):
                    with self.subTest(value=type(bad).__name__), self.assertRaises(MODULE.Incomplete):
                        MODULE.admit_core_equivalence_scope(bad)

    def test_full_vectors_extend_exactly_once_and_keep_old_namespaces(self):
        package, checks, _ = MODULE.historical_cohort_budget_vectors(require_limits=False)
        self.assertEqual(len(package), 1508)
        chart = {key: value for key, value in package.items() if key.startswith('chart.')}
        self.assertEqual(len(chart), 296)
        self.assertEqual(len(set(package) - set(chart)), 1212)
        self.assertEqual(package['charged_fit_work_upper'], 35291)
        self.assertEqual(package['point_admissions_upper'], 1292054)
        self.assertEqual(checks['coverage']['point_admissions_upper'], 1292051)
        self.assertEqual(checks['contracts']['charged_fit_work_upper'], 499)
        self.assertEqual(checks['contracts']['point_admissions_upper'], 17256)
        self.assertEqual(package['standard_errors.geometry.report_copies_upper'], 35291)
        self.assertEqual(checks['contracts']['standard_errors.geometry.report_copies_upper'], 499)
        self.assertEqual(chart, MODULE.chart_budget_vector(include_doctests=True))
        self.assertTrue(all(checks['coverage'][key] == 0 for key in chart if key.startswith('chart.doctest.')))
        self.assertEqual(MODULE.chart_numeric_work(), {'charged_fit_work_upper': 5952, 'point_admissions_upper': 205536})
        self.assertEqual(MODULE.chart_numeric_work(external_legal=True), {'charged_fit_work_upper': 186, 'point_admissions_upper': 6423})

    def test_named_outcomes_require_actual_complete_rows_not_json_or_partial_text(self):
        result = MODULE.chart_test_observation(self.PASSED, require_complete=True)
        self.assertFalse(result['chart_source_only_pending'])
        self.assertIsNone(result['chart_observed_operation_counts'])
        self.assertEqual(result['chart_recorded_source_work_upper'], MODULE.chart_numeric_work())
        for bad in (self.PASSED.replace(' ... ok', ' ... FAILED', 1),
                    self.PASSED.replace(' ... ok', ' ... ignored', 1),
                    self.PASSED.split('\n', 1)[1], self.PASSED.rstrip('\n'),
                    self.PASSED + self.PASSED.splitlines(keepends=True)[0],
                    json.dumps({'message': self.PASSED}) + '\n'):
            with self.subTest(text=bad[:80]), self.assertRaises(MODULE.Incomplete):
                MODULE.chart_test_observation(bad, require_complete=True)
        partial = MODULE.chart_test_observation(self.PASSED.splitlines(keepends=True)[0])
        self.assertTrue(partial['chart_source_only_pending'])
        self.assertIsNone(partial['chart_recorded_source_work_upper'])

    def test_baseline_original_docs_are_required_but_mutants_may_move_lines(self):
        self.assertTrue(MODULE.chart_test_observation(self.PASSED + self.DOCS,
                        require_complete=True, require_doctests=True)['chart_original_doctests_complete'])
        for bad in (self.PASSED, self.PASSED + self.DOCS.replace('(line 23)', '(line 24)'),
                    self.PASSED + self.DOCS + self.DOCS.splitlines(keepends=True)[0]):
            with self.subTest(text=bad[-60:]), self.assertRaises(MODULE.Incomplete):
                MODULE.chart_test_observation(bad, require_complete=True, require_doctests=True)
        shifted = MODULE.chart_test_observation(self.PASSED + self.DOCS.replace('(line 23)', '(line 24)'), require_complete=True)
        self.assertFalse(shifted['chart_original_doctests_complete'])

    def test_partial_new_only_prefix_is_retained_without_operation_count_claim(self):
        text = self.PASSED.splitlines(keepends=True)[0]
        result = MODULE.profile_counts(text, require_fit=False)
        self.assertTrue(result['chart_source_only_pending'])
        self.assertIsNone(result['chart_observed_operation_counts'])
        with self.assertRaises(MODULE.Incomplete):
            MODULE.profile_counts(text, require_fit=False, require_chart=True)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.profile_counts(MODULE.CHART_CONTRACTS.MARKER + ' labels=5\n', require_fit=False)

    def test_literal_doctests_keep_the_chart_recipe_outside_unchanged_legacy_caps(self):
        counts = MODULE.profile_counts('CORE_PROFILE_COUNTS fits=1 points=3\n' + self.PASSED)
        self.assertEqual((counts['charged_fit_work_upper'], counts['point_admissions_upper']), (5953, 205539))
        result = MODULE.account_literal_doctests(counts)
        self.assertEqual((result['charged_fit_work_upper'], result['point_admissions_upper']), (5953, 205542))
        self.assertEqual(result['chart_recorded_source_work_upper'], MODULE.chart_numeric_work())
        self.assertIsNone(result['chart_observed_operation_counts'])
        self.assertEqual(result['direct_fit_calls'], 1)

    def test_package_and_legal_reservations_are_exact_and_never_measured_operations(self):
        package = {**MODULE.package_charged_upper(), **MODULE.selection_reservation(package=True), 'observation_pending': True}
        report = {}; MODULE.record_counts(report, 'chart-package', package)
        self.assertEqual(report['core_chart_reserved_totals'], MODULE.chart_budget_vector())
        self.assertEqual(report['core_chart_source_test_pending_invocations'], ['chart-package'])
        self.assertIsNone(report['core_chart_observed_operation_counts'])
        key = next(iter(package['chart_reserved_budget']))
        bad = copy.deepcopy(package); bad['chart_reserved_budget'][key] += 1
        with self.assertRaises(MODULE.Incomplete): MODULE.record_counts({}, 'bad', bad)
        bad = copy.deepcopy(package); del bad['chart_reserved_budget']
        with self.assertRaises(MODULE.Incomplete): MODULE.record_counts({}, 'missing', bad)
        legal = {**MODULE.chart_numeric_work(external_legal=True),
                 'chart_external_reserved_budget': MODULE.chart_budget_vector(external_legal=True),
                 'chart_batch_external_reserved_budget': MODULE.chart_batch_budget_vector(external_legal=True),
                 'chart_principal_external_reserved_budget': MODULE.chart_principal_budget_vector(external_legal=True),
                 'observation_pending': True}
        report = {}; MODULE.record_counts(report, 'chart-legal', legal)
        row = report['core_invocations'][0]
        self.assertEqual(row['standard_error_reserved_budget']['standard_errors.geometry.report_copies_upper'], 186)
        self.assertNotIn('tracking_reserved_budget', row)
        self.assertEqual(report['core_chart_reserved_totals'], legal['chart_external_reserved_budget'])

    def test_chart_allowances_cannot_omit_units_or_reset_accumulated_work(self):
        package = MODULE.chart_budget_vector(include_doctests=True)
        legal = MODULE.chart_budget_vector(external_legal=True)
        limits = {key: max(value, legal[key]) for key, value in package.items()}
        self.assertEqual(MODULE.admit_chart_allowances(limits), limits)
        key = next(key for key, value in package.items() if value > 0)
        for bad in (None, {}, {**limits, key: True}, {**limits, key: limits[key] - 1},
                    {**limits, key: 1 << 63}, {**limits, 'chart.extra': 0}):
            with self.subTest(value=type(bad).__name__), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_chart_allowances(bad)
        with patch.dict(MODULE.CHART_TELEMETRY, {'global_limits': limits}):
            report = {}; counts = {**MODULE.package_charged_upper(), **MODULE.selection_reservation(package=True)}
            MODULE.record_counts(report, 'first', counts)
            with self.assertRaisesRegex(MODULE.Incomplete, 'Chart subset'):
                MODULE.record_counts(report, 'second', counts)

    def test_pending_graph_rejects_before_session_and_prospective_chart_limit_is_real(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); path = root / MODULE.POLICY; path.parent.mkdir(parents=True)
            policy = {**copy.deepcopy(MODULE.FIXED), 'admission_status': 'admitted-final-source',
                      'scope': {}, 'inputs_sha256': {}, 'mutation_equivalences': [],
                      'coverage_guided_fuzzing': {}, 'historical_projection_inputs_sha256': {},
                      'historical_projection_scope': {}}
            policy['graph_status'] = 'pending-S154c-source-inventory-and-cohort-binding'
            path.write_text(json.dumps(policy))
            with patch.object(MODULE, 'ROOT', root), \
                 patch.dict(MODULE.FIXED, {'graph_status': policy['graph_status']}), \
                 patch.object(MODULE.COMMON, 'Session') as session:
                with self.assertRaisesRegex(MODULE.Incomplete, 'Chart source inventory'):
                    MODULE.execute(SimpleNamespace(lane='coverage', subject='all'), {})
                session.assert_not_called()
        coverage = MODULE.prospective_scope_reservation('coverage', dict(MODULE.SUBJECTS))
        self.assertEqual(coverage['chart_source_work_upper'], MODULE.chart_budget_vector())
        contracts = MODULE.prospective_scope_reservation('contracts', dict(MODULE.SUBJECTS))
        self.assertEqual(contracts['charged_upper'], {'charged_fit_work_upper': 685, 'point_admissions_upper': 23679})
        self.assertEqual(contracts['standard_error_source_work_upper']['standard_errors.geometry.report_copies_upper'], 685)
        limits = dict(MODULE.CHART_TELEMETRY['global_limits'])
        key = next(key for key, value in MODULE.chart_budget_vector().items() if value > 0)
        limits[key] = 0
        with patch.dict(MODULE.CHART_TELEMETRY, {'global_limits': limits}), self.assertRaises(MODULE.Incomplete):
            MODULE.prospective_scope_reservation('coverage', dict(MODULE.SUBJECTS))

    def test_registry_joins_legal_neighbors_and_retains_exact_legal_transport(self):
        cases = MODULE.historical_s158_contract_examples()
        legal = [case for case in cases.values() if case['code'] is None]
        self.assertEqual((len(legal), len(cases) - len(legal)), (4, 77))
        self.assertEqual(MODULE.HISTORICAL_S152B_EXTERNAL_CONSUMER_LIMITS,
                         {'legal_executions': 2, 'negative_compilations': 37})
        expected = MODULE.CHART_CONTRACTS.legal_expected()
        raw = MODULE.CHART_CONTRACTS.MARKER + ' ' + ' '.join(f'{key}={value}' for key, value in expected.items()) + '\n' + MODULE.CHART_CONTRACTS.SUCCESS + '\n'
        self.assertEqual(MODULE.CHART.chart_legal_observation(raw, MODULE.CHART_CONTRACTS), expected)
        self.assertEqual(MODULE.CHART.chart_legal_observation(raw.replace('\n', '\r\n'), MODULE.CHART_CONTRACTS), expected)
        for bad in (raw.rstrip('\n'), raw + raw, raw.replace('labels=5', 'labels=4')):
            with self.subTest(raw=bad[:40]), self.assertRaises(MODULE.CHART.Incomplete):
                MODULE.CHART.chart_legal_observation(bad, MODULE.CHART_CONTRACTS)


class ChartBatchCheckpointTests(HistoricalS14cTestCase):
    """Independent S155 receiver controls; every outcome here is synthetic."""

    NAMES = (
        'chart_batch_cases::chart_batch_selects_genuine_majority_at_common_time',
        'chart_batch_cases::chart_batch_source_loss_does_not_lower_minimum',
        'chart_batch_cases::chart_batch_empty_and_capacity_boundaries_use_real_owners',
        'chart_batch_cases::chart_batch_compares_every_chart_frame_field',
        'chart_batch_cases::chart_batch_orders_domain_scalar_time_and_entry_mismatches',
        'chart_batch_cases::chart_batch_preserves_real_duplicate_cause_and_redaction',
        'chart_batch_cases::chart_batch_equal_descriptor_values_preserve_pointers_and_zero_bits',
        'chart_batch::tests::raw_chart_batch_header_checks_inclusive_domain',
    )
    PASSED = ''.join('test ' + name + ' ... ok\n' for name in NAMES)

    def setUp(self):
        fund_synthetic_source_recipes(self)

    def test_selected_owner_and_complete_manifest_keep_distinct_scope(self):
        self.assertEqual(MODULE.SUBJECTS, S14C_SUBJECTS)
        self.assertEqual(len(MODULE.COMPLETE_CRITICAL_SUBJECTS), 24)
        self.assertEqual((MODULE.SOURCE_FILE_COUNT, len(set(MODULE.BOUND_INPUTS))), (156, 156))
        self.assertEqual(MODULE.INVENTORY_FIXED['listing_maximum_total'], 512)
        self.assertEqual(MODULE.INVENTORY_FIXED['mutation_error_values'], {})
        self.assertEqual(MODULE.COORDINATOR_COHORTS['schema_version'], 15)
        self.assertEqual(MODULE.COORDINATOR_COHORTS['contract_attempt_limits'],
                         {'legal_executions': 1, 'negative_compilations': 6})
        MODULE.admit_core_equivalence_scope([])
        for old in (S154CEquivalenceControls().records(), S152BEquivalenceControls().records(),
                    None, {}, (), [None]):
            with self.subTest(old=type(old).__name__), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_core_equivalence_scope(old)

    def test_supplement_extends_old1508_exactly_once_with_frozen_numeric_facts(self):
        old, old_checks, _ = MODULE.historical_cohort_budget_vectors(require_limits=False)
        package, checks, limits = MODULE.historical_s157_cohort_budget_vectors(require_limits=False)
        self.assertEqual(len(old), 1508)
        self.assertEqual((len(package), len(checks['coverage']), len(limits)), (2214, 2214, 2214))
        aggregate = {'charged_fit_work_upper', 'point_admissions_upper',
                     'standard_errors.geometry.report_copies_upper',
                     'standard_errors.geometry.logical_bytes_upper'}
        self.assertEqual({key: package[key] for key in old if key not in aggregate},
                         {key: value for key, value in old.items() if key not in aggregate})
        self.assertEqual(MODULE.chart_batch_numeric_work(),
                         {'charged_fit_work_upper': 5952, 'point_admissions_upper': 205536})
        self.assertEqual(MODULE.chart_batch_numeric_work(external_legal=True),
                         {'charged_fit_work_upper': 0, 'point_admissions_upper': 0})
        self.assertEqual((package['charged_fit_work_upper'], package['point_admissions_upper']),
                         (48807, 1758792))
        self.assertEqual(checks['coverage']['point_admissions_upper'], 1758789)
        self.assertEqual(checks['contracts']['charged_fit_work_upper'], 499)
        self.assertEqual(checks['contracts']['point_admissions_upper'], 17256)
        self.assertEqual(package['standard_errors.geometry.report_copies_upper'], 48807)
        self.assertEqual(package['standard_errors.geometry.logical_bytes_upper'], 1561824)
        self.assertEqual(old_checks['contracts']['charged_fit_work_upper'], 499)

    def test_eight_named_outcomes_are_complete_or_explicitly_pending(self):
        self.assertEqual(tuple(MODULE.CHART.BATCH_ORDINARY_TESTS), self.NAMES)
        result = MODULE.chart_batch_test_observation(self.PASSED, require_complete=True)
        self.assertFalse(result['chart_batch_source_only_pending'])
        self.assertEqual(result['chart_batch_recorded_source_work_upper'],
                         {'charged_fit_work_upper': 5952, 'point_admissions_upper': 205536})
        self.assertIsNone(result['chart_batch_observed_operation_counts'])
        for bad in (self.PASSED.replace(' ... ok', ' ... FAILED', 1),
                    self.PASSED.replace(' ... ok', ' ... ignored', 1),
                    self.PASSED.split('\n', 1)[1], self.PASSED.rstrip('\n'),
                    self.PASSED + self.PASSED.splitlines(keepends=True)[0],
                    json.dumps({'rendered': self.PASSED}) + '\n'):
            with self.subTest(bad=bad[:40]), self.assertRaises(MODULE.Incomplete):
                MODULE.chart_batch_test_observation(bad, require_complete=True)
        partial = MODULE.profile_counts(self.PASSED.splitlines(keepends=True)[0], require_fit=False)
        self.assertTrue(partial['chart_batch_source_only_pending'])
        self.assertIsNone(partial['chart_batch_recorded_source_work_upper'])

    def test_new_named_work_stays_outside_old_counter_caps_and_is_not_doubled(self):
        counts = MODULE.profile_counts('CORE_PROFILE_COUNTS fits=1 points=3\n' + self.PASSED)
        self.assertEqual((counts['charged_fit_work_upper'], counts['point_admissions_upper']),
                         (5953, 205539))
        accounted = MODULE.account_literal_doctests(counts)
        self.assertEqual((accounted['charged_fit_work_upper'], accounted['point_admissions_upper']),
                         (5953, 205542))
        self.assertEqual(accounted['direct_fit_calls'], 1)
        self.assertIsNone(accounted['chart_batch_observed_operation_counts'])

    def test_package_supplement_is_mandatory_and_preserves_pending_observation(self):
        counts = {**MODULE.package_charged_upper(), **MODULE.selection_reservation(package=True),
                  'observation_pending': True}
        report = {}; MODULE.record_counts(report, 'synthetic-batch-package', counts)
        self.assertEqual(report['core_chart_batch_reserved_totals'], MODULE.chart_batch_budget_vector())
        self.assertEqual(report['core_chart_batch_source_test_pending_invocations'],
                         ['synthetic-batch-package'])
        self.assertIsNone(report['core_chart_batch_observed_operation_counts'])
        key = next(iter(counts['chart_batch_reserved_budget']))
        for change in ('missing', 'changed', 'boolean'):
            bad = copy.deepcopy(counts)
            if change == 'missing': del bad['chart_batch_reserved_budget']
            if change == 'changed': bad['chart_batch_reserved_budget'][key] += 1
            if change == 'boolean': bad['chart_batch_reserved_budget'][key] = True
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.record_counts({}, 'bad', bad)

    def test_supplemental_allowance_is_exact_bounded_and_cumulative(self):
        package = MODULE.chart_batch_budget_vector(include_doctests=True)
        legal = MODULE.chart_batch_budget_vector(external_legal=True)
        limits = {key: max(value, legal[key]) for key, value in package.items()}
        self.assertEqual(MODULE.admit_chart_batch_allowances(limits), limits)
        key = next(key for key, value in package.items() if value)
        for bad in (None, {}, {**limits, key: True}, {**limits, key: limits[key] - 1},
                    {**limits, key: 1 << 63}, {**limits, 'chart_batch.extra': 0}):
            with self.subTest(bad=type(bad).__name__), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_chart_batch_allowances(bad)
        with patch.dict(MODULE.CHART_BATCH_TELEMETRY, {'global_limits': limits}):
            counts = {**MODULE.package_charged_upper(), **MODULE.selection_reservation(package=True)}
            report = {}; MODULE.record_counts(report, 'first', counts)
            with self.assertRaisesRegex(MODULE.Incomplete, 'Chart-batch subset'):
                MODULE.record_counts(report, 'second', counts)


class S154CEquivalenceControls(HistoricalS14cTestCase):
    """Independent literal reviewed descriptors; no synthetic outcome grants E."""

    CANDIDATE = 'd3d0a834459550a8ea6a08904b3ec69499ff90f46dd199bed7fb37a5952807f0'
    SOURCE = '6b7b079f7d820e611bd13bb96361c243973d231a7f65a07fb6d4350a63282023'
    PROOF_SHA256 = '36c6dfd5d60dfbd0882f89ed8e6a511681df64b1c446827c47bee4b678b7aad4'
    DESCRIPTORS = (('S154C-ENC-E01',
  {'file': 'crates/time-core/src/projection_enclosure.rs',
   'function': {'function_name': 'center_admissible',
                'return_type': '-> bool',
                'span': {'end': {'column': 2, 'line': 156}, 'start': {'column': 1, 'line': 145}}},
   'genre': 'BinaryOperator',
   'name': 'crates/time-core/src/projection_enclosure.rs:155:15: replace >> with << in '
           'center_admissible',
   'package': 'time-core',
   'replacement': '<<',
   'span': {'end': {'column': 17, 'line': 155}, 'start': {'column': 15, 'line': 155}}}),
 ('S154C-ENC-E02',
  {'file': 'crates/time-core/src/projection_enclosure.rs',
   'function': {'function_name': 'distance_admissible',
                'return_type': '-> bool',
                'span': {'end': {'column': 2, 'line': 169}, 'start': {'column': 1, 'line': 158}}},
   'genre': 'BinaryOperator',
   'name': 'crates/time-core/src/projection_enclosure.rs:166:15: replace >> with << in '
           'distance_admissible',
   'package': 'time-core',
   'replacement': '<<',
   'span': {'end': {'column': 17, 'line': 166}, 'start': {'column': 15, 'line': 166}}}),
 ('S154C-ENC-E03',
  {'file': 'crates/time-core/src/projection_enclosure.rs',
   'function': {'function_name': 'Fraction::compare',
                'return_type': '-> Result<Ordering, FixtureEnclosureError>',
                'span': {'end': {'column': 6, 'line': 267}, 'start': {'column': 5, 'line': 235}}},
   'genre': 'BinaryOperator',
   'name': 'crates/time-core/src/projection_enclosure.rs:251:31: replace < with <= in Fraction::compare',
   'package': 'time-core',
   'replacement': '<=',
   'span': {'end': {'column': 32, 'line': 251}, 'start': {'column': 31, 'line': 251}}}),
 ('S154C-ENC-E04',
  {'file': 'crates/time-core/src/projection_enclosure.rs',
   'function': {'function_name': 'ScaledValue::from_bits',
                'return_type': '-> Result<Self, FixtureEnclosureError>',
                'span': {'end': {'column': 6, 'line': 325}, 'start': {'column': 5, 'line': 300}}},
   'genre': 'BinaryOperator',
   'name': 'crates/time-core/src/projection_enclosure.rs:314:28: replace | with ^ in '
           'ScaledValue::from_bits',
   'package': 'time-core',
   'replacement': '^',
   'span': {'end': {'column': 29, 'line': 314}, 'start': {'column': 28, 'line': 314}}}))

    def records(self):
        return [{'id': name, 'candidate_sha256': self.CANDIDATE,
                 'source_sha256': self.SOURCE, 'mutant': copy.deepcopy(descriptor),
                 'requirement': 'CHECK-7/8/14/25',
                 'rationale': 'Independent full-boundary source proof; native outcome still required.',
                 'proof': {'path': 'tasks/s1.2-chart-equivalences.md',
                           'sha256': self.PROOF_SHA256, 'section': name,
                           'reviewer': 'root; pure_core_next_owner; s154c_equivalence_review',
                           'verdict': 'independently-justified-equivalent'}}
                for name, descriptor in self.DESCRIPTORS]

    def test_empty_exact_four_and_reordered_records_preserve_historical_empty_scope(self):
        proof = (ROOT / 'tasks/s1.2-chart-equivalences.md').read_bytes()
        self.assertEqual(hashlib.sha256(proof).hexdigest(), self.PROOF_SHA256)
        sections = proof.decode().split('## S154C-ENC-')[1:]
        retained = tuple(('S154C-ENC-' + section.splitlines()[0].strip(),
                          json.loads(section.split('```json\n', 1)[1].split('\n```', 1)[0]))
                         for section in sections)
        self.assertEqual(retained, self.DESCRIPTORS)
        good = self.records()
        MODULE.historical_s154c_admit_core_equivalence_scope([])
        MODULE.historical_s154c_admit_core_equivalence_scope(good)
        MODULE.historical_s154c_admit_core_equivalence_scope(list(reversed(good)))
        MODULE.historical_s154b_admit_core_equivalence_scope([])
        with self.assertRaises(MODULE.Incomplete):
            MODULE.historical_s154b_admit_core_equivalence_scope(good)

    def test_subset_extra_duplicate_and_malformed_record_sets_reject(self):
        good = self.records()
        for bad in (None, {}, tuple(good), good[:-1], good + good[:1],
                    [good[0], good[0], *good[2:]], [None, *good[1:]]):
            with self.subTest(value=type(bad).__name__), self.assertRaises(MODULE.Incomplete):
                MODULE.historical_s154c_admit_core_equivalence_scope(bad)
        for key in good[0]:
            bad = copy.deepcopy(good); del bad[0][key]
            with self.subTest(missing=key), self.assertRaises(MODULE.Incomplete):
                MODULE.historical_s154c_admit_core_equivalence_scope(bad)
        bad = copy.deepcopy(good); bad[0]['unknown'] = 1
        with self.assertRaises(MODULE.Incomplete):
            MODULE.historical_s154c_admit_core_equivalence_scope(bad)

    def test_candidate_source_and_complete_native_descriptor_identity_reject_changes(self):
        changes = [(('id',), 'S154C-ENC-E99'), (('candidate_sha256',), '0' * 64),
                   (('source_sha256',), '1' * 64), (('requirement',), 'CHECK-25'),
                   (('mutant',), None), (('mutant', 'file'), 'crates/time-core/src/chart_enclosure.rs'),
                   (('mutant', 'package'), 'ntp-wire'), (('mutant', 'genre'), 'FnValue'),
                   (('mutant', 'name'), 'different native descriptor'),
                   (('mutant', 'replacement'), '>>'),
                   (('mutant', 'function', 'function_name'), 'distance_admissible'),
                   (('mutant', 'function', 'return_type'), '-> u64'),
                   (('mutant', 'function', 'span', 'start', 'line'), 144),
                   (('mutant', 'function', 'span', 'end', 'column'), 3),
                   (('mutant', 'span', 'start', 'column'), 16),
                   (('mutant', 'span', 'end', 'line'), 156)]
        for path, value in changes:
            bad = self.records(); target = bad[0]
            for key in path[:-1]: target = target[key]
            target[path[-1]] = value
            with self.subTest(field=path), self.assertRaises(MODULE.Incomplete):
                MODULE.historical_s154c_admit_core_equivalence_scope(bad)
        bad = self.records(); bad[0]['mutant']['unknown'] = 1
        with self.assertRaises(MODULE.Incomplete):
            MODULE.historical_s154c_admit_core_equivalence_scope(bad)

    def test_proof_binding_review_and_rationale_bounds_reject_malformed_values(self):
        for key, value in [('path', 'tasks/s1.3-enclosure-equivalences.md'),
                           ('sha256', '2' * 64), ('section', 'S154C-ENC-E02'),
                           ('reviewer', 'author only'), ('verdict', 'untested')]:
            bad = self.records(); bad[0]['proof'][key] = value
            with self.subTest(proof=key), self.assertRaises(MODULE.Incomplete):
                MODULE.historical_s154c_admit_core_equivalence_scope(bad)
        for proof in (None, [], {}, {**self.records()[0]['proof'], 'unknown': True}):
            bad = self.records(); bad[0]['proof'] = proof
            with self.subTest(proof=type(proof).__name__), self.assertRaises(MODULE.Incomplete):
                MODULE.historical_s154c_admit_core_equivalence_scope(bad)
        for rationale in ('', ' \n ', 'x' * 4097, None, True, []):
            bad = self.records(); bad[0]['rationale'] = rationale
            with self.subTest(rationale=type(rationale).__name__), self.assertRaises(MODULE.Incomplete):
                MODULE.historical_s154c_admit_core_equivalence_scope(bad)


class TrackingCheckpointTests(HistoricalS14cTestCase):
    """Literal S152 outcome and conservative owner-bound accounting controls."""

    NAMES = (
        'fitted_tracking_journey::fitted_journey_preserves_raw_witness_legacy_mode_and_real_credit',
        'fitted_tracking_journey::legacy_raw_only_neighbor_remains_unchanged',
        'fitted_tracking_journey::post_prune_tracking_uses_fitted_suffix_and_oldest_fitted_anchor',
        'fitted_tracking_journey::distinct_same_context_histories_keep_their_own_tracking_payloads',
        'fitted_tracking_journey::missing_later_excluded_tracking_preserves_complete_lifecycle',
        'fitted_tracking_journey::unavailable_principal_preserves_no_estimate_with_mixed_attachments',
        'fitted_tracking_journey::untracked_nonwitness_does_not_block_actual_witness',
        'fitted_tracking_journey::singleton_zero_and_original_pair_denominators_are_distinct',
        'fitted_tracking_journey::projection_errors_precede_invalid_tracking_skew',
        'regression::projection::projection_guards::fitted_tracking_raw_controls::raw_skew_uses_mandatory_rule_and_preserves_legal_bits',
        'regression::projection::projection_guards::fitted_tracking_raw_controls::final_tracking_candidate_preserves_field_domain_and_precedence',
        'regression::projection::projection_guards::fitted_tracking_raw_controls::closed_tracking_steps_preserve_operation_and_typed_failure',
        'regression::projection::projection_guards::fitted_tracking_raw_controls::tracking_errors_preserve_nested_causes_and_bounded_formats',
        'combination::admission::receiving_tests::tracking_source_receiving_preserves_actual_entry_identity',
        'fitted_tracking_journey::sixteen_tracking_supporters_preserve_complete_witness',
    )
    PASSED = ''.join('test ' + name + ' ... ok\n' for name in NAMES)

    def test_literal_scope_and_adaptive_partition_are_exact(self):
        self.assertEqual(tuple(MODULE.TRACKING_ORDINARY_TESTS), self.NAMES)
        self.assertEqual(MODULE.tracking_numeric_work(),
                         {'charged_fit_work_upper': 6758, 'point_admissions_upper': 233369})
        vector = MODULE.tracking_budget_vector()
        self.assertEqual(vector['tracking.fixture.histories'], 34)
        self.assertEqual(vector['tracking.fixture.projections'], 54)
        self.assertEqual(vector['tracking.fixture.tracking_projections'], 47)
        self.assertEqual(vector['tracking.adaptive_work.charged_fit_work_upper'], 6758)
        self.assertEqual(vector['tracking.adaptive_work.point_admissions_upper'], 233369)
        self.assertEqual(vector['tracking.method.standard_error_binary_primitives_upper'], 376)
        self.assertEqual(vector['tracking.method.standard_error_square_roots_upper'], 94)
        self.assertEqual(vector['tracking.method.tracking_binary_primitives_upper'], 188)
        self.assertEqual(vector['tracking.method.tracking_holder_entries_upper'], 240)
        self.assertEqual(vector['tracking.method.tracking_holder_getters_upper'], 13290)
        self.assertEqual(vector['tracking.fixture.direct_principal_advances'], 2)
        self.assertEqual(vector['tracking.fixture.layout_checks'], 2)
        self.assertEqual(vector['tracking.private.raw_receiving_validations_upper'], 2)
        self.assertEqual(vector['tracking.private.raw_receiving_garde_callbacks_upper'], 2)
        self.assertEqual(vector['tracking.private.receiving_predicate_passes_upper'], 4)
        # Each of four real raw receiving passes checks frame, generation and at.
        self.assertEqual(vector['tracking.private.receiving_context_comparisons_upper'], 12)
        self.assertEqual(vector['tracking.private.direct_receiving_admissions_upper'], 2)
        self.assertEqual(vector['tracking.private.record_source_get_calls_upper'], 4)
        self.assertEqual(vector['tracking.private.record_source_len_calls_upper'], 8)
        self.assertEqual(vector['tracking.private.direct_receiving_getter_requests_upper'], 124)
        self.assertEqual(vector['tracking.private.direct_entry_pointer_comparisons_upper'], 5)
        self.assertEqual(vector['tracking.lifecycle_work.charged_fit_work_upper'], 0)
        self.assertEqual(vector['tracking.adapter_work.projection_capsule_and_kernel_entries_upper'], 108)
        # Both raw-anchor and tracking fixture routes borrow adaptive histories.
        self.assertEqual(vector['tracking.adapter_work.projection_adaptive_extra_internal_getters_upper'], 54)
        external = MODULE.tracking_budget_vector(package=False)
        self.assertEqual(set(vector), set(external))
        self.assertTrue(all(not value for key, value in external.items() if not key.startswith('tracking.shared.')))
        self.assertGreater(external['tracking.shared.record_view_requests_upper'], 0)

    def test_all_fifteen_actual_names_required_and_duplicates_rejected(self):
        result = MODULE.tracking_test_observation(self.PASSED, require_complete=True)
        self.assertFalse(result['tracking_source_only_pending'])
        self.assertEqual(result['tracking_recorded_source_work_upper'],
                         {'charged_fit_work_upper': 6758, 'point_admissions_upper': 233369})
        for bad in (self.PASSED.split('\n', 1)[1], self.PASSED.replace('ok\n', 'FAILED\n', 1),
                    self.PASSED.replace('ok\n', 'ignored\n', 1), self.PASSED.rstrip('\n'),
                    json.dumps({'rendered': self.PASSED}) + '\n',
                    self.PASSED + self.PASSED.splitlines(keepends=True)[0]):
            with self.subTest(bad=bad), self.assertRaises(MODULE.Incomplete):
                MODULE.tracking_test_observation(bad, require_complete=True)

    def test_partial_prefix_is_pending_and_old_gates_remain_active(self):
        partial = 'test ' + self.NAMES[0] + ' ... FAILED\n'
        result = MODULE.profile_counts(partial, require_fit=False)
        self.assertTrue(result['tracking_source_only_pending'])
        self.assertIsNone(result['tracking_recorded_source_work_upper'])
        self.assertFalse(result['legacy_profile_observation_present'])
        for forbidden in ('--- STDERR:', 'Summary [', 'CORE_FUZZ_COUNTS'):
            with self.subTest(forbidden=forbidden), self.assertRaises(MODULE.Incomplete):
                MODULE.profile_counts(partial + forbidden + '\n', require_fit=False)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.profile_counts('CORE_PROFILE_COUNTS fits=999999 points=3\n' + self.PASSED)

    def test_limits_reject_missing_bool_and_underbudget_without_zeroing_pending(self):
        full = MODULE.tracking_budget_vector()
        self.assertEqual(MODULE.admit_tracking_allowances(full), full)
        for bad in ({}, {**full, 'tracking.method.tracking_binary_primitives_upper': 119},
                    {**full, 'tracking.fixture.histories': True}):
            with self.subTest(bad=bad), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_tracking_allowances(bad)


class S152BEquivalenceControls(HistoricalS14cTestCase):
    """Exact frozen descriptor controls; no synthetic evidence becomes E."""

    DESCRIPTORS = (('S152B-C-E-001',
  {'file': 'crates/time-core/src/combination.rs',
   'function': {'function_name': "TrackingSupporters<'entry>::from_decision",
                'return_type': '-> Result<Self, FixtureTrackingCombinationError>',
                'span': {'end': {'column': 6, 'line': 537}, 'start': {'column': 5, 'line': 482}}},
   'genre': 'BinaryOperator',
   'name': 'crates/time-core/src/combination.rs:503:23: replace || with && in '
           "TrackingSupporters<'entry>::from_decision",
   'package': 'time-core',
   'replacement': '&&',
   'span': {'end': {'column': 25, 'line': 503}, 'start': {'column': 23, 'line': 503}}}),
 ('S152B-C-E-002',
  {'file': 'crates/time-core/src/combination.rs',
   'function': {'function_name': "TrackingSupporters<'entry>::from_decision",
                'return_type': '-> Result<Self, FixtureTrackingCombinationError>',
                'span': {'end': {'column': 6, 'line': 537}, 'start': {'column': 5, 'line': 482}}},
   'genre': 'BinaryOperator',
   'name': 'crates/time-core/src/combination.rs:503:49: replace || with && in '
           "TrackingSupporters<'entry>::from_decision",
   'package': 'time-core',
   'replacement': '&&',
   'span': {'end': {'column': 51, 'line': 503}, 'start': {'column': 49, 'line': 503}}}),
 ('S152B-C-E-003',
  {'file': 'crates/time-core/src/combination.rs',
   'function': {'function_name': 'admit_record_source',
                'return_type': '-> Result<(), FixtureCombinationError>',
                'span': {'end': {'column': 2, 'line': 896}, 'start': {'column': 1, 'line': 842}}},
   'genre': 'BinaryOperator',
   'name': 'crates/time-core/src/combination.rs:859:19: replace || with && in admit_record_source',
   'package': 'time-core',
   'replacement': '&&',
   'span': {'end': {'column': 21, 'line': 859}, 'start': {'column': 19, 'line': 859}}}),
 ('S152B-C-E-004',
  {'file': 'crates/time-core/src/combination.rs',
   'function': {'function_name': 'admit_record_source',
                'return_type': '-> Result<(), FixtureCombinationError>',
                'span': {'end': {'column': 2, 'line': 896}, 'start': {'column': 1, 'line': 842}}},
   'genre': 'BinaryOperator',
   'name': 'crates/time-core/src/combination.rs:859:45: replace || with && in admit_record_source',
   'package': 'time-core',
   'replacement': '&&',
   'span': {'end': {'column': 47, 'line': 859}, 'start': {'column': 45, 'line': 859}}}),
 ('S152B-C-E-005',
  {'file': 'crates/time-core/src/combination.rs',
   'function': {'function_name': 'combine_records',
                'return_type': '-> Result<FixtureCombinationKernelOutcome, FixtureCombinationError>',
                'span': {'end': {'column': 2, 'line': 1291}, 'start': {'column': 1, 'line': 1227}}},
   'genre': 'BinaryOperator',
   'name': 'crates/time-core/src/combination.rs:1251:26: replace || with && in combine_records',
   'package': 'time-core',
   'replacement': '&&',
   'span': {'end': {'column': 28, 'line': 1251}, 'start': {'column': 26, 'line': 1251}}}),
 ('S152B-A-E-001',
  {'file': 'crates/time-core/src/combination/admission.rs',
   'function': {'function_name': 'admit_lifecycle_source',
                'return_type': "-> Result<Option<AdmittedFixtureSupporters<'input, 'entry>>, "
                               'FixtureCombinationError>',
                'span': {'end': {'column': 2, 'line': 182}, 'start': {'column': 1, 'line': 122}}},
   'genre': 'BinaryOperator',
   'name': 'crates/time-core/src/combination/admission.rs:145:21: replace || with && in '
           'admit_lifecycle_source',
   'package': 'time-core',
   'replacement': '&&',
   'span': {'end': {'column': 23, 'line': 145}, 'start': {'column': 21, 'line': 145}}}),
 ('S152B-A-E-002',
  {'file': 'crates/time-core/src/combination/admission.rs',
   'function': {'function_name': 'contributor_count',
                'return_type': '-> Result<usize, FixtureCombinationError>',
                'span': {'end': {'column': 2, 'line': 223}, 'start': {'column': 1, 'line': 190}}},
   'genre': 'BinaryOperator',
   'name': 'crates/time-core/src/combination/admission.rs:197:28: replace || with && in contributor_count',
   'package': 'time-core',
   'replacement': '&&',
   'span': {'end': {'column': 30, 'line': 197}, 'start': {'column': 28, 'line': 197}}}),
 ('S152B-L-E-001',
  {'file': 'crates/time-core/src/combination/lifecycle.rs',
   'function': {'function_name': 'estimate_relation_valid',
                'return_type': '-> bool',
                'span': {'end': {'column': 2, 'line': 1119}, 'start': {'column': 1, 'line': 1086}}},
   'genre': 'BinaryOperator',
   'name': 'crates/time-core/src/combination/lifecycle.rs:1110:13: replace || with && in '
           'estimate_relation_valid',
   'package': 'time-core',
   'replacement': '&&',
   'span': {'end': {'column': 15, 'line': 1110}, 'start': {'column': 13, 'line': 1110}}}))
    SOURCES = {
        'crates/time-core/src/combination.rs': 'f0de15bb4d11019094292ce4b79d9e82d2f5270b305e1b7a4b670f27f31621c6',
        'crates/time-core/src/combination/admission.rs': 'a7281c21735d235581da6401a3f6824a17242e3724da913e780f63f3f13a1bdf',
        'crates/time-core/src/combination/lifecycle.rs': '28c6b7d19ce5edd77c2d39062d92b1c8d595760c60825f4414d5cc3f1871ea28',
    }
    CANDIDATE = 'c' * 64

    def records(self):
        return [{'id': name, 'candidate_sha256': self.CANDIDATE,
                 'source_sha256': self.SOURCES[descriptor['file']],
                 'mutant': copy.deepcopy(descriptor), 'requirement': 'CHECK-7/8/14/25',
                 'rationale': 'bounded synthetic independent-review control',
                 'proof': {'path': 'tasks/s1.3-combination-equivalences.md',
                           'sha256': MODULE.S152B_EQUIVALENCE_PROOF,
                           'section': name, 'reviewer': 'root; adaptive_contract_review',
                           'verdict': 'independently-justified-equivalent'}}
                for name, descriptor in self.DESCRIPTORS]

    def test_unbound_candidate_and_empty_pending_remain_closed(self):
        with patch.object(MODULE, 'S152B_EQUIVALENCE_CANDIDATE', None), \
             patch.dict(MODULE.HISTORICAL_S152B_MUTATION_INVENTORIES, {}, clear=True):
            MODULE.historical_s152b_admit_core_equivalence_scope([])
            with self.assertRaises(MODULE.Incomplete):
                MODULE.historical_s152b_admit_core_equivalence_scope(self.records())
        with patch.dict(MODULE.HISTORICAL_S152B_MUTATION_INVENTORIES, {'combination': {}}, clear=True):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.historical_s152b_admit_core_equivalence_scope([])

    def test_exact_eight_and_malformed_neighbors(self):
        good = self.records()
        with patch.object(MODULE, 'S152B_EQUIVALENCE_CANDIDATE', self.CANDIDATE):
            MODULE.historical_s152b_admit_core_equivalence_scope(good)
            MODULE.historical_s152b_admit_core_equivalence_scope(list(reversed(good)))
            for bad in (good[:-1], good + good[:1], good[:4] + good[:1],
                        [*good[:1], *good[2:]], None, {}):
                with self.subTest(kind=type(bad).__name__), self.assertRaises(MODULE.Incomplete):
                    MODULE.historical_s152b_admit_core_equivalence_scope(bad)
            for path, changed in [(('id',), 'other'), (('candidate_sha256',), 'd' * 64),
                                  (('source_sha256',), 'e' * 64), (('requirement',), 'CHECK-25'),
                                  (('rationale',), ''), (('mutant', 'replacement'), '||'),
                                  (('proof', 'path'), 'tasks/other.md'), (('proof', 'sha256'), 'f' * 64),
                                  (('proof', 'section'), 'S152B-C-E-002'),
                                  (('proof', 'reviewer'), 'author only'),
                                  (('proof', 'verdict'), 'untested')]:
                bad = copy.deepcopy(good); target = bad[0]
                for key in path[:-1]: target = target[key]
                target[path[-1]] = changed
                with self.subTest(field=path), self.assertRaises(MODULE.Incomplete):
                    MODULE.historical_s152b_admit_core_equivalence_scope(bad)
            extra = copy.deepcopy(good); extra[0]['unknown'] = 1
            with self.assertRaises(MODULE.Incomplete): MODULE.historical_s152b_admit_core_equivalence_scope(extra)


class StandardErrorCheckpointTests(HistoricalS14cTestCase):
    """Independent literal S151 outcome and source-accounting controls."""

    NAMES = (
        'same_fit_standard_errors::weighted_common_divisor_scale_and_exact_zero',
        'same_fit_standard_errors::actual_adaptive_selected_fit_excludes_auxiliary_row',
        'same_fit_standard_errors::tiny_short_span_is_healthy_and_long_span_rejects_rate_variance',
        'regression::residuals::residual_guards::standard_error_controls::coefficient_scalar_admission_covers_invalid_and_legal_neighbors',
        'regression::residuals::residual_guards::standard_error_controls::binary_steps_preserve_actual_operation_stage_and_cause',
        'regression::residuals::residual_guards::standard_error_controls::root_steps_preserve_actual_unary_cause_and_legal_neighbors',
        'regression::residuals::residual_guards::standard_error_controls::error_format_and_sources_cover_closed_variants',
    )
    PASSED = ''.join('test ' + name + ' ... ok\n' for name in NAMES)
    DOCS = (
        'test crates/time-core/src/regression/residuals.rs - regression::residuals::FixtureStandardErrors (line 434) - compile fail ... ok\n'
        'test crates/time-core/src/regression/residuals.rs - regression::residuals::FixtureStandardErrors (line 441) - compile fail ... ok\n'
    )

    def test_exact_recipe_and_geometry_producer_are_added_once(self):
        self.assertEqual(tuple(MODULE.STANDARD_ERROR_ORDINARY_TESTS), self.NAMES)
        self.assertEqual(MODULE.standard_error_numeric_work(),
                         {'charged_fit_work_upper': 563, 'point_admissions_upper': 19284})
        vector = MODULE.standard_error_budget_vector()
        self.assertEqual(len(vector), 100)
        self.assertEqual(vector['standard_errors.adaptive_work.charged_fit_work_upper'], 558)
        self.assertEqual(vector['standard_errors.adaptive_work.point_admissions_upper'], 19269)
        self.assertEqual(vector['standard_errors.fixture_call.statistics'], 5)
        self.assertEqual(vector['standard_errors.fixture_call.points'], 15)
        self.assertEqual(vector['standard_errors.geometry.report_copies_upper'], 63129)
        self.assertEqual(vector['standard_errors.geometry.logical_bytes_upper'], 2020128)
        self.assertEqual(vector['standard_errors.work.binary_primitives_upper'], 64)
        self.assertEqual(vector['standard_errors.work.square_roots_upper'], 20)
        package, checks, _ = MODULE.historical_cohort_budget_vectors(require_limits=False)
        self.assertEqual(len(package), 1508)
        self.assertEqual(package['charged_fit_work_upper'], 35291)
        self.assertEqual(checks['coverage']['point_admissions_upper'], 1292051)
        self.assertEqual(checks['contracts']['standard_errors.geometry.logical_bytes_upper'], 15968)
        self.assertFalse(any(value for key, value in checks['contracts'].items()
                             if key.startswith('standard_errors.') and '.geometry.' not in key))

    def test_seven_native_passes_and_two_original_docs_are_distinct_obligations(self):
        value = MODULE.standard_error_test_observation(self.PASSED, require_complete=True)
        self.assertFalse(value['standard_error_source_only_pending'])
        self.assertFalse(value['standard_error_original_doctests_complete'])
        self.assertEqual(value['standard_error_recorded_source_work_upper'],
                         {'charged_fit_work_upper': 563, 'point_admissions_upper': 19284})
        with self.assertRaisesRegex(MODULE.Incomplete, 'both exact'):
            MODULE.standard_error_test_observation(self.PASSED, require_complete=True, require_doctests=True)
        value = MODULE.standard_error_test_observation(self.PASSED + self.DOCS,
                                                       require_complete=True, require_doctests=True)
        self.assertTrue(value['standard_error_original_doctests_complete'])
        shifted = self.DOCS.replace('line 434', 'line 437').replace('line 441', 'line 444')
        self.assertFalse(MODULE.standard_error_test_observation(
            self.PASSED + shifted, require_complete=True)['standard_error_original_doctests_complete'])
        self.assertFalse(MODULE.standard_error_test_observation(
            self.PASSED.replace('test same_fit_', 'test residuals::same_fit_'),
            require_complete=True)['standard_error_source_only_pending'])

    def test_missing_failed_ignored_json_and_replayed_outcomes_cannot_qualify(self):
        for bad in (self.PASSED.split('\n', 1)[1], self.PASSED.replace('ok\n', 'FAILED\n', 1),
                    self.PASSED.replace('ok\n', 'ignored\n', 1), self.PASSED.rstrip('\n'),
                    json.dumps({'rendered': self.PASSED}) + '\n',
                    self.PASSED + self.PASSED.splitlines(keepends=True)[0],
                    self.PASSED + self.PASSED.splitlines(keepends=True)[0].rstrip('\n')):
            with self.subTest(bad=bad), self.assertRaises(MODULE.Incomplete):
                MODULE.standard_error_test_observation(bad, require_complete=True)
        with self.assertRaisesRegex(MODULE.Incomplete, 'duplicated'):
            MODULE.standard_error_test_observation(self.PASSED + self.DOCS + self.DOCS)

    def test_partial_prefix_has_no_invented_counters_or_inner_work(self):
        partial = 'test ' + self.NAMES[0] + ' ... FAILED\n'
        value = MODULE.profile_counts(partial, require_fit=False)
        self.assertTrue(value['standard_error_source_only_pending'])
        self.assertIsNone(value['standard_error_recorded_source_work_upper'])
        self.assertNotIn('direct_fit_calls', value)
        self.assertFalse(value['legacy_profile_observation_present'])
        for forbidden in ('--- STDERR:', 'Summary [', 'CORE_FUZZ_COUNTS'):
            with self.subTest(forbidden=forbidden), self.assertRaises(MODULE.Incomplete):
                MODULE.profile_counts(partial + forbidden + '\n', require_fit=False)

    def test_supplement_follows_legacy_gates_and_literal_docs_charge_once(self):
        text = 'CORE_PROFILE_COUNTS fits=1 points=3\n'
        before = MODULE.profile_counts(text)
        after = MODULE.profile_counts(text + self.PASSED, require_standard_errors=True)
        self.assertEqual(after['charged_fit_work_upper'] - before['charged_fit_work_upper'], 563)
        self.assertEqual(after['point_admissions_upper'] - before['point_admissions_upper'], 19284)
        charged = MODULE.account_literal_doctests(after)
        self.assertEqual(charged['charged_fit_work_upper'], after['charged_fit_work_upper'])
        self.assertEqual(charged['point_admissions_upper'], after['point_admissions_upper'] + 3)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.profile_counts('CORE_PROFILE_COUNTS fits=999999 points=3\n' + self.PASSED)

    def test_package_reserves_complete_source_work_before_any_observation(self):
        fund_synthetic_source_recipes(self)
        report = {}
        counts = {**MODULE.package_charged_upper(), **MODULE.selection_reservation(package=True),
                  'observation_pending': True}
        MODULE.record_counts(report, 'package', counts)
        self.assertEqual(report['core_standard_error_reserved_totals'], MODULE.standard_error_budget_vector())
        self.assertEqual(report['core_standard_error_source_test_pending_invocations'], ['package'])
        self.assertNotIn('standard_error_reserved_budget', counts)
        forged = {**counts, 'standard_error_reserved_budget': {}}
        with self.assertRaisesRegex(MODULE.Incomplete, 'Standard-error reservation'):
            MODULE.record_counts({}, 'forged', forged)

    def test_closed_units_exact_integer_bounds_and_allowances(self):
        vector = MODULE.standard_error_budget_vector()
        self.assertEqual(MODULE.admit_standard_error_allowances(vector), vector)
        for bad in ({}, {**vector, 'unknown': 0},
                    {**vector, 'standard_errors.work.square_roots_upper': 19},
                    {**vector, 'standard_errors.fixture_call.points': True}):
            with self.subTest(bad=bad), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_standard_error_allowances(bad)
        for fit in (None, True, -1, 1.0, 1 << 63):
            with self.subTest(fit=fit), self.assertRaises(MODULE.Incomplete):
                MODULE.standard_error_budget_vector(package=False, charged_fit_upper=fit)


class AdaptiveCheckpointTests(HistoricalS14cTestCase):
    """Independent receiving controls for the added family, without product runs."""

    def test_new_family_is_added_once_after_the_unchanged_legacy_gate(self):
        output = 'CORE_PROFILE_COUNTS fits=1 points=3\n' + ADAPTIVE_FIXTURES.COMPLETE_PROFILE
        actual = MODULE.profile_counts(output, require_adaptive=True)
        self.assertEqual(actual['direct_fit_calls'], 1)
        self.assertEqual(actual['legacy_point_admissions'], 3)
        self.assertEqual(actual['charged_fit_work_upper'], 14819)
        self.assertEqual(actual['point_admissions_upper'], 510931)
        self.assertTrue(actual['adaptive_observation_complete'])
        self.assertEqual(actual['adaptive_observed_calls'], {
            'retained_points': 125, 'retained_inputs': 7, 'retained_selections': 14,
            'adaptive_observations': 200, 'adaptive_appends': 200, 'adaptive_histories': 13,
            'raw_parameter_validations': 26, 'raw_ordinal_validations': 3,
            'raw_event_validations': 8, 'external_normalizations': 0})

    def test_new_raw_marker_is_checked_before_closed_legacy_dispatch(self):
        row = next(line for line in ADAPTIVE_FIXTURES.ROWS
                   if 'CORE_LIFECYCLE_RAW_VALIDATION_PROFILE' in line)
        with self.assertRaises(MODULE.COMBINATION.Incomplete):
            MODULE.COMBINATION.lifecycle_counts(row + '\n')
        healthy = 'CORE_PROFILE_COUNTS fits=1 points=3\n'
        actual = MODULE.profile_counts(healthy + row + '\n')
        self.assertFalse(actual['adaptive_observation_complete'])
        self.assertIsNone(actual['adaptive_observed_calls'])
        for invalid in (row + '\n' + row + '\n', row, row.replace('case=', 'case=unknown_') + '\n'):
            with self.subTest(invalid=invalid), self.assertRaises(MODULE.Incomplete):
                MODULE.profile_counts(healthy + invalid)

    def test_new_only_partial_prefix_does_not_invent_legacy_observations(self):
        actual = MODULE.profile_counts(ADAPTIVE_FIXTURES.ROWS[0] + '\n', require_fit=False)
        self.assertFalse(actual['legacy_profile_observation_present'])
        self.assertNotIn('direct_fit_calls', actual)
        self.assertFalse(actual['adaptive_observation_complete'])
        self.assertIsNone(actual['adaptive_observed_calls'])

    def test_new_only_partial_prefix_preserves_replay_and_wrong_owner_rejection(self):
        healthy = ADAPTIVE_FIXTURES.ROWS[0] + '\n'
        self.assertFalse(MODULE.profile_counts(healthy, require_fit=False)['adaptive_observation_complete'])
        for wrong in ('--- STDERR:', 'Summary [', 'CORE_HISTORY_FUZZ_COUNTS',
                      'CORE_NORMALIZATION_FUZZ_COUNTS', 'CORE_STATS_FUZZ_COUNTS', 'CORE_FUZZ_COUNTS'):
            with self.subTest(wrong=wrong), self.assertRaises(MODULE.Incomplete):
                MODULE.profile_counts(healthy + wrong + '\n', require_fit=False)

    def test_package_and_doctest_reservation_has_literal_independent_totals(self):
        self.assertEqual(MODULE.package_charged_upper(), {
            'charged_fit_work_upper': 63129, 'point_admissions_upper': 2253360})
        self.assertEqual(MODULE.package_charged_upper(include_doctests=True), {
            'charged_fit_work_upper': 63129, 'point_admissions_upper': 2253363})
        package = MODULE.adaptive_budget_vector(include_doctests=True)
        self.assertEqual(len(package), 144)
        self.assertEqual(package['adaptive.call.adaptive_appends'], 200)
        self.assertEqual(package['adaptive.work.charged_fit_work_upper'], 14818)
        self.assertEqual(package['adaptive.work.point_admissions_upper'], 510928)

    def test_external_source_binding_rejects_a_changed_neighbor(self):
        healthy = MODULE.adaptive_external_budget_vector()
        self.assertEqual(healthy['adaptive.call.adaptive_appends'], 3)
        self.assertEqual(healthy['adaptive.call.external_normalizations'], 1)
        self.assertTrue(all(value == 0 for key, value in healthy.items()
                            if key.startswith('adaptive.delegated_principal.')))
        source = MODULE.ADAPTIVE_CONTRACTS.legal_contract_source()
        with patch.object(MODULE.ADAPTIVE_CONTRACTS, 'legal_contract_source', return_value=source + '\n'), \
                self.assertRaisesRegex(MODULE.Incomplete, 'independent accounting binding'):
            MODULE.adaptive_external_budget_vector()

    def test_prior_equivalences_cannot_rebind_to_the_new_scope(self):
        with patch.dict(MODULE.MUTATION_INVENTORIES, {}, clear=True):
            MODULE.admit_core_equivalence_scope([])
        with self.assertRaises(MODULE.Incomplete):
            MODULE.admit_core_equivalence_scope(CombinationCheckpointTests.combination_equivalence_scope())

    def test_prospective_scope_checks_the_new_owner_before_effects(self):
        fund_synthetic_source_recipes(self)
        coverage = MODULE.prospective_scope_reservation('coverage', dict(MODULE.SUBJECTS))
        self.assertEqual(coverage['adaptive_source_work_upper']['adaptive.call.adaptive_appends'], 200)
        self.assertEqual(coverage['adaptive_source_work_upper']['adaptive.work.charged_fit_work_upper'], 14818)
        contracts = MODULE.prospective_scope_reservation('contracts', dict(MODULE.SUBJECTS))
        self.assertEqual(contracts['charged_upper'], {
            'charged_fit_work_upper': 685, 'point_admissions_upper': 23679})
        self.assertEqual(contracts['adaptive_source_work_upper']['adaptive.legal_aux.getters'], 29)
        limits = dict(MODULE.ADAPTIVE_TELEMETRY['global_limits'])
        limits['adaptive.call.adaptive_appends'] = 199
        with patch.dict(MODULE.ADAPTIVE_TELEMETRY, {'global_limits': limits}), \
                self.assertRaises(MODULE.Incomplete):
            MODULE.prospective_scope_reservation('coverage', dict(MODULE.SUBJECTS))


class ProjectionAdapterCheckpointTests(HistoricalS14cTestCase):
    """Independent S150 source reservations and named native-result admission."""

    PASSED = ('test projection::adaptive_anchor::'
              'adaptive_fitted_anchor_projects_through_enclosure_and_batch ... ok\n')

    def test_source_only_recipe_adds_setup_once_and_all_adapter_entries(self):
        budget = MODULE.projection_adapter_budget_vector()
        self.assertEqual(len(budget), 167)
        self.assertEqual(budget['projection_adapter.fixed_requests_upper'], 544)
        self.assertEqual(budget['projection_adapter.projection_call.projections'], 11)
        self.assertEqual(budget['projection_adapter.adaptive_work.charged_fit_work_upper'], 806)
        self.assertEqual(budget['projection_adapter.adaptive_work.point_admissions_upper'], 27833)
        self.assertEqual(budget['projection_adapter.adapter_work.projection_capsule_and_kernel_entries_upper'], 1110)
        self.assertEqual(budget['projection_adapter.adapter_work.projection_adaptive_extra_internal_getters_upper'], 11)
        self.assertEqual(budget['projection_adapter.downstream_work.delegated_batch_attempts_upper'], 1)
        self.assertEqual(MODULE.package_charged_upper(include_doctests=True),
                         {'charged_fit_work_upper': 63129, 'point_admissions_upper': 2253363})
        external = MODULE.projection_adapter_budget_vector(external=True)
        self.assertEqual(set(external), set(budget))
        self.assertEqual({key: value for key, value in external.items() if value}, {
            'projection_adapter.fixed_requests_upper': 1,
            'projection_adapter.adapter_work.projection_capsule_and_kernel_entries_upper': 2})

    def test_named_passing_outcome_is_required_outside_cargo_json(self):
        good = MODULE.projection_adapter_test_observation(self.PASSED, require_complete=True)
        self.assertEqual(good['projection_adapter_test_status'], 'ok')
        self.assertFalse(good['projection_adapter_source_only_pending'])
        self.assertEqual(good['projection_adapter_recorded_source_work_upper'],
                         {'charged_fit_work_upper': 806, 'point_admissions_upper': 27833})
        for bad in ('', self.PASSED.rstrip('\n'), self.PASSED.replace('ok', 'FAILED'),
                    self.PASSED.replace('ok', 'ignored'), self.PASSED + self.PASSED,
                    json.dumps({'reason': 'compiler-message', 'message': self.PASSED}) + '\n',
                    'replayed ' + self.PASSED):
            with self.subTest(bad=bad), self.assertRaises(MODULE.Incomplete):
                MODULE.projection_adapter_test_observation(bad, require_complete=True)

    def test_caught_or_missing_case_remains_pending_without_call_observations(self):
        for output, status in (('', 'missing'), (self.PASSED.replace('ok', 'FAILED'), 'FAILED'),
                               (self.PASSED.split(' ... ')[0] + ' ... ', 'reached-pending')):
            actual = MODULE.projection_adapter_test_observation(output)
            self.assertEqual(actual['projection_adapter_test_status'], status)
            self.assertTrue(actual['projection_adapter_source_only_pending'])
            self.assertIsNone(actual['projection_adapter_recorded_source_work_upper'])
        fund_synthetic_source_recipes(self)
        report = {}
        MODULE.record_counts(report, 'package', {**MODULE.package_charged_upper(),
            **MODULE.selection_reservation(package=True), 'observation_pending': True})
        self.assertEqual(report['core_projection_adapter_reserved_totals'], MODULE.projection_adapter_budget_vector())
        self.assertEqual(report['core_projection_adapter_source_test_pending_invocations'], ['package'])

    def test_old_attempt_profiles_are_not_fabricated_for_the_new_test(self):
        text = 'CORE_PROFILE_COUNTS fits=1 points=3\n' + ADAPTIVE_FIXTURES.COMPLETE_PROFILE
        before = MODULE.profile_counts(text)
        after = MODULE.profile_counts(text + self.PASSED, require_adaptive_projection=True)
        self.assertEqual(after['counter_records'], before['counter_records'])
        self.assertEqual(after['adaptive_observed_calls'], before['adaptive_observed_calls'])
        self.assertEqual(after['charged_fit_work_upper'] - before['charged_fit_work_upper'], 806)
        self.assertEqual(after['point_admissions_upper'] - before['point_admissions_upper'], 27833)

    def test_source_reservation_and_current_equivalence_scope_fail_closed(self):
        required = MODULE.projection_adapter_budget_vector()
        self.assertEqual(MODULE.admit_projection_adapter_allowances(required), required)
        for bad in (None, {}, {**required, 'extra': 0},
                    {**required, 'projection_adapter.fixed_requests_upper': 543}):
            with self.subTest(bad=type(bad)), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_projection_adapter_allowances(bad)
        with patch.dict(MODULE.MUTATION_INVENTORIES, {}, clear=True):
            MODULE.admit_core_equivalence_scope([])
        with patch.dict(MODULE.MUTATION_INVENTORIES, {'chart': {}}, clear=True):
            MODULE.admit_core_equivalence_scope([])
        with self.assertRaises(MODULE.Incomplete):
            MODULE.admit_core_equivalence_scope(AdaptiveEquivalenceScopeTests.records())


class AdaptiveEquivalenceScopeTests(HistoricalS14cTestCase):
    """Literal fresh descriptor controls; these establish no Rust equivalence."""

    @staticmethod
    def records():
        """Reconstruct reviewed descriptor facts independently of the whitelist."""
        filename = 'crates/time-core/src/regression/adaptive_history.rs'
        rows = (
            ('S149-AH-E01', 354, 9, 'validate_complete'),
            ('S149-AH-E02', 359, 9, 'validate_complete'),
            ('S149-AH-E03', 360, 9, 'validate_complete'),
            ('S149-AH-E04', 361, 9, 'validate_complete'),
            ('S149-AH-E05', 362, 9, 'validate_complete'),
            ('S149-AH-E06', 365, 9, 'validate_complete'),
            ('S149-AH-E07', 369, 9, 'validate_complete'),
            ('S149-AH-E08', 370, 9, 'validate_complete'),
            ('S149-AH-E09', 371, 9, 'validate_complete'),
            ('S149-AH-E10', 372, 9, 'validate_complete'),
            ('S149-AH-E11', 407, 21, 'StagedAppend::finish'),
            ('S149-AH-E12', 408, 21, 'StagedAppend::finish'),
            ('S149-AH-E13', 409, 21, 'StagedAppend::finish'),
            ('S149-AH-E14', 410, 21, 'StagedAppend::finish'),
            ('S149-AH-E15', 418, 21, 'StagedAppend::finish'),
            ('S149-AH-E16', 419, 21, 'StagedAppend::finish'),
            ('S149-AH-E17', 420, 21, 'StagedAppend::finish'),
        )
        functions = {
            'validate_complete': {'function_name': 'validate_complete',
                'return_type': '-> Result<(), AdaptiveHistoryError>',
                'span': {'start': {'line': 300, 'column': 1}, 'end': {'line': 378, 'column': 2}}},
            'StagedAppend::finish': {'function_name': 'StagedAppend::finish',
                'return_type': '-> Result<CommittedAdaptiveState, AdaptiveHistoryError>',
                'span': {'start': {'line': 381, 'column': 5}, 'end': {'line': 465, 'column': 6}}},
        }
        return [{
            'id': identifier,
            'candidate_sha256': 'ba623100d89bf6c034677e4ff9b04148be60c351fa1c0ce27962c9a15d275303',
            'source_sha256': 'dfe76dd00208d6cfded982b5cb98a77ec1c21cc02b7c2cdf4c26a0f837f225f5',
            'mutant': {'file': filename, 'package': 'time-core', 'genre': 'BinaryOperator',
                'name': f'{filename}:{line}:{column}: replace || with && in {function}',
                'function': copy.deepcopy(functions[function]), 'replacement': '&&',
                'span': {'start': {'line': line, 'column': column},
                         'end': {'line': line, 'column': column + 2}}},
            'requirement': 'REQ-8/REQ-14; CHECK-19/CHECK-25',
            'rationale': 'Synthetic bounded custody fixture; mathematical proof remains in the reviewed task artifact.',
            'proof': {'path': 'tasks/s1.2-adaptive-equivalences.md',
                'sha256': 'dd8ad1636e86f431455725affdaa85293c40a65901c9a8c7cbad8044bfb9d0ca',
                'section': identifier, 'reviewer': 'root; adaptive_contract_review; vfbasics_data_review',
                'verdict': 'independently-justified-equivalent'},
        } for identifier, line, column, function in rows]

    def test_complete_fresh_set_is_admitted_in_either_order(self):
        records = self.records()
        MODULE.admit_historical_s149_equivalence_scope(records)
        MODULE.admit_historical_s149_equivalence_scope(list(reversed(records)))

    def test_pending_empty_does_not_admit_a_completed_checkpoint(self):
        with patch.dict(MODULE.MUTATION_INVENTORIES, {}, clear=True):
            MODULE.admit_historical_s149_equivalence_scope([])
        with patch.dict(MODULE.MUTATION_INVENTORIES, {'synthetic': {}}), self.assertRaises(MODULE.Incomplete):
            MODULE.admit_historical_s149_equivalence_scope([])

    def test_incomplete_duplicate_unknown_and_malformed_sets_are_rejected(self):
        good = self.records()
        for bad in (None, False, {}, (), good[:-1], good + [good[0]], good[:-1] + [good[0]]):
            with self.subTest(value=type(bad)), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_historical_s149_equivalence_scope(bad)
        for field in tuple(good[0]):
            bad = copy.deepcopy(good); del bad[0][field]
            with self.subTest(missing=field), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_historical_s149_equivalence_scope(bad)
        bad = copy.deepcopy(good); bad[0]['extra'] = True
        with self.assertRaises(MODULE.Incomplete):
            MODULE.admit_historical_s149_equivalence_scope(bad)

    def test_every_descriptor_field_and_full_source_span_is_bound(self):
        mutations = (
            (('mutant', 'name'), 'unreviewed'), (('mutant', 'file'), 'crates/time-core/src/regression/runs.rs'),
            (('mutant', 'package'), 'other'), (('mutant', 'genre'), 'UnaryOperator'),
            (('mutant', 'replacement'), '||'), (('mutant', 'span', 'start', 'line'), 353),
            (('mutant', 'span', 'end', 'column'), 12),
            (('mutant', 'function', 'function_name'), 'other'),
            (('mutant', 'function', 'return_type'), '-> ()'),
            (('mutant', 'function', 'span', 'start', 'line'), 299),
            (('mutant', 'function', 'span', 'end', 'line'), 379),
        )
        for path, value in mutations:
            bad = self.records(); owner = bad[0]
            for key in path[:-1]: owner = owner[key]
            owner[path[-1]] = value
            with self.subTest(path=path), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_historical_s149_equivalence_scope(bad)
        bad = self.records(); bad[0]['mutant'], bad[1]['mutant'] = bad[1]['mutant'], bad[0]['mutant']
        with self.assertRaises(MODULE.Incomplete):
            MODULE.admit_historical_s149_equivalence_scope(bad)

    def test_candidate_proof_and_review_cannot_be_rebound(self):
        mutations = (
            (('id',), 'S149-AH-E18'), (('candidate_sha256',), '0' * 64),
            (('source_sha256',), '0' * 64), (('requirement',), 'unrelated'),
            (('rationale',), ''), (('proof', 'path'), 'tasks/old.md'),
            (('proof', 'sha256'), '0' * 64), (('proof', 'section'), 'S149-AH-E02'),
            (('proof', 'reviewer'), 'unreviewed'), (('proof', 'verdict'), 'pending'),
        )
        for path, value in mutations:
            bad = self.records(); owner = bad[0]
            for key in path[:-1]: owner = owner[key]
            owner[path[-1]] = value
            with self.subTest(path=path), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_historical_s149_equivalence_scope(bad)


class SpentContractAttemptTests(HistoricalS14cTestCase):
    """Synthetic bounded receipts exercise retry custody, never success credit."""

    def fixture(self, directory, change=None):
        """Write a synthetic prior failure with exact independently stated counts."""
        receipt = {
            'status': 'incomplete', 'selected_lane': 'contracts',
            'error': 'Adaptive legal output is incomplete or has extra records.',
            'analysis_admission_after': {'unchanged': True},
            'lanes': [
                {'lane': 'contract-library', 'exit_code': 0, 'failure': None},
                {'lane': 'external-consumer-old-legal', 'exit_code': 0, 'failure': None},
                {'lane': 'external-consumer-adaptive-legal', 'exit_code': 0, 'failure': None},
                {'lane': 'external-legal-execution', 'exit_code': 0, 'failure': None},
                {'lane': 'external-adaptive-legal-execution', 'exit_code': 0, 'failure': None},
                *({'lane': f'external-consumer-negative-{index}', 'exit_code': 1, 'failure': None}
                  for index in range(24)),
            ],
        }
        if change is not None:
            change(receipt)
        path = directory / 'prior.json'
        path.write_text(json.dumps(receipt))
        prior = {**MODULE.HISTORICAL_S148_COORDINATOR_COHORTS['spent_contract_attempt'],
                 'receipt': str(path), 'bytes': path.stat().st_size,
                 'sha256': hashlib.sha256(path.read_bytes()).hexdigest()}
        return path, prior

    def test_exact_spent_receipt_remains_fully_reserved_without_success_credit(self):
        with tempfile.TemporaryDirectory() as temporary:
            _, prior = self.fixture(Path(temporary))
            with patch.dict(MODULE.HISTORICAL_S148_COORDINATOR_COHORTS, {'spent_contract_attempt': prior}):
                admitted = MODULE.admit_spent_contract_attempt()
            self.assertEqual(admitted, prior)
            self.assertEqual(admitted['reservation'], 'one-full-contracts-vector')
            self.assertIs(admitted['success_credit'], False)
        self.assertEqual(MODULE.HISTORICAL_S148_COORDINATOR_COHORTS['checkpoint_check_totals'],
                         {'coverage': 1, 'contracts': 2})
        self.assertEqual(MODULE.HISTORICAL_S148_COORDINATOR_COHORTS['remaining_contract_attempts'], 1)
        self.assertEqual(MODULE.HISTORICAL_S148_COORDINATOR_COHORTS['contract_attempt_limits'],
                         {'legal_executions': 4, 'negative_compilations': 64})
        self.assertIs(MODULE.HISTORICAL_S148_COORDINATOR_COHORTS['duplicate_mutation_receipts_or_retries_admitted'], False)

    def test_missing_changed_or_linked_spent_receipt_rejects(self):
        for change in ('missing', 'same_size_drift', 'linked', 'changed_during_read'):
            with self.subTest(change=change), tempfile.TemporaryDirectory() as temporary:
                directory = Path(temporary)
                path, prior = self.fixture(directory)
                if change == 'missing':
                    path.unlink()
                elif change == 'same_size_drift':
                    path.write_bytes(path.read_bytes().replace(b'incomplete', b'incompletX', 1))
                elif change == 'linked':
                    actual = directory / 'actual.json'
                    path.rename(actual)
                    path.symlink_to(actual)
                with patch.dict(MODULE.HISTORICAL_S148_COORDINATOR_COHORTS, {'spent_contract_attempt': prior}):
                    if change == 'changed_during_read':
                        reader = MODULE.native_receipt_document
                        def change_after_read(document):
                            value = reader(document)
                            document.write_text(document.read_text() + ' ')
                            return value
                        with patch.object(MODULE, 'native_receipt_document', side_effect=change_after_read), \
                                self.assertRaises(MODULE.Incomplete):
                            MODULE.admit_spent_contract_attempt()
                    else:
                        with self.assertRaises(MODULE.Incomplete):
                            MODULE.admit_spent_contract_attempt()

    def test_matching_digest_cannot_promote_success_or_wrong_native_counts(self):
        changes = (
            lambda row: row.update(status='passed'),
            lambda row: row.update(selected_lane='coverage'),
            lambda row: row.update(error='different failure'),
            lambda row: row['analysis_admission_after'].update(unchanged=False),
            lambda row: row['lanes'][3].update(exit_code=1),
            lambda row: row['lanes'][3].update(exit_code=False),
            lambda row: row['lanes'][4].update(lane='external-legal-execution'),
            lambda row: row['lanes'][-1].update(exit_code=0),
            lambda row: row['lanes'].pop(),
        )
        for index, change in enumerate(changes):
            with self.subTest(index=index), tempfile.TemporaryDirectory() as temporary:
                _, prior = self.fixture(Path(temporary), change)
                with patch.dict(MODULE.HISTORICAL_S148_COORDINATOR_COHORTS, {'spent_contract_attempt': prior}), \
                        self.assertRaises(MODULE.Incomplete):
                    MODULE.admit_spent_contract_attempt()

    def test_current_lanes_require_exact_fresh_outputs_before_session(self):
        class ReachedSession(Exception):
            """Stop at the real receiving boundary without native execution."""

        for lane in ('coverage', 'contracts'):
            with self.subTest(lane=lane), tempfile.TemporaryDirectory() as temporary:
                base = Path(temporary)
                output = base / lane
                with patch.dict(MODULE.COORDINATOR_COHORTS, {'nonmutation_outputs': {lane: str(output)}}), \
                        patch.object(MODULE, 'read_policy', return_value={}), \
                        patch.object(MODULE, 'prospective_scope_reservation', return_value={}), \
                        patch.object(MODULE.COMMON, 'Session', side_effect=ReachedSession) as session:
                    for wrong in (base / 'another-attempt', base):
                        with self.subTest(output=wrong), self.assertRaises(MODULE.Incomplete):
                            MODULE.execute(SimpleNamespace(lane=lane, subject='all', output_dir=wrong), {})
                    session.assert_not_called()
                    report = {}
                    with self.assertRaises(ReachedSession):
                        MODULE.execute(SimpleNamespace(lane=lane, subject='all', output_dir=output), report)
                    session.assert_called_once()
                    self.assertNotIn('core_spent_contract_attempt', report)
                    self.assertEqual(report['core_checkpoint_reservation_scope'],
                                     MODULE.COORDINATOR_COHORTS['reservation_scope'])
                    session.reset_mock()
                    output.mkdir()
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.execute(SimpleNamespace(lane=lane, subject='all', output_dir=output), {})
                    session.assert_not_called()


class LifecycleCheckpointTests(HistoricalS14cTestCase):
    """Independent lifecycle wrapper obligations, without native/tool execution."""

    FIELDS = ('histories', 'appends', 'observations', 'contexts', 'monotonic',
              'coordinates', 'exchanges', 'getters', 'terms', 'projections', 'enclosures',
              'batches', 'minima', 'principal_parameters', 'strata', 'combination_parameters',
              'seeds', 'reachability_updates', 'reachability_records', 'advances',
              'uncertainties', 'equalities')

    def setUp(self):
        fund_synthetic_source_recipes(self)

    def test_package_reservation_charges_the_private_producer_once(self):
        reserved = MODULE.lifecycle_reservation()
        self.assertEqual(reserved['lifecycle_reserved_calls']['appends'], 232)
        self.assertEqual(reserved['lifecycle_reserved_calls']['advances'], 167)
        self.assertEqual(reserved['lifecycle_reserved_bridge_calls'],
                         {'appends': 6, 'admissions': 5, 'masks': 5, 'garde': 4})
        self.assertEqual(reserved['lifecycle_reserved_counter_calls'], {
            'attempts': 4, 'source_constructions': 24, 'source_predicates': 24, 'garde': 24,
            'fixture_setups': 1, 'outcome_predicates': 34, 'estimate_predicates': 18, 'outcome_garde': 34})
        work = reserved['lifecycle_reserved_source_work_uppers']
        self.assertEqual(work['charged_fit_work_upper'], 250)
        self.assertEqual(work['point_admissions_upper'], 16000)
        self.assertEqual(work['lifecycle_compatibility_checked_binary_calls_upper'], 11098)
        self.assertEqual(work['lifecycle_source_counter_constructions_upper'], 9330)
        self.assertEqual(work['uncertainty_constructor_attempts_upper'], 218)

    def test_private_guard_setup_is_charged_once_separately_from_raw_probe_counts(self):
        """Independent raw-prefix/setup expectations; no native outcomes are implied."""
        public = dict.fromkeys(self.FIELDS, 0)
        bridge = {'appends': 0, 'admissions': 0, 'masks': 0, 'garde': 0}
        prefix = {'attempts': 4, 'source_constructions': 24, 'source_predicates': 24,
                  'garde': 24, 'fixture_setups': 0, 'outcome_predicates': 0,
                  'estimate_predicates': 0, 'outcome_garde': 0}
        before_setup = MODULE.lifecycle_source_work(public, bridge, prefix)
        self.assertEqual((before_setup['charged_fit_work_upper'],
                          before_setup['point_admissions_upper']), (0, 0))
        self.assertEqual(before_setup['lifecycle_source_counter_constructions_upper'], 24)
        self.assertEqual(before_setup['lifecycle_source_counter_predicate_calls_upper'], 96)
        self.assertEqual(before_setup['lifecycle_garde_validate_calls_upper'], 48)
        setup = {**prefix, 'fixture_setups': 1}
        complete = {**setup, 'outcome_predicates': 34, 'estimate_predicates': 18,
                    'outcome_garde': 34}
        for counter in (setup, complete):
            with self.subTest(counter=counter):
                work = MODULE.lifecycle_source_work(public, bridge, counter)
                # One fixed genuine four-history setup has twelve appends;
                # the many postcondition probes never create another producer.
                self.assertEqual((work['charged_fit_work_upper'],
                                  work['point_admissions_upper']), (12, 768))
                self.assertEqual(work['lifecycle_source_counter_constructions_upper'], 776)
                self.assertEqual(work['lifecycle_compatibility_checked_binary_calls_upper'], 744)
        completed = MODULE.lifecycle_source_work(public, bridge, complete)
        self.assertEqual(completed['lifecycle_output_predicate_calls_upper'], 92)
        observed_bridge = {'appends': 6, 'admissions': 5, 'masks': 5, 'garde': 4}
        for declared in (False, True):
            with self.subTest(private_bridge=declared):
                both = MODULE.lifecycle_source_work(
                    public, observed_bridge, complete, private_bridge=declared)
                # Six private-bridge appends and twelve guard-setup appends;
                # declaring an already observed bridge must not debit it twice.
                self.assertEqual((both['charged_fit_work_upper'],
                                  both['point_admissions_upper']), (18, 1152))
                self.assertEqual(both['uncertainty_constructor_attempts_upper'], 19)

    def test_empty_observation_is_not_a_complete_lifecycle_package(self):
        legacy = 'CORE_PROFILE_COUNTS fits=1 points=3\n'
        observed = MODULE.profile_counts(legacy)
        self.assertEqual(observed['lifecycle_counter_records'], 0)
        self.assertTrue(observed['lifecycle_observation_pending'])
        self.assertIsNone(observed['lifecycle_observed_calls'])
        with self.assertRaises(MODULE.Incomplete):
            MODULE.profile_counts(legacy, require_lifecycle=True)

    def test_successful_baseline_requires_public_bridge_and_counter_families(self):
        helper = CoreAnalysisTests()
        complete = helper.completed_mutation_log(helper.complete_runs_rows()).replace(
            '96 passed', '281 passed')
        outcome = {'scenario': 'Baseline', 'summary': 'Success', 'log_path': 'baseline.log',
                   'phase_results': helper.phase_results('Success')}
        families = ('CORE_COMBINATION_LIFECYCLE_PROFILE ',
                    'CORE_LIFECYCLE_BRIDGE_PROFILE ', 'CORE_LIFECYCLE_COUNTER_PROFILE ')
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            for missing in (None, *families):
                text = complete if missing is None else ''.join(
                    line for line in complete.splitlines(keepends=True) if not line.startswith(missing))
                (root / 'baseline.log').write_text(text)
                report = {}
                MODULE.record_counts(report, 'lifecycle:baseline', {
                    **MODULE.package_charged_upper(include_doctests=True),
                    **MODULE.selection_reservation(package=True, include_doctests=True),
                    'literal_doctests_reserved': True, 'observation_pending': True})
                reserved = report['core_invocations'][0]
                with self.subTest(missing=missing):
                    if missing is None:
                        MODULE.observe_mutation_outcome(report, root / 'outcomes.json',
                                                        'lifecycle:baseline', outcome, reserved)
                        self.assertFalse(reserved['observation_pending'])
                        self.assertEqual(reserved['observed_counts']['lifecycle_counter_records'], 14)
                    else:
                        with self.assertRaises(MODULE.Incomplete):
                            MODULE.observe_mutation_outcome(report, root / 'outcomes.json',
                                                            'lifecycle:baseline', outcome, reserved)
                        self.assertTrue(reserved['observation_pending'])

    def test_caught_zero_bridge_receiving_retains_partial_and_fixed_producer_upper(self):
        helper = CoreAnalysisTests()
        outcome = {'scenario': {'Mutant': {'name': 'inert_fixture_fault'}},
                   'summary': 'CaughtMutant', 'log_path': 'caught.log',
                   'phase_results': helper.phase_results('CaughtMutant')}
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            for issue in ('OutOfRange', 'NonFinite'):
                text = helper.completed_mutation_log(
                    LIFECYCLE_FIXTURES.zero_bridge_caught_text(issue), include_selection=False)
                (root / 'caught.log').write_text(text)
                report = {}
                MODULE.record_counts(report, 'bridge:caught', {
                    **MODULE.package_charged_upper(include_doctests=True),
                    **MODULE.selection_reservation(package=True, include_doctests=True),
                    'literal_doctests_reserved': True, 'observation_pending': True})
                reserved = report['core_invocations'][0]
                MODULE.observe_mutation_outcome(report, root / 'outcomes.json',
                                                'bridge:caught', outcome, reserved)
                self.assertFalse(reserved['observation_pending'])
                observed = reserved['observed_counts']
                self.assertEqual(observed['lifecycle_observation_status'], 'partial-source-recipe')
                self.assertEqual(observed['lifecycle_bridge_profiles'], {
                    LIFECYCLE_FIXTURES.BRIDGE_CASE: {'appends': 0, 'admissions': 0, 'masks': 0, 'garde': 0}})
                self.assertIsNone(observed['lifecycle_observed_bridge_calls'])
                work = observed['lifecycle_source_owned_work_uppers']
                self.assertEqual((work['charged_fit_work_upper'], work['point_admissions_upper']), (6, 384))
                self.assertEqual(report['core_charged_subset_totals'], {
                    'charged_fit_work_upper': 63129, 'point_admissions_upper': 2253363})
                self.assertEqual(report['mutation_counter_prefix_observations'], ['bridge:caught'])

    def test_caught_zero_event_receiving_preserves_zero_and_reached_fixture(self):
        helper = CoreAnalysisTests()
        outcome = {'scenario': {'Mutant': {'name': 'inert_fixture_fault'}},
                   'summary': 'CaughtMutant', 'log_path': 'caught.log',
                   'phase_results': helper.phase_results('CaughtMutant')}
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            for bridge in ('', LIFECYCLE_FIXTURES.zero_bridge_caught_text('OutOfRange')):
                text = helper.completed_mutation_log(
                    ADAPTIVE_FIXTURES.RAW_EVENT_ZERO + bridge, include_selection=False)
                (root / 'caught.log').write_text(text)
                report = {}
                MODULE.record_counts(report, 'event:caught', {
                    **MODULE.package_charged_upper(include_doctests=True),
                    **MODULE.selection_reservation(package=True, include_doctests=True),
                    'literal_doctests_reserved': True, 'observation_pending': True})
                reserved = report['core_invocations'][0]
                MODULE.observe_mutation_outcome(report, root / 'outcomes.json',
                                                'event:caught', outcome, reserved)
                self.assertFalse(reserved['observation_pending'])
                observed = reserved['observed_counts']
                self.assertEqual(observed['adaptive_recorded_calls']['raw_event_validations'], 0)
                self.assertIsNone(observed['adaptive_observed_calls'])
                self.assertIn('lifecycle_raw_event_validation', observed['adaptive_partial_cases'])
                work = observed['adaptive_recorded_source_work_upper']
                self.assertEqual(work['raw_lifecycle_validations_upper'], 0)
                self.assertEqual(work['raw_event_fixture_entries_upper'], 1)
                self.assertEqual(work['raw_event_update_constructions_upper'], 9)
                self.assertEqual(report['core_charged_subset_totals'], {
                    'charged_fit_work_upper': 63129, 'point_admissions_upper': 2253363})
                self.assertEqual(report['mutation_counter_prefix_observations'], ['event:caught'])
                if bridge:
                    self.assertEqual(observed['lifecycle_observation_status'], 'partial-source-recipe')
                    self.assertIsNone(observed['lifecycle_observed_bridge_calls'])

    def test_zero_event_cannot_activate_successful_or_compile_only_receiving(self):
        helper = CoreAnalysisTests()
        healthy = helper.completed_mutation_log(helper.complete_runs_rows()).replace('96 passed', '281 passed')
        zero = ('CORE_LIFECYCLE_RAW_VALIDATION_PROFILE '
                'case=lifecycle_raw_event_validation validations=0\n')
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            for summary in ('Success', 'MissedMutant', 'Unviable'):
                outcome = {'scenario': 'Baseline' if summary == 'Success' else {'Mutant': {'name': 'fixture'}},
                           'summary': summary, 'log_path': 'outcome.log',
                           'phase_results': helper.phase_results(summary)}
                text = (helper.semantic_mutation_log().replace(helper.mutation_phase_header('Build'),
                        helper.mutation_phase_header('Build') + zero, 1) if summary == 'Unviable'
                        else healthy.replace(ADAPTIVE_FIXTURES.ROWS[-1] + '\n', zero))
                (root / 'outcome.log').write_text(text)
                reserved = {'observation_pending': True}
                with self.subTest(summary=summary), \
                     patch.object(MODULE.ADAPTIVE, 'caught_raw_event_zero_prefix',
                                  wraps=MODULE.ADAPTIVE.caught_raw_event_zero_prefix) as context:
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.observe_mutation_outcome({}, root / 'outcomes.json',
                                                        'event:closed', outcome, reserved)
                    context.assert_not_called()
                    self.assertTrue(reserved['observation_pending'])
                    self.assertNotIn('observed_counts', reserved)

    def test_zero_bridge_cannot_activate_successful_or_compile_only_receiving(self):
        helper = CoreAnalysisTests()
        healthy = helper.completed_mutation_log(helper.complete_runs_rows()).replace('96 passed', '281 passed')
        zero = ('CORE_LIFECYCLE_BRIDGE_PROFILE case=' + LIFECYCLE_FIXTURES.BRIDGE_CASE
                + ' appends=0 admissions=0 masks=0 garde=0\n')
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            for summary in ('Success', 'MissedMutant', 'Unviable'):
                outcome = {'scenario': 'Baseline' if summary == 'Success' else {'Mutant': {'name': 'fixture'}},
                           'summary': summary, 'log_path': 'outcome.log',
                           'phase_results': helper.phase_results(summary)}
                text = (helper.semantic_mutation_log().replace(helper.mutation_phase_header('Build'),
                        helper.mutation_phase_header('Build') + zero, 1) if summary == 'Unviable'
                        else healthy.replace(LIFECYCLE_FIXTURES.BRIDGE_ROW, zero))
                (root / 'outcome.log').write_text(text)
                reserved = {'observation_pending': True}
                with self.subTest(summary=summary), \
                     patch.object(MODULE.COMBINATION, 'caught_bridge_zero_prefix',
                                  wraps=MODULE.COMBINATION.caught_bridge_zero_prefix) as context:
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.observe_mutation_outcome({}, root / 'outcomes.json',
                                                        'bridge:closed', outcome, reserved)
                    context.assert_not_called()
                    self.assertTrue(reserved['observation_pending'])
                    self.assertNotIn('observed_counts', reserved)

    def test_package_scope_cannot_omit_a_lifecycle_reservation(self):
        reserved = MODULE.selection_reservation(package=True)
        del reserved['lifecycle_reserved_counter_calls']
        with self.assertRaisesRegex(MODULE.Incomplete, 'Lifecycle full-package reservation'):
            MODULE.record_counts({}, 'missing-lifecycle',
                                 {**MODULE.package_charged_upper(), **reserved})

    def test_lifecycle_budget_vector_requires_every_closed_source_unit(self):
        required = MODULE.lifecycle_budget_vector()
        self.assertEqual(MODULE.admit_lifecycle_allowances(required), required)
        key = 'lifecycle.work.lifecycle_compatibility_checked_binary_calls_upper'
        self.assertIn(key, required)
        for invalid in (None, {}, {k:v for k,v in required.items() if k != key},
                        {**required, key: required[key] - 1}, {**required, key: True},
                        {**required, key: 1 << 64}, {**required, 'lifecycle.extra': 0}):
            with self.subTest(value=type(invalid)), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_lifecycle_allowances(invalid)

    def test_legal_consumer_reuses_fits_and_keeps_private_fixture_out(self):
        calls = MODULE.lifecycle_legal_calls()
        self.assertEqual(calls['getters'], 24)
        self.assertEqual(sum(calls.values()), 29)
        work = MODULE.lifecycle_legal_source_work()
        self.assertEqual(work['charged_fit_work_upper'], 0)
        self.assertEqual(work['point_admissions_upper'], 0)
        self.assertEqual(work['lifecycle_compatibility_checked_binary_calls_upper'], 62)
        self.assertEqual(work['uncertainty_constructor_attempts_upper'], 0)

    def test_timeout_retains_structured_lifecycle_prefix_without_qualification(self):
        prefix = ('CORE_COMBINATION_LIFECYCLE_PROFILE '
                  'case=lifecycle_strict_boundaries_and_original_principal'
                  + ''.join(' ' + name + '=' + str(value)
                            for name, value in zip(self.FIELDS, (1,) + (0,) * 21, strict=True))
                  + '\n')
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            (root / 'timeout.log').write_text(CoreAnalysisTests.mutation_phase_header('Test') + prefix)
            reserved = {'observation_pending': True,
                        **MODULE.package_charged_upper(include_doctests=True)}
            before = dict(reserved)
            MODULE.observe_mutation_outcome({}, root / 'outcomes.json', 'lifecycle:timeout',
                                            {'summary': 'Timeout', 'log_path': 'timeout.log'}, reserved)
            recorded = reserved['counter_prefix_counts']
            self.assertEqual(recorded['lifecycle_recorded_call_subtotals']['histories'], 1)
            self.assertIsNone(recorded['lifecycle_observed_calls'])
            self.assertTrue(recorded['lifecycle_observation_pending'])
            self.assertTrue(reserved['observation_pending'])
            self.assertEqual(reserved['observation_status'], 'timeout-prefix-unqualified')
            self.assertNotIn('observed_counts', reserved)
            for name, value in before.items():
                self.assertEqual(reserved[name], value)


class CombinationCheckpointTests(HistoricalS14cTestCase):
    """Independent current combination transport/accounting controls, never Rust runs."""

    FIELDS = ('histories', 'appends', 'observations', 'contexts', 'monotonic',
              'coordinates', 'exchanges', 'getters', 'terms', 'projections', 'enclosures',
              'batches', 'minima', 'parameters', 'strata', 'seeds', 'advances',
              'state_clones', 'value_equalities', 'state_formats', 'uncertainties', 'combinations')
    RECIPES = (
        ('combination_distinct_weights_and_principal_disagreement', (2,6,6,2,7,24,6,45,2,2,2,1,1,1,2,1,1,0,0,2,2,1)),
        ('combination_unavailable_and_checked_uncertainty', (1,3,3,1,4,12,3,22,1,1,1,1,1,1,1,1,1,0,0,3,1,2)),
        ('combination_singleton_signed_zero_and_retained', (1,3,3,1,4,12,3,67,1,1,1,1,1,1,1,1,2,0,0,0,1,2)),
        ('combination_raw_uncertainty_boundaries', (1,3,3,1,4,12,3,326,1,1,1,1,1,1,1,1,1,0,0,0,40,15)),
        ('combination_three_source_permutation_and_zero_phase_sd', (3,9,9,3,10,36,9,73,3,3,3,1,1,1,3,1,1,0,0,0,3,2)),
        ('combination_metadata_association_and_context', (6,18,18,6,22,72,18,76,6,6,6,2,2,2,6,2,2,0,0,0,7,10)),
        ('combination_zero_denominators_and_phase_neighbor', (4,12,12,4,14,48,12,55,4,4,4,2,2,2,4,2,2,0,0,0,8,6)),
        ('combination_finite_square_failures_and_neighbors', (2,6,6,2,7,24,6,31,2,2,2,1,1,1,2,1,1,0,0,0,13,12)),
        ('combination_capacity_and_weight_sum_overflow', (16,48,48,16,49,192,48,148,16,16,16,1,1,1,16,1,1,0,0,0,48,4)),
        ('combination_variance_mean_underflow_and_neighbor', (2,6,6,2,7,24,6,36,2,2,2,1,1,1,2,1,1,0,0,0,3,2)),
        ('combination_weighted_rate_subnormal_and_neighbor', (2,6,6,2,7,24,6,37,2,2,2,1,1,1,2,1,1,0,0,0,3,2)),
        ('combination_canonical_reduction_rounding_order', (3,9,9,3,10,36,9,61,3,3,3,1,1,1,3,1,1,0,0,0,3,2)),
    )

    def setUp(self):
        fund_synthetic_source_recipes(self)
        # Only these new global fields are pending current discovery. Inert
        # observer tests fund their finite source recipes explicitly; production
        # policy/campaign admission is neither called nor patched into success.
        limits = patch.dict(MODULE.SOURCE_RAW_PRIVATE_GLOBAL_LIMITS,
                            {'square_root_calls_upper': 33000, 'binary_arithmetic_calls_upper': 22000})
        limits.start(); self.addCleanup(limits.stop)

    @classmethod
    def row(cls, case, values):
        return 'CORE_COMBINATION_PROFILE case=' + case + ''.join(
            f' {field}={value}' for field, value in zip(cls.FIELDS, values, strict=True)) + '\n'

    @classmethod
    def complete_rows(cls):
        return ''.join(cls.row(case, values) for case, values in cls.RECIPES)

    def test_exact_complete_combination_profile_charges_its_disjoint_producers_once(self):
        counts = MODULE.profile_counts(self.complete_rows(), require_combination=True)
        actual = counts['combination_observed_calls']
        self.assertEqual(tuple(actual[name] for name in self.FIELDS),
                         (43,129,129,43,145,516,129,977,43,43,43,14,14,14,43,14,15,0,0,5,132,60))
        self.assertEqual(counts['combination_counter_records'], 12)
        self.assertFalse(counts['combination_observation_pending'])
        self.assertEqual((counts['charged_fit_work_upper'], counts['point_admissions_upper']), (129,8256))
        self.assertEqual((counts['direct_fit_calls'], counts['stats_calls']), (0,0))
        work = counts['combination_source_owned_work_uppers']
        self.assertEqual(work['uncertainty_constructor_attempts_upper'], 132)
        self.assertEqual(work['kernel_checked_square_root_calls_upper'], 180)
        self.assertEqual(work['kernel_checked_binary_calls_upper'], 17580)
        producer = MODULE.principal_source_work({name: actual[name] for name in self.FIELDS[:20]})
        for key, value in producer.items():
            self.assertEqual(work[key], value)
        self.assertIsNone(counts['principal_observed_calls'])

    def test_combination_is_not_ignored_when_a_healthy_legacy_stream_is_present(self):
        legacy = 'CORE_PROFILE_COUNTS fits=1 points=3\n'
        counts = MODULE.profile_counts(legacy + self.row(*self.RECIPES[0]))
        self.assertEqual((counts['charged_fit_work_upper'], counts['point_admissions_upper']), (7,387))
        self.assertEqual(counts['combination_recorded_call_subtotals']['uncertainties'], 2)
        self.assertIsNone(counts['combination_observed_calls'])
        self.assertEqual(len(counts['combination_missing_cases']), 11)
        self.assertTrue(counts['combination_observation_pending'])
        missing = MODULE.profile_counts(legacy)
        self.assertIsNone(missing['combination_recorded_call_subtotals'])
        self.assertIsNone(missing['combination_source_owned_work_uppers'])
        with self.assertRaises(MODULE.Incomplete):
            MODULE.profile_counts(legacy, require_combination=True)

    def test_partial_prefix_keeps_full_reservation_and_missing_remains_none(self):
        report = {}
        reservation = MODULE.selection_reservation(package=True)
        MODULE.record_counts(report, 'bounded-package', {
            **MODULE.package_charged_upper(), **reservation, 'observation_pending': True})
        self.assertIsNone(report['core_combination_recorded_call_subtotals'])
        case, values = self.RECIPES[0]
        partial = tuple(value if index < 20 else 0 for index, value in enumerate(values))
        observed = MODULE.profile_counts(self.row(case, partial))
        row = report['core_invocations'][0]
        row.update(observed_counts=observed, observation_pending=False)
        MODULE.refresh_accounting(report)
        self.assertEqual(report['core_combination_recorded_call_subtotals']['uncertainties'], 0)
        self.assertIsNone(report['core_combination_complete_observed_call_subtotals'])
        self.assertEqual(report['core_combination_pending_observation_invocations'], ['bounded-package'])
        self.assertEqual(report['core_combination_reserved_totals']['combination.call.uncertainties'], 132)
        self.assertEqual(row['combination_reserved_source_work_uppers']['charged_fit_work_upper'], 129)

    def test_unknown_malformed_duplicate_and_overflow_combination_markers_fail_closed(self):
        good = self.row(*self.RECIPES[0])
        invalid = [good.rstrip('\n'), good + good,
                   good.replace('CORE_COMBINATION_PROFILE', 'CORE_COMBINATION_UNRECOGNIZED'),
                   good.replace(self.RECIPES[0][0], 'combination_unknown'),
                   good.replace(' uncertainties=2', ' unknown=2'),
                   good.replace(' combinations=1', ' combinations=1 extra=0'),
                   good.replace(' combinations=1', ''),
                   good.replace(' uncertainties=2', ' uncertainties=18446744073709551616'),
                   good.replace(' uncertainties=2', ' uncertainties=3')]
        for bad in invalid:
            with self.subTest(record=bad[:90]), self.assertRaises(MODULE.Incomplete):
                MODULE.profile_counts('CORE_PROFILE_COUNTS fits=1 points=3\n' + bad)

    def test_zero_work_prefix_cannot_supply_a_successful_execution(self):
        zero = self.row(self.RECIPES[0][0], (0,) * 22)
        for require_fit in (False, True):
            with self.subTest(require_fit=require_fit), self.assertRaisesRegex(
                    MODULE.Incomplete, '^Profile case is unknown, duplicated or exceeds its source ceiling\\.$'):
                MODULE.profile_counts(zero, require_fit=require_fit)

    def test_constructor_and_kernel_units_cannot_be_omitted_or_double_charge_producers(self):
        calls = dict.fromkeys(self.FIELDS, 0)
        calls.update(uncertainties=1, combinations=1)
        work = MODULE.combination_source_work(calls)
        self.assertEqual(work['charged_fit_work_upper'], 0)
        self.assertEqual(work['point_admissions_upper'], 0)
        self.assertEqual(work['uncertainty_garde_rule_callbacks_upper'], 3)
        self.assertEqual(work['kernel_checked_binary_calls_upper'], 293)
        for name in ('uncertainties', 'combinations'):
            malformed = dict(calls); del malformed[name]
            with self.subTest(missing=name), self.assertRaises(MODULE.Incomplete):
                MODULE.combination_source_work(malformed)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.combination_source_work({**calls, 'combinations': (1 << 64) - 1})
        with patch.object(MODULE.COMBINATION, 'source_work_uppers', return_value={'charged_fit_work_upper':0}), \
                self.assertRaisesRegex(MODULE.Incomplete, 'units overlap'):
            MODULE.combination_source_work(calls)
        bad = MODULE.selection_reservation(package=True)
        del bad['combination_reserved_source_work_uppers']['uncertainty_constructor_attempts_upper']
        with self.assertRaisesRegex(MODULE.Incomplete, 'Combination full-package reservation'):
            MODULE.record_counts({}, 'missing-unit', {**MODULE.package_charged_upper(), **bad})

    def test_complete_cumulative_allowances_are_typed_and_cannot_drop_a_new_unit(self):
        required = MODULE.combination_budget_vector()
        self.assertEqual(MODULE.admit_combination_allowances(required), required)
        key = 'combination.work.kernel_checked_square_root_calls_upper'
        for malformed in (None, {}, {k:v for k,v in required.items() if k != key},
                          {**required, key: required[key] - 1}, {**required, key: True},
                          {**required, key: 1 << 64}, {**required, 'combination.extra': 0}):
            with self.subTest(limits=type(malformed)), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_combination_allowances(malformed)

    def test_current_checkpoint_counts_inventory_and_equivalences_stay_explicit(self):
        self.assertEqual(MODULE.SUBJECTS, S14C_SUBJECTS)
        self.assertEqual(len(MODULE.COMPLETE_CRITICAL_SUBJECTS), 24)
        self.assertEqual((MODULE.SOURCE_FILE_COUNT, len(MODULE.BOUND_INPUTS)), (156,156))
        self.assertEqual(MODULE.FINAL_PACKAGE_TEST_COUNTS, {'ordinary':258,'doctests':23,'baseline':281})
        self.assertEqual(MODULE.HISTORICAL_S151_TEST_COUNTS, {'ordinary':208,'doctests':9,'baseline':217})
        self.assertEqual(MODULE.INVENTORY_FIXED['schema_version'], 16)
        self.assertFalse(MODULE.INVENTORY_FIXED['campaigns_admitted'])
        self.assertEqual(MODULE.SOURCE_RAW_PRIVATE_WORK_LIMITS['square_root_calls_upper'], 33)
        self.assertEqual(MODULE.SOURCE_RAW_PRIVATE_WORK_LIMITS['binary_arithmetic_calls_upper'], 22)
        with patch.dict(MODULE.MUTATION_INVENTORIES, {}, clear=True):
            MODULE.admit_core_equivalence_scope([])
        with self.assertRaises(MODULE.Incomplete):
            MODULE.admit_core_equivalence_scope(CoreAnalysisTests.repaired_selection_equivalence_scope())
        with patch.dict(MODULE.MUTATION_INVENTORIES, {}, clear=True), self.assertRaisesRegex(MODULE.Incomplete, 'inventory remains pending'):
            MODULE.checkpoint_source_reservation()

    @staticmethod
    def combination_equivalence_scope():
        """Independent literal scope fixtures, not full proofs or native E outcomes."""
        names = (
            ('S13-C-E-001', '641:19: replace || with && in admit_records'),
            ('S13-C-E-002', '641:45: replace || with && in admit_records'),
            ('S13-C-E-003', '1015:26: replace || with && in combine_fixture_supporters'),
        )
        return [
            {'id': identifier,
             'candidate_sha256': '509affa74871944548317221b58b45d878ee2b75d15103cc9bc2bde694be30a1',
             'source_sha256': '67e083d7a0e752566879754e9fb406bde7e1d89243a3866743a9a0fc601db47f',
             'mutant': {'name': 'crates/time-core/src/combination.rs:' + suffix,
                        'package': 'time-core', 'file': 'crates/time-core/src/combination.rs',
                        'replacement': '&&'},
             'proof': {'path': 'tasks/s1.3-lifecycle-equivalences.md'}}
            for identifier, suffix in names
        ] + [{
            'id': 'S13-P-E-001',
            'candidate_sha256': '509affa74871944548317221b58b45d878ee2b75d15103cc9bc2bde694be30a1',
            'source_sha256': 'aa7256364c3006943f614a9eb21d4ef94a0e0f30ecd147439c93d34fa17e7da9',
            'mutant': {'name': 'crates/time-core/src/principal.rs:945:17: replace < with <= in updated_score',
                       'package': 'time-core', 'file': 'crates/time-core/src/principal.rs',
                       'replacement': '<='},
            'proof': {'path': 'tasks/s1.3-lifecycle-equivalences.md'},
        }, {
            'id': 'S13-L-E-001',
            'candidate_sha256': '509affa74871944548317221b58b45d878ee2b75d15103cc9bc2bde694be30a1',
            'source_sha256': '3d1294fedcc5d3a7036b43512ccd8a1d2b52167f8867b820eb6a7a71af35f17d',
            'mutant': {'name': 'crates/time-core/src/combination/lifecycle.rs:1031:13: replace || with && in estimate_relation_valid',
                       'package': 'time-core', 'file': 'crates/time-core/src/combination/lifecycle.rs',
                       'replacement': '&&'},
            'proof': {'path': 'tasks/s1.3-lifecycle-equivalences.md'},
        }] + [
            {'id': identifier,
             'candidate_sha256': '509affa74871944548317221b58b45d878ee2b75d15103cc9bc2bde694be30a1',
             'source_sha256': '9225260bf71cebf50618057984d2e3f8b865830bf48bbde7b0904f17d546c5b4',
             'mutant': {'name': 'crates/time-core/src/combination/admission.rs:' + suffix,
                        'package': 'time-core', 'file': 'crates/time-core/src/combination/admission.rs',
                        'replacement': '&&'},
             'proof': {'path': 'tasks/s1.3-lifecycle-equivalences.md'}}
            for identifier, suffix in (
                ('S13-A-E-001', '104:21: replace || with && in admit_lifecycle_supporters'),
                ('S13-A-E-002', '150:28: replace || with && in contributor_count'),
            )
        ]

    def test_current_equivalence_scope_accepts_empty_or_exact_seven_source_bound_faults(self):
        MODULE.admit_historical_lifecycle_equivalence_scope([])
        good = self.combination_equivalence_scope()
        MODULE.admit_historical_lifecycle_equivalence_scope(good)
        MODULE.admit_historical_lifecycle_equivalence_scope(list(reversed(good)))

    def test_current_equivalence_scope_rejects_incomplete_duplicate_and_malformed_sets(self):
        good = self.combination_equivalence_scope()
        malformed = (None, False, {}, (), good[:1], good[:2], good[:3], good[:4], good[:5], good[:6], good[1:],
                     good + [good[0]], [good[0]] * 7,
                     [None, *good[1:]], [False, *good[1:]], [{}, *good[1:]])
        for value in malformed:
            with self.subTest(value=value), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_historical_lifecycle_equivalence_scope(value)
        for missing in ('id', 'candidate_sha256', 'source_sha256', 'mutant', 'proof'):
            altered = copy.deepcopy(good); del altered[0][missing]
            with self.subTest(missing=missing), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_historical_lifecycle_equivalence_scope(altered)

    def test_current_equivalence_scope_rejects_changed_identity_candidate_source_and_proof(self):
        good = self.combination_equivalence_scope()
        for index in range(7):
            for field, value in (
                ('id', 'S13-C-E-004'), ('id', []), ('id', True),
                ('candidate_sha256', '0' * 64), ('candidate_sha256', None),
                ('candidate_sha256', '83a13d0de0069c0fa7e5a6204cda617bfc725f570b4922cbc391f7dd833b5594'),
                ('candidate_sha256', 'be5734cfc20b1b18569ac3a66c15a51d4120e60ab3eee479d26bd17dd16189b5'),
                ('source_sha256', '0' * 64), ('source_sha256', None),
                ('mutant', None), ('mutant', []), ('proof', None), ('proof', []),
            ):
                altered = copy.deepcopy(good); altered[index][field] = value
                with self.subTest(index=index, field=field, value=value), self.assertRaises(MODULE.Incomplete):
                    MODULE.admit_historical_lifecycle_equivalence_scope(altered)
            for owner, field, value in (
                ('mutant', 'name', 'crates/time-core/src/combination.rs:641:20: replace || with && in admit_records'),
                ('mutant', 'package', 'time-domain'),
                ('mutant', 'file', 'crates/time-core/src/regression.rs'),
                ('mutant', 'replacement', '||'),
                ('proof', 'path', 'tasks/old-combination-equivalences.md'),
            ):
                altered = copy.deepcopy(good); altered[index][owner][field] = value
                with self.subTest(index=index, owner=owner, field=field), self.assertRaises(MODULE.Incomplete):
                    MODULE.admit_historical_lifecycle_equivalence_scope(altered)

    def test_current_equivalence_scope_rejects_cross_owner_digest_and_operator_swaps(self):
        good = self.combination_equivalence_scope()
        for index, other in ((0, 3), (3, 0), (4, 3), (5, 3), (6, 3)):
            for field in ('source_sha256', 'replacement'):
                altered = copy.deepcopy(good)
                if field == 'source_sha256':
                    altered[index][field] = good[other][field]
                else:
                    altered[index]['mutant'][field] = good[other]['mutant'][field]
                with self.subTest(index=index, field=field), self.assertRaises(MODULE.Incomplete):
                    MODULE.admit_historical_lifecycle_equivalence_scope(altered)

    def test_current_equivalence_scope_never_excludes_the_boundary_counterexamples(self):
        for name in (
            'crates/time-core/src/combination.rs:649:9: replace || with && in admit_records',
            'crates/time-core/src/combination.rs:948:19: replace || with && in complete_estimate',
            'crates/time-core/src/combination/lifecycle.rs:1032:13: replace || with && in estimate_relation_valid',
            'crates/time-core/src/combination/admission.rs:160:18: replace >= with < in contributor_count',
            'crates/time-core/src/combination/admission.rs:172:19: replace || with && in contributor_count',
        ):
            for index in range(7):
                altered = self.combination_equivalence_scope()
                altered[index]['mutant']['name'] = name
                with self.subTest(name=name, index=index), self.assertRaises(MODULE.Incomplete):
                    MODULE.admit_historical_lifecycle_equivalence_scope(altered)

    def test_timeout_recognizes_combo_prefix_and_rejects_unrecognized_family(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); path = root / 'outcomes.json'
            log = root / 'timeout.log'
            outcome = {'summary':'Timeout', 'log_path':'timeout.log'}
            for marker, valid in ((self.row(self.RECIPES[0][0], (1,) + (0,) * 21), True),
                                  ('CORE_COMBINATION_UNKNOWN value=1\n', False)):
                log.write_text(CoreAnalysisTests.mutation_phase_header('Test') + marker)
                reserved = {'observation_pending':True, **MODULE.package_charged_upper(include_doctests=True)}
                if valid:
                    MODULE.observe_mutation_outcome({}, path, 'timeout', outcome, reserved)
                    self.assertEqual(reserved['observation_status'], 'timeout-prefix-unqualified')
                    self.assertTrue(reserved['observation_pending'])
                    self.assertNotIn('observed_counts', reserved)
                else:
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.observe_mutation_outcome({}, path, 'timeout', outcome, reserved)

    def test_successful_complete_baseline_requires_every_combination_case(self):
        helper = CoreAnalysisTests()
        history_fields = MODULE.HISTORY_TELEMETRY['observed_fields']
        ordinary = ('CORE_PROFILE_COUNTS fits=1 points=3\n'
                    'CORE_STATS_PROFILE_COUNTS stats=1 points=3\n'
                    + helper.norm_row(normalizations=0, observations=0, contexts=0,
                                      monotonic=0, coordinates=0, exchanges=0, getters=0) + '\n'
                    + 'CORE_HISTORY_PROFILE_COUNTS ' + ' '.join(field + '=0' for field in history_fields) + '\n'
                    + helper.complete_runs_rows()
                    + 'test result: ok. 281 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;\n')
        complete = helper.completed_mutation_log(ordinary)
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); path = root / 'outcomes.json'; log = root / 'baseline.log'
            outcome = {'scenario':'Baseline', 'summary':'Success', 'log_path':'baseline.log',
                       'phase_results':helper.phase_results('Success')}
            for text, failure in ((complete, None),
                                  (complete.replace(self.complete_rows(), ''), 'Combination complete package'),
                                  (complete.replace('281 passed', '0 passed'), 'no observed passing tests|did not execute all')):
                log.write_text(text)
                report = {}
                MODULE.record_counts(report, 'baseline', {
                    **MODULE.package_charged_upper(include_doctests=True),
                    **MODULE.selection_reservation(package=True, include_doctests=True),
                    'literal_doctests_reserved':True, 'observation_pending':True})
                reserved = report['core_invocations'][-1]
                if failure is None:
                    MODULE.observe_mutation_outcome(report, path, 'baseline', outcome, reserved)
                    self.assertFalse(reserved['observation_pending'])
                    self.assertEqual(reserved['observed_counts']['combination_counter_records'], 12)
                    self.assertEqual(report['core_combination_complete_observed_call_subtotals']['combinations'], 60)
                else:
                    with self.assertRaisesRegex(MODULE.Incomplete, failure):
                        MODULE.observe_mutation_outcome(report, path, 'baseline', outcome, reserved)


class CombinationContractTests(HistoricalS14cTestCase):
    def setUp(self):
        fund_synthetic_source_recipes(self)

    """Inert contract transport/ownership controls; actual compiler causes are separate."""

    @staticmethod
    def contract_row():
        # This literal expected delta deliberately has no producer operations.
        values = (0,0,0,0,0,0,0,12,0,0,0,0,0,0,0,0,0,0,0,0,1,1)
        return 'CORE_COMBINATION_CONTRACT_PROFILE' + ''.join(
            f' {field}={value}' for field, value in zip(CombinationCheckpointTests.FIELDS, values, strict=True)) + '\n'

    @staticmethod
    def lifecycle_contract_row():
        values = (0,0,0,0,0,0,0,24,0,0,0,0,0,0,0,1,1,1,1,1,0,0)
        return 'CORE_CONTRACT_LIFECYCLE_PROFILE' + ''.join(
            f' {field}={value}' for field, value in zip(LifecycleCheckpointTests.FIELDS, values, strict=True)) + '\n'

    def test_exact_contract_delta_rejects_missing_partial_mixed_and_package_streams(self):
        good = self.contract_row()
        expected = dict(zip(CombinationCheckpointTests.FIELDS,
                            (0,0,0,0,0,0,0,12,0,0,0,0,0,0,0,0,0,0,0,0,1,1), strict=True))
        for text in (good, 'legal-core-contract\n' + good, good.replace('\n','\r\n')):
            self.assertEqual(MODULE.combination_contract_counts(text), expected)
        malformed = ('', good.rstrip('\n'), good + good, 'prefix ' + good,
                     good.replace(' getters=12', ' getters=11'), good.replace(' combinations=1',''),
                     good.replace(' histories=0', ' histories=00'),
                     good.replace(' uncertainties=1', ' uncertainties=18446744073709551616'),
                     good.replace('PROFILE', 'UNKNOWN'), good.replace('\n',' extra=0\n'),
                     good + CombinationCheckpointTests.complete_rows())
        for text in malformed:
            with self.subTest(text=text[:70]), self.assertRaises(MODULE.Incomplete):
                MODULE.combination_contract_counts(text)
        # A new contract marker is never a substitute for package case evidence.
        with self.assertRaises(MODULE.Incomplete):
            MODULE.profile_counts(good, require_fit=False)

    def test_registry_has_one_legal_runtime_and_twenty_four_exact_negative_compilations(self):
        cases = MODULE.historical_contract_examples()
        expected = {
            'principal_private_parameter_fields': ('E0451','reselect_distance_seconds',3),
            'combination_uncertainty_private_fields': ('E0451','phase_standard_deviation_seconds',3),
            'combination_uncertainty_private_mutation': ('E0616','phase_standard_deviation_seconds',3),
            'combination_estimate_private_fields': ('E0451','phase_seconds',5),
            'combination_estimate_private_mutation': ('E0616','phase_seconds',3),
            'combination_outcome_private_fields': ('E0451','combined',3),
            'combination_outcome_private_mutation': ('E0616','combined',3),
        }
        lifecycle_expected = {
            'lifecycle_parameter_private_fields': ('E0451', {'private','combine_limit'}, 3),
            'lifecycle_parameter_private_mutation': ('E0616', {'private','combine_limit'}, 3),
            'lifecycle_parameter_fallible_constructor': ('E0308', {'Result'}, 3),
            'lifecycle_update_private_fields': ('E0451', {'private','ordinal'}, 4),
            'lifecycle_update_private_mutation': ('E0616', {'private','ordinal'}, 3),
            'lifecycle_update_fallible_constructor': ('E0308', {'Result'}, 4),
            'lifecycle_source_private_fields': ('E0451', {'private','penalty'}, 4),
            'lifecycle_source_private_mutation': ('E0616', {'private','penalty'}, 3),
            'lifecycle_state_private_mutation': ('E0616', {'private','inner'}, 3),
            'lifecycle_state_unchecked_default': ('E0277', {'Default'}, 3),
            'lifecycle_state_nonclone_credit': ('E0599', {'clone'}, 3),
            'lifecycle_principal_read_only': ('E0308', {'mutability'}, 3),
            'lifecycle_source_read_only': ('E0308', {'mutability'}, 5),
            'lifecycle_outcome_private_combined': ('E0616', {'private','combined'}, 3),
            'lifecycle_outcome_private_emission': ('E0616', {'private','emitted_round'}, 3),
            'lifecycle_bridge_private_module': ('E0603', {'private','combination'}, 1),
            'lifecycle_outcome_borrows_batch': ('E0515', {'return','batch'}, 6),
        }
        self.assertEqual(set(cases), {'legal_principal_checked_producer', *expected, *lifecycle_expected})
        self.assertEqual(MODULE.HISTORICAL_EXTERNAL_CONSUMER_LIMITS, {'legal_executions':1,'negative_compilations':24})
        self.assertEqual(MODULE.EXTERNAL_CONSUMER_LIMITS, {'legal_executions':1,'negative_compilations':6})
        historical = MODULE.historical_s158_contract_examples()
        self.assertEqual(sum(row['code'] is None for row in historical.values()), 4)
        self.assertEqual(sum(row['code'] is not None for row in historical.values()), 77)
        self.assertIsNone(cases['legal_principal_checked_producer']['code'])
        for name, (code, field, line) in expected.items():
            case = cases[name]
            self.assertEqual((case['code'], case['line']), (code,line))
            self.assertEqual(set(case['tokens']), {'private',field})
            self.assertIn(field, case['source'].splitlines()[line-1])
            for forbidden in ('try_append', 'FixtureHistory::new', 'combine_fixture_supporters(', 'FixtureSupporterUncertainty::new'):
                self.assertNotIn(forbidden, case['source'])
        for name, (code, tokens, line) in lifecycle_expected.items():
            case = cases[name]
            self.assertEqual((case['code'], set(case['tokens']), case['line']), (code, tokens, line))
            self.assertTrue(case['source'].splitlines()[line-1].strip())

    def test_private_diagnostics_bind_code_field_and_primary_attempt_without_type_name(self):
        for name, specimen in MODULE.combination_contract_examples().items():
            path = '/work/contract-inputs/' + name + '.rs'
            field = next(token for token in specimen['tokens'] if token != 'private')
            diagnostic = synthetic_compiler_diagnostic(
                specimen['code'], field + ' is private', path, specimen['line'])
            result = {'exit_code':1,'failure':None,'output':json.dumps(diagnostic)}
            MODULE.COMMON.compiler_cause(result, specimen, path)
            for changed in ('code','field','line'):
                wrong = copy.deepcopy(diagnostic)
                if changed == 'code': wrong['code']['code'] = 'E0308'
                elif changed == 'field': wrong['message'] = 'another_field is private'
                else:
                    wrong['spans'][0]['line_start'] += 1
                    wrong['spans'][0]['line_end'] += 1
                with self.subTest(case=name, changed=changed), self.assertRaises(MODULE.Incomplete):
                    MODULE.COMMON.compiler_cause({**result,'output':json.dumps(wrong)}, specimen, path)

    def test_legal_fixture_retains_old_producer_and_precharges_exact_singleton_delta(self):
        case = MODULE.historical_s158_contract_examples()['legal_principal_checked_producer']
        self.assertEqual(tuple(case['principal_calls'][field] for field in MODULE.PRINCIPAL_FIELDS),
                         (1,3,3,1,5,1,1,6,1,1,1,1,1,1,1,1,1,0,0,0))
        self.assertEqual(case['combination_calls'], MODULE.combination_contract_counts(self.contract_row()))
        work = MODULE.combination_source_work(case['combination_calls'])
        self.assertEqual((work['charged_fit_work_upper'],work['point_admissions_upper']), (0,0))
        self.assertEqual(work['uncertainty_constructor_attempts_upper'], 1)
        self.assertEqual(work['kernel_checked_binary_calls_upper'], 293)
        self.assertIn('combination_calls.0[$slot]+=1; $value',case['source'])
        self.assertEqual(case['source'].count('combination_call!(7,'), 12)
        self.assertIn('FixtureSupporterUncertainty::new(&entries[0],-0.0,0.0,0.0)',case['source'])
        self.assertIn('phase_standard_deviation_seconds()).to_bits(),0x8000_0000_0000_0000',case['source'])

    def test_external_delta_requires_all_three_recipes_and_accumulates_without_new_fits(self):
        principal = MODULE.principal_legal_calls(); delta = MODULE.combination_legal_calls()
        principal_work = MODULE.principal_source_work(principal)
        delta_work = MODULE.combination_source_work(delta)
        lifecycle = MODULE.lifecycle_contract_counts(self.lifecycle_contract_row())
        lifecycle_work = MODULE.lifecycle_legal_source_work()
        reservation = {**{key:principal_work[key]+delta_work[key]+lifecycle_work[key] for key in MODULE.GLOBAL_LIMITS},
                       'projection_adapter_reserved_budget':MODULE.projection_adapter_budget_vector(external=True),
                       'principal_external_reserved_calls':principal,
                       'principal_external_reserved_source_work_uppers':principal_work,
                       'combination_external_reserved_calls':delta,
                       'combination_external_reserved_source_work_uppers':delta_work,
                       'lifecycle_external_reserved_calls':lifecycle,
                       'lifecycle_external_reserved_source_work_uppers':lifecycle_work,
                       'observation_pending':True}
        for key in ('principal_external_reserved_calls', 'principal_external_reserved_source_work_uppers',
                    'combination_external_reserved_calls', 'combination_external_reserved_source_work_uppers',
                    'lifecycle_external_reserved_calls', 'lifecycle_external_reserved_source_work_uppers'):
            wrong = copy.deepcopy(reservation); del wrong[key]
            with self.subTest(missing=key), self.assertRaises(MODULE.Incomplete):
                MODULE.record_counts({},'missing-recipe',wrong)
        missing_unit = copy.deepcopy(reservation)
        del missing_unit['combination_external_reserved_source_work_uppers']['uncertainty_constructor_attempts_upper']
        with self.assertRaises(MODULE.Incomplete): MODULE.record_counts({},'missing-unit',missing_unit)
        report = {}; MODULE.record_counts(report,'legal-singleton',reservation)
        self.assertEqual(report['core_charged_subset_totals'], {'charged_fit_work_upper':3,'point_admissions_upper':192})
        self.assertIsNone(report['core_combination_recorded_call_subtotals'])
        self.assertEqual(report['core_combination_reserved_totals']['combination.call.getters'],12)
        self.assertEqual(report['core_combination_reserved_totals']['combination.work.charged_fit_work_upper'],0)
        report['core_invocations'][0].update(observation_pending=False, observed_counts={
            'principal_recorded_call_subtotals':principal,'principal_observation_pending':False,
            'combination_recorded_call_subtotals':delta,'combination_observed_calls':delta,
            'combination_observation_pending':False,
            'lifecycle_recorded_call_subtotals':lifecycle,'lifecycle_observed_calls':lifecycle,
            'lifecycle_observation_pending':False})
        MODULE.refresh_accounting(report)
        self.assertEqual(report['core_combination_complete_observed_call_subtotals'],delta)
        self.assertEqual(report['core_combination_pending_observation_invocations'],[])

    def test_contract_prospective_and_cohort_vectors_include_exact_delta(self):
        limits = {key:value*1000 for key,value in MODULE.combination_budget_vector().items()}
        with patch.dict(MODULE.COMBINATION_TELEMETRY, {'global_limits':limits}):
            reserved = MODULE.prospective_scope_reservation('contracts',dict(MODULE.SUBJECTS))
            _, checks, _ = MODULE.cohort_budget_vectors(require_limits=False)
        self.assertEqual(reserved['negative_consumer_compilations_upper'],6)
        self.assertEqual(reserved['legal_consumer_executions_upper'],1)
        self.assertEqual(reserved['charged_upper'], {'charged_fit_work_upper':685,'point_admissions_upper':23679})
        expected = {**{'combination.call.'+key:value for key,value in MODULE.combination_legal_calls().items()},
                    **{'combination.work.'+key:value for key,value in MODULE.combination_source_work(MODULE.combination_legal_calls()).items()}}
        self.assertEqual(reserved['combination_source_work_upper'],expected)
        self.assertEqual({key:value for key,value in checks['contracts'].items() if key.startswith('combination.')},expected)
        self.assertEqual(checks['contracts']['charged_fit_work_upper'],685)
        self.assertEqual(checks['contracts']['point_admissions_upper'],23679)

    def test_contract_orchestrator_reserves_before_one_run_and_requires_all_three_marker_families(self):
        # Preserve this old three-marker orchestration contract in its reviewed
        # four-caller/77-negative context; the current reference registry has
        # independent receiving and precharge controls below.
        current_vector = MODULE.chart_combination_budget_vector

        def historical_vector(*, include_doctests=False, external_legal=False, prior_budget=None):
            if not external_legal:
                return current_vector(include_doctests=include_doctests,
                                      external_legal=False, prior_budget=prior_budget)
            if prior_budget is None:
                _, checks, _ = MODULE.historical_s157_cohort_budget_vectors(require_limits=False)
                prior_budget = checks['contracts']
            owner = MODULE.CHART_COMBINATION.ContractOwner(
                fields=tuple(MODULE.CHART_COMBINATION_CONTRACTS.FIELDS),
                expected=MODULE.CHART_COMBINATION_CONTRACTS.legal_expected,
                source=MODULE.CHART_COMBINATION_CONTRACTS.legal_contract_source)
            return MODULE.CHART_COMBINATION.external_legal_reservation(
                MODULE.chart_combination_work_owners(), owner, prior_budget=prior_budget,
                reference_input=False)

        principal = MODULE.principal_legal_calls()
        principal_row = 'CORE_PRINCIPAL_CONTRACT_PROFILE' + ''.join(
            f' {field}={principal[field]}' for field in MODULE.PRINCIPAL_FIELDS17) + '\n'
        healthy = 'legal-core-contract\n' + principal_row + self.contract_row() + self.lifecycle_contract_row()
        for text, succeeds in ((healthy,True), (healthy.replace(self.contract_row(),''),False),
                               (healthy.replace(self.lifecycle_contract_row(),''),False)):
            with self.subTest(complete=succeeds), tempfile.TemporaryDirectory() as temporary:
                report = {}; commands = []
                class Session:
                    def protect(self, _path): pass
                    def run(self, label, command, _timeout, **kwargs):
                        commands.append((label,kwargs))
                        if label == 'contract-library':
                            initial = report['core_invocations'][0]
                            if (not initial['observation_pending'] or
                                    initial['chart_combination_external_reserved_budget'] !=
                                    MODULE.chart_combination_budget_vector(external_legal=True)):
                                raise AssertionError('Library command preceded complete new source reservation')
                            return 'inert selected artifact envelope'
                        if label.startswith('external-consumer-'): return ''
                        if label == 'external-legal-execution':
                            self_report = report['core_invocations'][-1]
                            if not self_report['observation_pending']:
                                raise AssertionError('Runtime preceded source reservation')
                            if self_report['combination_external_reserved_calls']['uncertainties'] != 1:
                                raise AssertionError('Missing singleton source reservation')
                            return text
                        if label == 'external-chart-combination-legal-execution':
                            current = report['core_invocations'][0]
                            if not current['observation_pending']:
                                raise AssertionError('New legal reservation cleared before output')
                            return ('CORE_CHART_COMBINATION_CONTRACT_PROFILE labels=5 contexts=2 monotonic=4 '
                                    'coordinates=1 exchanges=1 observations=3 histories=1 appends=3 '
                                    'frames=1 domains=1 charts=1 prepares=1 terms=1 tracked_projections=1 '
                                    'chart_enclosures=1 raw_enclosures=1 raw_batches=1 principal_parameters=1 '
                                    'combination_parameters=1 minima=1 strata=1 combination_seeds=2 '
                                    'chart_batches=1 chart_advances=3 bootstrap_getters=5 outcome_getters=17 '
                                    'estimate_getters=21 iterator_next=2 state_equalities=2\n'
                                    'legal-chart-combination-contract\n')
                        if label == 'external-chart-legal-execution':
                            chart = report['core_invocations'][-1]
                            if not chart['observation_pending'] or chart['charged_fit_work_upper'] != 186:
                                raise AssertionError('Chart runtime preceded its own source reservation')
                            return ('CORE_CHART_CONTRACT_PROFILE labels=5 contexts=2 monotonic=4 coordinates=1 '
                                    'exchanges=1 observations=3 histories=1 appends=3 frames=1 domains=1 charts=1 '
                                    'prepares=4 terms=1 tracked_projections=1 enclosures=1 getters=22 '
                                    'batches=1 minima=1 selections=1 batch_getters=22 '
                                    'raw_enclosures=1 raw_batches=1 principal_parameters=1 '
                                    'principal_strata=1 principal_seeds=1 principal_advances=1 '
                                    'chart_principal_advances=1 principal_getters=27\n'
                                    'legal-chart-contract\n')
                        if label == 'external-adaptive-legal-execution':
                            adaptive = report['core_invocations'][-1]
                            if (not adaptive['observation_pending']
                                    or adaptive['adaptive_external_reserved_budget']['adaptive.call.adaptive_appends'] != 3):
                                raise AssertionError('Adaptive runtime preceded its own source reservation')
                            return ('CORE_ADAPTIVE_CONTRACT_PROFILE retained_points=0 retained_inputs=0 '
                                    'retained_selections=2 adaptive_observations=3 adaptive_appends=3 '
                                    'adaptive_histories=1 raw_parameter_validations=0 raw_ordinal_validations=0 '
                                    'raw_event_validations=0 external_normalizations=1 contexts=1 monotonic=3 '
                                    'coordinates=1 exchanges=1 getters=29\nlegal-core-contract\n')
                        raise AssertionError(label)
                session = Session(); session.work = Path(temporary); session.stable = Path('/inert-stable')
                with patch.object(MODULE.COMMON,'library_artifact',side_effect=['/work/core.rlib','/work/domain.rlib']), \
                     patch.object(MODULE,'copy_contract_dependencies',return_value=session.work/'contract-deps'), \
                     patch.object(MODULE,'contract_examples',side_effect=MODULE.historical_s158_contract_examples), \
                     patch.dict(MODULE.FIXED, MODULE.S14A_FIXED, clear=True), \
                     patch.object(MODULE,'SUBJECTS',S158_SUBJECTS), \
                     patch.dict(MODULE.EXTERNAL_CONSUMER_LIMITS, {'legal_executions':4,'negative_compilations':77}), \
                     patch.object(MODULE,'chart_combination_budget_vector',side_effect=historical_vector), \
                     patch.object(MODULE.PRODUCT,'inventory',return_value={}):
                    if succeeds:
                        MODULE.contracts(session,{'contracts_wall_seconds':300,'jobs':10},report)
                    else:
                        with self.assertRaisesRegex(MODULE.Incomplete,'one contract marker'):
                            MODULE.contracts(session,{'contracts_wall_seconds':300,'jobs':10},report)
                self.assertEqual(sum(label=='external-legal-execution' for label,_ in commands),1)
                expected = ({'charged_fit_work_upper':685,'point_admissions_upper':23679}
                            if succeeds else {'charged_fit_work_upper':189,'point_admissions_upper':6615})
                self.assertEqual(report['core_charged_subset_totals'], expected)
                self.assertEqual(report['core_invocations'][0]['observation_pending'],not succeeds)
                if succeeds:
                    rejected = [kwargs['specimen'] for label,kwargs in commands
                                if label.startswith('external-consumer-') and 'specimen' in kwargs and kwargs['specimen']]
                    self.assertEqual(len(rejected),77)
                    self.assertEqual(sum(label=='external-adaptive-legal-execution' for label,_ in commands),1)
                    self.assertEqual(report['contracts']['literal_combination_legal_calls'],MODULE.combination_legal_calls())
                    self.assertEqual(report['core_combination_recorded_call_subtotals'],MODULE.combination_legal_calls())


class PrincipalCheckpointTests(HistoricalS14cTestCase):
    """Current principal records must be charged, never silently ignored."""

    def setUp(self):
        fund_synthetic_source_recipes(self)
        limits = patch.dict(MODULE.SOURCE_RAW_PRIVATE_GLOBAL_LIMITS,
                            {'square_root_calls_upper':33000, 'binary_arithmetic_calls_upper':22000})
        limits.start(); self.addCleanup(limits.stop)

    def test_private_decision_source_upper_preserves_public_call_identity(self):
        """Seven direct selections/completions add work without invented advances."""
        calls = dict.fromkeys(MODULE.PRINCIPAL_FIELDS, 0)
        ordinary = MODULE.principal_source_work(calls)
        private = MODULE.principal_source_work(calls, private_decision_case=True)
        selector = MODULE.projected_batch_source_work({
            name: 7 if name == 'selections' else 0 for name in MODULE.PROJECTED_BATCH.FIELDS})
        self.assertEqual(set(private), set(ordinary))
        for key, value in selector.items():
            self.assertEqual(private[key], value + (448 if key == 'internal_product_getter_requests_upper' else 0))
        self.assertEqual(private['principal_key_comparisons_upper'], 448)
        self.assertEqual(private['principal_checked_scalar_primitives_upper'], 0)
        self.assertEqual(private['principal_state_clone_cells_upper'], 0)
        self.assertEqual(private['principal_garde_validation_calls_upper'], 0)
        self.assertEqual(calls['advances'], 0)
        self.assertEqual(MODULE.principal_source_work(calls, private_decision_case=False), ordinary)
        for value in (None, 0, 1, 'true', [], {}):
            with self.subTest(value=value), self.assertRaises(MODULE.Incomplete):
                MODULE.principal_source_work(calls, private_decision_case=value)

    def test_principal_floor_equivalence_scope_is_one_exact_closed_fault(self):
        entry = {
            'id': 'S13-P-E-001',
            'source_sha256': '876b647b1d8f1df7156b7383c42fcc0a0a3497e0561ebb331079c120888da658',
            'mutant': {'name': 'crates/time-core/src/principal.rs:945:17: replace < with <= in updated_score',
                       'file': 'crates/time-core/src/principal.rs', 'replacement': '<='},
            'proof': {'path': 'tasks/s1.3-principal-equivalences.md'},
        }
        with patch.dict(MODULE.MUTATION_INVENTORIES, {}, clear=True):
            MODULE.admit_core_equivalence_scope([])
        MODULE.historical_admit_core_equivalence_scope([entry])
        for group, field, value in (
                (None, 'id', 'unreviewed'), (None, 'source_sha256', '0' * 64),
                ('mutant', 'name', entry['mutant']['name'].replace('945:17', '1063:9')),
                ('mutant', 'file', 'crates/time-core/src/regression.rs'),
                ('mutant', 'replacement', '=='), ('proof', 'path', 'tasks/unreviewed.md')):
            altered = json.loads(json.dumps(entry))
            (altered if group is None else altered[group])[field] = value
            with self.subTest(group=group, field=field), self.assertRaises(MODULE.Incomplete):
                MODULE.historical_admit_core_equivalence_scope([altered])
        with self.assertRaises(MODULE.Incomplete):
            MODULE.historical_admit_core_equivalence_scope([entry, entry])

    FIRST = ('CORE_PRINCIPAL_PROFILE case=principal_seed_and_genuine_update_hysteresis '
             'histories=2 appends=10 observations=10 contexts=2 monotonic=33 coordinates=1 '
             'exchanges=1 getters=259 terms=18 projections=18 enclosures=18 batches=9 '
             'minima=1 parameters=1 strata=2 seeds=1 advances=9\n')

    @classmethod
    def complete_rows(cls):
        """Literal fourteen-case recipes, independent of parser tables."""
        fields = ('histories','appends','observations','contexts','monotonic','coordinates',
                  'exchanges','getters','terms','projections','enclosures','batches','minima',
                  'parameters','strata','seeds','advances','state_clones','value_equalities','state_formats')
        rows = (
            ('principal_threshold_exact_and_neighbors',(22,66,66,22,99,44,22,153,22,22,22,11,3,3,22,3,11,0,0,0)),
            ('principal_unrelated_event_and_pending_wait',(24,72,72,24,104,48,24,128,24,24,24,8,1,1,24,1,8,0,0,0)),
            ('principal_lifecycle_rejection_is_atomic_and_recovers',(28,84,84,28,126,56,28,136,28,28,28,14,1,1,28,1,20,18,24,6)),
            ('principal_agreement_loss_preserves_minimum_and_observation_identity',(29,87,87,29,126,58,29,179,29,29,29,10,5,5,29,5,15,0,0,0)),
            ('principal_cached_absence_and_present_nonmember_keep_score_order',(11,33,33,11,48,22,11,68,11,11,11,4,1,1,11,1,4,2,0,0)),
            ('principal_raw_scalar_and_stratum_neighbors',(0,0,0,0,0,0,0,24,0,0,0,0,0,20,5,0,0,0,1,0)),
            ('principal_metadata_and_roster_rejections_preserve_state',(4,12,12,4,18,8,4,57,4,4,4,3,1,1,5,5,11,5,10,5)),
            ('principal_numeric_rejection_is_atomic_and_recovers',(6,18,18,6,27,12,6,28,6,6,6,3,1,1,6,1,4,2,2,1)),
            ('principal_cardinality_zero_distance_and_stratum_weight_neighbors',(36,108,108,36,149,72,36,231,36,36,36,5,5,5,36,5,5,0,0,0)),
            ('principal_error_sources_preserve_embedded_identity',(0,0,0,0,0,0,0,18,0,0,0,0,0,0,0,0,0,0,0,0)),
            ('principal_equality_distinguishes_context_and_pending_credit',(2,6,6,2,10,4,2,28,2,2,2,2,1,1,2,2,1,1,4,0)),
            ('principal_inferior_update_floors_score_without_reprojection_credit',(4,12,12,4,18,8,4,43,4,4,4,2,1,1,4,1,3,1,1,0)),
            ('principal_private_decision_relations',(2,6,6,2,3,1,1,63,2,2,2,3,1,1,2,1,4,0,0,0)),
        )
        return cls.FIRST + ''.join('CORE_PRINCIPAL_NEXT_PROFILE case=' + case + ''.join(
            ' ' + field + '=' + str(value) for field, value in zip(fields, values, strict=True)) + '\n'
            for case, values in rows)

    def test_real_principal_profile_adds_its_own_producer_work(self):
        legacy = 'CORE_PROFILE_COUNTS fits=1 points=3\n'
        self.assertEqual(MODULE.profile_counts(legacy)['charged_fit_work_upper'], 1)
        counts = MODULE.profile_counts(legacy + self.FIRST)
        self.assertEqual(counts['charged_fit_work_upper'], 11)
        self.assertEqual(counts['point_admissions_upper'], 643)
        self.assertEqual(counts.get('principal_counter_records'), 1)

    def test_unknown_principal_marker_is_rejected_in_a_healthy_legacy_stream(self):
        legacy = 'CORE_PROFILE_COUNTS fits=1 points=3\n'
        self.assertEqual(MODULE.profile_counts(legacy)['point_admissions_upper'], 3)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.profile_counts(legacy + self.FIRST.replace(
                'case=principal_seed_and_genuine_update_hysteresis', 'case=unadmitted_case'))

    def test_all_means_the_chart_trio_checkpoint_without_a_full_manifest_claim(self):
        self.assertEqual(MODULE.SCOPE.select('all'), S14C_SUBJECTS)
        claim = MODULE.checkpoint_scope_claim(MODULE.SCOPE.select('all'))
        self.assertTrue(claim['checkpoint_complete_selection'])
        self.assertFalse(claim['critical_manifest_complete_selection'])
        self.assertIn('24-owner', claim['qualification_scope'])

    def test_full_actual_rows_and_partial_prefixes_stay_distinct(self):
        complete = MODULE.principal_counts(self.complete_rows(), require_complete=True)
        self.assertEqual(complete['principal_counter_records'], 14)
        self.assertEqual(complete['principal_observed_calls']['getters'], 1415)
        self.assertEqual(complete['principal_observed_calls']['value_equalities'], 42)
        self.assertTrue(complete['principal_private_case_source_upper_present'])
        partial = MODULE.principal_counts(self.FIRST)
        self.assertIsNone(partial['principal_observed_calls'])
        self.assertFalse(partial['principal_private_case_source_upper_present'])
        self.assertEqual(len(partial['principal_missing_cases']), 13)
        for altered in (self.complete_rows() + self.FIRST, self.complete_rows().rstrip('\n'),
                        self.complete_rows().replace('getters=259', 'getters=18446744073709551616'),
                        self.complete_rows().replace('value_equalities=24', 'state_equalities=24')):
            with self.subTest(value=altered[:70]), self.assertRaises(MODULE.Incomplete):
                MODULE.principal_counts(altered, require_complete=True)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.principal_counts(self.complete_rows().replace('value_equalities=', 'state_equalities='))

    def test_successful_baseline_cannot_omit_the_principal_family(self):
        helper = CoreAnalysisTests()
        output = helper.completed_mutation_log(helper.complete_runs_rows()).replace('96 passed', '281 passed')
        outcome = {'scenario': 'Baseline', 'summary': 'Success', 'log_path': 'baseline.log',
                   'phase_results': helper.phase_results('Success')}
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / 'baseline.log'
            for text, valid in ((output, True), (output.replace(self.complete_rows(), ''), False)):
                path.write_text(text)
                report = {}
                MODULE.record_counts(report, 'principal:mutation-baseline', {
                    **MODULE.package_charged_upper(include_doctests=True),
                    'literal_doctests_reserved':True,
                    **MODULE.selection_reservation(package=True, include_doctests=True), 'observation_pending': True})
                if valid:
                    MODULE.observe_mutation_outcome(report, path.parent / 'outcomes.json',
                        'principal:mutation-baseline', outcome, report['core_invocations'][0])
                else:
                    with self.assertRaisesRegex(MODULE.Incomplete, 'Principal'):
                        MODULE.observe_mutation_outcome(report, path.parent / 'outcomes.json',
                            'principal:mutation-baseline', outcome, report['core_invocations'][0])

    def test_principal_only_timeout_routes_known_and_unknown_markers_to_parser(self):
        helper = CoreAnalysisTests()
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / 'timeout.log'
            outcome = {'scenario': {'Mutant': {'name': 'stalled'}}, 'summary': 'Timeout', 'log_path': path.name}
            for prefix, valid in ((self.FIRST, True), ('CORE_PRINCIPAL_FAKE value=1\n', False)):
                path.write_text(helper.mutation_phase_header('Test') + prefix)
                report = {}
                MODULE.record_counts(report, 'principal:mutation-1', {
                    'charged_fit_work_upper': 63129, 'point_admissions_upper': 2253360,
                    **MODULE.selection_reservation(package=True), 'observation_pending': True})
                row = report['core_invocations'][0]
                if valid:
                    MODULE.observe_mutation_outcome(report, path.parent / 'outcomes.json',
                                                   'principal:mutation-1', outcome, row)
                    self.assertEqual(row['observation_status'], 'timeout-prefix-unqualified')
                else:
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.observe_mutation_outcome(report, path.parent / 'outcomes.json',
                                                       'principal:mutation-1', outcome, row)
                self.assertTrue(row['observation_pending'])


class CoreAnalysisTests(HistoricalS14cTestCase):
    """Challenge actual admission/counter/cause behavior with valid neighbors."""

    @staticmethod
    def repaired_selection_equivalence_scope():
        """Independent fresh-listing identities; not native outcome evidence."""
        names = (
            ('TC-S13-E-001', '694:26: replace || with && in maximum_components'),
            ('TC-S13-E-002', '756:52: replace > with < in common_witness'),
            ('TC-S13-E-003', '756:62: replace || with && in common_witness'),
        )
        replacements = {'TC-S13-E-001': '&&', 'TC-S13-E-002': '<', 'TC-S13-E-003': '&&'}
        return [{'id': identifier, 'mutant': {'name': 'crates/time-core/src/selection.rs:' + suffix,
                                            'file': 'crates/time-core/src/selection.rs',
                                            'replacement': replacements[identifier]},
                 'source_sha256': 'da8b87fe69215b71419ff370d4032f0dc49363131c75e70076cb2cce6305d19e',
                 'proof': {'path': 'tasks/s1.3-selection-equivalences.md'}}
                for identifier, suffix in names]

    @classmethod
    def combined_equivalence_scope(cls):
        """Literal reviewed own-domain identities; not a native score or outcome."""
        names = (
            ('TC-S13-E-004', '155:15: replace >> with << in center_admissible', '<<'),
            ('TC-S13-E-005', '166:15: replace >> with << in distance_admissible', '<<'),
            ('TC-S13-E-006', '251:31: replace < with <= in Fraction::compare', '<='),
            ('TC-S13-E-007', '314:28: replace | with ^ in ScaledValue::from_bits', '^'),
        )
        return cls.repaired_selection_equivalence_scope() + [
            {'id': identifier, 'mutant': {'name': 'crates/time-core/src/projection_enclosure.rs:' + suffix,
                                        'file': 'crates/time-core/src/projection_enclosure.rs',
                                        'replacement': replacement},
             'source_sha256': '219142ff1ce58475b4707306e5f85909eb03918d9b3e21d0f5181e2bee49fe36',
             'proof': {'path': 'tasks/s1.3-enclosure-equivalences.md'}}
            for identifier, suffix, replacement in names]

    def test_core_equivalence_scope_accepts_empty_or_exact_repaired_trio(self):
        MODULE.historical_admit_core_equivalence_scope([])
        MODULE.historical_admit_core_equivalence_scope(self.repaired_selection_equivalence_scope())
        MODULE.historical_admit_core_equivalence_scope(list(reversed(self.repaired_selection_equivalence_scope())))

    def test_core_equivalence_scope_accepts_only_the_complete_combined_seven(self):
        good = self.combined_equivalence_scope()
        MODULE.historical_admit_core_equivalence_scope(good)
        MODULE.historical_admit_core_equivalence_scope(list(reversed(good)))
        for subset in (good[3:], good[:6], good[1:], good[:2] + good[3:4], good[:3] + [good[3]] * 4):
            with self.subTest(subset=subset), self.assertRaises(MODULE.Incomplete):
                MODULE.historical_admit_core_equivalence_scope(subset)

    def test_combined_equivalence_scope_rejects_wrong_owners_operators_and_unreviewed_complement(self):
        good = self.combined_equivalence_scope()
        for index in range(7):
            for owner, field, value in (
                ('mutant', 'file', 'crates/time-core/src/regression.rs'),
                ('mutant', 'replacement', '!='),
                ('proof', 'path', 'tasks/other-equivalence.md'),
            ):
                bad = copy.deepcopy(good)
                bad[index][owner][field] = value
                with self.subTest(index=index, owner=owner, field=field), self.assertRaises(MODULE.Incomplete):
                    MODULE.historical_admit_core_equivalence_scope(bad)
            bad = copy.deepcopy(good)
            bad[index]['source_sha256'] = ('219142ff1ce58475b4707306e5f85909eb03918d9b3e21d0f5181e2bee49fe36'
                                           if index < 3 else 'da8b87fe69215b71419ff370d4032f0dc49363131c75e70076cb2cce6305d19e')
            with self.subTest(index=index, field='source_sha256'), self.assertRaises(MODULE.Incomplete):
                MODULE.historical_admit_core_equivalence_scope(bad)
        bad = copy.deepcopy(good)
        bad[3]['mutant'].update({
            'name': 'crates/time-core/src/projection_enclosure.rs:275:62: replace || with && in Fraction::complement',
            'replacement': '&&'})
        with self.assertRaises(MODULE.Incomplete):
            MODULE.historical_admit_core_equivalence_scope(bad)

    def test_core_equivalence_scope_rejects_old_or_different_faults_and_unbound_proofs(self):
        good = self.repaired_selection_equivalence_scope()
        bad = [None, False, {}, good[:2], good + [good[0]], [good[0]] * 3]
        for field, value in (
            ('id', 'TC-S13-E-004'),
            ('source_sha256', '0071a1a407a922f6d2019741a12aea24663e0a3b9e37f7e3a135aefcdae94840'),
            ('mutant', None), ('proof', None),
            ('proof', {'path': 'tasks/old-proof.md'}),
        ):
            changed = copy.deepcopy(good)
            changed[0][field] = value
            bad.append(changed)
        for wrong_fault in (
            'crates/time-core/src/selection.rs:691:26: replace || with && in maximum_components',
            'crates/time-core/src/selection.rs:652:41: replace || with && in maximum_components',
            'crates/time-core/src/selection.rs:756:52: replace > with == in common_witness',
        ):
            changed = copy.deepcopy(good)
            changed[0]['mutant']['name'] = wrong_fault
            bad.append(changed)
        for value in bad:
            with self.subTest(value=value), self.assertRaises(MODULE.Incomplete):
                MODULE.historical_admit_core_equivalence_scope(value)

    def setUp(self):
        """Replace only adjacent product admission for inert reader fixtures."""
        fund_synthetic_source_recipes(self)
        # The source-identity layer has dedicated real-candidate controls. This
        # reader fixture substitutes inert files and isolates the two active
        # current guards. The23-source tests use actual product bytes directly.
        for owner, guard in ((MODULE.CHART, 'admit_composition_sources'),
                             (MODULE.CHART_COMBINATION, 'admit_achieved_phase_sources')):
            identity = patch.object(owner, guard, return_value=None)
            identity.start(); self.addCleanup(identity.stop)
        coordinator = patch.dict(MODULE.COORDINATOR_COHORTS, {'status': 'admitted-exact-source-and-cohorts'})
        coordinator.start(); self.addCleanup(coordinator.stop)
        prior = patch.object(MODULE, 'admit_spent_contract_attempt', return_value={'synthetic': True})
        prior.start(); self.addCleanup(prior.stop)
        inventories = patch.dict(MODULE.MUTATION_INVENTORIES, {
            name: {'count':1, 'sha256':'1' * 64} for name in MODULE.SUBJECTS}, clear=True)
        inventories.start(); self.addCleanup(inventories.stop)
        # This inert candidate has no mathematical-equivalence proof. Exercise
        # its empty-E boundary locally; AdaptiveEquivalenceScopeTests and the
        # real checked-in policy control exercise the strict current18 owner.
        def inert_equivalences(entries):
            if type(entries) is not list or entries:
                raise MODULE.Incomplete('Synthetic product admits no mutation equivalences.')
        equivalences = patch.object(MODULE, 'admit_core_equivalence_scope', side_effect=inert_equivalences)
        equivalences.start(); self.addCleanup(equivalences.stop)
        admission = patch.object(MODULE.PRODUCT, 'admit_product', side_effect=self.inert_product)
        admission.start()
        self.addCleanup(admission.stop)
        runs_gate = patch.object(MODULE, 'RUNS_TELEMETRY_STATUS',
                                 'admitted-reviewed-runs-telemetry-and-campaign-budget')
        runs_gate.start()
        self.addCleanup(runs_gate.stop)
        # These are historical synthetic parser fixtures, not new discovery.
        for mapping, values in (
            (MODULE.FIXED, {'graph_status': 'admitted-final-source-inventory-and-cohort-binding'}),
            (MODULE.PROJECTION_ADAPTER_TELEMETRY, {'status': 'admitted-source-only-recipe'}),
            (MODULE.STANDARD_ERROR_TELEMETRY, {'status': 'admitted-source-only-recipe'}),
            (MODULE.TRACKING_TELEMETRY, {'status': 'admitted-source-only-recipe'}),
            (MODULE.CHART_TELEMETRY, {'status': 'admitted-source-only-recipe'}),
            (MODULE.CHART_BATCH_TELEMETRY, {'status': 'admitted-source-only-recipe'}),
            (MODULE.CHART_PRINCIPAL_TELEMETRY, {'status': 'admitted-source-only-recipe'}),
            (MODULE.PROJECTION_TELEMETRY, {'status': 'admitted-source-owned-observer'}),
            (MODULE.COORDINATOR_COHORTS, {'status': 'admitted-exact-source-and-cohorts'}),
            (MODULE.ENCLOSURE_TELEMETRY, {'status': 'admitted-source-owned-observer',
                                       'case_recipes': MODULE.ENCLOSURE.case_recipes(),
                                       'source_owned_private_work_per_package': MODULE.ENCLOSURE.SOURCE_PRIVATE_WORK,
                                       'literal_doctests_added': 0}),
            (MODULE.PROJECTED_BATCH_TELEMETRY, {
                'status': 'admitted-source-owned-observer',
                # Deliberate inert parser allowances, never candidate quotas.
                'global_limits': {key: amount * 1000
                                  for key, amount in MODULE.projected_batch_budget_vector().items()}}),
            (MODULE.PRINCIPAL_TELEMETRY, {
                'status': 'admitted-source-owned-profiles',
                'global_limits': {key: amount * 1000 for key, amount in MODULE.principal_budget_vector().items()}}),
            (MODULE.COMBINATION_TELEMETRY, {
                'status': 'admitted-source-owned-profiles',
                'global_limits': {key: amount * 1000 for key, amount in MODULE.combination_budget_vector().items()}}),
            (MODULE.LIFECYCLE_TELEMETRY, {
                'status': 'admitted-source-owned-profiles',
                'global_limits': {key: amount * 1000 for key, amount in MODULE.lifecycle_budget_vector().items()}}),
            (MODULE.ADAPTIVE_TELEMETRY, {
                'status': 'admitted-source-owned-profiles',
                'global_limits': {key: amount * 1000 + MODULE.adaptive_external_budget_vector()[key]
                                  for key, amount in MODULE.adaptive_budget_vector(include_doctests=True).items()}}),
            (MODULE.SOURCE_RAW_PRIVATE_GLOBAL_LIMITS,
                {'square_root_calls_upper': 33000, 'binary_arithmetic_calls_upper': 22000}),
            (MODULE.FINAL_PACKAGE_TEST_COUNTS, {'ordinary': 94, 'doctests': 2, 'baseline': 96}),
        ):
            fixture = patch.dict(mapping, values)
            fixture.start(); self.addCleanup(fixture.stop)

    def test_current_policy_does_not_require_historical_spent_contract_receipt(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            self.fixture(root)
            with patch.object(MODULE, 'admit_spent_contract_attempt',
                              side_effect=MODULE.Incomplete('historical spent receipt unavailable')) as prior:
                policy = MODULE.read_policy(root, 'contracts')
            self.assertEqual(policy['coordinator_cohorts']['checkpoint'], 'S1.4c')
            prior.assert_not_called()

    @staticmethod
    def norm_row(**changes):
        """Independent literal nine-field transport recipe, not module schema reuse."""
        values = dict(normalizations=1, observations=3, contexts=1, monotonic=3,
                      coordinates=12, exchanges=3, getters=5, stats=0, points=0)
        values.update(changes)
        return 'CORE_NORMALIZATION_PROFILE_COUNTS ' + ' '.join(f'{key}={value}' for key, value in values.items())

    @classmethod
    def fuzz_row(cls, raw=1, **changes):
        """One actual-parser cumulative fuzz control; no simulated Rust claim."""
        changes.setdefault('stats', 1)
        return cls.norm_row(**changes).replace('CORE_NORMALIZATION_PROFILE_COUNTS ',
            f'CORE_NORMALIZATION_FUZZ_COUNTS raw={raw} ')

    @staticmethod
    def complete_projection_rows():
        """Independent authored fixture recipes; no native result is inferred."""
        recipes = (
            ('terms_admission', (0,0,0,0,0,0,0,36,48,0)),
            ('linear_identity', (1,4,4,1,8,16,4,280,1,6)),
            ('shape_controls', (2,6,6,1,8,24,6,290,1,2)),
            ('origin_durations', (2,6,6,1,10,24,6,290,1,2)),
            ('integer_envelope', (1,3,3,1,7,12,3,170,1,3)),
            ('error_precedence', (3,4,4,4,9,16,4,205,2,12)),
            ('evicted_anchor', (1,65,65,1,68,260,65,1489,1,2)),
            ('numeric_failures', (1,3,3,1,7,12,3,170,5,6)),
            ('signed_zero_consumption', (1,3,3,1,5,12,3,520,8,16)),
            ('projection_private_predicate', (0,)*10),
            ('projection_private_candidate', (0,)*10),
            ('projection_private_arithmetic', (0,)*10),
            ('projection_private_errors', (0,)*10),
        )
        fields = ('histories','appends','observations','contexts','monotonic',
                  'coordinates','exchanges','getters','terms','projections')
        return ''.join('CORE_PROJECTION_PROFILE case=' + case + ''.join(
            f' {field}={value}' for field,value in zip(fields, values, strict=True)) + '\n'
            for case,values in recipes)

    @staticmethod
    def complete_runs_rows():
        """Independent synthetic complete scopes, never Rust execution evidence."""
        external = [(75,3,3),(21,3,3),(12,1,1),(12,1,1),(12,1,1),
                    (3,1,1),(15,5,5),(6,2,2),(3,1,1)]
        guards = [(64,0,0,0,0,0,0,0,0),(1,13,0,0,0,0,0,0,0),
                  (0,0,21,0,0,0,0,0,0,0),(0,0,0,64,1,5,0,0,196)]
        rows = [f'CORE_RUNS_PROFILE points={p} inputs={i} selections={s}\n' for p,i,s in external]
        facts = ('Success(SuccessFacts { start: 8, count: 4, frame: 17, stop: RunsPassed, '
                 'decision_start: 4, decision_count: 8, decision_runs: 3, fit_runs: 3, '
                 'statistics_frame: 17, statistics_count: 4, degrees: 2, line_frame: 17, '
                 'numeric_bits: [13799016943732968640, 13805783338290302156, 4553143622606572749, '
                 '4548640022979202253, 4548640022979202253] })')
        rows.append('CORE_RUNS_SIGN_RECEIPT counts=Counts { points: 24, inputs: 2, selectors: 2 } '
                    'observer=SignObserver { positive: Some(4517753797628002304), '
                    'negative: Some(13751419776488144896), positive_seen: Once, '
                    'negative_seen: Once, calls: 102 } observed=' + facts + ' ordinary=' + facts +
                    ' preservation=[true, true, true, true, true, true] '
                    'identities=[((17, 12), (17, 12)), ((17, 12), (17, 12))]\n')
        rows.append('CORE_RUNS_SIGN_PROFILE points=24 inputs=2 selectors=2\n')
        rows.extend('CORE_RUNS_GUARDS table={} runs={} contexts={} points={} inputs={} prepares={} direct_fits={} selectors={} returned_window_rows={}\n'.format(*row) for row in guards)
        return ''.join(rows)

    @staticmethod
    def mutation_phase_header(phase):
        argv = ['cargo', 'test', *(['--no-run'] if phase == 'Build' else []),
                '--verbose', '--package=time-core@0.0.0', '--message-format=json', '--frozen']
        return '*** ' + ' '.join(argv) + '\n'

    @staticmethod
    def complete_selection_rows():
        """Literal independent package marker recipes; no Rust execution claim."""
        return ('S13_SELECTION_PROFILE scope=interval intervals=8 minima=0 batches=0 selections=0\n'
                'S13_SELECTION_PROFILE scope=minimum intervals=0 minima=6 batches=0 selections=0\n'
                'S13_SELECTION_PROFILE scope=batch intervals=19 minima=1 batches=8 selections=1\n'
                'S13_SELECTION_PROFILE scope=literal_permutations intervals=446 minima=65 batches=65 selections=65\n'
                'S13_BATCH_GUARD_PROFILE {"case":"capacity","intervals":1,"batches":1}\n'
                'S13_BATCH_GUARD_PROFILE {"case":"capacity_neighbor","intervals":16,"batches":2}\n'
                'S13_BATCH_GUARD_PROFILE {"case":"duplicate_recovery","intervals":2,"batches":5}\n'
                'SELECTION_HELPER_PROFILE case=capacity_neighbor read_adapters=2 prefix_sorts=3 '
                'cell_depths=2 label_projections=5 endpoint_collections=2 depth_builds=2 '
                'component_scans=3 witness_builds=4\n'
                'SELECTION_HELPER_PROFILE case=duplicate_recovery read_adapters=0 prefix_sorts=0 '
                'cell_depths=0 label_projections=0 endpoint_collections=3 depth_builds=3 '
                'component_scans=2 witness_builds=0\n')

    def completed_mutation_log(self, test_output, *, include_selection=True):
        finished = '{"reason":"build-finished","success":true}\n'
        if include_selection:
            test_output += self.complete_selection_rows()
            test_output += self.complete_projection_rows()
            # The new family is independently fixed by its source recipe parser.
            test_output += ''.join(
                'CORE_ENCLOSURE_PROFILE case=' + case + ''.join(
                    ' ' + field + '=' + str(value)
                    for field, value in zip(MODULE.ENCLOSURE.FIELDS, values, strict=True)) + '\n'
                for case, values in MODULE.ENCLOSURE.SOURCE_RECIPES)
            test_output += ProjectedBatchWrapperTests.complete_rows()
            test_output += PrincipalCheckpointTests.complete_rows()
            test_output += CombinationCheckpointTests.complete_rows()
            test_output += LIFECYCLE_FIXTURES.complete_lifecycle_text()
            test_output += ADAPTIVE_FIXTURES.COMPLETE_PROFILE
            test_output += ProjectionAdapterCheckpointTests.PASSED
            test_output += StandardErrorCheckpointTests.PASSED + StandardErrorCheckpointTests.DOCS
            test_output += TrackingCheckpointTests.PASSED
            test_output += ChartCheckpointTests.PASSED + ChartCheckpointTests.DOCS
            test_output += ''.join('test ' + name + ' ... ok\n' for name in MODULE.CHART.BATCH_ORDINARY_TESTS)
            test_output += RetainedChartPrincipalIntegrationTests.PASSED
            test_output += RetainedChartPrincipalIntegrationTests.DOCS
            test_output += ChartCombinationIntegrationTests.PASSED + ChartCombinationIntegrationTests.DOCS
            test_output += ''.join('test ' + name + ' ... ok\n'
                                   for name in MODULE.CHART.COMPOSITION_CURRENT_ORDINARY_TESTS)
            test_output += AchievedPhaseIntegrationTests.PASSED + AchievedPhaseIntegrationTests.DOCS
        if 'test result:' not in test_output:
            test_output += 'test result: ok. 96 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;\n'
        return (self.mutation_phase_header('Build') + finished
                + self.mutation_phase_header('Test') + finished + test_output)

    def semantic_record(self):
        """Synthetic relocation of the independently captured native Cargo E0308 shape."""
        root = '/work/tmp/cargo-mutants-source-v2TRlL.tmp'
        return {
            'reason': 'compiler-message',
            'package_id': 'path+file://' + root + '/crates/time-core#0.0.0',
            'manifest_path': root + '/crates/time-core/Cargo.toml',
            'target': {'kind': ['lib'], 'crate_types': ['lib'], 'name': 'time_core',
                       'src_path': root + '/crates/time-core/src/lib.rs',
                       'edition': '2024', 'doc': True, 'doctest': True, 'test': True},
            'message': {'$message_type': 'diagnostic', 'level': 'error',
                        'message': 'mismatched types', 'rendered': 'synthetic diagnostic',
                        'children': [], 'code': {'code': 'E0308', 'explanation': None},
                        'spans': [{'file_name': 'crates/time-core/src/regression.rs',
                                   'is_primary': True, 'line_start': 42, 'column_start': 7}]},
        }

    def semantic_mutation_log(self, record=None):
        record = self.semantic_record() if record is None else record
        note = copy.deepcopy(record)
        note['message'] = {'$message_type': 'diagnostic', 'level': 'failure-note',
                           'message': 'For more information, try rustc --explain E0308.',
                           'rendered': None, 'children': [], 'code': None, 'spans': []}
        return (self.mutation_phase_header('Build') + json.dumps(record) + '\n' + json.dumps(note) + '\n'
                + '{"reason":"build-finished","success":false}\n')

    def unused_comparison_record(self, operator='<'):
        """Literal native selection lint, relocated to the fixed synthetic worker."""
        record = self.semantic_record()
        record['message'].update(
            code={'code': 'unused_comparisons', 'explanation': None},
            message='comparison is useless due to type limits',
            spans=[{'file_name': 'crates/time-core/src/selection.rs', 'is_primary': True,
                    'line_start': 526, 'column_start': 15,
                    'text': [{'text': '        while position ' + operator
                              + ' /* ~ changed by cargo-mutants ~ */ 0 {'}]}],
            children=[{'level': level, 'message': message, 'children': [],
                       'code': None, 'spans': [], 'rendered': None}
                      for level, message in (
                          ('note', '`-D unused-comparisons` implied by `-D warnings`'),
                          ('help', 'to override `-D warnings` add `#[allow(unused_comparisons)]`'))])
        return record

    @staticmethod
    def build_script_record():
        """Observed native rustversion record, with a synthetic inert worker path."""
        return {
            'reason': 'build-script-executed',
            'package_id': 'registry+https://github.com/rust-lang/crates.io-index#rustversion@1.0.23',
            'linked_libs': [], 'linked_paths': [], 'cfgs': [], 'env': [],
            'out_dir': '/work/tmp/cargo-mutants-source-yfcbrg.tmp/target/debug/build/'
                       'rustversion-d2854a6eb6777e68/out',
        }

    def test_runs_admission_remains_pending_without_coordinator_binding(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); self.fixture(root)
            with patch.object(MODULE, 'RUNS_TELEMETRY_STATUS', 'pending-reviewed-runs-telemetry-and-campaign-budget'):
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.read_policy(root, 'contracts')
            self.assertEqual(MODULE.read_policy(root, 'contracts')['source_file_count'], 156)

    def test_final_projected_batch_admission_requires_every_finite_nominal_allowance(self):
        required = MODULE.projected_batch_budget_vector()
        self.assertEqual(len(required), 68)
        key = 'projected_batch.call.batches'
        bad = [None, {}, {name: amount for name, amount in required.items() if name != key},
               {**required, 'projected_batch.extra': 1}]
        bad.extend({**required, key: amount} for amount in (True, -1, 1 << 64, required[key] - 1))
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            with patch.dict(MODULE.PROJECTED_BATCH_TELEMETRY, {'global_limits': required}):
                policy = self.fixture(root)
                self.assertEqual(MODULE.read_policy(root, 'contracts')['projected_batch_telemetry']['global_limits'], required)
            for limits in bad:
                with self.subTest(limits=limits), patch.dict(
                        MODULE.PROJECTED_BATCH_TELEMETRY, {'global_limits': limits}):
                    changed = copy.deepcopy(policy)
                    changed['projected_batch_telemetry']['global_limits'] = limits
                    self.write_policy(root, changed)
                    with self.assertRaisesRegex(MODULE.Incomplete, 'Projected-batch cumulative allowances'):
                        MODULE.read_policy(root, 'contracts')

    def test_runs_counter_families_require_exact_complete_scopes_and_charge_prefixes(self):
        text = self.complete_runs_rows()
        complete = MODULE.runs_counts(text, require_complete=True)
        self.assertEqual((complete['runs_points'], complete['runs_inputs'], complete['runs_selections']), (247, 21, 20))
        self.assertEqual((complete['runs_guard_candidates'], complete['runs_guard_returned_window_rows']), (5, 196))
        self.assertEqual(complete['runs_guard_count_calls'], 13)
        self.assertEqual(complete['runs_fit_upper'], 1245)
        self.assertEqual(complete['runs_point_copy_upper'], 41860)
        self.assertEqual(complete['runs_decision_residual_upper'], 62280)
        self.assertEqual(complete['runs_observer_hooks_upper'], 3098)
        self.assertEqual(complete['runs_observer_pair_comparisons_upper'], 12392)
        self.assertEqual((complete['runs_input_slots_upper'],complete['runs_residual_slots_upper']), (79680,79680))
        for altered in [text.replace('CORE_RUNS_SIGN_PROFILE points=24 inputs=2 selectors=2\n', ''),
                        text + 'CORE_RUNS_SIGN_PROFILE points=24 inputs=2 selectors=2\n',
                        text.replace('CORE_RUNS_PROFILE points=75 inputs=3 selections=3', 'CORE_RUNS_PROFILE points=75 inputs=3 selections=3 extra=0'),
                        text.replace('CORE_RUNS_PROFILE points=21 inputs=3 selections=3', 'CORE_RUNS_PROFILE points=3 inputs=1 selections=1'),
                        text.replace('CORE_RUNS_PROFILE points=21 inputs=3 selections=3', 'CORE_RUNS_PROFILE points=12 inputs=2 selections=2'),
                        text.replace('CORE_RUNS_PROFILE points=21 inputs=3 selections=3', 'CORE_RUNS_PROFILE points=20 inputs=3 selections=3'),
                        text.replace('CORE_RUNS_PROFILE points=21 inputs=3 selections=3', 'CORE_RUNS_PROFILE points=21 inputs=2 selections=3'),
                        text.replace('CORE_RUNS_PROFILE points=21 inputs=3 selections=3', 'CORE_RUNS_PROFILE points=21 inputs=3 selections=2'),
                        text.replace('CORE_RUNS_GUARDS table=64', 'CORE_RUNS_GUARDS table=65'),
                        text.replace('runs=13 contexts=0', 'runs=11 contexts=0'),
                        text.replace('runs=13 contexts=0', 'runs=14 contexts=0'),
                        text.replace('prepares=5 direct_fits=0', 'prepares=4 direct_fits=0'),
                        text.replace('prepares=5 direct_fits=0', 'prepares=6 direct_fits=0'),
                        text.replace('inputs=1 prepares=5', 'inputs=0 prepares=5'),
                        text.replace('prepares=5 direct_fits=0', 'prepares=5 direct_fits=1'),
                        text.replace('returned_window_rows=196', 'returned_window_rows=257'),
                        text.replace('CORE_RUNS_SIGN_PROFILE points=24', 'CORE_RUNS_SIGN_PROFILE points=18446744073709551616'),
                        text.replace('positive: Some(4517753797628002304)', 'positive: Some(0)'),
                        text.replace('counts=Counts { points: 24', 'counts=Counts { points: 23'),
                        text.replace('positive_seen: Once', 'positive_seen: Duplicate'),
                        text.replace('calls: 102', 'calls: 103'),
                        text.replace('preservation=[true, true, true, true, true, true]', 'preservation=[true, true, true, true, true, false]'),
                        text.replace(' identities=[', ' extra=x identities=['),
                        text.replace('CORE_RUNS_PROFILE points=75 inputs=3 selections=3', 'CORE_RUNS_PROFILE points=7 inputs=1 selections=1')]:
            with self.subTest(value=altered[:80]), self.assertRaises(MODULE.Incomplete):
                MODULE.runs_counts(altered, require_complete=True)
        receipt = next(line for line in text.splitlines(keepends=True) if line.startswith('CORE_RUNS_SIGN_RECEIPT '))
        with self.assertRaises(MODULE.Incomplete):
            MODULE.runs_counts(text.replace(receipt, '') + receipt.rstrip('\n'), require_complete=True)
        placeholder = 'CORE_RUNS_SIGN_RECEIPT counts=Counts { synthetic } observer=SignObserver { synthetic } observed=ok ordinary=ok preservation=ok identities=[]\n'
        with self.assertRaises(MODULE.Incomplete):
            MODULE.runs_counts(text.replace(receipt, placeholder), require_complete=True)
        partial = MODULE.runs_counts('CORE_RUNS_PROFILE points=2 inputs=0 selections=0\n')
        self.assertEqual((partial['runs_points'], partial['runs_selections'], partial['runs_fit_upper']), (2, 0, 0))
        with self.assertRaises(MODULE.Incomplete):
            MODULE.runs_counts('CORE_RUNS_PROFILE points=2 inputs=0 selections=0\n', require_complete=True)
        combined = MODULE.profile_counts(text, require_runs=True)
        self.assertEqual(combined['charged_fit_work_upper'], 1245)
        self.assertEqual(combined['point_admissions_upper'], 42107)
        legacy = 'CORE_PROFILE_COUNTS fits=640 points=65533\n'
        endpoint = MODULE.account_literal_doctests(MODULE.profile_counts(legacy + text, require_runs=True))
        self.assertEqual(endpoint['charged_fit_work_upper'], 1885)
        self.assertEqual(endpoint['point_admissions_upper'], 107643)
        with patch.dict(MODULE.PROFILE_LIMITS, {'charged_fit_work_upper': 1884}):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.profile_counts(legacy + text, require_runs=True)
            with self.assertRaises(MODULE.Incomplete):
                MODULE.profile_counts(legacy + text + self.complete_projection_rows(),
                                      require_runs=True, require_projection=True)
        with patch.dict(MODULE.PROFILE_LIMITS, {'point_admissions_upper': 107642}):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.account_literal_doctests(MODULE.profile_counts(legacy + text, require_runs=True))
            with self.assertRaises(MODULE.Incomplete):
                MODULE.account_literal_doctests(MODULE.profile_counts(
                    legacy + text + self.complete_projection_rows(), require_runs=True,
                    require_projection=True))
        with self.assertRaises(MODULE.Incomplete):
            MODULE.profile_counts(legacy.replace('fits=640', 'fits=641') + text, require_runs=True)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.account_literal_doctests(MODULE.profile_counts(
                legacy.replace('points=65533', 'points=65534') + text, require_runs=True))

    def test_runs_partial_guard_bounds_charge_all_five_preparations(self):
        row = ('CORE_RUNS_GUARDS table=0 runs=13 contexts=0 points=64 inputs=1 '
               'prepares=5 direct_fits=0 selectors=0 returned_window_rows=196\n')
        counts = MODULE.runs_counts(row)
        self.assertEqual((counts['runs_guard_count_calls'], counts['runs_guard_candidates']), (13, 5))
        self.assertEqual((counts['runs_fit_upper'], counts['runs_point_copy_upper']), (5, 320))
        for changed in (row.replace('runs=13', 'runs=14'),
                        row.replace('prepares=5', 'prepares=6'),
                        row.replace('inputs=1 prepares=5', 'inputs=0 prepares=5')):
            with self.subTest(row=changed), self.assertRaises(MODULE.Incomplete):
                MODULE.runs_counts(changed)

    def test_runs_receipt_matches_profile_and_all_new_counter_rows_require_newline(self):
        full = self.complete_runs_rows()
        MODULE.runs_counts(full, require_complete=True)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.runs_counts(full, require_complete=True, validate_receipt=False)
        contradiction = full.replace(
            'CORE_RUNS_SIGN_PROFILE points=24 inputs=2 selectors=2\n',
            'CORE_RUNS_SIGN_PROFILE points=0 inputs=0 selectors=0\n')
        with self.assertRaises(MODULE.Incomplete):
            MODULE.runs_counts(contradiction)
        for marker in ('CORE_RUNS_PROFILE ', 'CORE_RUNS_SIGN_PROFILE ', 'CORE_RUNS_GUARDS '):
            lines = full.splitlines(keepends=True)
            index = next(i for i, line in enumerate(lines) if line.startswith(marker))
            last = lines.pop(index)
            truncated = ''.join(lines) + last.rstrip('\n')
            with self.subTest(marker=marker), self.assertRaises(MODULE.Incomplete):
                MODULE.runs_counts(truncated, require_complete=True)
        prefix = MODULE.runs_counts('CORE_RUNS_PROFILE points=2 inputs=0 selections=0\n')
        self.assertEqual(prefix['runs_points'], 2)

    def test_runs_legal_and_privacy_specimens_preserve_distinct_causes(self):
        cases = MODULE.runs_contract_examples()
        legal = cases['legal_runs_selection_and_error']
        self.assertIsNone(legal['code'])
        self.assertEqual((legal['calls']['runs_points'], legal['calls']['runs_inputs'],
                          legal['calls']['runs_selections']), (3,1,2))
        for name, code in [('runs_selection_default','E0277'),
                           ('runs_selection_private_update','E0451'),
                           ('runs_selection_mutate_start','E0616'),
                           ('runs_error_private_update','E0451'),
                           ('runs_error_mutate_point_index','E0616'),
                           ('runs_statistics_mutate','E0594'),
                           ('runs_raw_input','E0308'),('runs_line_injection','E0061'),
                           ('runs_private_terminalcandidate','E0432'),
                           ('runs_private_runsobserver','E0432'),
                           ('runs_private_select_fixture_runs_observed','E0432')]:
            self.assertEqual(cases[name]['code'], code)
            self.assertTrue(cases[name]['tokens'])
            self.assertTrue(cases[name]['source'].splitlines()[cases[name]['line']-1].strip())

    def test_current_all_scope_cannot_reuse_the_historical_global_allowance(self):
        with patch.dict(MODULE.SELECTION_GLOBAL_LIMITS, {'intervals': 255354}), self.assertRaises(MODULE.Incomplete):
            MODULE.prospective_scope_reservation('all', dict(MODULE.SUBJECTS))
        proof = MODULE.prospective_scope_reservation('coverage', dict(MODULE.SUBJECTS))
        self.assertEqual(proof['mutation_package_invocations_upper'], 0)
        self.assertEqual(proof['coverage_package_invocations_upper'], 1)
        self.assertEqual(proof['charged_upper'], {'charged_fit_work_upper': 63129,
                                                 'point_admissions_upper': 2253360})
        self.assertEqual(proof['package_test_count_upper_each'], 96)
        for lane, subjects in [('host', dict(MODULE.SUBJECTS)), ('all', {}),
                               ('all', {'selection': 'crates/time-core/src/regression.rs'})]:
            with self.subTest(lane=lane, subjects=subjects), self.assertRaises(MODULE.Incomplete):
                MODULE.prospective_scope_reservation(lane, subjects)
        with patch.dict(MODULE.GLOBAL_LIMITS, {'point_admissions_upper': 226748}):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.prospective_scope_reservation('coverage', dict(MODULE.SUBJECTS))

    def test_focused_mutation_reserves_actual_inventory_and_its_own_baseline(self):
        proof = MODULE.prospective_scope_reservation('mutation', {'reference_input': MODULE.SUBJECTS['reference_input']})
        # setUp declares one inert variant, not the historical217-variant listing.
        self.assertEqual(proof['mutation_package_invocations_upper'], 2)
        self.assertEqual(proof['coverage_package_invocations_upper'], 0)
        self.assertEqual(proof['package_invocations_upper'], 2)
        self.assertEqual(proof['legal_consumer_executions_upper'], 0)
        self.assertEqual(proof['charged_upper'], {'charged_fit_work_upper': 126258,
                                                  'point_admissions_upper': 4506726})
        with patch.dict(MODULE.GLOBAL_LIMITS, {'point_admissions_upper': 1466475}):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.prospective_scope_reservation('mutation', {'reference_input': MODULE.SUBJECTS['reference_input']})

    def test_selection_complete_package_and_missing_private_recipe_remain_distinct(self):
        full = self.complete_runs_rows() + self.complete_selection_rows()
        counts = MODULE.profile_counts(full, require_runs=True, require_selection=True)
        self.assertEqual(counts['selection_observed_calls'],
                         {'intervals': 492, 'minima': 72, 'batches': 81, 'selections': 66})
        self.assertEqual(counts['selection_helper_observed_calls'],
                          {'read_adapters': 2, 'prefix_sorts': 3, 'cell_depths': 2, 'label_projections': 5,
                           'endpoint_collections': 5, 'depth_builds': 5, 'component_scans': 5, 'witness_builds': 4})
        self.assertEqual(counts['selection_helper_records'], 2)
        self.assertEqual(sum(counts['selection_helper_observed_calls'].values()), 31)
        missing = full.replace(
            'SELECTION_HELPER_PROFILE case=capacity_neighbor read_adapters=2 prefix_sorts=3 '
            'cell_depths=2 label_projections=5 endpoint_collections=2 depth_builds=2 '
            'component_scans=3 witness_builds=4\n', '')
        with self.assertRaises(MODULE.Incomplete):
            MODULE.profile_counts(missing, require_selection=True)
        partial = MODULE.profile_counts(missing)
        self.assertIsNone(partial['selection_helper_observed_calls'])
        self.assertTrue(partial['selection_helper_observation_pending'])
        self.assertEqual(partial['selection_helper_records'], 1)
        self.assertEqual(partial['selection_helper_missing_cases'], ['capacity_neighbor'])
        self.assertEqual(partial['selection_helper_recorded_call_subtotals']['endpoint_collections'], 3)
        malformed = full + '{"reason":"compiler-message","message":{"rendered":"S13_SELECTION_PROFILE scope=interval intervals=8 minima=0 batches=0 selections=0"}}\n'
        with self.assertRaises(MODULE.Incomplete):
            MODULE.profile_counts(malformed)

    def test_selection_external_consumer_rows_are_closed_separate_and_bounded(self):
        row = 'CORE_SELECTION_EXTERNAL_PROFILE intervals=6 minima=3 batches=4 selections=2\n'
        counts = MODULE.profile_counts(row)
        self.assertEqual(counts['selection_external_consumer_calls'],
                         {'intervals': 6, 'minima': 3, 'batches': 4, 'selections': 2})
        self.assertEqual(counts['direct_fit_calls'], 0)
        self.assertIsNone(counts['selection_observed_calls'])
        self.assertEqual(MODULE.selection_external_counts('')['selection_external_consumer_calls'], None)
        for wrong in [row+row, row.rstrip('\n'), row.replace('6', '06', 1),
                      row.replace('intervals=6', 'intervals=7'), row.replace('minima=3', 'minima=4'),
                      row.replace('batches=4', 'batches=3'), row.replace('selections=2', 'selections=true'),
                      row.replace(' selections=', ' extra=0 selections='), 'prefix '+row,
                      '{"rendered":'+json.dumps(row)+'}\n']:
            with self.subTest(wrong=wrong), self.assertRaises(MODULE.Incomplete):
                MODULE.selection_external_counts(wrong)
        for mixed in (row + 'CORE_PROFILE_COUNTS fits=0 points=0\n', row + self.complete_selection_rows()):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.profile_counts(mixed)
        prefix = MODULE.selection_external_counts('CORE_SELECTION_EXTERNAL_PROFILE intervals=1 minima=0 batches=0 selections=0\n')
        self.assertEqual(prefix['selection_external_consumer_calls']['intervals'], 1)

    def test_selection_reservation_retains_missing_observations_without_zeroing(self):
        report = {}
        MODULE.record_counts(report, 'package', {
            'charged_fit_work_upper': 63129, 'point_admissions_upper': 2253360,
            **MODULE.selection_reservation(package=True), 'observation_pending': True})
        self.assertEqual(report['core_selection_reserved_totals']['intervals'], 492)
        self.assertIsNone(report['core_selection_package_observed_calls'])
        self.assertEqual(report['core_observation_pending_invocations'], ['package'])
        row = report['core_invocations'][0]
        row['observed_counts'] = MODULE.profile_counts(
            'S13_SELECTION_PROFILE scope=interval intervals=8 minima=0 batches=0 selections=0\n',
            require_fit=False)
        row['observation_pending'] = False
        MODULE.refresh_accounting(report)
        self.assertEqual(report['core_observation_pending_invocations'], [])
        self.assertEqual(report['core_selection_pending_observation_invocations'], ['package'])
        self.assertIsNone(report['core_selection_package_observed_calls'])
        self.assertEqual(report['core_selection_reserved_totals']['intervals'], 492)
        self.assertEqual(report['core_selection_reported_record_subtotals']['intervals'], 8)
        bad = {**MODULE.selection_reservation(package=True), 'charged_fit_work_upper': 0,
               'point_admissions_upper': 0}
        bad['selection_reserved_calls']['intervals'] = 0
        with self.assertRaises(MODULE.Incomplete):
            MODULE.record_counts({}, 'bad', bad)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.selection_reservation(external={'intervals': True, 'minima': 3, 'batches': 4, 'selections': 2})

    def test_raw_helper_canonical_projection_has_its_own_reserved_scope_and_rejects_forgery(self):
        expected = {
            'direct_read_adapter_attempts_upper': 2, 'endpoint_collection_writes_upper': 160,
            'prefix_and_endpoint_sort_comparisons_upper': 3968, 'endpoint_dedup_comparisons_upper': 155,
            'cell_visits_upper': 315, 'membership_probes_upper': 5072,
            'component_visits_upper': 315, 'witness_probes_upper': 64,
            'label_sort_comparisons_upper': 1080, 'label_prefix_assignments_upper': 144,
            'intersection_reductions_upper': 120}
        package = MODULE.selection_reservation(package=True)
        external = MODULE.selection_reservation(external={'intervals': 6, 'minima': 3, 'batches': 4, 'selections': 2})
        for label, reservation, literal, fits, points, forged in (
                ('package', package, expected, 63129, 2253360, 0),
                ('external', external, dict.fromkeys(expected, 0), 0, 0, 1)):
            with self.subTest(scope=label):
                self.assertEqual(reservation['selection_reserved_helper_canonical_source_work_uppers'], literal)
                report = {}
                counts = {'charged_fit_work_upper': fits, 'point_admissions_upper': points,
                          **reservation, 'observation_pending': True}
                MODULE.record_counts(report, label, counts)
                self.assertEqual(report['core_selection_helper_canonical_source_work_uppers'], literal)
                self.assertIsNone(report['core_selection_helper_observed_calls'])
                bad = copy.deepcopy(counts)
                bad['selection_reserved_helper_canonical_source_work_uppers']['membership_probes_upper'] = forged
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.record_counts({}, 'forged-' + label, bad)
        package['selection_reserved_helper_canonical_source_work_uppers']['membership_probes_upper'] = 0
        self.assertEqual(MODULE.selection_reservation(package=True)
                         ['selection_reserved_helper_canonical_source_work_uppers'], expected)

    def test_selection_external_catalog_pins_intended_causes_and_receiving_operations(self):
        cases = MODULE.SELECTION_CONTRACTS.contract_examples()
        self.assertEqual(len(cases), 27)
        legal = cases['legal_selection_construction_and_output']
        self.assertIsNone(legal['code'])
        self.assertEqual(legal['selection_calls'], {'intervals': 6, 'minima': 3, 'batches': 4, 'selections': 2})
        self.assertEqual(sum(item['code'] is None for item in MODULE.legacy_contract_examples().values()), 10)
        for name, specimen in cases.items():
            if specimen['code'] is not None:
                line = specimen['source'].splitlines()[specimen['line']-1]
                self.assertTrue(line.strip(), name)
                self.assertIn(specimen['code'], {'E0277', 'E0451', 'E0616', 'E0603', 'E0308'})
                self.assertTrue(specimen['tokens'])

    def test_healthy_mutation_package_requires_exact_96_tests_and_all_selection_records(self):
        with tempfile.TemporaryDirectory() as temp:
            output = Path(temp)
            outcome = {'scenario': 'Baseline', 'summary': 'Success', 'log_path': 'baseline.log',
                       'phase_results': self.phase_results('Success')}
            complete = self.completed_mutation_log(self.complete_runs_rows())
            for text, message in [(complete.replace('96 passed', '95 passed'), '96'),
                                  (complete.replace(self.complete_selection_rows(), ''), 'Selection')]:
                (output/'baseline.log').write_text(text)
                report = {}
                MODULE.record_counts(report, 'selection:mutation-baseline', {
                    'charged_fit_work_upper': 63129, 'point_admissions_upper': 2253360,
                    **MODULE.selection_reservation(package=True), 'observation_pending': True})
                with self.subTest(message=message), self.assertRaisesRegex(MODULE.Incomplete, message):
                    MODULE.observe_mutation_outcome(report, output/'outcomes.json',
                        'selection:mutation-baseline', outcome, report['core_invocations'][0])
                self.assertTrue(report['core_invocations'][0]['observation_pending'])

    def test_timeout_selector_only_prefixes_are_parsed_and_never_qualified(self):
        prefixes = [
            'S13_SELECTION_PROFILE scope=interval intervals=1 minima=0 batches=0 selections=0\n',
            'S13_BATCH_GUARD_PROFILE {"case":"capacity","intervals":1,"batches":0}\n',
            'SELECTION_HELPER_PROFILE case=capacity_neighbor read_adapters=1 prefix_sorts=0 '
            'cell_depths=0 label_projections=0 endpoint_collections=0 depth_builds=0 '
            'component_scans=0 witness_builds=0\n',
            'SELECTION_HELPER_PROFILE case=duplicate_recovery read_adapters=0 prefix_sorts=0 '
            'cell_depths=0 label_projections=0 endpoint_collections=1 depth_builds=0 '
            'component_scans=0 witness_builds=0\n',
        ]
        with tempfile.TemporaryDirectory() as temporary:
            output = Path(temporary)
            outcome = {'scenario': {'Mutant': {'name': 'stalled'}}, 'summary': 'Timeout',
                       'log_path': 'timeout.log'}
            for prefix in prefixes:
                for text, valid in [(prefix, True), (prefix+prefix, False), (prefix.rstrip('\n'), False)]:
                    (output/'timeout.log').write_text(self.mutation_phase_header('Test')+text)
                    report = {}
                    MODULE.record_counts(report, 'selection:mutation-1', {
                        'charged_fit_work_upper': 63129, 'point_admissions_upper': 2253360,
                        **MODULE.selection_reservation(package=True), 'observation_pending': True})
                    row = report['core_invocations'][0]
                    with self.subTest(prefix=prefix, valid=valid):
                        if valid:
                            MODULE.observe_mutation_outcome(report, output/'outcomes.json',
                                                            'selection:mutation-1', outcome, row)
                            self.assertEqual(row['observation_status'], 'timeout-prefix-unqualified')
                        else:
                            with self.assertRaises(MODULE.Incomplete):
                                MODULE.observe_mutation_outcome(report, output/'outcomes.json',
                                                                'selection:mutation-1', outcome, row)
                        self.assertTrue(row['observation_pending'])
                        self.assertEqual(row['charged_fit_work_upper'], 63129)
                        self.assertEqual(row['selection_reserved_calls']['intervals'], 492)
                        self.assertNotIn('observed_counts', row)

    def inert_product(self, root):
        """Return the synthetic product inventory; never qualify a real graph."""
        return {}, {name: MODULE.PRODUCT.sha256(root / name) for name in MODULE.BOUND_INPUTS}

    def fixture(self, root):
        """Create the explicitly historical inert shape and exact owner manifest."""
        for name in set(MODULE.BOUND_INPUTS) | set(MODULE.CHART.SOURCE_SHA256) | set(MODULE.CHART.BATCH_SOURCE_INPUTS):
            path = root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text('// isolated synthetic input; never compiled\n')
        critical = MODULE.COMPLETE_CRITICAL_SUBJECTS
        manifest = 'critical_files=[' + ','.join(json.dumps(n) for n in critical.values()) + ']\n'
        for name in critical.values():
            manifest += '[[security_modules]]\npackage="time-core"\nfiles=[' + json.dumps(name) + ']\n'
        (root / 'policy/verification-manifest.toml').write_text(manifest)
        policy = json.loads((ROOT / MODULE.POLICY).read_text())
        if 'rate_delivery_telemetry' not in MODULE.FIXED:
            # This known successor field belongs only to the current profile.
            # Preserve every other unknown field for the strict reader to reject.
            policy.pop('rate_delivery_telemetry', None)
        # This inert source tree has no reviewed product-equivalence candidate.
        # Exact repaired-source E admission has its own controls above.
        policy['mutation_equivalences'] = []
        policy.update(copy.deepcopy(MODULE.FIXED))
        policy['admission_status'] = 'admitted-final-source'
        policy['normalization_telemetry'] = copy.deepcopy(MODULE.NORMALIZATION_TELEMETRY)
        self.rebind(root, policy)
        return policy

    def rebind(self, root, policy):
        """Rebind deliberate synthetic bytes only; production policy stays pending."""
        policy['inputs_sha256'] = self.inert_product(root)[1]
        self.write_policy(root, policy)

    def test_historical_fixture_survives_successor_template_between_healthy_templates(self):
        """A new checked-in key cannot leak into this explicitly historical profile."""
        current = json.loads((ROOT / MODULE.POLICY).read_text())
        self.assertIn('rate_delivery_telemetry', current)
        predecessor = copy.deepcopy(current)
        del predecessor['rate_delivery_telemetry']
        read_text = Path.read_text
        foreign = {**current, 'unreviewed_template_axis': True}
        for stage, template in (('before', predecessor), ('successor-key', current),
                                ('foreign-key', foreign), ('after', predecessor)):
            def template_text(path, *args, value=template, **kwargs):
                if path == ROOT / MODULE.POLICY:
                    return json.dumps(value)
                return read_text(path, *args, **kwargs)
            with self.subTest(stage=stage), tempfile.TemporaryDirectory() as temporary, \
                    patch.object(Path, 'read_text', template_text):
                root = Path(temporary)
                policy = self.fixture(root)
                if stage == 'foreign-key':
                    self.assertIs(policy['unreviewed_template_axis'], True)
                    with self.assertRaisesRegex(MODULE.Incomplete, 'Unreviewed core analysis scope'):
                        MODULE.read_policy(root, 'contracts')
                    continue
                admitted = MODULE.read_policy(root, 'contracts')
                self.assertEqual(admitted['schema_version'], 25)
                self.assertNotIn('rate_delivery_telemetry', admitted)
                # The receiving boundary remains closed even when a caller
                # directly adds the successor key after fixture construction.
                self.write_policy(root, {**policy, 'rate_delivery_telemetry': current['rate_delivery_telemetry']})
                with self.assertRaisesRegex(MODULE.Incomplete, 'Unreviewed core analysis scope'):
                    MODULE.read_policy(root, 'contracts')
                self.write_policy(root, policy)
                self.assertEqual(MODULE.read_policy(root, 'contracts'), admitted)

    def write_policy(self, root, policy):
        """Write one owned temporary policy fixture, never candidate policy."""
        (root / MODULE.POLICY).write_text(json.dumps(policy))

    def fuzz_fixture(self, root, policy):
        """Satisfy adjacent cohort metadata in an inert positive reader control."""
        for name in MODULE.FUZZ_INPUTS:
            path = root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            if not path.exists():
                path.write_text('inert standalone cohort input; never executed\n')
        cohort = json.loads((ROOT / 'policy/time-domain-analysis.json').read_text())
        (root / 'policy/time-domain-analysis.json').write_text(json.dumps(cohort))
        selected = policy['coverage_guided_fuzzing']
        selected['status'] = 'admitted-test-only-source'
        selected['runtime_profile_reuse_review'] = 'reviewed-history-source-and-resource-envelope'
        selected['profile'] = 'time-core-history-s12d'
        selected['fuzzer_flags'] = ['-runs=4096','-max_len=1561','-timeout=2','-rss_limit_mb=512','-malloc_limit_mb=256','-max_total_time=15']
        selected['target_contract'] = 'FixtureHistory::try_append-and-committed-observation-report-public-entrypoints'
        selected['inputs_sha256'] = {name: MODULE.PRODUCT.sha256(root / name) for name in MODULE.FUZZ_INPUTS}
        self.write_policy(root, policy)

    @staticmethod
    def historical_scope(*names):
        """Select inert legacy parser owners without widening the checkpoint."""
        selected = names or tuple(MODULE.HISTORICAL_SUBJECTS)
        return MODULE.COMMON.Scope('time-core', tuple(
            (name, MODULE.HISTORICAL_SUBJECTS[name]) for name in selected), MODULE.POLICY)

    def test_pending_source_is_not_a_pass_and_fixed_synthetic_neighbor_works(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); policy = self.fixture(root)
            admitted = MODULE.read_policy(root, 'contracts')
            self.assertEqual(admitted['critical_files'], list(MODULE.SUBJECTS.values()))
            self.assertEqual(MODULE.SCOPE.select('reference_input'), {'reference_input': MODULE.SUBJECTS['reference_input']})
            with self.assertRaises(MODULE.Incomplete): MODULE.SCOPE.select('chart_combination_receiving')
            with self.assertRaises(MODULE.Incomplete): MODULE.SCOPE.select('adaptive_history')
            self.assertEqual(MODULE.SCOPE.select('all'), MODULE.SUBJECTS)
            self.assertEqual(len(admitted['inputs_sha256']), 156)
            policy['admission_status'] = 'pending-final-source-and-inventory-binding'
            self.write_policy(root, policy)
            with self.assertRaises(MODULE.Incomplete):
                MODULE.read_policy(root, 'contracts')

    def test_core_policy_admits_only_known_optional_tool_admission_profiles(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); policy = self.fixture(root)
            policy.pop('tool_admission_profile', None)
            self.write_policy(root, policy)
            self.assertNotIn('tool_admission_profile', MODULE.read_policy(root, 'contracts'))
            for profile in ('full', 'stable-core'):
                policy['tool_admission_profile'] = profile
                self.write_policy(root, policy)
                self.assertEqual(MODULE.read_policy(root, 'contracts')['tool_admission_profile'], profile)
            for profile in (None, True, '', 'stable', 'stable-core ', ['stable-core']):
                policy['tool_admission_profile'] = profile
                self.write_policy(root, policy)
                with self.subTest(profile=profile), self.assertRaisesRegex(MODULE.Incomplete, 'admission profile'):
                    MODULE.read_policy(root, 'contracts')

    def test_source_binding_drift_and_scope_threshold_budget_changes_reject(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); healthy = self.fixture(root)
            for key, value in [('coverage_minimum_line_percent', 89), ('mutation_minimum_viable_killed_percent', 94),
                               ('maximum_mutants', 513), ('jobs', True), ('mutation_jobs', 12),
                               ('mutation_jobs', 2), ('mutation_jobs', True), ('mutation_jobs', 0),
                               ('mutation_process_limit', 128), ('mutation_process_limit', True),
                               ('mutation_process_limit', 512.0),
                               ('mutation_process_limit', 256), ('mutation_process_limit', 0),
                               ('mutation_message_format', 'human'), ('unexpected_lane', 'host'),
                               ('mutation_wall_seconds', 2400), ('source_file_count', 68)]:
                policy = copy.deepcopy(healthy); policy[key] = value
                self.write_policy(root, policy)
                with self.subTest(key=key), self.assertRaises(MODULE.Incomplete):
                    MODULE.read_policy(root, 'coverage')
            self.write_policy(root, healthy)
            for name in ['crates/time-core/src/regression.rs', 'crates/time-core/tests/residuals.rs',
                         'crates/time-core/tests/support/residual_facts.rs', 'crates/time-core/PROVENANCE.md']:
                path = root / name; original = path.read_bytes(); path.write_bytes(original + b'changed\n')
                with self.subTest(changed=name), self.assertRaises(MODULE.Incomplete):
                    MODULE.read_policy(root, 'coverage')
                path.write_bytes(original)

    def test_missing_bindings_equivalences_and_inline_coverage_padding_reject(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); healthy = self.fixture(root)
            for change in ['missing', 'null', 'equivalence']:
                policy = copy.deepcopy(healthy)
                if change == 'missing': policy['inputs_sha256'].pop('Cargo.lock')
                if change == 'null': policy['inputs_sha256']['Cargo.lock'] = None
                if change == 'equivalence': policy['mutation_equivalences'] = [{'historical': 'C'}]
                self.write_policy(root, policy)
                with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                    MODULE.read_policy(root, 'coverage')
            for name in MODULE.SUBJECTS.values():
                policy = copy.deepcopy(healthy); path = root / name; original = path.read_bytes()
                path.write_text('#[test]\nfn padding_is_not_production_coverage(){}\n')
                self.rebind(root, policy)
                with self.subTest(inline=name), self.assertRaises(MODULE.Incomplete):
                    MODULE.read_policy(root, 'coverage')
                path.write_bytes(original)

    def test_complete_inventory_rejects_missing_extra_or_changed_product_shape(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); policy = self.fixture(root); actual = self.inert_product(root)[1]
            for name in ['crates/time-core/src/regression/residuals.rs',
                         'crates/time-core/tests/private/residual_guards.rs']:
                changed = dict(actual); changed.pop(name)
                with self.subTest(missing=name), patch.object(MODULE.PRODUCT, 'admit_product', return_value=({}, changed)):
                    with self.assertRaises(MODULE.Incomplete): MODULE.read_policy(root, 'coverage')
            changed = dict(actual, unexpected='f' * 64)
            with patch.object(MODULE.PRODUCT, 'admit_product', return_value=({}, changed)):
                with self.assertRaises(MODULE.Incomplete): MODULE.read_policy(root, 'coverage')
            changed = dict(actual); changed['crates/time-core/tests/residuals.rs'] = 'f' * 64
            with patch.object(MODULE.PRODUCT, 'admit_product', return_value=({}, changed)):
                with self.assertRaises(MODULE.Incomplete): MODULE.read_policy(root, 'coverage')
            self.assertEqual(MODULE.read_policy(root, 'coverage')['inputs_sha256'], policy['inputs_sha256'])

    def test_each_critical_file_requires_unique_security_and_critical_registration(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); healthy = self.fixture(root); path = root / 'policy/verification-manifest.toml'
            original = path.read_text()
            for filename in MODULE.SUBJECTS.values():
                quoted = json.dumps(filename)
                for change in [original.replace(quoted, '"missing.rs"', 1),
                               original.replace('critical_files=[', 'critical_files=[' + quoted + ',', 1),
                               original + '[[security_modules]]\npackage="time-core"\nfiles=[' + quoted + ']\n']:
                    policy = copy.deepcopy(healthy); path.write_text(change); self.rebind(root, policy)
                    with self.subTest(file=filename), self.assertRaises(MODULE.Incomplete):
                        MODULE.read_policy(root, 'coverage')
                path.write_text(original)
            self.rebind(root, healthy)
            MODULE.read_policy(root, 'coverage')

    def test_profile_counters_preserve_zero_private_guard_rows_and_sum_real_executables(self):
        text = ('CORE_PROFILE_COUNTS fits=0 points=0\nCORE_PROFILE_COUNTS fits=285 points=1329\n'
                'CORE_STATS_PROFILE_COUNTS stats=169 points=1330\n')
        result = MODULE.profile_counts(text)
        self.assertEqual(result['direct_fit_calls'], 285)
        self.assertEqual(result['fit_calls'], 285)
        self.assertEqual(result['stats_calls'], 169)
        self.assertEqual(result['internal_fit_upper_bound'], 169)
        self.assertEqual(result['observed_charged_fit_work'], 454)
        self.assertEqual(result['charged_fit_work_upper'], 454)
        self.assertEqual(result['point_admissions'], 2659)
        self.assertEqual(result['counter_records'], 3)
        self.assertEqual(MODULE.profile_counts('CORE_STATS_PROFILE_COUNTS stats=0 points=0', require_fit=False)['stats_calls'], 0)
        for value in ['', 'test result: ok. 1 passed; 0 failed; 0 ignored;',
                      'CORE_PROFILE_COUNTS fits=0 points=3', 'CORE_PROFILE_COUNTS fits=641 points=3',
                      'CORE_PROFILE_COUNTS fits=1 points=65537',
                      'CORE_PROFILE_COUNTS fits=1 points=3\nCORE_PROFILE_COUNTS fits=-1 points=4',
                      'CORE_STATS_PROFILE_COUNTS stats=-1 points=3',
                      'CORE_STATS_PROFILE_COUNTS stats=1 points=3\nCORE_STATS_PROFILE_COUNTS incomplete',
                      '--- STDERR:\nCORE_PROFILE_COUNTS fits=1 points=3',
                      'Summary [0s]\nCORE_STATS_PROFILE_COUNTS stats=1 points=3']:
            with self.subTest(value=value), self.assertRaises(MODULE.Incomplete): MODULE.profile_counts(value)

    def test_profile_work_and_point_endpoints_include_both_families(self):
        for stats, accepted in [(354, True), (355, True), (356, False)]:
            text = f'CORE_PROFILE_COUNTS fits=285 points=1329\nCORE_STATS_PROFILE_COUNTS stats={stats} points=3\n'
            if accepted: self.assertEqual(MODULE.profile_counts(text)['charged_fit_work_upper'], 285 + stats)
            else:
                with self.assertRaises(MODULE.Incomplete): MODULE.profile_counts(text)
        for points, accepted in [(65535, True), (65536, True), (65537, False)]:
            text = f'CORE_PROFILE_COUNTS fits=1 points=3\nCORE_STATS_PROFILE_COUNTS stats=1 points={points - 3}\n'
            if accepted: self.assertEqual(MODULE.profile_counts(text)['point_admissions_upper'], points)
            else:
                with self.assertRaises(MODULE.Incomplete): MODULE.profile_counts(text)

    def test_literal_doctests_are_upper_allowances_without_fabricating_observations(self):
        observed = MODULE.profile_counts('CORE_PROFILE_COUNTS fits=285 points=1329\nCORE_STATS_PROFILE_COUNTS stats=169 points=1330\n')
        charged = MODULE.account_literal_doctests(observed)
        self.assertEqual(charged['direct_fit_calls'], 285)
        self.assertEqual(charged['stats_calls'], 169)
        self.assertEqual(charged['observed_charged_fit_work'], 454)
        self.assertEqual(charged['charged_fit_work_upper'], 454)
        self.assertEqual(charged['point_admissions'], 2659)
        self.assertEqual(charged['point_admissions_upper'], 2662)
        self.assertEqual(charged['internal_fit_upper_bound'], 169)
        self.assertNotIn('literal_doctest_stats_calls_upper', observed)
        self.assertEqual(MODULE.account_literal_doctests(MODULE.profile_counts('CORE_PROFILE_COUNTS fits=640 points=0'))['charged_fit_work_upper'],640)
        for text in ['CORE_STATS_PROFILE_COUNTS stats=1 points=65534']:
            with self.subTest(text=text), self.assertRaises(MODULE.Incomplete):
                MODULE.account_literal_doctests(MODULE.profile_counts(text))

    def test_fuzz_distinguishes_raw_point_and_stats_counters_and_incomplete_controls(self):
        healthy = '\n'.join(self.fuzz_row(raw=n) for n in range(1, 4097)) + '\nDone 4096 runs\n'
        result = MODULE.fuzz_counts(healthy, complete=True)
        self.assertEqual(result['raw_driver_runs'], 4096)
        self.assertEqual(result['stats_calls'], 1)
        self.assertEqual(result['direct_fit_calls'], 0)
        self.assertEqual(result['fit_calls'], 0)
        self.assertEqual(result['internal_fit_upper_bound'], 1)
        self.assertEqual(result['point_admissions'], 0)
        self.assertEqual(result['generated_point_admissions_upper'], 64)
        control = self.fuzz_row() + '\nindependent normalization oracle'
        self.assertEqual(MODULE.fuzz_counts(control, complete=False)['stats_calls'], 1)
        for output in [control, self.fuzz_row(normalizations=0,stats=0,getters=0),
                       self.fuzz_row(observations=65), self.fuzz_row(stats=2),
                       self.fuzz_row()+'\n'+self.fuzz_row(),
                       'CORE_FUZZ_COUNTS raw=1 points=3 fits=1',
                       self.fuzz_row(raw=2)]:
            with self.subTest(output=output[:70]), self.assertRaises(MODULE.Incomplete): MODULE.fuzz_counts(output, complete=True)
        for tail in [self.fuzz_row(raw=2,observations=2),self.fuzz_row(raw=2,stats=0)]:
            with self.subTest(decrease=tail), self.assertRaises(MODULE.Incomplete):
                MODULE.fuzz_counts(control.splitlines()[0] + '\n' + tail, complete=False)

    def test_checkpoint_refuses_fuzz_even_with_historical_resource_profile_and_cohort(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); healthy = self.fixture(root); self.fuzz_fixture(root, healthy)
            MODULE.read_policy(root, 'contracts')
            with self.assertRaisesRegex(MODULE.Incomplete, 'one named selected-owner mutation cohort only'):
                MODULE.read_policy(root, 'fuzz')
            for key, value in [('profile', 'unreviewed-profile'), ('target_contract', 'fit-only'),
                               ('runtime_profile_reuse_review', 'pending-independent-resource-contract-equivalence'),
                               ('fuzzer_flags', ['-runs=1000000']), ('status', 'pending-s12b-target-source-binding')]:
                changed = copy.deepcopy(healthy); changed['coverage_guided_fuzzing'][key] = value
                self.write_policy(root, changed)
                with self.subTest(key=key), self.assertRaisesRegex(MODULE.Incomplete, 'one named selected-owner mutation cohort only'):
                    MODULE.read_policy(root, 'fuzz')
            changed = copy.deepcopy(healthy); changed['coverage_guided_fuzzing']['invocation_limits']['wall_seconds'] = 16
            self.write_policy(root, changed)
            with self.assertRaisesRegex(MODULE.Incomplete, 'one named selected-owner mutation cohort only'): MODULE.read_policy(root, 'fuzz')
            changed = copy.deepcopy(healthy); changed['coverage_guided_fuzzing']['selected_grants'] = {}
            self.write_policy(root, changed)
            with self.assertRaisesRegex(MODULE.Incomplete, 'one named selected-owner mutation cohort only'): MODULE.read_policy(root, 'fuzz')

    def test_real_consumer_specimens_have_ten_legal_controls_and_exact_declared_causes(self):
        cases = MODULE.legacy_contract_examples(); legal = [v for v in cases.values() if v['code'] is None]
        self.assertEqual(len(legal), 10)
        self.assertEqual(sum(v['code'] is not None for v in cases.values()), 120)
        self.assertEqual({k: sum(v['calls'][k] for v in legal) for k in ['direct_fit_calls', 'stats_calls', 'point_admissions']},
                         {'direct_fit_calls': 2, 'stats_calls': 2, 'point_admissions': 15})
        for name in ['point_construction', 'line_construction', 'residual_raw_update']:
            self.assertEqual(cases[name]['code'], 'E0451')
        for name in ['point_mutation_relative_seconds', 'input_mutation_len', 'line_mutation_phase_seconds']:
            self.assertEqual(cases[name]['code'], 'E0616')
        for name, case in cases.items():
            if case['code']:
                self.assertTrue(case['source'].splitlines()[case['line'] - 1].strip(), name)
                self.assertTrue(case['tokens'], name)
                source = '/work/contract-inputs/' + name + '.rs'
                diagnostic = synthetic_compiler_diagnostic(
                    case['code'], ' '.join(case['tokens']), source, case['line'])
                good = {'exit_code': 1, 'failure': None, 'output': json.dumps(diagnostic)}
                MODULE.COMMON.compiler_cause(good, case, source)
                wrong_code = 'E0308' if case['code'] == 'E0432' else 'E0432'
                for altered in [dict(diagnostic, code={'code': wrong_code, 'explanation': None}),
                                dict(diagnostic, message='unrelated setup failure'),
                                dict(diagnostic, spans=[synthetic_compiler_span(source, 999)]),
                                dict(diagnostic, spans=[synthetic_compiler_span(source, case['line'], is_primary=False)])]:
                    with self.subTest(case=name), self.assertRaises(MODULE.Incomplete):
                        MODULE.COMMON.compiler_cause(dict(good, output=json.dumps(altered)), case, source)
                with self.assertRaises(MODULE.Incomplete): MODULE.COMMON.compiler_cause(dict(good, exit_code=0), case, source)
                with self.assertRaises(MODULE.Incomplete): MODULE.COMMON.compiler_cause(dict(good, failure='timeout'), case, source)

    def test_mutation_receiving_requires_exact_fresh_output_before_session(self):
        """An admitted card cannot redirect or repeat this checkpoint's one attempt."""
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            for case in ('healthy', 'other-fresh', 'existing', 'dangling-symlink'):
                expected = root / case
                if case == 'existing': expected.mkdir()
                if case == 'dangling-symlink': expected.symlink_to(root / 'absent-target')
                actual = root / 'retry-output' if case == 'other-fresh' else expected
                session = Mock()
                report = {}
                with self.admitted_mutation_execution_fixture(session, report) as args, \
                     patch.dict(MODULE.COORDINATOR_COHORTS, {'mutation_outputs': {'reference_input': str(expected)}}), \
                     patch.object(MODULE.COMMON, 'Session', side_effect=MODULE.Incomplete('healthy session boundary')) as start:
                    args.output_dir = actual
                    expected_error = ('healthy session boundary' if case == 'healthy' else
                                      'exact fresh admitted mutation output directory')
                    with self.subTest(case=case):
                        with self.assertRaisesRegex(MODULE.Incomplete, expected_error):
                            MODULE.execute(args, report)
                        if case == 'healthy': start.assert_called_once()
                        else: start.assert_not_called()
                session.run.assert_not_called()
                session.finish.assert_not_called()

    def test_owned_mutation_raw_accounting_retains_two_files_and_unviable_survivors(self):
        with tempfile.TemporaryDirectory() as temp:
            work = Path(temp)
            for subject in MODULE.HISTORICAL_S149_SUBJECTS:
                out = work / 'mutation' / subject / 'mutants.out'; out.mkdir(parents=True)
                outcomes = [{'scenario': 'Baseline', 'summary': 'Success', 'log_path': 'baseline.log',
                             'phase_results': self.phase_results('Success')},
                            {'scenario': {'Mutant': {'name': 'guard_removed'}}, 'summary': 'CaughtMutant', 'log_path': 'killed.log',
                             'phase_results': self.phase_results('CaughtMutant')},
                            {'scenario': {'Mutant': {'name': 'wrong_sign'}}, 'summary': 'MissedMutant', 'log_path': 'missed.log',
                             'phase_results': self.phase_results('MissedMutant')},
                            {'scenario': {'Mutant': {'name': 'type_error'}}, 'summary': 'Unviable', 'log_path': 'compile.log',
                             'phase_results': self.phase_results('Unviable')}]
                common = ('CORE_PROFILE_COUNTS fits=1 points=3\n'
                          'CORE_STATS_PROFILE_COUNTS stats=1 points=3\n' + self.complete_runs_rows())
                for name in ['baseline.log', 'missed.log']:
                    (out / name).write_text(self.completed_mutation_log(common))
                (out / 'killed.log').write_text(self.completed_mutation_log(
                    common + '\nfailures:\n\n    frozen_assertion\n\n'
                    'test result: FAILED. 0 passed; 1 failed; 0 ignored;\n'))
                (out / 'compile.log').write_text(self.semantic_mutation_log())
                (out / 'outcomes.json').write_text(json.dumps({'total_mutants': 3, 'outcomes': outcomes}))
            report = {'subjects': dict(MODULE.HISTORICAL_S149_SUBJECTS)}
            # Inert parser control only; the real 640-work policy remains unchanged.
            with patch.dict(MODULE.PROFILE_LIMITS, {'charged_fit_work_upper': 2000}):
                MODULE.mutation_accounting(report, work)
            self.assertFalse(report['mutation_full_manifest_selected'])
            self.assertFalse(report['mutation_checkpoint_complete_selection'])
            self.assertEqual(report['mutation_baselines_required'], 2)
            self.assertEqual(report['mutation_baselines_observed'], 2)
            self.assertEqual(report['mutation_missing_selected_files'], [])
            for subject in MODULE.HISTORICAL_S149_SUBJECTS:
                row = report['mutation_full_accounting'][subject]
                self.assertEqual(row['counts'], {'K': 1, 'M': 1, 'U': 1, 'T': 0, 'X': 0, 'E': 0})
                self.assertEqual(row['accounted'], 3)
            # Two files each charge baseline, caught and missed packages; U adds no execution.
            self.assertEqual(report['core_charged_subset_totals'], {'charged_fit_work_upper': 378774, 'point_admissions_upper': 13520178})
            self.assertEqual(report['core_direct_and_stats_observations'], {'direct_fit_calls': 6, 'stats_calls': 6, 'point_admissions': 36})
            self.assertEqual(report['core_runs_external_observations']['runs_selections'], 120)
            self.assertTrue(report['core_recorded_observations_parsed'])
            self.assertFalse(report['mutation_counter_observation_complete'])
            self.assertEqual(report['core_invocations'][0]['observed_counts']['runs_selections'],20)
            baseline = work / 'mutation/adaptive_history/mutants.out/baseline.log'
            original = baseline.read_text(); baseline.write_text(self.completed_mutation_log(
                'test result: ok. 96 passed; 0 failed; 0 ignored;'))
            incomplete = {'subjects': dict(MODULE.HISTORICAL_S149_SUBJECTS)}
            with patch.dict(MODULE.PROFILE_LIMITS, {'charged_fit_work_upper': 2000}):
                with self.assertRaises(MODULE.Incomplete): MODULE.mutation_accounting(incomplete, work)
            self.assertEqual(incomplete['mutation_full_accounting']['adaptive_history']['accounted'], 3)
            principal_baseline = next(row for row in incomplete['core_invocations']
                                      if row['invocation'] == 'adaptive_history:mutation-baseline')
            self.assertEqual(principal_baseline['charged_fit_work_upper'],63129)
            self.assertNotIn('observed_counts',principal_baseline)
            self.assertFalse(incomplete['core_recorded_observations_parsed'])
            baseline.write_text(original)
            path = work / 'mutation/runs/mutants.out/outcomes.json'; raw = json.loads(path.read_text())
            raw['outcomes'][1]['log_path'] = '../outside.log'; path.write_text(json.dumps(raw))
            with patch.dict(MODULE.PROFILE_LIMITS, {'charged_fit_work_upper': 2000}):
                with self.assertRaises(MODULE.Incomplete): MODULE.mutation_accounting({'subjects': dict(MODULE.HISTORICAL_S149_SUBJECTS)}, work)

    def test_timeout_remains_unqualified_and_full_charge_without_complete_receipt(self):
        with tempfile.TemporaryDirectory() as temp:
            work = Path(temp); output = work / 'mutation/runs/mutants.out'
            output.mkdir(parents=True)
            (output / 'baseline.log').write_text(self.completed_mutation_log(self.complete_runs_rows()))
            (output / 'timeout.log').write_text(
                self.mutation_phase_header('Test') + 'CORE_RUNS_PROFILE points=2 inputs=0 selections=0\n')
            outcomes = [
                {'scenario': 'Baseline', 'summary': 'Success', 'log_path': 'baseline.log',
                 'phase_results': self.phase_results('Success')},
                {'scenario': {'Mutant': {'name': 'stalled'}}, 'summary': 'Timeout', 'log_path': 'timeout.log'},
                {'scenario': {'Mutant': {'name': 'missing'}}, 'summary': 'Timeout', 'log_path': 'missing.log'},
            ]
            (output / 'outcomes.json').write_text(json.dumps({'total_mutants': 2, 'outcomes': outcomes}))
            report = {'subjects': self.historical_scope('runs').select('all')}
            with patch.dict(MODULE.PROFILE_LIMITS, {'charged_fit_work_upper': 2000}):
                MODULE.mutation_accounting(report, work)
            # The successful baseline and both timeouts retain three complete package charges.
            self.assertEqual(report['core_charged_subset_totals'],
                             {'charged_fit_work_upper': 189387, 'point_admissions_upper': 6760089})
            self.assertEqual(report['mutation_full_accounting']['runs']['counts']['T'], 2)
            self.assertEqual(report['mutation_full_accounting']['runs']['counts']['K'], 0)
            self.assertFalse(report['mutation_counter_observation_complete'])
            self.assertEqual(report['core_observation_pending_invocations'], ['runs:mutation-1', 'runs:mutation-2'])
            for row in report['core_invocations'][1:]:
                self.assertEqual((row['charged_fit_work_upper'], row['point_admissions_upper']), (63129, 2253363))
                self.assertTrue(row['observation_pending'])
                self.assertNotIn('observed_counts', row)
            self.assertEqual(report['core_invocations'][1]['observation_status'], 'timeout-prefix-unqualified')
            self.assertEqual(report['core_invocations'][2]['observation_status'], 'timeout-log-missing')

    def test_failed_parse_keeps_full_pending_debit_and_explicit_incomplete_status(self):
        with tempfile.TemporaryDirectory() as temp:
            work = Path(temp); output = work / 'mutation/runs/mutants.out'
            output.mkdir(parents=True)
            (output / 'bad.log').write_text(self.completed_mutation_log(
                'CORE_RUNS_PROFILE points=2 inputs=0 selections=0 extra=1\n'))
            (output / 'outcomes.json').write_text(json.dumps({'total_mutants': 0, 'outcomes': [
                {'scenario': 'Baseline', 'summary': 'Success', 'log_path': 'bad.log',
                 'phase_results': self.phase_results('Success')}]}))
            report = {'subjects': self.historical_scope('runs').select('all')}
            with self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_accounting(report, work)
            self.assertEqual(report['core_charged_subset_totals'],
                             {'charged_fit_work_upper': 63129, 'point_admissions_upper': 2253363})
            self.assertFalse(report['core_recorded_observations_parsed'])
            self.assertFalse(report['mutation_counter_observation_complete'])
            self.assertEqual(report['core_observation_pending_invocations'], ['runs:mutation-baseline'])
            self.assertNotIn('observed_counts', report['core_invocations'][0])

    def test_timeout_with_existing_log_but_no_counter_prefix_remains_pending(self):
        with tempfile.TemporaryDirectory() as temp:
            work = Path(temp); output = work / 'mutation/runs/mutants.out'
            output.mkdir(parents=True)
            (output / 'baseline.log').write_text(self.completed_mutation_log(self.complete_runs_rows()))
            (output / 'timeout.log').write_text('child stopped before emitting a counter\n')
            (output / 'outcomes.json').write_text(json.dumps({'total_mutants': 1, 'outcomes': [
                {'scenario': 'Baseline', 'summary': 'Success', 'log_path': 'baseline.log',
                 'phase_results': self.phase_results('Success')},
                {'scenario': {'Mutant': {'name': 'no_prefix'}}, 'summary': 'Timeout', 'log_path': 'timeout.log'},
            ]}))
            report = {'subjects': self.historical_scope('runs').select('all')}
            with patch.dict(MODULE.PROFILE_LIMITS, {'charged_fit_work_upper': 2000}):
                MODULE.mutation_accounting(report, work)
            self.assertEqual(report['core_charged_subset_totals'],
                             {'charged_fit_work_upper': 126258, 'point_admissions_upper': 4506726})
            row = report['core_invocations'][1]
            self.assertEqual(row['observation_status'], 'timeout-no-counter-prefix')
            self.assertTrue(row['observation_pending'])
            self.assertNotIn('observed_counts', row)
            self.assertEqual(row['raw_log_sha256'], hashlib.sha256(
                b'child stopped before emitting a counter\n').hexdigest())
            self.assertFalse(report['mutation_counter_observation_complete'])

    def test_bad_early_subject_preserves_later_known_subject_pending_charge(self):
        with tempfile.TemporaryDirectory() as temp:
            work = Path(temp)
            for subject in ('runs', 'adaptive_history'):
                output = work / 'mutation' / subject / 'mutants.out'
                output.mkdir(parents=True)
                text = ('CORE_RUNS_PROFILE points=2 inputs=0 selections=0 extra=1\n'
                        if subject == 'runs' else self.complete_runs_rows())
                (output / 'baseline.log').write_text(self.completed_mutation_log(text))
                (output / 'outcomes.json').write_text(json.dumps({'total_mutants': 0, 'outcomes': [
                    {'scenario': 'Baseline', 'summary': 'Success', 'log_path': 'baseline.log',
                     'phase_results': self.phase_results('Success')}]}))
            report = {'subjects': dict(MODULE.HISTORICAL_S149_SUBJECTS)}
            with self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_accounting(report, work)
            self.assertEqual(report['core_charged_subset_totals'],
                             {'charged_fit_work_upper': 126258, 'point_admissions_upper': 4506726})
            self.assertEqual(report['core_observation_pending_invocations'],
                             ['runs:mutation-baseline', 'adaptive_history:mutation-baseline'])
            self.assertFalse(report['mutation_counter_observation_complete'])
            self.assertTrue(all('observed_counts' not in row for row in report['core_invocations']))

    def test_timeout_can_end_after_sign_receipt_before_owner_drop(self):
        with tempfile.TemporaryDirectory() as temp:
            work = Path(temp); output = work / 'mutation/runs/mutants.out'
            output.mkdir(parents=True)
            (output / 'baseline.log').write_text(self.completed_mutation_log(self.complete_runs_rows()))
            (output / 'timeout.log').write_text(
                self.mutation_phase_header('Test') + 'CORE_RUNS_SIGN_RECEIPT partial-fixture-result\n')
            (output / 'outcomes.json').write_text(json.dumps({'total_mutants': 1, 'outcomes': [
                {'scenario': 'Baseline', 'summary': 'Success', 'log_path': 'baseline.log',
                 'phase_results': self.phase_results('Success')},
                {'scenario': {'Mutant': {'name': 'before_drop'}},
                 'summary': 'Timeout', 'log_path': 'timeout.log'}]}))
            report = {'subjects': self.historical_scope('runs').select('all')}
            with patch.dict(MODULE.PROFILE_LIMITS, {'charged_fit_work_upper': 2000}):
                MODULE.mutation_accounting(report, work)
            row = report['core_invocations'][1]
            self.assertEqual(row['observation_status'], 'timeout-receipt-unqualified')
            self.assertTrue(row['observation_pending'])
            self.assertNotIn('observed_counts', row)
            self.assertEqual(row['charged_fit_work_upper'], 63129)
            self.assertFalse(report['mutation_counter_observation_complete'])

    def test_all_known_outcomes_are_charged_even_when_subset_cap_is_exceeded(self):
        with tempfile.TemporaryDirectory() as temp:
            work = Path(temp)
            for subject in ('runs', 'adaptive_history'):
                output = work / 'mutation' / subject / 'mutants.out'
                output.mkdir(parents=True)
                (output / 'outcomes.json').write_text(json.dumps({'total_mutants': 0, 'outcomes': [
                    {'scenario': 'Baseline', 'summary': 'Success', 'log_path': 'unread.log',
                     'phase_results': self.phase_results('Success')}]}))
            report = {'subjects': dict(MODULE.HISTORICAL_S149_SUBJECTS)}
            with patch.dict(MODULE.GLOBAL_LIMITS, {'charged_fit_work_upper': 63129}):
                with self.assertRaisesRegex(MODULE.Incomplete, 'global charged allowance'):
                    MODULE.mutation_accounting(report, work)
            self.assertEqual(report['core_charged_subset_totals'],
                             {'charged_fit_work_upper': 126258, 'point_admissions_upper': 4506726})
            self.assertEqual(len(report['core_observation_pending_invocations']), 2)
            self.assertFalse(report['mutation_counter_observation_complete'])

    @contextlib.contextmanager
    def admitted_mutation_execution_fixture(self, session, report):
        """Mock adjacent admission/custody only, to reach native-error cleanup.

        The complete coordinator reader has separate closed-plan controls. This
        inert fixture represents its admitted focused result and launches no tool.
        """
        plan_path = ROOT / 'tasks/inert-coordinator-plan.json'
        args = SimpleNamespace(lane='mutation', subject='reference_input', cohort_plan=plan_path,
                               cohort_id='reference_input', output_dir=Path('/inert-output'))
        policy = {'mutation_inventories': {name: {'count': 1, 'sha256': 'a'*64}
                                           for name in MODULE.SUBJECTS}}
        plan = {'candidate_sha256': 'b'*64, 'recipe_sha256': 'c'*64,
                'product_inputs_sha256': {}, 'tool_inputs_sha256': {}, 'tool_payloads_sha256': {},
                'cohorts': [{'id': 'reference_input', 'subject': 'reference_input', 'output_dir': '/inert-output'}]}
        session.product_identity = {}; session.identity = {}; session.source = Path('/inert-source')
        report['tools_sha256'] = {}
        with patch.dict(MODULE.COORDINATOR_COHORTS, {'mutation_outputs': {'reference_input': '/inert-output'}}), \
             patch.object(MODULE, 'read_policy', return_value=policy), \
             patch.object(MODULE, 'read_cohort_plan', return_value=(plan, {'synthetic_reservation': True}, plan_path)), \
             patch.object(MODULE.COHORTS, 'cohort_binding', return_value=plan['cohorts'][0]), \
             patch.object(MODULE, 'inventory_document', return_value=plan), \
             patch.object(MODULE.COMMON, 'Session', return_value=session):
            yield args

    def test_campaign_primary_error_survives_accounting_and_cleanup_failures(self):
        session = Mock(work=Path('/inert'), tools=Path('/inert-tools'))
        session.finish.side_effect = OSError('cleanup-failed')
        report = {}
        with self.admitted_mutation_execution_fixture(session, report) as args, \
             patch.object(MODULE.COMMON, 'run_analysis_lanes', side_effect=OSError('native-EAGAIN')), \
             patch.object(MODULE, 'mutation_accounting', side_effect=MODULE.Incomplete('accounting-malformed')):
            with self.assertRaisesRegex(OSError, 'native-EAGAIN'):
                MODULE.execute(args, report)
        self.assertEqual(report['secondary_errors'], [
            {'phase': 'mutation_accounting', 'type': 'Incomplete', 'error': 'accounting-malformed'},
            {'phase': 'session_finish', 'type': 'OSError', 'error': 'cleanup-failed'},
        ])
        session.finish.assert_called_once()

    def test_timeout_cannot_be_promoted_to_pass_after_common_lane_returns(self):
        session = Mock(work=Path('/inert'), tools=Path('/inert-tools'))
        report = {}
        def accounting(target, _work):
            target['mutation_missing_selected_files'] = []
            target['mutation_baselines_observed'] = 1
            target['mutation_full_accounting'] = {'chart': {'counts': {'T': 1}}}
        with self.admitted_mutation_execution_fixture(session, report) as args, \
             patch.object(MODULE.COMMON, 'run_analysis_lanes', return_value=None), \
             patch.object(MODULE, 'mutation_accounting', side_effect=accounting):
            with self.assertRaisesRegex(MODULE.Incomplete, 'unresolved native timeouts'):
                MODULE.execute(args, report)
        session.finish.assert_called_once()

    def test_accounting_error_remains_primary_when_only_cleanup_also_fails(self):
        session = Mock(work=Path('/inert'), tools=Path('/inert-tools'))
        session.finish.side_effect = OSError('cleanup-failed')
        report = {}
        with self.admitted_mutation_execution_fixture(session, report) as args, \
             patch.object(MODULE.COMMON, 'run_analysis_lanes', return_value=None), \
             patch.object(MODULE, 'mutation_accounting', side_effect=MODULE.Incomplete('accounting-malformed')):
            with self.assertRaisesRegex(MODULE.Incomplete, 'accounting-malformed'):
                MODULE.execute(args, report)
        self.assertEqual(report['secondary_errors'],
                         [{'phase': 'session_finish', 'type': 'OSError', 'error': 'cleanup-failed'}])

    def test_unviable_rejects_disjoint_or_forged_human_log_fragments(self):
        """Synthetic text fragments cannot independently establish native provenance."""
        outcome = {'summary': 'Unviable', 'phase_results': self.phase_results('Unviable')}
        owned = ' --> crates/time-core/src/regression/runs.rs:42:7\n'
        foreign = ' --> foreign/src/lib.rs:9:2\n'
        compile_end = ('error: could not compile ' + chr(96) + 'time-core' + chr(96)
                       + ' due to 1 previous error\n')
        dependency_end = ('error: could not compile ' + chr(96) + 'unrelated-dependency'
                          + chr(96) + ' due to 1 previous error\n')
        cases = {
            'disjoint_error_and_owned_note':
                'error[E0308]: mismatched types\n' + foreign
                + 'note: unrelated owned source\n' + owned + compile_end,
            'fake_diff_header':
                '--- displayed mutant diff, not compiler output ---\n'
                + 'error[E0308]: mismatched types\n' + owned + compile_end,
            'dependency_error_plus_owned_note':
                'error[E0308]: dependency type mismatch\n' + foreign
                + dependency_end + 'note: unrelated owned source\n' + owned,
            'semantic_plus_infrastructure_error':
                'error[E0308]: mismatched types\n' + owned + compile_end
                + 'error: failed to load manifest for workspace member: permission denied\n',
        }
        for name, text in cases.items():
            with self.subTest(case=name), self.assertRaises(MODULE.Incomplete):
                MODULE.unviable_compile_failure(outcome, text)

    def test_unviable_json_accepts_observed_unused_imports(self):
        outcome = {'summary': 'Unviable', 'phase_results': self.phase_results('Unviable')}
        record = self.semantic_record()
        record['message']['code'] = {'code': 'unused_imports', 'explanation': None}
        record['message']['message'] = 'unused imports: `ArithmeticStage` and `arithmetic`'
        record['message']['spans'] = [
            {'file_name': 'crates/time-core/src/regression/residuals.rs',
             'is_primary': True, 'line_start': 48, 'column_start': 26},
            {'file_name': 'crates/time-core/src/regression/residuals.rs',
             'is_primary': True, 'line_start': 49, 'column_start': 5}]
        MODULE.unviable_compile_failure(outcome, self.semantic_mutation_log(record))

    def test_unviable_json_accepts_only_observed_owned_unused_comparisons(self):
        outcome = {'summary': 'Unviable', 'phase_results': self.phase_results('Unviable')}
        for operator in ('<', '>='):
            record = self.unused_comparison_record(operator)
            log = (self.mutation_phase_header('Build') + json.dumps(record) + '\n'
                   + '{"reason":"build-finished","success":false}\n')
            with self.subTest(operator=operator):
                MODULE.unviable_compile_failure(outcome, log)

    def test_unused_comparison_lint_keeps_exact_cause_provenance_and_failure_guards(self):
        outcome = {'summary': 'Unviable', 'phase_results': self.phase_results('Unviable')}
        for change in ('unknown_code', 'message', 'explanation', 'code_shape', 'package',
                       'manifest', 'target', 'unowned_span', 'nonprimary_span', 'warning',
                       'infrastructure_child', 'infrastructure_raw', 'success', 'duplicate_json'):
            record = self.unused_comparison_record()
            if change == 'unknown_code':
                record['message']['code']['code'] = 'unused_comparisons_extra'
            elif change == 'message':
                record['message']['message'] += ' unrelated cause'
            elif change == 'explanation':
                record['message']['code']['explanation'] = 'unexpected explanation'
            elif change == 'code_shape':
                record['message']['code']['unexpected'] = None
            elif change == 'package':
                record['package_id'] = 'registry+https://example.invalid/dependency#1.0.0'
            elif change == 'manifest':
                record['manifest_path'] = '/work/tmp/cargo-mutants-source-v2TRlL.tmp/foreign/Cargo.toml'
            elif change == 'target':
                record['target']['kind'] = ['test']
            elif change == 'unowned_span':
                record['message']['spans'][0]['file_name'] = 'foreign/src/lib.rs'
            elif change == 'nonprimary_span':
                record['message']['spans'][0]['is_primary'] = False
            elif change == 'warning':
                record['message']['level'] = 'warning'
            elif change == 'infrastructure_child':
                record['message']['children'][0]['message'] = 'failed to spawn helper process'
            log = (self.mutation_phase_header('Build') + json.dumps(record) + '\n'
                   + '{"reason":"build-finished","success":false}\n')
            if change == 'infrastructure_raw':
                log += 'error: could not execute process: Resource temporarily unavailable (os error 11)\n'
            elif change == 'success':
                log = log.replace('"success":false', '"success":true')
            elif change == 'duplicate_json':
                log = log.replace('"code": "unused_comparisons"',
                                  '"code": "unused_comparisons", "code": "unused_comparisons"')
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.unviable_compile_failure(outcome, log)

    def test_unviable_json_accepts_observed_owned_test_relative_path(self):
        outcome = {'summary': 'Unviable', 'phase_results': self.phase_results('Unviable')}
        record = self.semantic_record()
        record['message']['code'] = {'code': 'arithmetic_overflow', 'explanation': None}
        record['message']['message'] = 'this arithmetic operation will overflow'
        record['message']['spans'][0] = {
            'file_name': 'crates/time-core/src/regression/../../tests/private/normalization_guards.rs',
            'is_primary': True, 'line_start': 340, 'column_start': 10}
        MODULE.unviable_compile_failure(outcome, self.semantic_mutation_log(record))

    def test_unviable_json_accepts_observed_const_trait_companion(self):
        outcome = {'summary': 'Unviable', 'phase_results': self.phase_results('Unviable')}
        coded = self.semantic_record()
        coded['message']['code'] = {'code': 'E0658', 'explanation': 'An unstable feature was used.'}
        coded['message']['message'] = ('cannot call conditionally-const method '
                                       '`<u64 as TryInto<NonZero<u64>>>::try_into` in constant functions')
        coded['message']['spans'][0] = {
            'file_name': 'crates/time-core/src/regression/residuals.rs',
            'is_primary': True, 'line_start': 349, 'column_start': 11}
        companion = copy.deepcopy(coded)
        companion['message']['code'] = None
        companion['message']['message'] = '`TryInto` is not yet stable as a const trait'
        companion['message']['spans'][0]['column_start'] = 9
        log = self.semantic_mutation_log(coded).replace(
            '{"reason":"build-finished","success":false}',
            json.dumps(companion) + '\n{"reason":"build-finished","success":false}')
        MODULE.unviable_compile_failure(outcome, log)

    def test_unviable_json_new_diagnostic_forms_fail_closed(self):
        outcome = {'summary': 'Unviable', 'phase_results': self.phase_results('Unviable')}
        base = self.semantic_record()
        for filename in ('crates/time-core/src/regression/../../../foreign.rs',
                         'crates/time-core/src/regression/../../tests/private/unowned.rs',
                         'crates/time-core/src/regression/../../../../crates/time-core/src/regression.rs'):
            record = copy.deepcopy(base)
            record['message']['code'] = {'code': 'arithmetic_overflow', 'explanation': None}
            record['message']['spans'][0]['file_name'] = filename
            with self.subTest(filename=filename), self.assertRaises(MODULE.Incomplete):
                MODULE.unviable_compile_failure(outcome, self.semantic_mutation_log(record))
        for code in ('unknown_lint', None):
            record = copy.deepcopy(base)
            record['message']['code'] = None if code is None else {'code': code, 'explanation': None}
            with self.subTest(code=code), self.assertRaises(MODULE.Incomplete):
                MODULE.unviable_compile_failure(outcome, self.semantic_mutation_log(record))
        coded = copy.deepcopy(base)
        coded['message']['code'] = {'code': 'E0658', 'explanation': None}
        coded['message']['message'] = ('cannot call conditionally-const method '
                                       '`<u64 as TryInto<NonZero<u64>>>::try_into` in constant functions')
        coded['message']['spans'][0].update(
            file_name='crates/time-core/src/regression/residuals.rs', line_start=349)
        companion = copy.deepcopy(coded)
        companion['message']['code'] = None
        companion['message']['message'] = '`TryInto` is not yet stable as a const trait'
        with self.assertRaises(MODULE.Incomplete):
            MODULE.unviable_compile_failure(outcome, self.semantic_mutation_log(companion))
        wrong_coded = copy.deepcopy(coded)
        wrong_coded['message']['code']['code'] = 'E0308'
        log = self.semantic_mutation_log(wrong_coded).replace(
            '{"reason":"build-finished","success":false}',
            json.dumps(companion) + '\n{"reason":"build-finished","success":false}')
        with self.assertRaises(MODULE.Incomplete):
            MODULE.unviable_compile_failure(outcome, log)
        for change in ('message', 'unowned', 'other_line', 'dependency', 'infrastructure'):
            invalid = copy.deepcopy(companion)
            if change == 'message':
                invalid['message']['message'] += ' unexpected'
            elif change == 'unowned':
                invalid['message']['spans'][0]['file_name'] = 'foreign/src/lib.rs'
            elif change == 'other_line':
                invalid['message']['spans'][0]['line_start'] = 350
            elif change == 'dependency':
                invalid['package_id'] = 'registry+https://example.invalid/dependency#1.0.0'
            else:
                invalid['message']['children'] = [{
                    'level': 'note', 'message': 'failed to spawn helper process',
                    'children': [], 'code': None, 'spans': [], 'rendered': None}]
            log = self.semantic_mutation_log(coded).replace(
                '{"reason":"build-finished","success":false}',
                json.dumps(invalid) + '\n{"reason":"build-finished","success":false}')
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.unviable_compile_failure(outcome, log)

    def test_unviable_json_requires_one_owned_same_record_and_build_failure(self):
        outcome = {'summary': 'Unviable', 'phase_results': self.phase_results('Unviable')}
        MODULE.unviable_compile_failure(outcome, self.semantic_mutation_log())
        for code in ('unused_variables', 'dead_code'):
            lint = self.semantic_record()
            lint['message']['code'] = {'code': code, 'explanation': None}
            lint['message']['children'] = [{
                'message': 'the lint level is defined here', 'level': 'note', 'children': [],
                'code': None, 'rendered': None, 'spans': []}]
            with self.subTest(admitted_lint=code):
                MODULE.unviable_compile_failure(outcome, self.semantic_mutation_log(lint))
        base = self.semantic_record()
        wrong_package = copy.deepcopy(base)
        wrong_package['package_id'] = 'registry+https://example.invalid/dependency#1.0.0'
        wrong_manifest = copy.deepcopy(base)
        wrong_manifest['manifest_path'] = '/work/tmp/cargo-mutants-source-v2TRlL.tmp/foreign/Cargo.toml'
        wrong_target = copy.deepcopy(base)
        wrong_target['target']['src_path'] = '/work/tmp/cargo-mutants-source-v2TRlL.tmp/foreign/src/lib.rs'
        wrong_span = copy.deepcopy(base)
        wrong_span['message']['spans'][0]['file_name'] = 'foreign/src/lib.rs'
        null_code = copy.deepcopy(base)
        null_code['message']['code'] = None
        unknown_code = copy.deepcopy(base)
        unknown_code['message']['code']['code'] = 'unsupported_lint'
        mixed_infrastructure = copy.deepcopy(base)
        mixed_infrastructure['message']['children'] = [{
            'level': 'note', 'message': 'failed to spawn helper process',
            'children': [], 'code': None, 'spans': [], 'rendered': None}]
        wrong_flag = copy.deepcopy(outcome)
        wrong_flag['phase_results'][0]['argv'].remove('--message-format=json')
        bad_status = copy.deepcopy(outcome)
        bad_status['phase_results'][0]['process_status'] = {'Failure': 2}
        duplicate = self.semantic_mutation_log().replace(
            '"level": "error"', '"level": "error", "level": "error"')
        nonfinite = self.semantic_mutation_log().replace('"line_start": 42', '"line_start": NaN')
        overflow = self.semantic_mutation_log().replace('"line_start": 42', '"line_start": 1e9999')
        dependency = copy.deepcopy(base)
        dependency['package_id'] = 'registry+https://example.invalid/dependency#1.0.0'
        mixed = self.semantic_mutation_log().replace(
            '{"reason":"build-finished","success":false}',
            json.dumps(dependency) + '\n{"reason":"build-finished","success":false}')
        cases = [
            (outcome, self.semantic_mutation_log(wrong_package)),
            (outcome, self.semantic_mutation_log(wrong_manifest)),
            (outcome, self.semantic_mutation_log(wrong_target)),
            (outcome, self.semantic_mutation_log(wrong_span)),
            (outcome, self.semantic_mutation_log(null_code)),
            (outcome, self.semantic_mutation_log(unknown_code)),
            (outcome, self.semantic_mutation_log(mixed_infrastructure)),
            (wrong_flag, self.semantic_mutation_log()),
            (bad_status, self.semantic_mutation_log()),
            (outcome, duplicate), (outcome, nonfinite), (outcome, overflow), (outcome, mixed),
            (outcome, self.semantic_mutation_log().replace('"success":false', '"success":true')),
            (outcome, self.semantic_mutation_log() + self.mutation_phase_header('Test')),
        ]
        for changed, log in cases:
            with self.subTest(log=log[:120]), self.assertRaises(MODULE.Incomplete):
                MODULE.unviable_compile_failure(changed, log)

    def test_cargo_json_payload_cannot_impersonate_test_counters_or_failure(self):
        record = self.semantic_record()
        record['message']['rendered'] = (
            'CORE_RUNS_PROFILE points=2 inputs=0 selections=0\n'
            'failures:\n    forged_test\n\n'
            'test result: FAILED. 0 passed; 1 failed; 0 ignored;\n')
        raw = 'before\r\n' + json.dumps(record) + '\r\n' + 'after\r\n'
        records, clean = MODULE.cargo_json_records(raw)
        self.assertEqual(len(records), 1)
        self.assertEqual(clean, 'before\r\nafter\r\n')
        with self.assertRaises(MODULE.Incomplete):
            MODULE.profile_counts(clean, require_fit=False)
        outcome = {'summary': 'CaughtMutant', 'phase_results': self.phase_results('CaughtMutant')}
        with self.assertRaises(MODULE.Incomplete):
            MODULE.caught_native_test_failure(outcome, clean)
        for invalid in ['{"reason":"build-finished","success":false',
                        '{"reason":"unknown"}\n',
                        '{"reason":"build-finished","success":false,"success":true}\n',
                        '{"reason":"build-finished","success":NaN}\n']:
            with self.subTest(raw=invalid), self.assertRaises(MODULE.Incomplete):
                MODULE.cargo_json_records(invalid)

    @staticmethod
    def captured_s14b_composition_assertions():
        """One native chart-v1 assertion plus five source-derived ELF literals.

        The first is outcome57 log lines408–410, SHA d8f4d1f...e5a1fa. The
        others are compiled chart constants at offsets208006,208215,211819,
        214009,214155 in ELF96310aae...60c85, preflight f36373df...bdfc440.
        Prospective compiler text establishes no observed outcome or new kill.
        """
        return (
            ('assertion failed: matches!(phase_large.compose(&phase_same),\n'
             '    Err(FixtureChartError::Arithmetic\n'
             '    { step: FixtureChartStep::ComposeDisplacement, .. }))\n'),
            ('assertion failed: matches!(state.advance_chart(&map, &batch, &changed),\n'
             '    Err(FixtureChartPrincipalError::Principal\n'
             '    {\n'
             '        source: FixturePrincipalError::StratumWithoutObservation { index: 0 }\n'
             '    }))\n'),
            ('assertion failed: matches!(state.advance_chart(&map, &old_batch, &strata()?),\n'
             '    Err(FixtureChartPrincipalError::Principal\n'
             '    { source: FixturePrincipalError::BackwardObservation { index: 0 } }))\n'),
            ('assertion failed: matches!(error, FixtureChartBatchError::Batch\n'
             '    { source: BatchError::DuplicateLabel { later_index: 1 } })\n'),
            ('assertion failed: matches!(rate_large.compose(&rate_same),\n'
             '    Err(FixtureChartError::Arithmetic\n'
             '    { step: FixtureChartStep::ComposeRate, .. }))\n'),
            ('assertion failed: matches!(phase_large.compose(&phase_cancel),\n'
             '    Err(FixtureChartError::Bound\n'
             '    {\n'
             '        operation: FixtureChartBoundOperation::Add, issue:\n'
             '        FixtureChartBoundIssue::NonFiniteResult\n'
             '    }))\n'),
        )

    def test_s14b_composition_transport_healthy_neighbor(self):
        cargo = '{"reason":"build-finished","success":true}\n'
        ordinary = 'running 1 test\ntest ordinary_native_neighbor ... ok\n'
        for index, _block in enumerate(self.captured_s14b_composition_assertions()):
            for newline in ('\n', '\r\n'):
                with self.subTest(block=index, newline=repr(newline)):
                    records, cleaned = MODULE.cargo_json_records(
                        (cargo + ordinary).replace('\n', newline), test_phase=True)
                    self.assertEqual(records, [{'reason': 'build-finished', 'success': True}])
                    self.assertEqual(cleaned, ordinary.replace('\n', newline))

    def test_s14b_composition_assertion_preserves_exact_test_bytes(self):
        cargo = '{"reason":"build-finished","success":true}\n'
        ordinary = 'running 1 test\ntest ordinary_native_neighbor ... ok\n'
        for block in self.captured_s14b_composition_assertions():
            for newline in ('\n', '\r\n'):
                stream = (cargo + ordinary + block + 'native-after\n').replace('\n', newline)
                expected = (ordinary + block + 'native-after\n').replace('\n', newline)
                with self.subTest(block=block, newline=repr(newline)):
                    records, cleaned = MODULE.cargo_json_records(stream, test_phase=True)
                    self.assertEqual(records, [{'reason': 'build-finished', 'success': True}])
                    self.assertEqual(cleaned, expected)
                    caught = {'summary': 'CaughtMutant', 'phase_results': self.phase_results('CaughtMutant')}
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.caught_native_test_failure(caught, cleaned)

    def test_s14b_composition_assertion_keeps_partial_near_miss_and_build_closed(self):
        for block in self.captured_s14b_composition_assertions():
            lines = block.splitlines(keepends=True)
            brace_index = next(index for index, line in enumerate(lines)
                               if line.lstrip().startswith('{'))
            for bad in (
                lines[brace_index], block.partition('\n')[2], block.rstrip('\n'),
                block.replace('assertion failed: matches!(', 'assertion failed: other!(', 1),
                block.replace('FixtureChart', 'FixtureAlien', 1),
                block.replace('    {', '   {', 1), '    {\n',
            ):
                with self.subTest(block=block, bad=bad), self.assertRaises(MODULE.Incomplete):
                    MODULE.cargo_json_records(bad, test_phase=True)
            prefix = ''.join(lines[:brace_index])
            self.assertEqual(MODULE.cargo_json_records(prefix, test_phase=True), ([], prefix))
            caught = {'summary': 'CaughtMutant', 'phase_results': self.phase_results('CaughtMutant')}
            with self.assertRaises(MODULE.Incomplete):
                MODULE.caught_native_test_failure(caught, prefix)
            for options in ({}, {'test_phase': False}):
                with self.subTest(block=block, options=options), self.assertRaises(MODULE.Incomplete):
                    MODULE.cargo_json_records(block, **options)
            unviable = {'summary': 'Unviable', 'phase_results': self.phase_results('Unviable')}
            healthy = self.semantic_mutation_log()
            MODULE.unviable_compile_failure(unviable, healthy)
            build = healthy.replace(self.mutation_phase_header('Build'),
                                    self.mutation_phase_header('Build') + block, 1)
            with self.subTest(block=block), self.assertRaises(MODULE.Incomplete):
                MODULE.unviable_compile_failure(unviable, build)

    def test_s14b_composition_assertion_keeps_malformed_cargo_closed(self):
        for block in self.captured_s14b_composition_assertions():
            for malformed in (
                '{"reason":"build-finished","success":true\n',
                '{"reason":"build-finished","success":false,"success":true}\n',
                '{"reason":"build-finished","success":NaN}\n',
                '{"reason":"build-finished","success":Infinity}\n',
                '{"reason":"build-finished","success":1e999}\n',
                '{"reason":"unknown"}\n',
                '{"reason":"build-finished","success":true,"extra":null}\n',
            ):
                with self.subTest(block=block, malformed=malformed), self.assertRaises(MODULE.Incomplete):
                    MODULE.cargo_json_records(block + malformed, test_phase=True)

    @staticmethod
    def captured_standard_error_root_assertion():
        """Exact S151 outcome16 Test panic, retained log lines346–351."""
        return (
            'assertion failed: matches!(error.source().and_then(|source| source.downcast_ref::<FitError>()),\n'
            '    Some(FitError::SquareRoot\n'
            '    {\n'
            '        stage: ArithmeticStage::Response, issue:\n'
            '        ArithmeticIssue::NegativeRadicand\n'
            '    }))\n'
        )

    def test_standard_error_assertion_preserves_exact_test_bytes(self):
        block = self.captured_standard_error_root_assertion()
        cargo = '{"reason":"build-finished","success":true}\n'
        neighbor = 'running 1 test\ntest ordinary_native_neighbor ... ok\n'
        for newline in ('\n', '\r\n'):
            stream = (cargo + neighbor + block + 'native-after\n').replace('\n', newline)
            expected = (neighbor + block + 'native-after\n').replace('\n', newline)
            with self.subTest(newline=repr(newline)):
                records, cleaned = MODULE.cargo_json_records(stream, test_phase=True)
                self.assertEqual(records, [{'reason': 'build-finished', 'success': True}])
                self.assertEqual(cleaned, expected)
                caught = {'summary': 'CaughtMutant', 'phase_results': self.phase_results('CaughtMutant')}
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.caught_native_test_failure(caught, cleaned)

    def test_standard_error_assertion_rejects_partial_and_near_miss_blocks(self):
        block = self.captured_standard_error_root_assertion()
        rejected = (
            '    {\n', block.partition('\n')[2], block.rstrip('\n'),
            block.rsplit('\n', 2)[0] + '\n',
            block.replace('NegativeRadicand', 'NonFinite'),
            block.replace('ArithmeticStage::Response', 'ArithmeticStage::Geometry'),
            block.replace('FitError::SquareRoot', 'FitError::Arithmetic'),
            block.replace('error.source()', 'other.source()'),
        )
        for raw in rejected:
            with self.subTest(raw=raw), self.assertRaises(MODULE.Incomplete):
                MODULE.cargo_json_records(raw, test_phase=True)

    def test_standard_error_assertion_does_not_admit_build_or_bad_json(self):
        block = self.captured_standard_error_root_assertion()
        for mode in (None, False):
            with self.subTest(mode=mode), self.assertRaises(MODULE.Incomplete):
                if mode is None:
                    MODULE.cargo_json_records(block)
                else:
                    MODULE.cargo_json_records(block, test_phase=mode)
        for malformed in (
            '{"reason":"build-finished","success":true\n',
            '{"reason":"build-finished","success":false,"success":true}\n',
            '{"reason":"build-finished","success":NaN}\n',
            '{"reason":"unknown"}\n',
        ):
            with self.subTest(malformed=malformed), self.assertRaises(MODULE.Incomplete):
                MODULE.cargo_json_records(block + malformed, test_phase=True)

    @staticmethod
    def captured_principal_assertion_blocks():
        """Literal native Test assertions from the retained principal-v1 logs.

        The first block is from outcome15/log381. The second occurs in
        outcomes107/109/128; its two brace-only lines are assertion source,
        rather than Cargo transport records. No recorded log is rewritten.
        """
        return (
            'assertion failed: matches!(rejected,Err(FixturePrincipalError::InvalidParameter\n'
            '    {field:actual_field,issue:actual_issue}) if actual_field==field &&\n'
            '    actual_issue==issue)\n',
            'assertion failed: matches!(rejected,\n'
            '    Err(FixturePrincipalError::Arithmetic\n'
            '    {\n'
            '        step: FixturePrincipalStep::StratumPenalty, source:\n'
            '        FitError::Arithmetic\n'
            '        {\n'
            '            stage: ArithmeticStage::Principal, operation:\n'
            '            ArithmeticOperation::Multiply, issue: ArithmeticIssue::NonFinite\n'
            '        },\n'
            '    }))\n',
        )

    def test_cargo_json_test_context_preserves_complete_native_principal_assertions(self):
        cargo = '{"reason":"build-finished","success":true}\n'
        ordinary = 'running 1 test\ntest ordinary_native_neighbor ... ok\n'
        records, cleaned = MODULE.cargo_json_records(cargo + ordinary, test_phase=True)
        self.assertEqual(records, [{'reason': 'build-finished', 'success': True}])
        self.assertEqual(cleaned, ordinary)
        for block in self.captured_principal_assertion_blocks():
            for newline in ('\n', '\r\n'):
                stream = (cargo + ordinary + block + 'native-after\n').replace('\n', newline)
                expected = (ordinary + block + 'native-after\n').replace('\n', newline)
                with self.subTest(block=block, newline=repr(newline)):
                    records, cleaned = MODULE.cargo_json_records(stream, test_phase=True)
                    self.assertEqual(records, [{'reason': 'build-finished', 'success': True}])
                    self.assertEqual(cleaned, expected)

    def test_principal_assertion_transport_keeps_unknown_partial_and_bad_json_closed(self):
        parameter, arithmetic = self.captured_principal_assertion_blocks()
        rejected = (
            '    {\n',
            '        {\n',
            parameter.splitlines(keepends=True)[1],
            parameter.replace('InvalidParameter', 'UnknownParameter'),
            parameter.replace('actual_field==field', 'actual_field!=field'),
            arithmetic.replace('StratumPenalty', 'UnknownStep'),
            arithmetic.replace('ArithmeticOperation::Multiply', 'ArithmeticOperation::Add'),
            parameter.rsplit('\n', 2)[0] + '\n',
            arithmetic.rsplit('\n', 3)[0] + '\n',
            parameter.rstrip('\n'),
            arithmetic.rstrip('\n'),
        )
        for block in rejected:
            with self.subTest(block=block), self.assertRaises(MODULE.Incomplete):
                MODULE.cargo_json_records(block, test_phase=True)
        malformed = (
            '{"reason":"build-finished","success":true\n',
            '{"reason":"build-finished","success":false,"success":true}\n',
            '{"reason":"build-finished","success":NaN}\n',
            '{"reason":"build-finished","success":Infinity}\n',
            '{"reason":"build-finished","success":1e999}\n',
        )
        for block in (parameter, arithmetic):
            for record in malformed:
                with self.subTest(block=block, record=record), self.assertRaises(MODULE.Incomplete):
                    MODULE.cargo_json_records(block + record, test_phase=True)

    def test_principal_assertion_transport_never_admits_a_build_or_unviable_cause(self):
        unviable = {'summary': 'Unviable', 'phase_results': self.phase_results('Unviable')}
        healthy = self.semantic_mutation_log()
        MODULE.unviable_compile_failure(unviable, healthy)
        for block in self.captured_principal_assertion_blocks():
            for mode in (None, False):
                with self.subTest(block=block, mode=mode), self.assertRaises(MODULE.Incomplete):
                    if mode is None:
                        MODULE.cargo_json_records(block)
                    else:
                        MODULE.cargo_json_records(block, test_phase=mode)
            build = healthy.replace(self.mutation_phase_header('Build'),
                                    self.mutation_phase_header('Build') + block, 1)
            with self.subTest(block=block), self.assertRaises(MODULE.Incomplete):
                MODULE.unviable_compile_failure(unviable, build)

    @staticmethod
    def captured_lifecycle_missing_metadata_assertion():
        """Exact principal-v5 outcome110/126 panic from lifecycle source1806."""
        return (
            'assertion failed: matches!(missing,\n'
            '    Err(FixtureCombinationLifecycleError::Combination\n'
            '    { source: FixtureCombinationError::MetadataCountMismatch }))\n'
        )

    def test_cargo_json_test_context_preserves_complete_lifecycle_metadata_assertion(self):
        block = self.captured_lifecycle_missing_metadata_assertion()
        cargo = '{"reason":"build-finished","success":true}\n'
        ordinary = 'running 1 test\ntest ordinary_native_neighbor ... ok\n'
        for newline in ('\n', '\r\n'):
            stream = (cargo + ordinary + block + 'native-after\n').replace('\n', newline)
            expected = (ordinary + block + 'native-after\n').replace('\n', newline)
            records, cleaned = MODULE.cargo_json_records(stream, test_phase=True)
            self.assertEqual(records, [{'reason': 'build-finished', 'success': True}])
            self.assertEqual(cleaned, expected)
            caught = {'summary': 'CaughtMutant', 'phase_results': self.phase_results('CaughtMutant')}
            with self.assertRaises(MODULE.Incomplete):
                MODULE.caught_native_test_failure(caught, cleaned)

    def test_lifecycle_metadata_assertion_keeps_near_misses_and_bad_json_closed(self):
        block = self.captured_lifecycle_missing_metadata_assertion()
        rejected = (
            block.splitlines(keepends=True)[-1],
            block.replace('matches!(missing,', 'matches!(rejected,'),
            block.replace('MetadataCountMismatch', 'MetadataMismatch'),
            block.replace('FixtureCombinationLifecycleError', 'FixturePrincipalError'),
            block.partition('\n')[2],
            block.rstrip('\n'),
            '    {\n',
        )
        for value in rejected:
            with self.subTest(value=value), self.assertRaises(MODULE.Incomplete):
                MODULE.cargo_json_records(value, test_phase=True)
        malformed = (
            '{"reason":"build-finished","success":true\n',
            '{"reason":"build-finished","success":false,"success":true}\n',
            '{"reason":"build-finished","success":NaN}\n',
            '{"reason":"build-finished","success":Infinity}\n',
            '{"reason":"build-finished","success":1e999}\n',
            '{"reason":"unknown"}\n',
        )
        for record in malformed:
            with self.subTest(record=record), self.assertRaises(MODULE.Incomplete):
                MODULE.cargo_json_records(block + record, test_phase=True)

    def test_lifecycle_metadata_assertion_cannot_supply_a_build_or_unviable_cause(self):
        block = self.captured_lifecycle_missing_metadata_assertion()
        for options in ({}, {'test_phase': False}):
            with self.subTest(options=options), self.assertRaises(MODULE.Incomplete):
                MODULE.cargo_json_records(block, **options)
        unviable = {'summary': 'Unviable', 'phase_results': self.phase_results('Unviable')}
        healthy = self.semantic_mutation_log()
        build = healthy.replace(self.mutation_phase_header('Build'),
                                self.mutation_phase_header('Build') + block, 1)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.unviable_compile_failure(unviable, build)
        MODULE.unviable_compile_failure(unviable, healthy)


    @staticmethod
    def captured_s157_chart_principal_assertions():
        """Exact principal-v1 outcomes114/132 and chart_principal-v2 outcomes25/26.

        The new v2 blocks come from unchanged logs8ac6000b...5f17 and
        ebd80e05...1c89. They establish only non-JSON Test text.
        """
        return (
            'assertion failed: matches!(state.advance_chart(&map, &both_batch, &strata()?),\n'
            '    Err(FixtureChartPrincipalError::Principal\n'
            '    { source: FixturePrincipalError::MultipleObservations }))\n',
            'assertion failed: matches!(state.advance_chart(&map, &batch, &strata()?),\n'
            '    Err(FixtureChartPrincipalError::Principal\n'
            '    { source: FixturePrincipalError::BackwardEvaluation }))\n',
            'assertion failed: matches!(state.advance_chart(&expected, &batch, &strata()?),\n'
            '    Err(FixtureChartPrincipalError::ChartMismatch\n'
            '    { kind: FixtureChartMismatchKind::Frames }))\n',
            'assertion failed: matches!(state.advance_chart(&expected, &batch, &many),\n'
            '    Err(FixtureChartPrincipalError::Principal\n'
            '    { source: FixturePrincipalError::MetadataCapacityExceeded }))\n',
        )

    def test_s157_complete_chart_principal_assertions_are_only_retained_test_text(self):
        cargo = '{"reason":"build-finished","success":true}\n'
        ordinary = 'running 1 test\ntest ordinary_native_neighbor ... ok\n'
        self.assertEqual(MODULE.cargo_json_records(cargo + ordinary, test_phase=True),
                         ([{'reason': 'build-finished', 'success': True}], ordinary))
        for block in self.captured_s157_chart_principal_assertions():
            for newline in ('\n', '\r\n'):
                text = (cargo + ordinary + block + 'native-after\n').replace('\n', newline)
                expected = (ordinary + block + 'native-after\n').replace('\n', newline)
                with self.subTest(block=block, newline=repr(newline)):
                    records, cleaned = MODULE.cargo_json_records(text, test_phase=True)
                    self.assertEqual(records, [{'reason': 'build-finished', 'success': True}])
                    self.assertEqual(cleaned, expected)
                    caught = {'summary': 'CaughtMutant', 'phase_results': self.phase_results('CaughtMutant')}
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.caught_native_test_failure(caught, cleaned)

    def test_s157_chart_assertions_keep_bare_partial_near_miss_and_bad_json_closed(self):
        for block in self.captured_s157_chart_principal_assertions():
            for bad in (
                block.splitlines(keepends=True)[-1],
                block.partition('\n')[2],
                block.rstrip('\n'),
                block.replace('FixtureChartPrincipalError', 'FixtureAlienError'),
                block.replace('state.advance_chart', 'other.advance_chart'),
                block.replace('    {', '   {'),
                '    {\n',
            ):
                with self.subTest(bad=bad), self.assertRaises(MODULE.Incomplete):
                    MODULE.cargo_json_records(bad, test_phase=True)
            for malformed in (
                '{"reason":"build-finished","success":true\n',
                '{"reason":"build-finished","success":false,"success":true}\n',
                '{"reason":"build-finished","success":NaN}\n',
                '{"reason":"build-finished","success":Infinity}\n',
                '{"reason":"build-finished","success":1e999}\n',
                '{"reason":"unknown"}\n',
            ):
                with self.subTest(malformed=malformed), self.assertRaises(MODULE.Incomplete):
                    MODULE.cargo_json_records(block + malformed, test_phase=True)

    def test_s157_chart_assertions_never_relax_build_json_receiving(self):
        unviable = {'summary': 'Unviable', 'phase_results': self.phase_results('Unviable')}
        healthy = self.semantic_mutation_log()
        MODULE.unviable_compile_failure(unviable, healthy)
        for block in self.captured_s157_chart_principal_assertions():
            for options in ({}, {'test_phase': False}):
                with self.subTest(block=block, options=options), self.assertRaises(MODULE.Incomplete):
                    MODULE.cargo_json_records(block, **options)
            build = healthy.replace(self.mutation_phase_header('Build'),
                                    self.mutation_phase_header('Build') + block, 1)
            with self.subTest(block=block), self.assertRaises(MODULE.Incomplete):
                MODULE.unviable_compile_failure(unviable, build)

    def test_s157_zero_prefix_qualification_requires_caught_and_native_test_101(self):
        text = ADAPTIVE_FIXTURES.RAW_EVENT_ZERO.replace(
            'FixtureCombinationLifecycleError', 'FixturePrincipalError')
        caught = {'summary': 'CaughtMutant', 'phase_results': self.phase_results('CaughtMutant')}
        MODULE.caught_native_test_failure(caught, text)
        token = MODULE.ADAPTIVE.caught_raw_event_zero_prefix(text)
        self.assertIsNotNone(token)
        for summary in ('MissedMutant', 'Unviable', 'Success'):
            rejected = dict(caught, summary=summary)
            with self.subTest(summary=summary), self.assertRaises(MODULE.Incomplete):
                MODULE.caught_native_test_failure(rejected, text)
        wrong_exit = copy.deepcopy(caught)
        wrong_exit['phase_results'][-1]['process_status'] = 'Success'
        with self.assertRaises(MODULE.Incomplete):
            MODULE.caught_native_test_failure(wrong_exit, text)
        with self.assertRaises(MODULE.ADAPTIVE.Incomplete):
            MODULE.ADAPTIVE.package_counts(text, require_complete=True, event_zero_prefix=token)

    def test_cargo_json_test_context_preserves_only_literal_assertion_tails(self):
        """Literal transport controls preserve panic bytes; they prove no Rust failure or kill."""
        tails = (
            '    { stage: SelectionInvariantStage::Component }))\n',
            '    { stage: SelectionInvariantStage::Support }))\n',
            '    { stage: SelectionInvariantStage::Intersection }))\n',
        )
        for tail in tails:
            panic = (
                "thread 'selection::selection_guards::capacity_neighbor_copies_all_values_in_order' "
                '(12567) panicked at crates/time-core/src/../tests/private/selection_guards.rs:365:5:\n'
                'assertion failed: matches!(cell_depth(batch, 1, 0),\n'
                '    Err(SelectionError::Invariant\n' + tail
                + 'note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n')
            for newline in ('\n', '\r\n'):
                raw = ('native-before\n' + panic + 'native-after\n').replace('\n', newline)
                stream = ('native-before\n{"reason":"build-finished","success":true}\n'
                          + panic + 'native-after\n').replace('\n', newline)
                with self.subTest(tail=tail, newline=repr(newline)):
                    records, cleaned = MODULE.cargo_json_records(stream, test_phase=True)
                    self.assertEqual(records, [{'reason': 'build-finished', 'success': True}])
                    self.assertEqual(cleaned, raw)
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.cargo_json_records(stream)
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.cargo_json_records(stream, test_phase=False)
        for mode in (None, 0, 1, 1.0, 'true', [], {}):
            with self.subTest(mode=mode), self.assertRaisesRegex(MODULE.Incomplete, 'mode must be Boolean'):
                MODULE.cargo_json_records('{"reason":"build-finished","success":true}\n', test_phase=mode)

    def test_cargo_json_test_context_keeps_near_misses_and_malformed_records_closed(self):
        tail = '    { stage: SelectionInvariantStage::Component }))\n'
        invalid = [
            tail.lstrip(),
            tail[1:],
            ' ' + tail,
            '\t' + tail.lstrip(),
            tail.replace('::Component', '::Slot'),
            tail.replace('::Component', '::Unknown'),
            tail.replace('SelectionInvariantStage::', ''),
            tail.replace(' }))', ' })) extra'),
            tail.replace(' }))', ' })) '),
            tail.replace(' }))', ' })'),
            tail.rstrip('\n'),
            tail.rstrip('\n') + '\r',
            '{"reason":"build-finished","success":true',
            '{"reason":"build-finished","success":false,"success":true}\n',
            '{"reason":"build-finished","success":NaN}\n',
            '{"reason":"build-finished","success":Infinity}\n',
            '{"reason":"build-finished","success":1e999}\n',
            '{"reason":"unknown"}\n',
            '{"reason":"build-finished"}\n',
            '{"reason":"build-finished","success":true,"extra":null}\n',
            '{"reason":"build-finished","success":1}\n',
        ]
        for record in invalid:
            with self.subTest(record=repr(record)), self.assertRaises(MODULE.Incomplete):
                MODULE.cargo_json_records(tail + record, test_phase=True)

    def test_literal_assertion_tail_alone_establishes_neither_compile_failure_nor_caught_test(self):
        tail = '    { stage: SelectionInvariantStage::Component }))\n'
        records, raw = MODULE.cargo_json_records(tail, test_phase=True)
        self.assertEqual(records, [])
        self.assertEqual(raw, tail)
        caught = {'summary': 'CaughtMutant', 'phase_results': self.phase_results('CaughtMutant')}
        with self.assertRaises(MODULE.Incomplete):
            MODULE.caught_native_test_failure(caught, raw)
        failure = ('failures:\n\n'
                   '    selection::selection_guards::capacity_neighbor_copies_all_values_in_order\n\n'
                   'test result: FAILED. 0 passed; 1 failed; 0 ignored;\n')
        MODULE.caught_native_test_failure(caught, failure)
        unviable = {'summary': 'Unviable', 'phase_results': self.phase_results('Unviable')}
        MODULE.unviable_compile_failure(unviable, self.semantic_mutation_log())
        build = (self.mutation_phase_header('Build') + tail
                 + '{"reason":"build-finished","success":false}\n')
        with self.assertRaises(MODULE.Incomplete):
            MODULE.unviable_compile_failure(unviable, build)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.unviable_compile_failure(unviable, self.semantic_mutation_log() + tail)

    def test_mutation_test_consumers_opt_in_to_literal_assertion_transport(self):
        """Synthetic phase-routing controls do not qualify a real campaign or native cause."""
        panic = (
            "thread 'selection::selection_guards::capacity_neighbor_copies_all_values_in_order' "
            '(12567) panicked at crates/time-core/src/../tests/private/selection_guards.rs:365:5:\n'
            'assertion failed: matches!(cell_depth(batch, 1, 0),\n'
            '    Err(SelectionError::Invariant\n'
            '    { stage: SelectionInvariantStage::Component }))\n')
        profile = 'S13_SELECTION_PROFILE scope=interval intervals=1 minima=0 batches=0 selections=0\n'
        failure = ('failures:\n\n'
                   '    selection::selection_guards::capacity_neighbor_copies_all_values_in_order\n\n'
                   'test result: FAILED. 0 passed; 1 failed; 0 ignored;\n')
        with tempfile.TemporaryDirectory() as temporary:
            output = Path(temporary)
            caught = {'scenario': {'Mutant': {'name': 'literal-assertion-tail'}},
                      'summary': 'CaughtMutant', 'log_path': 'caught.log',
                      'phase_results': self.phase_results('CaughtMutant')}
            (output / 'caught.log').write_text(self.completed_mutation_log(
                panic + profile + failure, include_selection=False))
            timeout = {'scenario': {'Mutant': {'name': 'literal-assertion-tail-timeout'}},
                       'summary': 'Timeout', 'log_path': 'timeout.log'}
            (output / 'timeout.log').write_text(
                self.mutation_phase_header('Test')
                + '{"reason":"build-finished","success":true}\n' + panic + profile)
            for outcome in (caught, timeout):
                report = {}
                label = 'selection:' + outcome['summary']
                MODULE.record_counts(report, label, {
                    'charged_fit_work_upper': 63129, 'point_admissions_upper': 2253360,
                    **MODULE.selection_reservation(package=True), 'observation_pending': True})
                row = report['core_invocations'][0]
                with self.subTest(summary=outcome['summary']):
                    MODULE.observe_mutation_outcome(report, output / 'outcomes.json', label, outcome, row)
                    self.assertEqual(row['charged_fit_work_upper'], 63129)
                    self.assertEqual(row['point_admissions_upper'], 2253360)
                    if outcome['summary'] == 'CaughtMutant':
                        self.assertFalse(row['observation_pending'])
                        self.assertEqual(row['observed_counts']['selection_recorded_call_subtotals']['intervals'], 1)
                        self.assertIsNone(row['observed_counts']['selection_observed_calls'])
                        self.assertEqual(report['mutation_counter_prefix_observations'], [label])
                    else:
                        self.assertTrue(row['observation_pending'])
                        self.assertEqual(row['observation_status'], 'timeout-prefix-unqualified')
                        self.assertNotIn('observed_counts', row)

    def test_observed_build_script_transport_is_bounded_and_never_an_error_or_counter(self):
        record = self.build_script_record()
        payload = 'before\r\n' + json.dumps(record) + '\r\nafter\r\n'
        parsed, raw = MODULE.cargo_json_records(payload)
        self.assertEqual(parsed, [record])
        self.assertEqual(raw, 'before\r\nafter\r\n')
        outcome = {'summary': 'Unviable', 'phase_results': self.phase_results('Unviable')}
        semantic = self.semantic_mutation_log().replace(
            self.mutation_phase_header('Build'),
            self.mutation_phase_header('Build') + json.dumps(record) + '\n', 1)
        MODULE.unviable_compile_failure(outcome, semantic)
        script_only = (self.mutation_phase_header('Build') + json.dumps(record) + '\n'
                       + '{"reason":"build-finished","success":false}\n')
        with self.assertRaises(MODULE.Incomplete):
            MODULE.unviable_compile_failure(outcome, script_only)
        synthetic_env = copy.deepcopy(record)
        synthetic_env['env'] = [['KEY', 'CORE_RUNS_PROFILE points=2 inputs=0 selections=0']]
        _, cleaned = MODULE.cargo_json_records(json.dumps(synthetic_env) + '\n')
        self.assertEqual(cleaned, '')
        bad = []
        for env in ([['KEY']], [['KEY', 1]], {'KEY': 'value'}, [['a', 'b', 'c']], [['a', 'b']] * 257):
            changed = copy.deepcopy(record); changed['env'] = env; bad.append(changed)
        missing = copy.deepcopy(record); del missing['cfgs']; bad.append(missing)
        extra = copy.deepcopy(record); extra['unreviewed'] = True; bad.append(extra)
        foreign = copy.deepcopy(record); foreign['package_id'] = 'path+file:///foreign#0.0.0'; bad.append(foreign)
        path = copy.deepcopy(record); path['out_dir'] = '/tmp/unowned/out'; bad.append(path)
        overlong = copy.deepcopy(record); overlong['cfgs'] = ['x' * 4097]; bad.append(overlong)
        unknown = copy.deepcopy(record); unknown['reason'] = 'unknown'; bad.append(unknown)
        for changed in bad:
            with self.subTest(record=repr(changed)[:120]), self.assertRaises(MODULE.Incomplete):
                MODULE.cargo_json_records(json.dumps(changed) + '\n')

    def test_caught_outcome_rejects_json_error_despite_real_failed_test_block(self):
        with tempfile.TemporaryDirectory() as temp:
            output = Path(temp)
            failure = ('CORE_RUNS_PROFILE points=2 inputs=0 selections=0\n'
                       '\nfailures:\n    actual_test\n\n'
                       'test result: FAILED. 0 passed; 1 failed; 0 ignored;\n')
            log = self.completed_mutation_log(json.dumps(self.semantic_record()) + '\n' + failure)
            (output / 'mutant.log').write_text(log)
            outcome = {'scenario': {'Mutant': {'name': 'one'}}, 'summary': 'CaughtMutant',
                       'log_path': 'mutant.log', 'phase_results': self.phase_results('CaughtMutant')}
            report = {}
            MODULE.record_counts(report, 'runs:mutation-1', {
                'charged_fit_work_upper': 63129, 'point_admissions_upper': 2253360,
                'observation_pending': True})
            with self.assertRaisesRegex(MODULE.Incomplete, 'conflicts with an error diagnostic'):
                MODULE.observe_mutation_outcome(report, output / 'outcomes.json',
                                                'runs:mutation-1', outcome, report['core_invocations'][0])
            self.assertTrue(report['core_invocations'][0]['observation_pending'])
            self.assertNotIn('observed_counts', report['core_invocations'][0])

    def test_unviable_requires_native_compile_diagnostic_not_environment_failure(self):
        with tempfile.TemporaryDirectory() as temp:
            work = Path(temp); output = work / 'mutation/runs/mutants.out'
            output.mkdir(parents=True)
            (output / 'baseline.log').write_text(self.completed_mutation_log(self.complete_runs_rows()))
            outcome = {'scenario': {'Mutant': {'name': 'build_failure'}}, 'summary': 'Unviable',
                       'log_path': 'compile.log', 'phase_results': self.phase_results('Unviable')}
            (output / 'outcomes.json').write_text(json.dumps({'total_mutants': 1, 'outcomes': [
                {'scenario': 'Baseline', 'summary': 'Success', 'log_path': 'baseline.log',
                 'phase_results': self.phase_results('Success')}, outcome]}))
            diagnostic = self.semantic_mutation_log()
            (output / 'compile.log').write_text(diagnostic)
            report = {'subjects': self.historical_scope('runs').select('all')}
            with patch.dict(MODULE.PROFILE_LIMITS, {'charged_fit_work_upper': 2000}):
                MODULE.mutation_accounting(report, work)
            self.assertEqual(report['mutation_full_accounting']['runs']['counts']['U'], 1)
            self.assertEqual(report['core_invocations'][1]['charged_fit_work_upper'], 0)
            for invalid in ['error: could not execute process: Resource temporarily unavailable (os error 11)\n',
                            diagnostic + 'error: could not execute process: Resource temporarily unavailable (os error 11)\n',
                            diagnostic + 'CORE_RUNS_PROFILE points=1 inputs=0 selections=0\n',
                            diagnostic + 'error: internal compiler error: unexpected panic\n',
                            diagnostic + "thread 'rustc' panicked at compiler.rs:1:1\n",
                            diagnostic.replace('crates/time-core/src/regression.rs', 'foreign/src/lib.rs')]:
                (output / 'compile.log').write_text(invalid)
                incomplete = {'subjects': self.historical_scope('runs').select('all')}
                with self.subTest(log=invalid), patch.dict(MODULE.PROFILE_LIMITS, {'charged_fit_work_upper': 2000}):
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.mutation_accounting(incomplete, work)
                self.assertEqual(incomplete['core_invocations'][1]['charged_fit_work_upper'], 63129)
                self.assertTrue(incomplete['core_invocations'][1]['observation_pending'])
            (output / 'compile.log').write_text(diagnostic)
            outcome['phase_results'][0]['process_status'] = {'Failure': 2}
            (output / 'outcomes.json').write_text(json.dumps({'total_mutants': 1, 'outcomes': [
                {'scenario': 'Baseline', 'summary': 'Success', 'log_path': 'baseline.log',
                 'phase_results': self.phase_results('Success')}, outcome]}))
            incomplete = {'subjects': self.historical_scope('runs').select('all')}
            with patch.dict(MODULE.PROFILE_LIMITS, {'charged_fit_work_upper': 2000}):
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.mutation_accounting(incomplete, work)
            self.assertEqual(incomplete['core_invocations'][1]['charged_fit_work_upper'], 63129)
            self.assertTrue(incomplete['core_invocations'][1]['observation_pending'])

    def test_failed_native_mutant_retains_valid_prefix_and_rejects_setup_or_resource_failures(self):
        with tempfile.TemporaryDirectory() as temp:
            work = Path(temp)
            output = work / 'mutation/runs/mutants.out'
            output.mkdir(parents=True)
            (output / 'baseline.log').write_text(self.completed_mutation_log(self.complete_runs_rows()))
            partial = 'CORE_RUNS_PROFILE points=2 inputs=0 selections=0\n'
            failure = '\nfailures:\n    frozen_assertion\n\ntest result: FAILED. 0 passed; 1 failed; 0 ignored;\n'
            (output / 'killed.log').write_text(self.completed_mutation_log(partial + failure))
            outcomes = [
                {'scenario': 'Baseline', 'summary': 'Success', 'log_path': 'baseline.log',
                 'phase_results': self.phase_results('Success')},
                {'scenario': {'Mutant': {'name': 'one_fixture_change'}},
                 'summary': 'CaughtMutant', 'log_path': 'killed.log',
                 'phase_results': self.phase_results('CaughtMutant')},
            ]
            (output / 'outcomes.json').write_text(json.dumps({'total_mutants': 1, 'outcomes': outcomes}))
            report = {'subjects': self.historical_scope('runs').select('all')}
            with patch.dict(MODULE.PROFILE_LIMITS, {'charged_fit_work_upper': 2000}):
                MODULE.mutation_accounting(report, work)
            self.assertEqual(report['core_charged_subset_totals'],
                             {'charged_fit_work_upper': 126258, 'point_admissions_upper': 4506726})
            caught = report['core_invocations'][1]
            self.assertEqual(caught['observed_counts']['runs_points'], 2)
            self.assertEqual(caught['raw_log_sha256'],
                             hashlib.sha256(self.completed_mutation_log(partial + failure).encode()).hexdigest())
            self.assertEqual(report['mutation_full_accounting']['runs']['counts']['K'], 1)
            self.assertEqual(report['mutation_counter_prefix_observations'], ['runs:mutation-1'])
            for invalid in [partial,
                            partial.replace('selections=0', 'selections=0 extra=1') + failure,
                            partial.replace('points=2', 'points=129') + failure,
                            partial + failure.replace('    frozen_assertion', '    src/lib.rs - (line 3)')]:
                (output / 'killed.log').write_text(self.completed_mutation_log(invalid))
                with self.subTest(log=invalid), patch.dict(MODULE.PROFILE_LIMITS, {'charged_fit_work_upper': 2000}):
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.mutation_accounting({'subjects': self.historical_scope('runs').select('all')}, work)
            (output / 'killed.log').write_text(self.completed_mutation_log(partial + failure))
            # Well-formed missing runs rows remain incomplete for a survivor.
            outcomes[1]['summary'] = 'MissedMutant'
            (output / 'outcomes.json').write_text(json.dumps({'total_mutants': 1, 'outcomes': outcomes}))
            with patch.dict(MODULE.PROFILE_LIMITS, {'charged_fit_work_upper': 2000}):
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.mutation_accounting({'subjects': self.historical_scope('runs').select('all')}, work)

    def test_mixed_direct_and_nested_observations_are_summed_once_with_pending_state(self):
        with tempfile.TemporaryDirectory() as temp:
            work = Path(temp)
            output = work / 'mutation/runs/mutants.out'
            output.mkdir(parents=True)
            text = ('CORE_PROFILE_COUNTS fits=1 points=3\n'
                    'CORE_STATS_PROFILE_COUNTS stats=1 points=3\n' + self.complete_runs_rows())
            for name in ['baseline.log', 'survivor.log']:
                (output / name).write_text(self.completed_mutation_log(text))
            (output / 'outcomes.json').write_text(json.dumps({'total_mutants': 1, 'outcomes': [
                {'scenario': 'Baseline', 'summary': 'Success', 'log_path': 'baseline.log',
                 'phase_results': self.phase_results('Success')},
                {'scenario': {'Mutant': {'name': 'unchanged_fixture'}},
                 'summary': 'MissedMutant', 'log_path': 'survivor.log',
                 'phase_results': self.phase_results('MissedMutant')}]}))
            report = {'subjects': self.historical_scope('runs').select('all')}
            MODULE.record_counts(report, 'known_direct', {
                'charged_fit_work_upper': 3, 'point_admissions_upper': 9,
                'direct_fit_calls': 2, 'stats_calls': 1, 'point_admissions': 9})
            with patch.dict(MODULE.PROFILE_LIMITS, {'charged_fit_work_upper': 2000}):
                MODULE.mutation_accounting(report, work)
            expected = {'direct_fit_calls': 4, 'stats_calls': 3, 'point_admissions': 21}
            self.assertEqual(report['core_direct_and_stats_observations'], expected)
            self.assertEqual(report['core_runs_external_observations']['runs_selections'], 40)
            # Two complete packages plus the separate three-fit/nine-point direct observation.
            self.assertEqual(report['core_charged_subset_totals'],
                         {'charged_fit_work_upper': 126261, 'point_admissions_upper': 4506735})
            self.assertTrue(report['core_recorded_observations_parsed'])
            MODULE.refresh_accounting(report)
            self.assertEqual(report['core_direct_and_stats_observations'], expected)
            self.assertEqual(report['core_runs_external_observations']['runs_selections'], 40)
            MODULE.record_counts(report, 'not_observed_yet', {
                'charged_fit_work_upper': 3, 'point_admissions_upper': 9, 'observation_pending': True})
            self.assertFalse(report['core_recorded_observations_parsed'])
            self.assertEqual(report['core_observation_pending_invocations'], ['not_observed_yet'])
            self.assertEqual(report['core_direct_and_stats_observations'], expected)
            report['core_invocations'][-1]['observed_counts'] = {
                'direct_fit_calls': 3, 'stats_calls': 0, 'point_admissions': 9}
            report['core_invocations'][-1]['observation_pending'] = False
            MODULE.refresh_accounting(report)
            self.assertTrue(report['core_recorded_observations_parsed'])
            self.assertEqual(report['core_direct_and_stats_observations'],
                             {'direct_fit_calls': 7, 'stats_calls': 3, 'point_admissions': 30})

    def test_missing_mutation_cohort_is_distinct_from_parsed_recorded_observations(self):
        with tempfile.TemporaryDirectory() as temp:
            report = {'subjects': self.historical_scope('runs').select('all')}
            MODULE.record_counts(report, 'known_direct', {
                'charged_fit_work_upper': 1, 'point_admissions_upper': 3,
                'direct_fit_calls': 1, 'stats_calls': 0, 'point_admissions': 3})
            MODULE.mutation_accounting(report, Path(temp))
            self.assertTrue(report['core_recorded_observations_parsed'])
            self.assertFalse(report['mutation_counter_observation_complete'])
            self.assertEqual(report['mutation_missing_selected_files'], ['runs'])

    def phase_results(self, summary):
        """Create inert pinned build/test phase observations for actual gate controls."""
        phases = []
        for phase in (['Build'] if summary == 'Unviable' else ['Build', 'Test']):
            failed = summary == 'Unviable' or (summary == 'CaughtMutant' and phase == 'Test')
            phases.append({'phase': phase, 'duration': 0.01,
                           'argv': ['cargo', 'test', *(['--no-run'] if phase == 'Build' else []),
                                    '--verbose', '--package=time-core@0.0.0', '--message-format=json', '--frozen'],
                           'process_status': {'Failure': 101} if failed else 'Success'})
        return phases

    def mutation_report(self, filename, caught, missed):
        """Produce complete independent synthetic fault identities and raw outcomes."""
        expected = {}; outcomes = [{'scenario': 'Baseline', 'summary': 'Success', 'log_path': 'baseline.log',
                                    'phase_results': self.phase_results('Success')}]
        for index in range(caught + missed):
            mutant = {'file': filename, 'package': 'time-core', 'name': filename + ':' + str(index),
                      'function': {'function_name': 'checked_guard', 'return_type': '-> bool',
                                   'span': {'start': {'line': 1, 'column': 1}, 'end': {'line': 2, 'column': 2}}},
                      'span': {'start': {'line': 2, 'column': 1}, 'end': {'line': 2, 'column': 2}},
                      'replacement': 'false', 'genre': 'FnValue'}
            expected[mutant['name']] = mutant
            summary = 'CaughtMutant' if index < caught else 'MissedMutant'
            outcomes.append({'scenario': {'Mutant': mutant}, 'summary': summary,
                             'phase_results': self.phase_results(summary)})
        return ({'cargo_mutants_version': '27.1.0', 'total_mutants': caught + missed, 'caught': caught,
                 'missed': missed, 'unviable': 0, 'timeout': 0, 'success': 0, 'outcomes': outcomes}, expected)

    def test_each_file_gate_preserves_95_percent_and_no_unresolved_rule(self):
        for filename in MODULE.SUBJECTS.values():
            bad, expected = self.mutation_report(filename, 19, 1)
            result = MODULE.COMMON.mutation_counts(MODULE.SCOPE, bad, expected, filename, native_exit_code=2, defer_gate=True)
            self.assertEqual(result['adjusted_viable_killed_percent'], 95)
            self.assertFalse(result['gate_passed'])
            self.assertEqual(len(result['unresolved_survivors']), 1)
            with self.assertRaises(MODULE.Incomplete):
                MODULE.COMMON.mutation_counts(MODULE.SCOPE, bad, expected, filename, native_exit_code=2)
            healthy, expected = self.mutation_report(filename, 3, 0)
            self.assertTrue(MODULE.COMMON.mutation_counts(MODULE.SCOPE, healthy, expected, filename)['gate_passed'])
            for changed in [dict(healthy, outcomes=healthy['outcomes'][1:]), dict(healthy, total_mutants=4),
                            dict(healthy, outcomes=healthy['outcomes'] + healthy['outcomes'][-1:]),
                            dict(healthy, timeout=1)]:
                with self.assertRaises(MODULE.Incomplete): MODULE.COMMON.mutation_counts(MODULE.SCOPE, changed, expected, filename)
            with self.assertRaises(MODULE.Incomplete): MODULE.COMMON.mutation_counts(MODULE.SCOPE, healthy, {}, filename)

    def test_complete_mutation_loop_has_one_package_baseline_and_one_shared_deadline(self):
        with tempfile.TemporaryDirectory() as temp:
            work = Path(temp); report = {}; clock = [100.0]; launches = []
            # Keep this nine-file timing fixture in its explicit historical role.
            selected = dict(S158_SUBJECTS)
            scope = MODULE.COMMON.Scope('time-core', tuple(selected.items()), MODULE.POLICY)
            def run(label, command, timeout, **options):
                launches.append((label, list(command), timeout, options))
                subject = label.split(':')[1]; filename = selected[subject]
                raw, expected = self.mutation_report(filename, 1, 0)
                if label.startswith('mutation-enumeration:'):
                    clock[0] += 50
                    return json.dumps(list(expected.values()))
                out = work / 'mutation' / subject / 'mutants.out'; out.mkdir(parents=True)
                (out / 'outcomes.json').write_text(json.dumps(raw))
                (out / 'baseline.log').write_text('test result: ok. 2 passed; 0 failed; 0 ignored;\n')
                clock[0] += 75
                return {'exit_code': 0, 'failure': None, 'output': 'inert actual-loop control'}
            with patch.object(MODULE.COMMON.time, 'monotonic', side_effect=lambda: clock[0]):
                MODULE.COMMON.run_analysis_lanes(scope, selected,
                    {'package': 'time-core', 'admission_status': 'admitted-final-source',
                     'mutation_equivalences': [], 'mutation_error_values': {},
                     'maximum_mutants': 512, 'mutation_wall_seconds': 1200},
                    work, Path('/inert-tools'), run, report, 'mutation')
            campaigns = [row for row in launches if row[0].startswith('mutation-campaign:')]
            self.assertEqual([row[0] for row in campaigns],
                             ['mutation-campaign:' + name for name in S158_SUBJECTS])
            # Nine50-second listings precede every campaign. Each campaign
            # then consumes75 seconds of the same1200-second absolute budget.
            self.assertEqual([row[2] for row in campaigns], [750, 675, 600, 525, 450, 375, 300, 225, 150])
            for _, command, _, _ in campaigns:
                self.assertEqual(command[command.index('--baseline') + 1], 'run')
                self.assertEqual(command[command.index('--build-timeout') + 1], '90')
                self.assertEqual(command[command.index('--timeout') + 1], '60')
            self.assertEqual(set(report['mutation']), set(selected))
            self.assertTrue(all(row['gate_passed'] for row in report['mutation'].values()))

    def test_combined_variant_cap_and_separate_global_custody_are_explicit(self):
        first = {str(i): {} for i in range(256)}; second = dict(first)
        self.assertEqual(MODULE.COMMON.combined_mutant_count({'regression': first, 'residuals': second}, 512), 512)
        second['extra'] = {}
        with self.assertRaises(MODULE.Incomplete): MODULE.COMMON.combined_mutant_count({'regression': first, 'residuals': second}, 512)
        with self.assertRaises(MODULE.Incomplete): MODULE.COMMON.combined_mutant_count({}, 512)
        # Cross-invocation global512 reservation is the coordinator's obligation;
        # this actual helper cap is per complete selected enumeration, not a fake
        # persistent ledger or permission to reset the global budget on retry.

    def test_corpus_has_frozen_representable_inputs_and_bounded_raw_extents(self):
        seeds = MODULE.fuzz_seeds()
        self.assertEqual(sum(name.startswith('normalization-') for name in seeds), 57)
        for name in ['normalization-one_ns_admitted', 'normalization-count_zero', 'malformed-count65', 'malformed-truncated']:
            self.assertIn(name, seeds)
        self.assertEqual(len(seeds['normalization-count_64_uniform']), 1537)
        self.assertEqual(len(seeds['normalization-one_ns_admitted']), 73)
        self.assertTrue(all(len(data) <= 1537 for data in seeds.values()))
        self.assertEqual(seeds['normalization-one_ns_admitted'][25:33], (62499999).to_bytes(8,'big'))
        with self.assertRaises(MODULE.Incomplete): MODULE.encode_case([(0.0, 0.0, 1.0)] * 65)
        with self.assertRaises(MODULE.Incomplete): MODULE.encode_bits_case([(0, 0, 0)] * 65)
        with self.assertRaises(MODULE.Incomplete): MODULE.encode_bits_case([(0, True, 0)])

    def test_global_subset_budget_counts_invocations_without_claiming_global_ownership(self):
        report = {}
        for label in ['coverage', 'mutation-baseline']:
            MODULE.record_counts(report, label, MODULE.profile_counts('CORE_PROFILE_COUNTS fits=285 points=1329\nCORE_STATS_PROFILE_COUNTS stats=169 points=1330\n'))
        self.assertEqual(report['core_charged_subset_totals'], {'charged_fit_work_upper': 908, 'point_admissions_upper': 5318})
        self.assertEqual(report['core_direct_and_stats_observations'], {'direct_fit_calls': 570, 'stats_calls': 338, 'point_admissions': 5318})
        self.assertIn('Coordinator', report['global_campaign_custody'])
        for changed in [{'charged_fit_work_upper': MODULE.GLOBAL_LIMITS['charged_fit_work_upper'], 'point_admissions_upper': 0},
                        {'charged_fit_work_upper': 0, 'point_admissions_upper': MODULE.GLOBAL_LIMITS['point_admissions_upper']},
                        {'fit_calls': 1, 'point_admissions': 3},
                        {'charged_fit_work_upper': True, 'point_admissions_upper': 0}]:
            with self.subTest(counts=changed), self.assertRaises(MODULE.Incomplete):
                MODULE.record_counts(copy.deepcopy(report), 'excess', changed)

    def test_cli_defaults_to_the_current_checkpoint_subject_and_named_diagnostics_remain_narrow(self):
        selected = []
        scopes = []
        def capture(args, report):
            selected.append(args.subject)
            scopes.append(report['scope'])
            raise MODULE.Incomplete('inert argument-selection probe')
        with tempfile.TemporaryDirectory() as temp:
            args = ['--output-dir', temp, '--vendor-dir', temp]
            with patch.object(MODULE, 'execute', side_effect=capture), contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(io.StringIO()):
                self.assertEqual(MODULE.main(args), 1)
                self.assertEqual(MODULE.main(args + ['--subject', 'reference_input']), 1)
                with self.assertRaises(SystemExit): MODULE.main(args + ['--subject', 'runs'])
        self.assertEqual(selected, ['all', 'reference_input'])
        self.assertEqual(scopes, [
            'Selected reference-input owner within the24-owner time-core manifest coverage'] * 2)
        self.assertEqual(len(MODULE.SCOPE.select('all')), 1)
        self.assertEqual(len(MODULE.SCOPE.select('reference_input')), 1)
        with self.assertRaises(MODULE.Incomplete): MODULE.SCOPE.select('../residuals')
        # The current checkpoint requires one receipt for its selected owner.
        with tempfile.TemporaryDirectory() as temp:
            base = ['--lane', 'cohort-union', '--cohort-plan', temp + '/plan.json', '--output-dir', temp]
            for count in (0, 1, 2, 3, 8, 9, 10):
                cli = base + ['--cohort-receipt', temp + '/receipt.json'] * count
                stderr = io.StringIO()
                with self.subTest(receipts=count), \
                     patch.object(MODULE, 'execute', side_effect=MODULE.Incomplete('healthy union argument boundary')) as run, \
                     contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(stderr):
                    if count == 1:
                        self.assertEqual(MODULE.main(cli), 1)
                        run.assert_called_once()
                        self.assertIn('healthy union argument boundary', stderr.getvalue())
                    else:
                        with self.assertRaises(SystemExit): MODULE.main(cli)
                        run.assert_not_called()
                        self.assertIn('one --cohort-receipt path per selected owner', stderr.getvalue())

    def test_fuzz_supervisor_uses_reviewed_wall_limit_for_both_processes(self):
        """Observe actual orchestration using explicitly synthetic process receipts."""
        with tempfile.TemporaryDirectory() as temp:
            base = Path(temp)
            root, work, output = base / 'root', base / 'work', base / 'output'
            for directory in [root, work, output]:
                directory.mkdir(mode=0o700)
            for name in MODULE.FUZZ_PATHS.values():
                path = root / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text('isolated orchestration fixture; never executed\n')
            installation = base / 'installation'
            launcher = installation / 'common/bin/cargo-fuzz'
            launcher.parent.mkdir(parents=True)
            launcher.write_bytes(b'isolated tool-copy fixture; never executed\n')
            calls = []
            report = {'lanes': []}

            def synthetic_history_row(raw):
                """Literal cumulative recipe: one history and three appends per callback."""
                return (f'CORE_HISTORY_FUZZ_COUNTS raw={raw} histories={raw} '
                        f'appends={3*raw} observations={3*raw} contexts={raw} '
                        f'monotonic={3*raw} coordinates={12*raw} exchanges={3*raw} '
                        f'getters={15*raw} normalizations=0 stats=0 fits=0 points=0\n')

            healthy_output = ''.join(synthetic_history_row(raw) for raw in range(1, 4097))
            healthy_output += ('HISTORY_FUZZ_KNOWN_REPORT_CHECKED synthetic-orchestration-fixture\n'
                               'Done 4096 runs\n')
            defect_output = (synthetic_history_row(1)
                             + "thread '<unnamed>' panicked at target.rs:1:1:\n"
                             'independent history oracle: frozen published report bits\n')

            class FakeSession:
                """Record commands, inert build bytes and explicitly synthetic lane receipts."""

                def configure(self, text):
                    self.configuration = text

                def protect(self, path):
                    pass

                def run(self, label, command, timeout, **options):
                    calls.append((label, tuple(command), timeout, options))
                    if label == 'fuzz-graph':
                        return '{}'
                    if label.startswith('fuzz-build-'):
                        binary = work / 'target-fuzz' / MODULE.ANALYSIS.PLATFORM / 'release/fit'
                        binary.parent.mkdir(parents=True, exist_ok=True)
                        binary.write_bytes(label.encode())
                        return ''
                    if label in {'fuzz-healthy', 'fuzz-defect'}:
                        text = healthy_output if label == 'fuzz-healthy' else defect_output
                        report['lanes'].append({'lane': label,
                            'exit_code': 0 if label == 'fuzz-healthy' else 77,
                            'failure': None, 'output': text})
                        # Real Session.run preserves the receipt's raw output,
                        # while require_ok strips its healthy return value.
                        return text if label == 'fuzz-defect' else text.strip()
                    raise AssertionError('Unexpected orchestration command: ' + label)

            session = FakeSession()
            session.work, session.output, session.base = work, output, installation
            session.vendor = base / 'unused-vendor'
            session.protected_tools = {}
            session.admitted = {'policy': {'common_nightly': 'fixture-nightly',
                'tools': {'fuzz': {'sha256': hashlib.sha256(launcher.read_bytes()).hexdigest()}}}}
            selected = {**MODULE.FUZZ_PATHS, 'locked_registry_packages': [],
                        'invocation_limits': {'wall_seconds': 15}}
            # Mock adjacent admission only. Real fuzz() selects every timeout,
            # binds each unique receipt, classifies its cause and parses counters.
            # These synthetic observations establish no Rust/native execution.
            with patch.object(MODULE, 'ROOT', root), \
                 patch.object(MODULE, 'admit_fuzz_sources', return_value={'locked_registry_packages': []}), \
                 patch.object(MODULE, 'fuzz_metadata', return_value={}), \
                 patch.object(MODULE.COMMON, 'fuzz_input_snapshot', return_value={'fixture': 'unchanged'}):
                MODULE.fuzz(session, selected, report)
            self.assertEqual([timeout for label, _, timeout, _ in calls if label == 'fuzz-graph'], [120])
            self.assertEqual([timeout for label, _, timeout, _ in calls if label.startswith('fuzz-build-')], [600, 600])
            actual = [(label, timeout) for label, _, timeout, _ in calls
                      if label in {'fuzz-healthy', 'fuzz-defect'}]
            self.assertEqual(actual, [('fuzz-healthy', 15), ('fuzz-defect', 15)])
            for label, _, _, options in calls:
                if label in {'fuzz-healthy', 'fuzz-defect'}:
                    self.assertEqual(options['mode'], 'sanitizer')
                    self.assertEqual(options['sanitizer_profile'], 'time-core-history-s12d')
                    self.assertEqual(options['negative'],
                        'independent history oracle: frozen published report bits'
                        if label == 'fuzz-defect' else None)

    def test_normalization_only_work_and_private_zero_records_are_distinct(self):
        result = MODULE.profile_counts(self.norm_row())
        self.assertEqual(result['normalization_requests'], 1)
        self.assertEqual(result['direct_fit_calls'], 0)
        self.assertEqual(result['charged_fit_work_upper'], 0)
        self.assertEqual(result['point_admissions'], 0)
        self.assertEqual(result['generated_point_admissions_upper'], 64)
        self.assertEqual(result['duration_operations_upper'], 64)
        self.assertEqual(result['fit_input_constructions_upper'], 1)
        zero = self.norm_row(**{key: 0 for key in ['normalizations','observations','contexts','monotonic','coordinates','exchanges','getters','stats','points']})
        with self.assertRaises(MODULE.Incomplete): MODULE.profile_counts(zero)
        observed = MODULE.profile_counts(zero+'\n'+zero, require_fit=False)
        self.assertEqual(observed['normalization_counter_records'], 2)
        self.assertEqual(observed['point_admissions_upper'], 0)
        # Identical zero scopes are legitimate. Missing whole zero rows or replay
        # of an otherwise identical named scope needs root discovery custody.

    def test_normalization_transport_rejects_malformed_wrong_schema_and_replay(self):
        valid = self.norm_row()
        bad = [valid.replace('observations=3','observations=-1'), valid.replace('contexts=1','contexts=True'),
               valid.replace('monotonic=3','monotonic=3.0'), valid.replace(' coordinates=12',''),
               valid+' raw=0', valid+'\nCORE_NORMALIZATION_PROFILE_COUNTS incomplete',
               valid.replace('getters=5','getters='+('9'*100)), valid+' '+valid,
               valid.replace('stats=0 points=0','points=0 stats=0'),
               '--- STDERR:\n'+valid, 'Summary [0s]\n'+valid,
               valid+'\nCORE_NORMALIZATION_FUZZ_COUNTS raw=1 normalizations=1']
        for text in bad:
            with self.subTest(text=text[:100]), self.assertRaises(MODULE.Incomplete): MODULE.profile_counts(text)

    def test_every_normalization_profile_cap_has_a_neighbor(self):
        caps = {'normalizations':128,'observations':8320,'contexts':256,'monotonic':8320,
                'coordinates':33280,'exchanges':8320,'getters':8448,'stats':16}
        for field, maximum in caps.items():
            changes = {field:maximum, 'normalizations':128}
            good = MODULE.profile_counts(self.norm_row(**changes))
            self.assertEqual(good['normalization_requests'],128)
            changes[field] = maximum+1
            with self.subTest(field=field), self.assertRaises(MODULE.Incomplete): MODULE.profile_counts(self.norm_row(**changes))
        with self.assertRaises(MODULE.Incomplete): MODULE.profile_counts(self.norm_row(getters=67))

    def test_disjoint_actual_points_and_generated_upper_share_the_point_budget(self):
        exact = MODULE.profile_counts(self.norm_row(points=65536-64))
        self.assertEqual(exact['point_admissions'],65472)
        self.assertEqual(exact['point_admissions_upper'],65536)
        with self.assertRaises(MODULE.Incomplete): MODULE.profile_counts(self.norm_row(points=65536-63))
        legacy='CORE_PROFILE_COUNTS fits=285 points=1329\nCORE_STATS_PROFILE_COUNTS stats=169 points=1330\n'
        counts=MODULE.profile_counts(legacy+self.norm_row(normalizations=128,stats=16))
        self.assertEqual(counts['stats_calls'],185)
        self.assertEqual(counts['charged_fit_work_upper'],470)
        self.assertEqual(counts['point_admissions'],2659)
        self.assertEqual(counts['point_admissions_upper'],10851)
        with self.assertRaises(MODULE.Incomplete): MODULE.profile_counts('CORE_PROFILE_COUNTS fits=640 points=0\n'+self.norm_row(stats=1))

    def test_normalization_doc_allowances_do_not_fabricate_actual_calls(self):
        observed=MODULE.profile_counts(self.norm_row())
        counts=MODULE.account_literal_doctests(observed)
        self.assertEqual(counts['normalization_requests'],1)
        self.assertEqual(counts['stats_calls'],0)
        self.assertEqual(counts['point_admissions'],0)
        self.assertEqual(counts['literal_doctest_normalization_requests_upper'],0)
        self.assertEqual(counts['literal_doctest_utc_coordinate_constructor_attempts_upper'],0)
        self.assertEqual(counts['literal_doctest_point_getter_requests_upper'],0)
        self.assertEqual(counts['point_admissions_upper'],67)
        self.assertNotIn('literal_doctest_normalization_requests_upper', observed)

    def test_pending_observer_and_normalization_path_drift_cannot_be_admitted(self):
        with tempfile.TemporaryDirectory() as temp:
            root=Path(temp); policy=self.fixture(root)
            policy['normalization_telemetry']['status']='pending-source-owned-observer-and-literal-audit'
            self.write_policy(root,policy)
            with self.assertRaises(MODULE.Incomplete): MODULE.read_policy(root,'contracts')
            policy=self.fixture(root)
            for name in ['crates/time-core/src/regression/normalization.rs','crates/time-core/tests/normalization.rs',
                         'crates/time-core/tests/support/normalization_facts.rs','crates/time-core/tests/private/normalization_guards.rs']:
                path=root/name; original=path.read_bytes(); path.write_bytes(original+b'drift')
                with self.subTest(path=name), self.assertRaises(MODULE.Incomplete): MODULE.read_policy(root,'contracts')
                path.write_bytes(original)

    def test_normalization_legal_consumers_and_eight_causal_negatives_have_finite_recipes(self):
        cases=MODULE.normalization_contract_examples(); legal=[case for case in cases.values() if case['code'] is None]
        self.assertEqual(len(legal),2); self.assertEqual(len(cases)-len(legal),8)
        self.assertEqual(sum(case['calls']['normalization_requests'] for case in legal),2)
        self.assertEqual(sum(case['calls']['observation_constructor_attempts'] for case in legal),5)
        self.assertEqual(sum(case['calls']['utc_coordinate_constructor_attempts'] for case in legal),20)
        self.assertEqual(sum(case['calls']['point_getter_requests'] for case in legal),5)
        for case in legal:
            self.assertEqual(case['calls']['stats_calls']+case['calls']['direct_fit_calls'],0)
            self.assertLessEqual(case['calls']['observation_constructor_attempts'],3)
            self.assertLessEqual(case['calls']['point_admissions']+case['internal_upper']['generated_point_admissions_upper'],3)
        self.assertEqual(cases['norm_raw_q33_constructor']['code'],'E0451')
        self.assertEqual(cases['norm_utc_substitution']['code'],'E0308')
        self.assertEqual(cases['norm_immutable_point']['code'],'E0594')

    def test_real_domain_artifact_selector_rejects_wrong_manifest_identity_or_path(self):
        manifest='/work/source/crates/time-domain/Cargo.toml'
        item={'reason':'compiler-artifact','manifest_path':manifest,
              'package_id':'path+file:///work/source/crates/time-domain#0.0.0',
              'target':{'name':'time_domain','kind':['lib']},
              'filenames':['/work/target/debug/deps/libtime_domain-reviewed.rlib']}
        text=json.dumps(item)
        self.assertEqual(MODULE.COMMON.library_artifact(text,'time-domain',manifest),item['filenames'][0])
        for altered in [dict(item,manifest_path='/wrong/Cargo.toml'),dict(item,package_id='registry+wrong'),
                        dict(item,target={'name':'time_core','kind':['lib']}),
                        dict(item,filenames=['/outside/libtime_domain.rlib']),
                        dict(item,filenames=[]),dict(item,filenames=[*item['filenames'],'/work/target/other.rlib'])]:
            with self.subTest(altered=altered), self.assertRaises(MODULE.Incomplete): MODULE.COMMON.library_artifact(json.dumps(altered),'time-domain',manifest)
        with self.assertRaises(MODULE.Incomplete): MODULE.COMMON.library_artifact(text+'\n'+text,'time-domain',manifest)

    def test_core_and_domain_artifacts_are_copied_and_sealed_without_aliases(self):
        with tempfile.TemporaryDirectory() as temp:
            work=Path(temp); deps=work/'target/debug/deps'; deps.mkdir(parents=True)
            core=deps/'libtime_core-hash.rlib'; domain=deps/'libtime_domain-hash.rlib'
            core.write_bytes(b'explicit inert core artifact'); domain.write_bytes(b'explicit inert domain artifact')
            (deps/'cargo-owned-domain-alias.rlib').hardlink_to(domain)
            class Session:
                def protect(self,path): self.sealed=MODULE.PRODUCT.inventory(path)
            session=Session(); session.work=work
            result=MODULE.copy_contract_dependencies(session,'/work/target/debug/deps/'+core.name,'/work/target/debug/deps/'+domain.name)
            self.assertEqual((result/'libtime_core.rlib').read_bytes(),core.read_bytes())
            self.assertEqual((result/'libtime_domain.rlib').read_bytes(),domain.read_bytes())
            self.assertEqual(session.sealed,MODULE.PRODUCT.inventory(result))
            self.assertNotEqual((result/'libtime_domain.rlib').stat().st_ino,domain.stat().st_ino)
            self.assertEqual((result/'libtime_domain.rlib').stat().st_mode & 0o222,0)

    def test_redirected_or_escaped_domain_artifacts_are_rejected(self):
        for mode in ['symlink','escaped']:
            with tempfile.TemporaryDirectory() as temp:
                work=Path(temp); deps=work/'target/debug/deps'; deps.mkdir(parents=True)
                core=deps/'core.rlib'; core.write_bytes(b'inert core')
                actual=deps/'actual.rlib'; actual.write_bytes(b'inert domain')
                domain=deps/'domain.rlib'
                if mode=='symlink': domain.symlink_to(actual)
                else: domain=work/'outside.rlib'; domain.write_bytes(b'escaped fixture')
                session=type('Session',(),{'work':work,'protect':lambda self,path:None})()
                relative='/work/'+str(domain.relative_to(work))
                with self.subTest(mode=mode), self.assertRaises(MODULE.Incomplete):
                    MODULE.copy_contract_dependencies(session,'/work/target/debug/deps/core.rlib',relative)

    def test_actual_contract_orchestrator_passes_both_explicit_externs(self):
        class Captured(Exception): pass
        with tempfile.TemporaryDirectory() as temp:
            work=Path(temp); commands=[]
            class Session:
                def protect(self,path): pass
                def run(self,label,command,timeout,**options):
                    commands.append((label,command,options))
                    if label=='contract-library': return 'inert compiler-artifact envelope selected by adjacent stub'
                    if label.startswith('external-consumer-'): raise Captured
                    raise AssertionError(label)
            session=Session(); session.work=work; session.stable=Path('/inert-stable')
            with patch.object(MODULE.COMMON,'library_artifact',side_effect=['/work/target/core.rlib','/work/target/domain.rlib']) as selector, \
                 patch.object(MODULE,'copy_contract_dependencies',return_value=work/'contract-deps') as copier:
                with self.assertRaises(Captured): MODULE.contracts(session,{'contracts_wall_seconds':300,'jobs':6},{})
            self.assertEqual(commands[0][1][commands[0][1].index('--jobs')+1], '6')
            self.assertEqual(selector.call_args_list[1].args[1:],('time-domain','/work/source/crates/time-domain/Cargo.toml'))
            self.assertEqual(copier.call_args.args[1:],('/work/target/core.rlib','/work/target/domain.rlib'))
            command=commands[-1][1]
            self.assertIn('time_core=/work/contract-deps/libtime_core.rlib',command)
            self.assertIn('time_domain=/work/contract-deps/libtime_domain.rlib',command)
            self.assertEqual(command.count('--extern'),2)
            self.assertEqual(len(commands[-1][2]['sealed']),2)

    def test_normalization_wire_encoder_preserves_raw_bits_and_rejects_unbounded_shapes(self):
        row=(2**64-1,-1,0x8000000000000000)
        data=MODULE.encode_normalization_case([row])
        self.assertEqual(data,b'\x01'+(2**64-1).to_bytes(8,'big')+(-1).to_bytes(8,'big',signed=True)+bytes.fromhex('8000000000000000'))
        self.assertEqual(len(MODULE.encode_normalization_case([row]*64)),1537)
        for rows in [[row]*65,[(True,0,0)],[(0,True,0)],[(0,0,True)],[(2**64,0,0)],[(0,2**63,0)],
                     [(0,-2**63,0),(0,-2**63+1,0)]]:
            with self.subTest(rows=str(rows)[:80]), self.assertRaises(MODULE.Incomplete): MODULE.encode_normalization_case(rows)

    def test_normalization_fuzz_rejects_every_callback_bound_and_wrong_owner(self):
        for key,value in [('normalizations',2),('observations',65),('contexts',3),('monotonic',65),
                          ('coordinates',257),('exchanges',65),('getters',67),('stats',2),('points',1)]:
            with self.subTest(key=key), self.assertRaises(MODULE.Incomplete): MODULE.fuzz_counts(self.fuzz_row(**{key:value}),complete=False)
        good=self.fuzz_row()
        for text in [good+' extra=0',good+'\nCORE_NORMALIZATION_FUZZ_COUNTS broken',
                     good+'\nCORE_STATS_FUZZ_COUNTS raw=1 points=3 stats=1',
                     '--- STDERR:\n'+good,good.replace('raw=1','raw=18446744073709551616')]:
            with self.subTest(text=text), self.assertRaises(MODULE.Incomplete): MODULE.fuzz_counts(text,complete=False)

    def test_three_local_fuzz_graph_and_registry_edges_are_exact(self):
        selected=json.loads((ROOT/MODULE.POLICY).read_text())['coverage_guided_fuzzing']
        locked=selected['locked_registry_packages']; edges=selected['gnu_dependency_edges']; features=selected['gnu_reachable_features']
        packages=[{'id':item['name'],'name':item['name'],'source':item['source'],'version':item['version']} for item in locked]
        packages += [{'id':name,'name':name,'source':None,'version':'0.0.0'} for name in ['time-core','time-domain','time-core-fuzz']]
        nodes=[{'id':item['id'],'features':features.get(item['name'],[]),
                'deps':[{'pkg':name} for name in edges.get(item['name'],[])]} for item in packages]
        metadata={'packages':packages,'resolve':{'root':'time-core-fuzz','nodes':nodes}}
        result=MODULE.fuzz_metadata(metadata,selected)
        self.assertEqual(result['dependency_edges']['time-domain'],['garde'])
        self.assertEqual(result['dependency_edges']['time-core'],['garde','time-domain'])
        for change in ['edge','feature','local-source','root']:
            altered=copy.deepcopy(metadata)
            if change=='edge': next(node for node in altered['resolve']['nodes'] if node['id']=='time-core')['deps']=[]
            if change=='feature': next(node for node in altered['resolve']['nodes'] if node['id']=='time-domain')['features']=['unexpected']
            if change=='local-source': next(item for item in altered['packages'] if item['id']=='time-domain')['source']='registry+wrong'
            if change=='root': altered['resolve']['root']='time-domain'
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete): MODULE.fuzz_metadata(altered,selected)


    @staticmethod
    def history_row(**changes):
        """Independent closed twelve-field transport, never real Rust telemetry."""
        values = dict(histories=1, appends=3, observations=3, contexts=1, monotonic=3,
                      coordinates=12, exchanges=3, getters=20, normalizations=0,
                      stats=0, fits=0, points=0)
        values.update(changes)
        return 'CORE_HISTORY_PROFILE_COUNTS ' + ' '.join(f'{key}={value}' for key,value in values.items())

    def test_history_observed_external_fields_are_separate_from_nested_upper_costs(self):
        counts=MODULE.profile_counts(self.history_row())
        self.assertEqual(counts.get('history_constructions'),1)
        self.assertEqual(counts.get('history_append_requests'),3)
        self.assertEqual(counts.get('history_product_getter_requests'),20)
        self.assertEqual(counts['normalization_requests'],0)
        self.assertEqual(counts['direct_fit_calls'],0)
        self.assertEqual(counts['stats_calls'],0)
        self.assertEqual(counts['observed_charged_fit_work'],0)
        self.assertEqual(counts['charged_fit_work_upper'],3)
        self.assertEqual(counts['internal_fit_upper_bound'],3)
        self.assertEqual(counts['history_normalization_entries_upper'],3)
        self.assertEqual(counts['history_statistics_entries_upper'],3)
        self.assertEqual(counts['generated_point_admissions_upper'],192)
        self.assertEqual(counts['history_staging_Copy_assignments_upper'],381)
        self.assertEqual((counts['counter_records'],counts['history_counter_records']),(1,1))

    def test_history_complete_source_recipe_charges_actual_appends_not_fake_observed_stats(self):
        legacy='CORE_PROFILE_COUNTS fits=285 points=1329\nCORE_STATS_PROFILE_COUNTS stats=169 points=1330\n'
        normal=self.norm_row(normalizations=113,observations=4449,contexts=201,monotonic=4449,
                             coordinates=17796,exchanges=4449,getters=186,stats=3)
        history=self.history_row(histories=17,appends=144,observations=144,contexts=21,monotonic=144,
                                 coordinates=576,exchanges=144,getters=32786)
        counts=MODULE.profile_counts(legacy+normal+'\n'+history)
        self.assertEqual(counts.get('history_append_requests'),144)
        self.assertEqual(counts['stats_calls'],172)
        self.assertEqual(counts['observed_charged_fit_work'],457)
        self.assertEqual(counts['charged_fit_work_upper'],601)
        self.assertEqual(counts['point_admissions'],2659)
        self.assertEqual(counts['point_admissions_upper'],19107)
        self.assertEqual(counts['history_statistics_entries_upper'],144)
        self.assertEqual(counts['history_direct_stats_calls'],0)

    def test_history_zero_constructor_and_raw_guard_scopes_remain_distinct(self):
        empty=self.history_row(histories=1,appends=0,observations=0,contexts=1,monotonic=0,coordinates=0,exchanges=0,getters=4)
        self.assertEqual(MODULE.profile_counts(empty)['history_constructions'],1)
        zero=self.history_row(histories=0,appends=0,observations=0,contexts=0,monotonic=0,coordinates=0,exchanges=0,getters=0)
        with self.assertRaises(MODULE.Incomplete): MODULE.profile_counts(zero)
        self.assertEqual(MODULE.profile_counts(zero,require_fit=False)['history_counter_records'],1)

    def test_history_closed_transport_rejects_malformed_extra_wrong_owner_and_replay(self):
        good=self.history_row()
        bad=[good.replace(' appends=3',''),good+' extra=0',good.replace(' appends=3',' appends=-1'),
             good.replace(' appends=3',' appends=18446744073709551616'),good+'\nCORE_HISTORY_PROFILE_COUNTS broken',
             good+'\nCORE_HISTORY_FUZZ_COUNTS raw=1','Summary [replay]\n'+good,'--- STDERR:\n'+good]
        for text in bad:
            with self.subTest(text=text[-80:]),self.assertRaises(MODULE.Incomplete): MODULE.profile_counts(text)

    def test_history_observation_ownership_rejects_untracked_internal_or_setup_fields(self):
        for changes in [dict(normalizations=1),dict(stats=1),dict(fits=1),dict(points=1),
                        dict(monotonic=2),dict(coordinates=11),dict(exchanges=2)]:
            with self.subTest(changes=changes),self.assertRaises(MODULE.Incomplete):
                MODULE.profile_counts(self.history_row(**changes))

    def test_history_explicit_profile_caps_have_valid_neighboring_records(self):
        good=self.history_row(histories=32,appends=160,observations=256,contexts=64,
                              monotonic=256,coordinates=1024,exchanges=256,getters=65536)
        self.assertEqual(MODULE.profile_counts(good)['history_append_requests'],160)
        for key,value in [('histories',33),('appends',161),('contexts',65),('getters',65537)]:
            with self.subTest(key=key),self.assertRaises(MODULE.Incomplete):
                MODULE.profile_counts(good.replace(f'{key}='+{'histories':'32','appends':'160','contexts':'64','getters':'65536'}[key],f'{key}={value}'))
        with self.assertRaises(MODULE.Incomplete):
            MODULE.profile_counts(self.history_row(observations=257,monotonic=257,coordinates=1028,exchanges=257))

    def test_history_combined_work_and_point_caps_charge_derived_work_once(self):
        h=self.history_row(appends=160,observations=0,monotonic=0,coordinates=0,exchanges=0)
        for fits,accepted in [(479,True),(480,True),(481,False)]:
            text=f'CORE_PROFILE_COUNTS fits={fits} points=0\n'+h
            if accepted:
                counts=MODULE.profile_counts(text)
                self.assertEqual(counts['charged_fit_work_upper'],fits+160)
                self.assertEqual(counts['observed_charged_fit_work'],fits)
            else:
                with self.assertRaises(MODULE.Incomplete): MODULE.profile_counts(text)
        self.assertEqual(MODULE.profile_counts('CORE_PROFILE_COUNTS fits=1 points=55296\n'+h)['point_admissions_upper'],65536)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.profile_counts('CORE_PROFILE_COUNTS fits=1 points=55297\n'+h)

    def test_history_actual_literal_docs_never_fabricate_unused_history_or_normalizer_calls(self):
        observed=MODULE.profile_counts(self.history_row())
        charged=MODULE.account_literal_doctests(observed)
        self.assertEqual(MODULE.LITERAL_DOCTEST_LIMITS['total_source_doctests_maximum'],5)
        self.assertEqual(MODULE.LITERAL_DOCTEST_LIMITS['legal_source_doctests_maximum'],2)
        self.assertEqual(charged['history_append_requests'],3)
        self.assertEqual(charged['stats_calls'],0)
        self.assertEqual(charged['charged_fit_work_upper'],3)
        self.assertEqual(charged['point_admissions_upper'],195)
        for key in ['literal_doctest_history_append_requests_upper','literal_doctest_normalization_requests_upper',
                    'literal_doctest_stats_calls_upper','literal_doctest_observation_constructor_attempts_upper']:
            self.assertEqual(charged[key],0)
        self.assertEqual(charged['literal_doctest_points_upper'],3)
        self.assertEqual(charged['literal_doctest_projection_terms_constructor_attempts_upper'],1)
        self.assertEqual(charged['literal_doctest_projection_terms_getter_requests_upper'],2)
        self.assertEqual(charged['literal_doctest_projection_term_garde_rule_callbacks_upper'],3)
        self.assertEqual(charged['literal_doctest_projection_term_predicate_calls_upper'],6)

    def test_history_pending_and_fully_bound_synthetic_schema_have_causal_neighbors(self):
        with tempfile.TemporaryDirectory() as temp:
            root=Path(temp);policy=self.fixture(root)
            self.assertEqual(MODULE.read_policy(root,'contracts')['source_file_count'],156)
            historical = self.historical_scope().select('all')
            self.assertEqual(historical['history'],'crates/time-core/src/regression/history.rs')
            self.assertEqual(list(historical),['regression','residuals','normalization','history','runs','selection','projection','enclosure','projected_batch'])
            self.assertEqual(MODULE.SCOPE.select('all'), S14C_SUBJECTS)
            for key in ['history_telemetry','normalization_telemetry']:
                bad=copy.deepcopy(policy);bad[key]['status']='pending-actual-source'
                self.write_policy(root,bad)
                with self.subTest(owner=key),self.assertRaises(MODULE.Incomplete): MODULE.read_policy(root,'contracts')
            self.write_policy(root,policy)
            for name in ['crates/time-core/src/regression/history.rs','crates/time-core/tests/history.rs','crates/time-core/tests/support/history_facts.rs']:
                path=root/name;old=path.read_bytes();path.write_bytes(old+b'drift')
                with self.subTest(path=name),self.assertRaises(MODULE.Incomplete): MODULE.read_policy(root,'contracts')
                path.write_bytes(old)

    def test_history_record_aggregation_retains_observed_and_upper_buckets(self):
        counts=MODULE.profile_counts(self.history_row())
        report={};MODULE.record_counts(report,'inert-history',counts)
        self.assertEqual(report['core_direct_and_stats_observations'],{'direct_fit_calls':0,'stats_calls':0,'point_admissions':0})
        self.assertEqual(report['core_history_external_observations']['history_append_requests'],3)
        self.assertEqual(report['core_history_source_owned_uppers']['history_statistics_entries_upper'],3)
        self.assertEqual(report['core_charged_subset_totals']['charged_fit_work_upper'],3)

    def test_history_two_actual_legal_and_eight_primary_diagnostics_have_finite_recipes(self):
        cases=MODULE.history_contract_examples();legal=[item for item in cases.values() if item['code'] is None]
        self.assertEqual(len(legal),2);self.assertEqual(len(cases)-len(legal),8)
        for field,total in [('history_constructions',2),('history_append_requests',5),
                            ('history_observation_constructor_attempts',5),('history_context_constructions',2),
                            ('history_utc_coordinate_constructor_attempts',20),('history_product_getter_requests',68)]:
            self.assertEqual(sum(item['calls'][field] for item in legal),total,field)
        self.assertEqual(sum(item['internal_upper']['history_statistics_entries_upper'] for item in legal),1)
        self.assertEqual(sum(item['internal_upper']['history_generated_point_admissions_upper'] for item in legal),3)
        for case in legal:
            self.assertEqual(case['calls']['direct_fit_calls']+case['calls']['stats_calls']+case['calls']['point_admissions'],0)
            self.assertLessEqual(case['calls']['history_product_getter_requests'],128)
        self.assertEqual(cases['history_append_borrow']['code'],'E0502')
        self.assertEqual(cases['history_report_private_construction']['code'],'E0451')
        self.assertEqual(cases['history_raw_append']['code'],'E0308')

    def test_nine_owner_variant_accounting_respects_one_selected_512_ceiling(self):
        historical = self.historical_scope().select('all')
        for amounts,accepted in [([57,57,57,57,57,57,57,57,56],True),([58,57,57,57,57,57,57,57,56],False)]:
            with tempfile.TemporaryDirectory() as temp:
                work=Path(temp)
                for subject,amount in zip(historical,amounts,strict=True):
                    out=work/'mutation'/subject/'mutants.out';out.mkdir(parents=True)
                    (out/'outcomes.json').write_text(json.dumps({'total_mutants':amount,'outcomes':[]}))
                if accepted:
                    report={'subjects': historical};MODULE.mutation_accounting(report,work)
                    self.assertEqual(report['mutation_generated_variants_selected_total'],512)
                    self.assertEqual(report['mutation_global_variant_ceiling'],512)
                else:
                    with self.assertRaisesRegex(MODULE.Incomplete, 'global512 ceiling'):
                        MODULE.mutation_accounting({'subjects': historical},work)


    def test_projection_missing_observations_and_nested_source_work_are_distinct(self):
        old = MODULE.profile_counts(self.history_row())
        self.assertIsNone(old['projection_recorded_call_subtotals'])
        self.assertIsNone(old['projection_source_owned_work_uppers'])
        counts = MODULE.profile_counts(self.complete_projection_rows(), require_projection=True)
        self.assertEqual(counts['projection_observed_calls'], {
            'histories': 12, 'appends': 94, 'observations': 94, 'contexts': 11,
            'monotonic': 122, 'coordinates': 376, 'exchanges': 94, 'getters': 3450,
            'terms': 68, 'projections': 49})
        self.assertEqual(counts['direct_fit_calls'], 0)
        self.assertEqual(counts['stats_calls'], 0)
        self.assertEqual(counts['charged_fit_work_upper'], 94)
        self.assertEqual(counts['point_admissions_upper'], 6016)
        self.assertEqual(counts['projection_source_owned_work_uppers']['projection_term_predicate_calls_upper'], 408)

    def test_complete_package_cannot_omit_projection_preeffect_charges(self):
        healthy = {'charged_fit_work_upper': 63129, 'point_admissions_upper': 2253360,
                   **MODULE.selection_reservation(package=True), 'observation_pending': True}
        MODULE.record_counts({}, 'synthetic-package', healthy)
        for field in ('projection_reserved_calls', 'projection_reserved_source_work_uppers',
                      'projection_reserved_private_source_work', 'enclosure_reserved_calls',
                      'enclosure_reserved_source_work_uppers', 'enclosure_reserved_private_source_work',
                      'charged_fit_work_upper'):
            bad = copy.deepcopy(healthy)
            if field == 'charged_fit_work_upper': bad[field] -= 1
            else: del bad[field]
            with self.subTest(field=field), self.assertRaises(MODULE.Incomplete):
                MODULE.record_counts({}, 'synthetic-package', bad)

    def test_all_mutation_union_above_512_refuses_before_session_effects(self):
        policy = {'mutation_inventories': {name: {'count': 513, 'sha256': 'a'*64}
                                           for name in MODULE.SUBJECTS}}
        args = SimpleNamespace(lane='mutation', subject='all', cohort_plan=Path('/inert-plan'),
                               cohort_id='all', output_dir=Path('/inert-output'))
        with patch.object(MODULE, 'read_policy', return_value=policy), \
             patch.object(MODULE.COMMON, 'Session') as session:
            with self.assertRaisesRegex(MODULE.Incomplete, 'exceeds512'):
                MODULE.execute(args, {})
            session.assert_not_called()


class InventoryAdmissionTests(HistoricalS14cTestCase):
    """Historical S158 closed discovery role; no tool or Rust runs."""

    def setUp(self):
        for name, value in (
                ('INVENTORY_FIXED', MODULE.S158_INVENTORY_FIXED),
                ('INVENTORY_SUBJECTS', MODULE.S158_INVENTORY_SUBJECTS),
                ('INVENTORY_BOUND_INPUTS', MODULE.S158_INVENTORY_BOUND_INPUTS),
                ('INVENTORY_SOURCE_FILE_COUNT', MODULE.S158_INVENTORY_SOURCE_FILE_COUNT),
                ('INVENTORY_ADMISSION', MODULE.S158_INVENTORY_ADMISSION),
                ('SOURCE_FILE_COUNT', MODULE.S158_INVENTORY_SOURCE_FILE_COUNT)):
            scoped = patch.object(MODULE, name, value)
            scoped.start()
            self.addCleanup(scoped.stop)

    NOW = 1700000100
    ERROR_VALUE = ('{ let _ = (projection, enclose_labeled_scalars, '
                   'FixtureChartProjection::enclosure_distance_seconds); FixtureChartError::Invariant }')
    SUBJECTS = {
        'combination': 'crates/time-core/src/combination.rs',
        'combination_admission': 'crates/time-core/src/combination/admission.rs',
        'combination_lifecycle': 'crates/time-core/src/combination/lifecycle.rs',
        'chart': 'crates/time-core/src/regression/chart.rs',
        'chart_roundoff': 'crates/time-core/src/regression/chart/roundoff.rs',
        'chart_combination_roundoff': 'crates/time-core/src/combination/chart/roundoff.rs',
        'chart_combination_receiving': 'crates/time-core/src/combination/chart/receiving.rs',
        'chart_combination_reduction': 'crates/time-core/src/combination/chart/reduction.rs',
        'chart_combination_lifecycle': 'crates/time-core/src/combination/lifecycle/chart.rs',
    }

    def fixture(self, root):
        """Create inert bytes and literal admission limits, not an admitted graph."""
        names = set(MODULE.INVENTORY_BOUND_INPUTS) | {
            'crates/time-core/src/selection.rs', 'crates/time-core/tests/selection.rs',
            'crates/time-core/tests/private/selection_guards.rs'}
        tools = {
            'scripts/verify-core-analysis.py', 'scripts/portable-analysis.py',
            'scripts/verify-time-domain-analysis.py', 'scripts/analysis-runtime.py',
            'scripts/core-selection-accounting.py', 'scripts/core-selection-contracts.py',
            'scripts/core-projection-accounting.py', 'scripts/core-analysis-cohorts.py',
            'scripts/core-enclosure-accounting.py', 'scripts/core-projected-batch-accounting.py',
            'scripts/core-combination-accounting.py',
            'scripts/core-adaptive-accounting.py', 'scripts/core-adaptive-contracts.py',
            'scripts/tests/test_core_adaptive_accounting.py',
            'scripts/tests/test_core_adaptive_contracts.py',
            'scripts/check-analysis.py', 'scripts/analysis-controls.py',
            'scripts/prepare-product.py', 'scripts/verify.py',
            'scripts/check-environment.py', 'scripts/check-strict.py',
            'policy/analysis-tools.json', *MODULE.CHART_INPUTS, *MODULE.CHART_COMBINATION_TOOL_INPUTS,
            *MODULE.RATE_DELIVERY_INPUTS}
        for name in names | tools:
            path = root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text('inert input ' + name + '\n')
        product = {name: MODULE.PRODUCT.sha256(root / name) for name in names}
        policy = {
            'schema_version': 13, 'admission_status': 'admitted-chart-combination-nine-owner-inventory-only',
            'tool_admission_profile': 'stable-core',
            'package': 'time-core', 'source_file_count': 152, 'subjects': dict(self.SUBJECTS),
            'maximum_mutants': 512, 'listing_maximum_per_file': 512,
            'listing_maximum_total': 4608, 'inventory_wall_seconds': 600,
            'mutation_jobs': 10, 'mutation_process_limit': 512, 'listing_process_limit': 128,
            'mutation_message_format': 'json',
            'mutation_error_values': {},
            'mutation_equivalences': [],
            'native_test_invocations': 0, 'campaigns_admitted': False,
            'root': str(root), 'created_unix_seconds': 1700000000,
            'expires_unix_seconds': 1700003600, 'product_inputs_sha256': product,
            'tool_inputs_sha256': {name: MODULE.PRODUCT.sha256(root / name) for name in tools},
        }
        path = root / MODULE.INVENTORY_ADMISSION
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps(policy, indent=2) + '\n')
        return policy, path

    def read(self, root, path, policy):
        with patch.object(MODULE.PRODUCT, 'admit_product', return_value=({}, policy['product_inputs_sha256'])), \
             patch.object(MODULE.time, 'time', return_value=self.NOW):
            return MODULE.read_inventory_admission(root, path)

    def test_exact_current_source_and_tools_admit_discovery_without_campaign_permission(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); policy, path = self.fixture(root)
            self.assertEqual(self.read(root, path, policy), policy)
            self.assertEqual(MODULE.INVENTORY_SUBJECTS, self.SUBJECTS)
            self.assertEqual(len(policy['product_inputs_sha256']), 152)
            self.assertEqual(MODULE.SUBJECTS, S14C_SUBJECTS)
            self.assertFalse(policy['campaigns_admitted'])
            self.assertEqual(MODULE.SOURCE_FILE_COUNT, 152)

    def test_unknown_fields_duplicate_fields_and_nonfinite_json_reject(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); policy, path = self.fixture(root)
            original = path.read_text()
            invalid = [json.dumps({**policy, 'unknown': 1}),
                       json.dumps({key: value for key, value in policy.items()
                                   if key != 'mutation_error_values'}),
                       original.replace('"schema_version": 13', '"schema_version": 13, "schema_version": 13', 1),
                       original.replace('"schema_version": 13', '"schema_version": NaN', 1),
                       '[]', 'null']
            for text in invalid:
                path.write_text(text)
                with self.subTest(text=text[:60]), self.assertRaises(MODULE.Incomplete):
                    self.read(root, path, policy)
            path.write_text(original)
            self.assertEqual(self.read(root, path, policy), policy)

    def test_fixed_limits_scope_status_and_timestamp_types_cannot_widen(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); healthy, path = self.fixture(root)
            changes = [('listing_maximum_per_file', True), ('listing_maximum_per_file', 513),
                       ('tool_admission_profile', 'full'), ('tool_admission_profile', None),
                       ('listing_maximum_total', 0), ('listing_maximum_total', 3073),
                       ('inventory_wall_seconds', 601), ('mutation_jobs', 8),
                       ('mutation_process_limit', 128), ('listing_process_limit', 512),
                       ('native_test_invocations', True), ('campaigns_admitted', 0),
                       ('source_file_count', 80), ('mutation_equivalences', [{}]),
                       ('mutation_error_values', None), ('mutation_error_values', []),
                       ('mutation_error_values', {'chart_batch': [self.ERROR_VALUE]}),
                       ('mutation_error_values', {'chart_enclosure': ['FixtureChartError::Invariant']}),
                       ('mutation_error_values', {'chart_enclosure': [self.ERROR_VALUE, self.ERROR_VALUE]}),
                       ('mutation_error_values', {'chart': [self.ERROR_VALUE]}),
                       ('schema_version', 9),
                       ('admission_status', 'admitted-final-source'), ('subjects', {}),
                       ('root', str(root / 'different')), ('created_unix_seconds', True),
                       ('created_unix_seconds', self.NOW + 1), ('expires_unix_seconds', self.NOW),
                       ('expires_unix_seconds', 1700003601)]
            for name, value in changes:
                changed = copy.deepcopy(healthy); changed[name] = value
                path.write_text(json.dumps(changed))
                with self.subTest(name=name, value=value), self.assertRaises(MODULE.Incomplete):
                    self.read(root, path, healthy)

    def test_product_tool_drift_missing_hash_extra_closure_and_redirect_reject(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); healthy, path = self.fixture(root)
            for field, name in [('product_inputs_sha256', 'crates/time-core/src/selection.rs'),
                                ('tool_inputs_sha256', 'scripts/portable-analysis.py')]:
                for value in (None, '0' * 63, 'A' * 64, '0' * 64):
                    changed = copy.deepcopy(healthy); changed[field][name] = value
                    path.write_text(json.dumps(changed))
                    with self.subTest(field=field, value=value), self.assertRaises(MODULE.Incomplete):
                        self.read(root, path, healthy)
                changed = copy.deepcopy(healthy); del changed[field][name]
                path.write_text(json.dumps(changed))
                with self.assertRaises(MODULE.Incomplete): self.read(root, path, healthy)
            path.write_text(json.dumps(healthy))
            source = root / 'crates/time-core/src/selection.rs'
            original = source.read_bytes(); source.write_bytes(original + b'changed')
            with self.assertRaises(MODULE.Incomplete): self.read(root, path, healthy)
            source.write_bytes(original)
            extra = root / 'scripts/new-tool.py'; extra.write_text('inert extra tooling')
            with self.assertRaises(MODULE.Incomplete): self.read(root, path, healthy)
            extra.unlink()
            redirect = root / 'tasks/redirect.json'; redirect.write_text(path.read_text())
            with self.assertRaises(MODULE.Incomplete): self.read(root, redirect, healthy)
            path.unlink(); path.symlink_to(redirect)
            with self.assertRaises(MODULE.Incomplete): self.read(root, path, healthy)

    def test_inventory_lists_nine_owners_with_separate_aggregate_and_campaign_caps(self):
        for per_file, total, exceeded in [(1, 9, False), (170, 1530, True), (512, 4608, True)]:
            with self.subTest(per_file=per_file), tempfile.TemporaryDirectory() as temporary:
                root = Path(temporary); policy, path = self.fixture(root)
                session = Mock(tools=Path('/inert-sealed-tools'), source=root,
                               product_identity=dict(policy['product_inputs_sha256']),
                               identity=dict(policy['tool_inputs_sha256']))
                def listing(label, command, timeout):
                    name = label.removeprefix('mutation-enumeration:')
                    self.assertEqual(command[-2:], ['--list', '--json'])
                    self.assertNotIn('--baseline', command)
                    if name == 'chart_enclosure':
                        self.assertEqual(command[-4:], ['--error', self.ERROR_VALUE, '--list', '--json'])
                        self.assertEqual(command.count('--error'), 1)
                    else:
                        self.assertNotIn('--error', command)
                    self.assertLessEqual(timeout, 120)
                    items = [{'package': 'time-core', 'file': self.SUBJECTS[name],
                              'name': name + '-' + str(index)} for index in range(per_file)]
                    return MODULE.COMMON.mutation_listing_warning(10) + json.dumps(items)
                session.run.side_effect = listing
                args = SimpleNamespace(lane='inventory', tool_admission_profile='stable-core', subject='all', inventory_admission=path,
                                       vendor_dir=root / 'unused-vendor', output_dir=root.parent / 'inert-output')
                report = {}
                with patch.object(MODULE, 'ROOT', root), \
                     patch.object(MODULE.PRODUCT, 'admit_product', return_value=({}, policy['product_inputs_sha256'])), \
                     patch.object(MODULE.time, 'time', return_value=self.NOW), \
                     patch.object(MODULE.COMMON, 'Session', return_value=session) as factory, \
                     patch.object(MODULE, 'read_policy', side_effect=AssertionError('ordinary policy used')), \
                     patch.object(MODULE.COMMON, 'run_analysis_lanes', side_effect=AssertionError('campaign used')), \
                     patch.object(MODULE, 'contracts', side_effect=AssertionError('contracts used')), \
                     patch.object(MODULE, 'fuzz', side_effect=AssertionError('fuzz used')):
                    MODULE.execute(args, report)
                self.assertEqual(factory.call_args.args[1].lane, 'mutation')
                self.assertEqual(factory.call_args.args[3].policy, 'tasks/s1.3-chart-combination-inventory-admission.json')
                self.assertEqual(session.run.call_count, 9)
                self.assertEqual(report['qualification_scope'],
                                 'selected critical-file inventory discovery only; no campaign or quality score')
                self.assertTrue(all(not call.kwargs for call in session.run.call_args_list))
                session.finish.assert_called_once()
                self.assertEqual(report['inventory_discovery']['total'], total)
                self.assertEqual(report['inventory_discovery']['exceeds_current_campaign_cap'], exceeded)
                self.assertEqual(set(report['inventory_discovery']['inventories']), set(self.SUBJECTS))
                self.assertIs(report['campaigns_admitted'], False)
                self.assertEqual(report['native_test_invocations'], 0)
                self.assertNotIn('mutation', report)

    def test_listing_receiving_guard_refuses_campaign_wrong_owner_and_profile_options(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); policy, path = self.fixture(root)
            for change in ('campaign', 'wrong-label', 'wrong-file', 'profile'):
                session = Mock(tools=Path('/inert-sealed-tools'), source=root,
                               product_identity=dict(policy['product_inputs_sha256']),
                               identity=dict(policy['tool_inputs_sha256']))
                args = SimpleNamespace(lane='inventory', tool_admission_profile='stable-core', subject='all', inventory_admission=path)
                def attempt(scope, subjects, admitted, tools, run, report, **_kwargs):
                    name = 'chart_combination_receiving'; label = 'mutation-enumeration:' + name
                    command = [str(tools / 'cargo-mutants'), 'mutants', '--package', 'time-core',
                               '--file', self.SUBJECTS[name], '--test-workspace', 'false',
                               '--cap-lints', 'false', '--jobs', '10', '--jobserver', 'true',
                               '--jobserver-tasks', '10', '--cargo-arg=--message-format=json', '--list', '--json']
                    options = {}
                    if change == 'campaign': command[-2:] = ['--baseline', 'run']
                    if change == 'wrong-label': label = 'mutation-campaign:principal'
                    if change == 'wrong-file': command[5] = 'crates/other.rs'
                    if change == 'profile': options['mutation_subject'] = 'principal'
                    run(label, command, 5, **options)
                with patch.object(MODULE, 'ROOT', root), \
                     patch.object(MODULE, 'read_inventory_admission', return_value=policy), \
                     patch.object(MODULE.time, 'time', return_value=self.NOW), \
                     patch.object(MODULE.COMMON, 'Session', return_value=session), \
                     patch.object(MODULE.COMMON, 'enumerate_mutation_inventories', side_effect=attempt):
                    with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                        MODULE.execute(args, {})
                session.run.assert_not_called()
                session.finish.assert_called_once()

    def test_nine_owner_receiver_rejects_old_or_extra_error_operators_before_run(self):
        """A literal healthy listing reaches Session; every wrong command stops first."""
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); policy, path = self.fixture(root)
            for change in ('healthy', 'missing-file', 'old-error', 'extra-error', 'wrong-owner'):
                session = Mock(tools=Path('/inert-sealed-tools'), source=root,
                               product_identity=dict(policy['product_inputs_sha256']),
                               identity=dict(policy['tool_inputs_sha256']))
                session.run.return_value = 'synthetic listing received'
                args = SimpleNamespace(lane='inventory', tool_admission_profile='stable-core',
                                       subject='all', inventory_admission=path)
                sent_commands = []

                def attempt(scope, subjects, admitted, tools, run, report, **_kwargs):
                    filename = self.SUBJECTS['chart_combination_receiving']
                    command = [str(tools / 'cargo-mutants'), 'mutants', '--package', 'time-core',
                               '--file', filename, '--test-workspace', 'false',
                               '--cap-lints', 'false', '--jobs', '10', '--jobserver', 'true',
                               '--jobserver-tasks', '10', '--cargo-arg=--message-format=json', '--list', '--json']
                    if change == 'missing-file': del command[4:6]
                    if change == 'old-error': command[-2:-2] = ['--error', self.ERROR_VALUE]
                    if change == 'extra-error': command[-2:-2] = ['--error', 'FixtureChartBatchError::Invariant']
                    if change == 'wrong-owner': command[5] = 'crates/time-core/src/chart_enclosure.rs'
                    sent_commands.append(command)
                    self.assertEqual(run('mutation-enumeration:chart_combination_receiving', command, 5),
                                     'synthetic listing received')
                    descriptor = {'package': 'time-core', 'file': filename,
                                  'name': 'synthetic-listed-descriptor'}
                    return SimpleNamespace(inventories={'chart_combination_receiving': {'synthetic': descriptor}},
                                           total=1, campaign_budget_exceeded=False)

                with patch.object(MODULE, 'ROOT', root), \
                     patch.object(MODULE, 'read_inventory_admission', return_value=policy), \
                     patch.object(MODULE.time, 'time', return_value=self.NOW), \
                     patch.object(MODULE.COMMON, 'Session', return_value=session) as factory, \
                     patch.object(MODULE.COMMON, 'enumerate_mutation_inventories', side_effect=attempt), \
                     self.subTest(change=change):
                    report = {}
                    if change == 'healthy':
                        MODULE.execute_inventory(args, report)
                        session.run.assert_called_once_with(
                            'mutation-enumeration:chart_combination_receiving', sent_commands[0], 5)
                        self.assertEqual(report['native_test_invocations'], 0)
                        self.assertFalse(report['campaigns_admitted'])
                    else:
                        with self.assertRaisesRegex(MODULE.Incomplete, 'non-listing command'):
                            MODULE.execute_inventory(args, report)
                        session.run.assert_not_called()
                    factory.assert_called_once()
                    session.finish.assert_called_once()

    def test_reader_to_session_handoff_rejects_changed_document_product_and_tool_maps(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); policy, path = self.fixture(root)
            original = path.read_text()
            for change in ('document', 'duplicate-document', 'product', 'tool', 'expiry'):
                path.write_text(original)
                session = Mock(tools=Path('/inert-sealed-tools'), source=root,
                               product_identity=dict(policy['product_inputs_sha256']),
                               identity=dict(policy['tool_inputs_sha256']))
                args = SimpleNamespace(lane='inventory', tool_admission_profile='stable-core', subject='all', inventory_admission=path)
                if change == 'document':
                    changed = copy.deepcopy(policy); changed['created_unix_seconds'] += 1
                    path.write_text(json.dumps(changed))
                if change == 'duplicate-document':
                    path.write_text(original.replace('"schema_version": 13',
                        '"schema_version": 13, "schema_version": 13', 1))
                if change == 'product': session.product_identity['Cargo.lock'] = '0' * 64
                if change == 'tool': session.identity['scripts/portable-analysis.py'] = '0' * 64
                now = policy['expires_unix_seconds'] if change == 'expiry' else self.NOW
                with patch.object(MODULE, 'ROOT', root), \
                     patch.object(MODULE, 'read_inventory_admission', return_value=policy), \
                     patch.object(MODULE.time, 'time', return_value=now), \
                     patch.object(MODULE.COMMON, 'Session', return_value=session), \
                     patch.object(MODULE.COMMON, 'enumerate_mutation_inventories') as listing:
                    with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                        MODULE.execute(args, {})
                listing.assert_not_called(); session.run.assert_not_called()
                session.finish.assert_called_once()

    def test_invalid_admission_and_old_lane_fail_before_inventory_or_session(self):
        session = Mock()
        with patch.object(MODULE, 'read_inventory_admission', side_effect=MODULE.Incomplete('bad inventory admission')), \
             patch.object(MODULE.COMMON, 'Session', session), \
             patch.object(MODULE.COMMON, 'enumerate_mutation_inventories') as listings:
            with self.assertRaisesRegex(MODULE.Incomplete, 'bad inventory admission'):
                MODULE.execute(SimpleNamespace(lane='inventory', tool_admission_profile='stable-core', subject='all', inventory_admission=Path('bad')), {})
        session.assert_not_called(); listings.assert_not_called()
        with patch.object(MODULE, 'read_policy', side_effect=MODULE.Incomplete('old five-owner gate')), \
             patch.object(MODULE, 'read_inventory_admission') as admission:
            with self.assertRaisesRegex(MODULE.Incomplete, 'old five-owner gate'):
                MODULE.execute(SimpleNamespace(lane='mutation', subject='all'), {})
        admission.assert_not_called()

    def test_inventory_profile_is_bound_before_any_session_effect(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            policy, path = self.fixture(root)
            for profile in ('stable-core', 'full', None, 'unknown'):
                args = SimpleNamespace(lane='inventory', subject='all', inventory_admission=path)
                if profile is not None:
                    args.tool_admission_profile = profile
                with patch.object(MODULE, 'ROOT', root), \
                     patch.object(MODULE, 'read_inventory_admission', return_value=policy), \
                     patch.object(MODULE.COMMON, 'Session', side_effect=MODULE.Incomplete('healthy session boundary')) as session:
                    with self.subTest(profile=profile):
                        if profile == 'stable-core':
                            with self.assertRaisesRegex(MODULE.Incomplete, 'healthy session boundary'):
                                MODULE.execute(args, {})
                            session.assert_called_once()
                            self.assertEqual(session.call_args.args[1].lane, 'mutation')
                            self.assertEqual(session.call_args.args[1].tool_admission_profile, 'stable-core')
                        else:
                            with self.assertRaises(MODULE.Incomplete):
                                MODULE.execute(args, {})
                            session.assert_not_called()
            for change in ('profile-missing', 'profile-full', 'scope', 'campaign'):
                changed = dict(policy)
                if change == 'profile-missing': changed.pop('tool_admission_profile')
                if change == 'profile-full': changed['tool_admission_profile'] = 'full'
                if change == 'scope': changed['subjects'] = {}
                if change == 'campaign': changed['campaigns_admitted'] = True
                args = SimpleNamespace(lane='inventory', subject='all', inventory_admission=path,
                                       tool_admission_profile='stable-core')
                with patch.object(MODULE, 'ROOT', root), \
                     patch.object(MODULE, 'read_inventory_admission', return_value=changed), \
                     patch.object(MODULE.COMMON, 'Session') as session, \
                     self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                    MODULE.execute(args, {})
                session.assert_not_called()

    def test_cli_requires_explicit_inventory_admission_and_complete_subjects(self):
        with tempfile.TemporaryDirectory() as temporary:
            base = ['--output-dir', temporary, '--vendor-dir', temporary]
            for options in (['--lane', 'inventory'],
                            ['--lane', 'inventory', '--inventory-admission', MODULE.INVENTORY_ADMISSION],
                            ['--lane', 'inventory', '--tool-admission-profile', 'stable-core'],
                            ['--lane', 'inventory', '--inventory-admission', 'tasks/s1.3-inventory-admission.json', '--subject', 'runs'],
                            ['--lane', 'coverage', '--inventory-admission', 'tasks/s1.3-inventory-admission.json']):
                with self.subTest(options=options), contextlib.redirect_stderr(io.StringIO()), \
                     self.assertRaises(SystemExit) as error:
                    MODULE.main(base + options)
                self.assertEqual(error.exception.code, 2)
            with patch.object(MODULE, 'execute') as execute, contextlib.redirect_stdout(io.StringIO()):
                self.assertEqual(MODULE.main(base + ['--lane', 'inventory', '--inventory-admission',
                                                     MODULE.INVENTORY_ADMISSION, '--tool-admission-profile', 'stable-core']), 0)
            self.assertEqual(execute.call_args.args[0].tool_admission_profile, 'stable-core')


class CoreDocumentReaderTests(HistoricalS14cTestCase):
    """Transport controls only; adjacent candidate/union admission stays separate."""

    @staticmethod
    def padded_json(path, total_bytes):
        """Write an independently sized complete ASCII JSON control."""
        prefix, suffix = b'{"padding":"', b'"}'
        if type(total_bytes) is not int or total_bytes < len(prefix) + len(suffix):
            raise ValueError("Invalid finite document fixture size.")
        path.write_bytes(prefix + b"x" * (total_bytes - len(prefix) - len(suffix)) + suffix)

    def test_union_requires_native_receipt_profile_to_match_bound_policy(self):
        """Metadata-only union accepts stable receipts without a native profile flag."""
        with tempfile.TemporaryDirectory() as temporary:
            base = Path(temporary)
            receipt = base / 'receipt.json'
            for index, profile in enumerate((None, 'full', 'unknown', 'stable-core')):
                payload = {} if profile is None else {'tool_admission_profile': profile}
                receipt.write_text(json.dumps(payload))
                args = SimpleNamespace(cohort_plan=base / 'plan.json', cohort_receipt=[receipt],
                                       output_dir=base / ('union-' + str(index)))
                with patch.object(MODULE, 'read_policy', return_value={'tool_admission_profile': 'stable-core'}), \
                     patch.object(MODULE, 'read_cohort_plan', return_value=({}, {}, args.cohort_plan)), \
                     patch.object(MODULE.COHORTS, 'reconcile_receipts', return_value={'status': 'synthetic'}) as reconcile:
                    if profile == 'stable-core':
                        MODULE.execute_cohort_union(args, {})
                        reconcile.assert_called_once()
                    else:
                        with self.assertRaisesRegex(MODULE.Incomplete, 'plan-bound policy'):
                            MODULE.execute_cohort_union(args, {})
                        reconcile.assert_not_called()
                        self.assertFalse(args.output_dir.exists())

    def test_explicit_stable_profile_cannot_enter_metadata_union(self):
        for lane in ('cohort-union',):
            args = SimpleNamespace(lane=lane, subject='all', tool_admission_profile='stable-core')
            with patch.object(MODULE, 'execute_inventory') as inventory, \
                 patch.object(MODULE, 'execute_cohort_union') as union, \
                 self.assertRaisesRegex(MODULE.Incomplete, 'requires the exact inventory card'):
                MODULE.execute(args, {})
            inventory.assert_not_called(); union.assert_not_called()
        with patch.object(MODULE, 'execute_cohort_union') as union:
            args = SimpleNamespace(lane='cohort-union', subject='all', tool_admission_profile='full')
            MODULE.execute(args, {})
            union.assert_called_once_with(args, {})

    def test_execute_union_uses_native_receipt_limit(self):
        """Behavioral RED on the existing entrypoint before the reader repair.

        One large raw object isolates transport from the separately controlled
        complete seven-owner union. The unchanged receiving entrypoint must pass
        its whole decoded object to that adjacent owner without any product run.
        """
        with tempfile.TemporaryDirectory() as temporary:
            base = Path(temporary)
            receipt = base / "verification.json"
            self.padded_json(receipt, 600 * 1024)
            args = SimpleNamespace(cohort_plan=base / "unused-plan.json",
                                   cohort_receipt=[receipt], output_dir=base / "union")
            report = {}
            expected_union = {"status": "synthetic-transport-only"}
            with patch.object(MODULE, "read_policy", return_value={}), \
                 patch.object(MODULE, "read_cohort_plan", return_value=({}, {}, args.cohort_plan)), \
                 patch.object(MODULE.COHORTS, "reconcile_receipts", return_value=expected_union) as union, \
                 patch.object(MODULE.COMMON, "Session") as session, \
                 patch.object(MODULE.COMMON, "run_analysis_lanes") as lanes:
                MODULE.execute_cohort_union(args, report)
            union.assert_called_once()
            self.assertEqual(len(union.call_args.args[1]), 1)
            self.assertEqual(len(union.call_args.args[1][0]["padding"]), 600 * 1024 - 14)
            self.assertEqual(report["core_cohort_union"], expected_union)
            self.assertEqual(report["owned_output"], str(args.output_dir))
            session.assert_not_called()
            lanes.assert_not_called()
            with self.assertRaisesRegex(MODULE.Incomplete, "fixed byte allowance"):
                MODULE.inventory_document(receipt)

    def test_inventory_exact_512_kib_and_native_positive_neighbor(self):
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "document.json"
            self.padded_json(path, 512 * 1024)
            self.assertEqual(len(MODULE.inventory_document(path)["padding"]), 512 * 1024 - 14)
            self.padded_json(path, 512 * 1024 + 1)
            self.assertEqual(len(MODULE.native_receipt_document(path)["padding"]), 512 * 1024 - 13)
            with self.assertRaisesRegex(MODULE.Incomplete, "fixed byte allowance"):
                MODULE.inventory_document(path)

    def test_native_exact_64_mib_neighbors_and_overflow(self):
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "verification.json"
            for length in (64 * 1024 * 1024 - 1, 64 * 1024 * 1024):
                with self.subTest(length=length):
                    self.padded_json(path, length)
                    self.assertEqual(len(MODULE.native_receipt_document(path)["padding"]), length - 14)
            self.padded_json(path, 64 * 1024 * 1024 + 1)
            with self.assertRaisesRegex(MODULE.Incomplete, "fixed byte allowance"):
                MODULE.native_receipt_document(path)

    def test_native_bound_counts_utf8_bytes_and_reads_the_complete_document(self):
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "verification.json"
            prefix, suffix = b'{"padding":"', b'"}'
            payload = prefix + "\u00e9".encode("utf-8") * ((64 * 1024 * 1024 - 14) // 2) + suffix
            self.assertEqual(len(payload), 64 * 1024 * 1024)
            path.write_bytes(payload)
            self.assertEqual(len(MODULE.native_receipt_document(path)["padding"]), (64 * 1024 * 1024 - 14) // 2)
            path.write_bytes(payload + b" ")
            with self.assertRaisesRegex(MODULE.Incomplete, "fixed byte allowance"):
                MODULE.native_receipt_document(path)

    def test_both_roles_reject_duplicate_fields_and_nonfinite_constants(self):
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "document.json"
            for reader in (MODULE.inventory_document, MODULE.native_receipt_document):
                path.write_bytes(b'{"number":0.5,"nested":{"value":1}}')
                self.assertEqual(reader(path)["number"], 0.5)
                for raw in (b'{"x":0,"x":1}', b'{"outer":{"x":0,"x":1}}',
                            b'{"x":NaN}', b'{"x":Infinity}', b'{"x":-Infinity}'):
                    path.write_bytes(raw)
                    with self.subTest(reader=reader.__name__, raw=raw), self.assertRaises(MODULE.Incomplete):
                        reader(path)

    def test_both_roles_reject_truncated_trailing_and_invalid_utf8_bytes(self):
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "document.json"
            for reader in (MODULE.inventory_document, MODULE.native_receipt_document):
                for raw in (b'{"x":', b'{}{}', b'{"x":0}ignored', b'{"x":"\xff"}'):
                    path.write_bytes(raw)
                    with self.subTest(reader=reader.__name__, raw=raw), self.assertRaises((MODULE.Incomplete, ValueError)):
                        reader(path)

    def test_both_roles_preserve_canonical_regular_path_checks(self):
        with tempfile.TemporaryDirectory() as temporary:
            base = Path(temporary)
            path = base / "document.json"
            path.write_bytes(b'{"value":1}')
            leaf_link = base / "linked.json"
            leaf_link.symlink_to(path)
            directory_link = base / "linked-directory"
            directory_link.symlink_to(base, target_is_directory=True)
            child = base / "child"
            child.mkdir()
            for reader in (MODULE.inventory_document, MODULE.native_receipt_document):
                self.assertEqual(reader(path), {"value": 1})
                for rejected in (base, leaf_link, directory_link / path.name, child / ".." / path.name):
                    with self.subTest(reader=reader.__name__, rejected=str(rejected)), self.assertRaises(MODULE.Incomplete):
                        reader(rejected)

    def test_unknown_roles_fail_before_any_path_operation(self):
        for role in (None, True, 0, [], {}, "native", "receipt", "inventory ", "native-receipt "):
            path = Mock()
            with self.subTest(role=role), self.assertRaises(MODULE.Incomplete):
                MODULE._core_json_document(path, role=role)
            path.resolve.assert_not_called()
            path.open.assert_not_called()


class ProjectedBatchWrapperTests(HistoricalS14cTestCase):
    """Independent literal new-family controls; no Rust or native outcomes."""

    def setUp(self):
        fund_synthetic_source_recipes(self)
        limits = patch.dict(MODULE.SOURCE_RAW_PRIVATE_GLOBAL_LIMITS,
                            {'square_root_calls_upper':33000, 'binary_arithmetic_calls_upper':22000})
        limits.start(); self.addCleanup(limits.stop)

    FIELDS = ('histories', 'appends', 'observations', 'contexts', 'monotonic',
              'coordinates', 'exchanges', 'getters', 'terms', 'projections',
              'enclosures', 'batches', 'minima', 'selections')
    RECIPES = (
        ('single_compatible', (1,3,3,1,6,12,3,24,1,1,1,1,1,1)),
        ('linear_selection', (3,9,9,3,16,36,9,425,3,3,3,2,2,3)),
        ('capacity_neighbors', (17,51,51,17,86,204,51,2234,17,17,17,5,1,3)),
        ('context_admission', (8,24,24,8,42,96,24,680,8,8,8,12,1,1)),
        ('duplicate_precedence', (3,9,9,3,16,36,9,305,3,3,3,3,1,1)),
        ('permutation_retention', (3,9,9,3,16,36,9,618,3,3,3,3,1,6)),
        ('subtick_overlap', (2,6,6,2,11,24,6,231,2,2,2,1,1,1)),
        ('projected_batch_private_admission', (1,3,3,1,5,12,3,6,1,1,1,3,0,0)),
        ('projected_batch_private_priority', (3,9,9,3,14,36,9,18,3,3,3,0,0,0)),
        ('projected_batch_private_delegation', (2,6,6,2,9,24,6,14,2,2,2,2,0,0)),
        ('projected_batch_private_errors', (0,) * 14),
    )

    @classmethod
    def row(cls, case, values):
        return 'CORE_PROJECTED_BATCH_PROFILE case=' + case + ''.join(
            ' ' + field + '=' + str(value) for field, value in zip(cls.FIELDS, values, strict=True)) + '\n'

    @classmethod
    def complete_rows(cls):
        public = ''.join(cls.row(case, values) for case, values in cls.RECIPES)
        guards = ''.join('CORE_PROJECTED_BATCH_GUARD_PROFILE case=projected_batch_private_' + case
                         + ' garde_callbacks=' + str(callbacks) + ' entry_read_attempts=' + str(reads) + '\n'
                         for case, callbacks, reads in (
                             ('admission', 3, 5), ('priority', 9, 20), ('delegation', 2, 12), ('errors', 0, 0)))
        return public + guards

    def test_disjoint_attempts_reuse_each_producer_and_selector_work_owner_once(self):
        counts = MODULE.profile_counts(self.complete_rows(), require_projected_batch=True)
        self.assertEqual(counts['projected_batch_observed_calls'], dict(zip(
            self.FIELDS, (43,129,129,43,221,516,129,4555,43,43,43,32,8,16), strict=True)))
        self.assertEqual((counts['direct_fit_calls'], counts['stats_calls']), (0, 0))
        self.assertEqual((counts['charged_fit_work_upper'], counts['point_admissions_upper']), (129, 8256))
        self.assertIsNone(counts['projection_observed_calls'])
        self.assertIsNone(counts['enclosure_observed_calls'])
        work = counts['projected_batch_source_owned_work_uppers']
        self.assertEqual(len(work), 49)
        self.assertEqual(work['enclosure_interval_constructor_attempts_upper'], 43)
        self.assertEqual(work['selection_batch_copy_adapter_attempts_upper'], 512)
        self.assertEqual(work['selection_membership_probes_upper'], 16128)
        self.assertEqual(work['minimum_garde_validate_calls_upper'], 8)
        self.assertEqual(work['scratch_Copy_assignments_upper'], 992)
        self.assertEqual(work['wrapper_debug_format_calls_upper'], 4555)
        self.assertEqual(MODULE.PROJECTED_BATCH.full_package_reservation()[
            'projected_batch_reserved_calls']['getters'], 4655)

    def test_observers_are_actual_overlapping_observations_and_add_no_charges(self):
        public = ''.join(self.row(case, values) for case, values in self.RECIPES)
        without = MODULE.profile_counts(public)
        with_guards = MODULE.profile_counts(self.complete_rows(), require_projected_batch=True)
        self.assertEqual(with_guards['projected_batch_guard_observed_calls'],
                         {'garde_callbacks': 14, 'entry_read_attempts': 37})
        for field in ('charged_fit_work_upper', 'point_admissions_upper', 'projected_batch_source_owned_work_uppers'):
            self.assertEqual(without[field], with_guards[field])
        self.assertIsNone(without['projected_batch_guard_observed_calls'])
        with self.assertRaises(MODULE.Incomplete):
            MODULE.profile_counts(public, require_projected_batch=True)

    def test_missing_and_partial_attempts_keep_full_preeffect_reservations(self):
        missing = MODULE.profile_counts('CORE_PROFILE_COUNTS fits=1 points=3\n')
        self.assertIsNone(missing['projected_batch_observed_calls'])
        self.assertIsNone(missing['projected_batch_source_owned_work_uppers'])
        reservation = MODULE.selection_reservation(package=True)
        self.assertEqual(reservation['projected_batch_reserved_calls']['appends'], 129)
        self.assertEqual(reservation['projected_batch_reserved_private_source_work']['direct_raw_authored_helper_entries_upper'], 91)
        report = {}
        MODULE.record_counts(report, 'synthetic-package', {
            **MODULE.package_charged_upper(), **reservation, 'observation_pending': True})
        self.assertIsNone(report['core_projected_batch_recorded_call_subtotals'])
        prefix = MODULE.profile_counts(self.row('single_compatible', (1,) + (0,) * 13))
        row = report['core_invocations'][0]
        row.update(observed_counts=prefix, observation_pending=False)
        MODULE.refresh_accounting(report)
        self.assertEqual(report['core_projected_batch_recorded_call_subtotals']['histories'], 1)
        self.assertIsNone(report['core_projected_batch_complete_observed_call_subtotals'])
        self.assertEqual(report['core_projected_batch_pending_observation_invocations'], ['synthetic-package'])
        self.assertEqual(report['core_projected_batch_reserved_totals']['projected_batch.call.appends'], 129)
        self.assertEqual(report['core_charged_subset_totals'],
                         {'charged_fit_work_upper': 63129, 'point_admissions_upper': 2253360})

    def test_package_reservation_rejects_each_missing_or_forged_new_work_owner(self):
        good = {**MODULE.package_charged_upper(), **MODULE.selection_reservation(package=True),
                'observation_pending': True}
        for field in ('projected_batch_reserved_calls', 'projected_batch_reserved_module_work_uppers',
                      'projected_batch_reserved_source_work_uppers', 'projected_batch_reserved_private_source_work'):
            for change in ('missing', 'zero'):
                bad = copy.deepcopy(good)
                if change == 'missing':
                    del bad[field]
                else:
                    bad[field] = {key: 0 for key in bad[field]}
                with self.subTest(field=field, change=change), self.assertRaises(MODULE.Incomplete):
                    MODULE.record_counts({}, 'forged', bad)
        bad = copy.deepcopy(good)
        del bad['selection_expected']
        with self.assertRaises(MODULE.Incomplete):
            MODULE.record_counts({}, 'unowned', bad)

    def test_guard_only_and_mixed_standalone_selection_streams_fail_closed(self):
        guard = 'CORE_PROJECTED_BATCH_GUARD_PROFILE case=projected_batch_private_errors garde_callbacks=0 entry_read_attempts=0\n'
        with self.assertRaises(MODULE.Incomplete):
            MODULE.profile_counts(guard)
        external = 'CORE_SELECTION_EXTERNAL_PROFILE intervals=1 minima=0 batches=0 selections=0\n'
        with self.assertRaises(MODULE.Incomplete):
            MODULE.profile_counts(external + self.complete_rows())

    def test_new_external_registry_is_compile_only_and_explicitly_paired(self):
        cases = MODULE.projected_batch_contract_examples()
        self.assertEqual(len(cases), 14)
        self.assertTrue(all(item['code'] is not None and 'calls' not in item for item in cases.values()))
        for name, item in cases.items():
            with self.subTest(name=name):
                self.assertEqual(item['external_legal_neighbor'], {
                    'file': 'crates/time-core/tests/projected_batch.rs',
                    'test': 'genuine_compatible_projection_reaches_real_batch_selection'})
                self.assertGreater(item['line'], 3)
                self.assertNotIn('unsafe', item['source'])
        self.assertEqual(cases['projected_batch_escaped_borrow']['code'], 'E0597')
        self.assertIn('batch.entries()', cases['projected_batch_escaped_borrow']['source'])

    def test_actual_elided_mutability_diagnostic_matches_the_intended_entries_probe(self):
        # Actual contracts-v1 receipt aa79e6fb...7734b35, final native lane134.
        # Reconstruct the complete pinned JSON schema with synthetic byte ranges;
        # preserve the observed causal text/locations independently of registry
        # tokens. Rustc elides the identical underlying type as `_`.
        source = '/work/contract-inputs/projected_batch_mutable_entries.rs'
        diagnostic = {
            '$message_type': 'diagnostic', 'message': 'mismatched types',
            'code': {'code': 'E0308', 'explanation': None}, 'level': 'error',
            'spans': [
                synthetic_compiler_span(source, 5, column_start=41, column_end=56,
                    label='types differ in mutability',
                    text=[{'text': ' let _: &mut [FixtureProjectedInterval]=batch.entries();',
                           'highlight_start': 41, 'highlight_end': 56}]),
                synthetic_compiler_span(source, 5, column_start=9, column_end=40,
                    is_primary=False, label='expected due to this'),
            ],
            'children': [{'message': 'expected mutable reference `&mut _`\n           found reference `&_`',
                          'code': None, 'level': 'note', 'spans': [], 'children': [], 'rendered': None}],
            'rendered': None,
        }
        specimen = MODULE.projected_batch_contract_examples()['projected_batch_mutable_entries']
        self.assertEqual(specimen['code'], 'E0308')
        self.assertEqual(specimen['line'], 5)
        self.assertEqual(specimen['source'].splitlines()[4],
                         ' let _: &mut [FixtureProjectedInterval]=batch.entries();')
        wrong_type = copy.deepcopy(diagnostic)
        wrong_type['spans'][0]['label'] = 'expected bool, found u64'
        wrong_type['children'][0]['message'] = 'expected `bool`, found `u64`'
        wrong_span = copy.deepcopy(diagnostic)
        wrong_span['spans'][0]['line_start'] = 6
        wrong_span['spans'][0]['line_end'] = 6
        for rejected in (wrong_type, wrong_span):
            with self.subTest(neighbor=rejected), self.assertRaises(MODULE.Incomplete):
                MODULE.COMMON.compiler_cause(
                    {'exit_code': 1, 'failure': None, 'output': json.dumps(rejected)}, specimen, source)
        try:
            MODULE.COMMON.compiler_cause(
                {'exit_code': 1, 'failure': None, 'output': json.dumps(diagnostic)}, specimen, source)
        except MODULE.Incomplete as error:
            self.fail('Actual intended mutability rejection was not admitted: ' + str(error))

    def test_nonmutation_prospective_source_vector_is_checked_before_effects(self):
        required = MODULE.projected_batch_budget_vector()
        subjects = dict(MODULE.SUBJECTS)
        with patch.dict(MODULE.MUTATION_INVENTORIES, {
                 name: {'count': 1, 'sha256': '1' * 64} for name in subjects}, clear=True), \
             patch.dict(MODULE.FINAL_PACKAGE_TEST_COUNTS, {'baseline': 1}), \
             patch.dict(MODULE.COMBINATION_TELEMETRY, {'global_limits': MODULE.combination_budget_vector()}):
            with patch.dict(MODULE.PROJECTED_BATCH_TELEMETRY, {'global_limits': required}):
                coverage = MODULE.prospective_scope_reservation('coverage', subjects)
                self.assertEqual(coverage['projected_batch_source_work_upper'], required)
                reserved = MODULE.prospective_scope_reservation('contracts', subjects)
                self.assertEqual(reserved['projected_batch_source_work_upper'],
                                 {key: 0 for key in required})
                for lane in ('all', 'fuzz'):
                    with self.subTest(lane=lane), self.assertRaisesRegex(MODULE.Incomplete, 'exact admitted lanes/owners'):
                        MODULE.prospective_scope_reservation(lane, subjects)
                # A one-package combination allowance cannot fund both cohorts.
                with self.assertRaisesRegex(
                        MODULE.Incomplete,
                        r'^Complete prospective core scope exceeds its admitted finite reservation\.$'):
                    MODULE.prospective_scope_reservation('mutation', subjects)
            for limits in (None, {**required, 'projected_batch.call.batches': required['projected_batch.call.batches'] - 1}):
                with self.subTest(limits=limits), patch.dict(
                        MODULE.PROJECTED_BATCH_TELEMETRY, {'global_limits': limits}), self.assertRaises(MODULE.Incomplete):
                    MODULE.prospective_scope_reservation('coverage', subjects)


class CheckedInCorePolicyTests(unittest.TestCase):
    """Exercise the real checked-in source admission outside synthetic fixtures."""

    def bound_source_mismatches(self, root, bindings):
        """Compare actual regular bytes with explicit frozen pins, without rebinding."""
        return {name: (expected, actual) for name, expected in bindings.items()
                if (actual := MODULE.PRODUCT.sha256(root / name)) != expected}

    def test_checked_in_control_does_not_require_machine_local_prior_receipt(self):
        """Simulate an absent private receipt without moving any retained bytes."""
        prior = Path(MODULE.HISTORICAL_S148_COORDINATOR_COHORTS['spent_contract_attempt']['receipt'])
        resolve = Path.resolve

        def absent_prior(path, *args, **kwargs):
            if path == prior:
                raise FileNotFoundError('simulated clean-checkout missing private receipt')
            return resolve(path, *args, **kwargs)

        with patch.object(Path, 'resolve', absent_prior):
            # Production custody still fails closed in this exact environment.
            with self.assertRaisesRegex(MODULE.Incomplete, 'receipt is unavailable'):
                MODULE.admit_spent_contract_attempt()
            try:
                self.test_changed_checkpoint_is_pending_or_exactly_bound_without_historical_credit()
            except MODULE.Incomplete as error:
                self.fail('Checked-in source control requires a machine-local receipt: ' + str(error))

    def test_changed_checkpoint_is_pending_or_exactly_bound_without_historical_credit(self):
        """Pending discovery must fail closed; admitted facts must bind current bytes."""
        self.assertEqual(MODULE.SOURCE_FILE_COUNT, 158)
        self.assertIn(MODULE.FINAL_PACKAGE_TEST_COUNTS,
                      ({'ordinary': 0, 'doctests': 0, 'baseline': 0},
                       {'ordinary': 306, 'doctests': 36, 'baseline': 342}))
        self.assertEqual(MODULE.SUBJECTS, MODULE.SETTLED_RATE_SUBJECTS)
        self.assertEqual(len(MODULE.COMPLETE_CRITICAL_SUBJECTS), 25)
        self.assertEqual(sum(row['count'] for row in
                             MODULE.HISTORICAL_LIFECYCLE_MUTATION_INVENTORIES.values()), 747)
        # Exercise both outcomes of byte classification independently of the
        # retired checkpoint. This fixture performs no product/campaign admission.
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); source = root / 'bound.rs'
            original = b'healthy independently bound source\n'
            source.write_bytes(original)
            pins = {'bound.rs': hashlib.sha256(original).hexdigest()}
            self.assertEqual(self.bound_source_mismatches(root, pins), {})
            source.write_bytes(b'changed independently bound source\n')
            changed = self.bound_source_mismatches(root, pins)
            self.assertEqual(set(changed), {'bound.rs'})
            self.assertNotEqual(*changed['bound.rs'])
        path = ROOT / MODULE.POLICY
        frozen_policy_bytes = path.read_bytes()
        document = json.loads(frozen_policy_bytes)
        live_drift = self.bound_source_mismatches(ROOT, MODULE.SETTLED_RATE_LIVE_SOURCE_SHA256)
        protected_drift = self.bound_source_mismatches(ROOT, document['inputs_sha256'])
        _, current_product = MODULE.PRODUCT.admit_product(ROOT)
        changed_product_set = set(current_product) != set(document['inputs_sha256'])
        if live_drift or protected_drift or changed_product_set:
            if live_drift:
                reason = r'^Settled-rate live chart, receiver or unchanged adapter source differs\.$'
            elif protected_drift:
                reason = r'^Core source/graph input is unbound or changed\.$'
            else:
                reason = (r'^Core analysis requires the exact complete'
                          + str(MODULE.SOURCE_FILE_COUNT) + r'-file product inventory\.$')
            for expected, actual in (*live_drift.values(), *protected_drift.values()):
                self.assertNotEqual(expected, actual)
            with patch.object(MODULE, 'admit_spent_contract_attempt', return_value=None) as prior:
                with self.assertRaisesRegex(MODULE.Incomplete, reason):
                    MODULE.read_policy(ROOT, 'coverage')
            prior.assert_not_called()
            self.assertEqual(path.read_bytes(), frozen_policy_bytes)
            return
        # This control admits checked-in source, not machine-local run evidence.
        # SpentContractAttemptTests separately retains S148 receipt custody;
        # current S1.4c admission must not require that machine-local artifact.
        if not MODULE.MUTATION_INVENTORIES:
            self.assertIsNone(MODULE.CHECKPOINT_SOURCE_RESERVATION)
            with patch.object(MODULE, 'admit_spent_contract_attempt', return_value=None), \
                    self.assertRaises(MODULE.Incomplete):
                MODULE.read_policy(ROOT, 'coverage')
            return
        with patch.object(MODULE, 'admit_spent_contract_attempt', return_value=None) as prior:
            policy = MODULE.read_policy(ROOT, 'coverage')
        prior.assert_not_called()
        self.assertEqual(policy['inputs_sha256'], MODULE.PRODUCT.admit_product(ROOT)[1])
        equivalents = policy['mutation_equivalences']
        self.assertEqual(equivalents, [])
        MODULE.admit_reference_input_equivalence_scope(equivalents)
        # Historical proofs do not enter this new candidate's empty-E scope.
        self.assertEqual(MODULE.COMMON.admit_mutation_equivalences(ROOT, MODULE.SCOPE, policy),
                         equivalents)
        self.assertEqual(set(policy['mutation_inventories']), set(MODULE.SUBJECTS))
        self.assertEqual(policy['mutation_inventories'], MODULE.MUTATION_INVENTORIES)
        self.assertNotEqual(policy['mutation_inventories'], MODULE.HISTORICAL_S148_MUTATION_INVENTORIES)
        reservation = MODULE.CHECKPOINT_SOURCE_RESERVATION
        self.assertEqual(reservation['mutation_variants'],
                         sum(row['count'] for row in policy['mutation_inventories'].values()))
        self.assertEqual(reservation['mutation_baselines'], 4)
        packages = sum(row['count'] + 1 for row in policy['mutation_inventories'].values())
        self.assertEqual(reservation['source_work']['charged_fit_work_upper'], packages * 63501 + 63501 + 685)
        self.assertEqual(reservation['source_work']['point_admissions_upper'], packages * 2266209 + 2266206 + 23679)
        self.assertEqual(reservation['mutation_package_invocations'],
                         reservation['mutation_variants'] + 4)
        self.assertEqual(set(reservation['source_work']),
                         set(MODULE.cohort_budget_vectors()[0]))

    def test_nonmutation_chart_helpers_enter_actual_root_and_copied_source_custody(self):
        """Exercise actual Session.check at an inert constructor boundary.

        Only the constructor/tool admission is replaced. Actual digest and
        copied-source inventory checks must reject every helper's changed bytes
        before a native boundary could be reached. The original product owner is
        isolated because these temporary files are deliberate synthetic inputs.
        """
        session_type = MODULE.COMMON.Session
        class CapturedBeforeNative(Exception):
            pass

        for lane in ('contracts', 'coverage'):
            with self.subTest(lane=lane), tempfile.TemporaryDirectory() as temporary:
                root = Path(temporary); output = root / 'owned-output'
                captured = []
                native_boundary = Mock()

                def check_before_native(root_arg, args, report, scope, policy, extra):
                    captured.extend(extra)
                    names = set(extra) | set(MODULE.CHART_INPUTS) | set(MODULE.CHART_COMBINATION_TOOL_INPUTS)
                    for name in names:
                        path = root / name; path.parent.mkdir(parents=True, exist_ok=True)
                        path.write_text('synthetic admitted input: ' + name + '\n')
                    receiving = object.__new__(session_type)
                    receiving.root = root
                    receiving.identity = {name: MODULE.PRODUCT.sha256(root / name) for name in set(extra)}
                    receiving.product_identity = {}
                    receiving.source = root / 'copied-source'; receiving.source.mkdir()
                    for name in receiving.identity:
                        target = receiving.source / name
                        target.parent.mkdir(parents=True, exist_ok=True)
                        target.write_bytes((root / name).read_bytes())
                    receiving.source_inventory = MODULE.PRODUCT.inventory(receiving.source)
                    receiving.config = root / 'owned-cargo-config'
                    receiving.config.write_text('synthetic inert configuration\n')
                    receiving.config_sha256 = MODULE.PRODUCT.sha256(receiving.config)
                    receiving.protected_tools = {}; receiving.protected_islands = {}
                    with patch.object(MODULE.COMMON.PRODUCT, 'admit_product', return_value=({}, {})):
                        session_type.check(receiving)
                        for name in (*MODULE.CHART_INPUTS, *MODULE.CHART_COMBINATION_TOOL_INPUTS):
                            for base in (root, receiving.source):
                                with self.subTest(lane=lane, helper=name, island=base.name):
                                    path = base / name
                                    self.assertTrue(path.is_file(), 'Session did not copy an admitted chart helper')
                                    original = path.read_bytes()
                                    path.write_bytes(original + b'changed helper bytes\n')
                                    try:
                                        with self.assertRaisesRegex(MODULE.COMMON.Incomplete,
                                                'input changed|source inventory changed'):
                                            session_type.check(receiving)
                                    finally:
                                        path.write_bytes(original)
                                    session_type.check(receiving)
                    self.assertTrue(set(MODULE.CHART_INPUTS) <= set(captured))
                    self.assertTrue(set(MODULE.CHART_COMBINATION_TOOL_INPUTS) <= set(captured))
                    # Raising here is the mock native boundary: no constructor,
                    # tool admission, compiler or execution API has been called.
                    raise CapturedBeforeNative

                with patch.object(MODULE, 'ROOT', root), \
                     patch.object(MODULE, 'read_policy', return_value={}), \
                     patch.dict(MODULE.COORDINATOR_COHORTS, {'nonmutation_outputs': {lane: str(output)}}), \
                     patch.object(MODULE, 'prospective_scope_reservation', return_value={}), \
                     patch.object(MODULE.COMMON, 'Session', side_effect=check_before_native), \
                     patch.object(MODULE.COMMON, 'run_analysis_lanes', native_boundary):
                    with self.assertRaises(CapturedBeforeNative):
                        MODULE.execute(SimpleNamespace(lane=lane, subject='all', output_dir=output), {})
                native_boundary.assert_not_called()

    def test_nonmutation_session_binds_enclosure_helper_before_native_work(self):
        """Capture the real execution boundary before any tool can be admitted."""
        class CapturedBeforeNative(Exception):
            """Terminate at Session construction without starting a subprocess."""

        for lane in ('coverage', 'contracts'):
            with self.subTest(lane=lane), tempfile.TemporaryDirectory() as temporary:
                captured = []
                retry = Path(temporary) / 'retry'

                def stop_before_native(root, args, report, scope, policy, extra):
                    captured.extend(extra)
                    raise CapturedBeforeNative

                with patch.object(MODULE, 'read_policy', return_value={}), \
                     patch.dict(MODULE.COORDINATOR_COHORTS, {'nonmutation_outputs': {lane: str(retry)}}), \
                     patch.object(MODULE, 'prospective_scope_reservation', return_value={}), \
                     patch.object(MODULE.COMMON, 'Session', side_effect=stop_before_native):
                    with self.assertRaises(CapturedBeforeNative):
                        MODULE.execute(SimpleNamespace(lane=lane, subject='all', output_dir=retry), {})
                # Healthy neighboring tool owners already cross this actual
                # boundary. The enclosure observer must receive the same custody.
                self.assertIn('scripts/core-projection-accounting.py', captured)
                self.assertIn('scripts/core-analysis-cohorts.py', captured)
                self.assertIn('scripts/core-enclosure-accounting.py', captured)
                self.assertIn('scripts/core-projected-batch-accounting.py', captured)
                self.assertIn('scripts/core-combination-accounting.py', captured)


class RetainedChartPrincipalIntegrationTests(HistoricalS14cTestCase):
    """Synthetic current receiving and source-reservation composition controls."""

    NAMES = (
        'chart_principal_cases::retained_owner_preserves_credit_and_actual_chart_provenance',
        'chart_principal_cases::genuine_new_observation_survives_chart_receiving_once',
        'chart_principal_cases::unexpected_chart_rejects_atomically_and_equal_neighbor_recovers',
        'chart_principal_cases::source_origin_and_evaluation_guards_remain_unavoidable',
        'chart_principal_cases::expected_frame_bits_and_metadata_capacity_reject_before_commit',
        'chart_principal_cases::chart_event_rejections_preserve_credit_for_valid_retry',
        'chart_principal_cases::chart_principal_exact_metadata_capacity_and_overflow_rollback',
        'chart_principal_pending::chart_replacement_consumes_preexisting_pending_credit_once',
    )
    PASSED = ''.join('test ' + name + ' ... ok\n' for name in NAMES)
    DOCS = (
        'test crates/time-core/src/principal/chart.rs - principal::chart::FixtureChartPrincipalDecision (line 123) ... ok\n'
        'test crates/time-core/src/principal/chart.rs - principal::chart::FixtureChartPrincipalDecision (line 138) - compile fail ... ok\n'
        'test crates/time-core/src/principal/chart.rs - principal::chart::FixtureChartPrincipalDecision (line 150) - compile fail ... ok\n'
    )

    def test_current_package_and_cohort_fund_new_producers_exactly_once(self):
        self.assertEqual(MODULE.historical_s157_package_charged_upper(), {
            'charged_fit_work_upper': 48807, 'point_admissions_upper': 1758789})
        self.assertEqual(MODULE.historical_s157_package_charged_upper(include_doctests=True), {
            'charged_fit_work_upper': 48807, 'point_admissions_upper': 1758792})
        package, checks, limits = MODULE.historical_s157_cohort_budget_vectors(require_limits=False)
        self.assertEqual([len(v) for v in (package, *checks.values(), limits)], [2214]*4)
        for vector, fits, points in [(package,48807,1758792),
                                     (checks['coverage'],48807,1758789),
                                     (checks['contracts'],499,17256)]:
            self.assertEqual((vector['charged_fit_work_upper'], vector['point_admissions_upper']), (fits,points))
        self.assertEqual(package['chart_principal.prior_adapter.get_original_dispatches_upper'], 51824)
        self.assertEqual(checks['contracts']['chart_principal.prior_adapter.get_original_dispatches_upper'],512)

    def test_named_new_outcomes_add_only_new_source_upper_and_require_all_cases(self):
        text=''.join('test '+name+' ... ok\n' for name in self.NAMES)
        observed=MODULE.profile_counts(text,require_fit=False,require_chart_principal=True)
        self.assertEqual((observed['charged_fit_work_upper'],observed['point_admissions_upper']), (7564,261202))
        self.assertFalse(observed['legacy_profile_observation_present'])
        self.assertIsNone(observed['chart_principal_observed_operation_counts'])
        self.assertFalse(observed['chart_principal_source_only_pending'])
        for bad in [text.splitlines(keepends=True)[0],text.replace(' ... ok\n',' ... FAILED\n',1),
                    text.replace('test chart_principal_cases::chart_principal_exact_metadata_capacity_and_overflow_rollback ... ok\n', ''),
                    text+text.splitlines(keepends=True)[0]]:
            with self.subTest(bad=bad[:100]),self.assertRaises(MODULE.Incomplete):
                MODULE.profile_counts(bad,require_fit=False,require_chart_principal=True)

    def test_new_source_work_stays_outside_preserved_literal_doctest_caps(self):
        legacy = MODULE.profile_counts('CORE_PROFILE_COUNTS fits=1 points=3\n')
        healthy = MODULE.account_literal_doctests(legacy)
        complete = MODULE.profile_counts(
            'CORE_PROFILE_COUNTS fits=1 points=3\n' + self.PASSED,
            require_chart_principal=True)
        accounted = MODULE.account_literal_doctests(complete)
        self.assertEqual((accounted['charged_fit_work_upper'], accounted['point_admissions_upper']),
                         (7565, 261208))
        self.assertEqual(accounted['legacy_charged_fit_work_upper'],
                         healthy['legacy_charged_fit_work_upper'])
        self.assertEqual(accounted['legacy_point_admissions_upper'],
                         healthy['legacy_point_admissions_upper'])
        self.assertEqual(accounted['chart_principal_recorded_source_work_upper'],
                         {'charged_fit_work_upper': 7564, 'point_admissions_upper': 261202})
        self.assertIsNone(accounted['chart_principal_observed_operation_counts'])
        # Separating a source-only delta cannot excuse genuinely over-budget
        # original-owner work. Keep that neighboring rejection independent.
        over = dict(complete)
        over['charged_fit_work_upper'] += MODULE.PROFILE_LIMITS['charged_fit_work_upper']
        with self.assertRaisesRegex(MODULE.Incomplete, 'preserved existing-owner aggregate profile'):
            MODULE.account_literal_doctests(over)


class S157CurrentFloorEquivalenceControls(HistoricalS14cTestCase):
    """Retained S157 proof controls; no candidate or E credit transfers to S158."""

    @staticmethod
    def historical_scope():
        return MODULE.COMMON.Scope('time-core', tuple(MODULE.HISTORICAL_S157_SUBJECTS.items()), MODULE.POLICY)

    @staticmethod
    def record():
        return {
            'id': 'S157-P-E-002',
            'candidate_sha256': 'ad59bd667d96779625a1368b21108ca86138bef8ee8b95a17785cf3962ae54d3',
            'source_sha256': 'cf4962c8d3536a528b7c783543379bdaeaf2e4070355fbeb7921fb5329b63d1d',
            'mutant': {
                'name': 'crates/time-core/src/principal.rs:991:17: replace < with <= in updated_score',
                'package': 'time-core', 'file': 'crates/time-core/src/principal.rs',
                'function': {'function_name': 'updated_score',
                             'return_type': '-> Result<f64, FixturePrincipalError>',
                             'span': {'start': {'line': 951, 'column': 1},
                                      'end': {'line': 992, 'column': 2}}},
                'span': {'start': {'line': 991, 'column': 17},
                         'end': {'line': 991, 'column': 18}},
                'replacement': '<=', 'genre': 'BinaryOperator',
            },
            'requirement': 'CHECK-7/8/14/25',
            'rationale': 'Independent finite binary64 floor proof; exact new native miss remains required.',
            'proof': {
                'path': 'tasks/s1.3-principal-floor-current-equivalence.md',
                'sha256': 'd4ed322e1dc80912280ea5d5da161a323d60ef502b19b70899894b664352dee5',
                'section': 'S157-P-E-002', 'reviewer': 'root; s157_gate_review',
                'verdict': 'independently-justified-equivalent',
            },
        }

    def test_empty_scope_preserves_no_implicit_equivalence(self):
        self.assertIsNone(MODULE.historical_s157_admit_core_equivalence_scope([]))

    def test_current_scope_rejects_retained_s157_proof(self):
        self.assertIsNone(MODULE.admit_core_equivalence_scope([]))
        with self.assertRaises(MODULE.Incomplete):
            MODULE.admit_core_equivalence_scope([self.record()])

    def test_one_exact_current_floor_proof_is_admitted(self):
        record = self.record()
        self.assertEqual(MODULE.COMMON.canonical_sha256(record['mutant']),
                         '90e01dbc036b481901e6c43ff7e0b94b6ad4f86098bd91c7ec0dc7de8c442f94')
        self.assertIsNone(MODULE.historical_s157_admit_core_equivalence_scope([record]))

    def test_historical_identity_candidate_source_and_open_shapes_reject(self):
        good = self.record()
        for invalid in (None, {}, (), [good, copy.deepcopy(good)], [None],
                        [{**good, 'unknown': 0}],
                        [{key: value for key, value in good.items() if key != 'proof'}]):
            with self.subTest(invalid=type(invalid).__name__), self.assertRaises(MODULE.Incomplete):
                MODULE.historical_s157_admit_core_equivalence_scope(invalid)
        for key, value in (
                ('id', 'S157-P-E-001'), ('id', 'S13-P-E-001'),
                ('id', 'S152B-C-E-001'), ('id', True),
                ('candidate_sha256', '119355aae391c83afbc36344362bb802b04716fc138f4cfee0db22ea1c27d8c9'),
                ('candidate_sha256', '0' * 64), ('source_sha256', '0' * 64),
                ('requirement', 'CHECK-25'), ('rationale', ''),
                ('rationale', ' '), ('rationale', 'a' * 4097),
                ('rationale', 'a' + ' ' * 4096), ('rationale', True)):
            changed = copy.deepcopy(good); changed[key] = value
            with self.subTest(key=key, value=str(value)[:80]), self.assertRaises(MODULE.Incomplete):
                MODULE.historical_s157_admit_core_equivalence_scope([changed])

    def test_every_mutant_identity_component_and_capacity_fault_reject(self):
        good = self.record()
        for key in ('name', 'package', 'file', 'replacement', 'genre'):
            changed = copy.deepcopy(good); changed['mutant'][key] = 'unreviewed'
            with self.subTest(key=key), self.assertRaises(MODULE.Incomplete):
                MODULE.historical_s157_admit_core_equivalence_scope([changed])
        for key in ('function_name', 'return_type'):
            changed = copy.deepcopy(good); changed['mutant']['function'][key] = 'unreviewed'
            with self.subTest(key=key), self.assertRaises(MODULE.Incomplete):
                MODULE.historical_s157_admit_core_equivalence_scope([changed])
        for scope in ('mutant', 'function'):
            for endpoint in ('start', 'end'):
                for coordinate in ('line', 'column'):
                    changed = copy.deepcopy(good)
                    span = changed['mutant']['span'] if scope == 'mutant' else changed['mutant']['function']['span']
                    span[endpoint][coordinate] += 1
                    with self.subTest(scope=scope, endpoint=endpoint, coordinate=coordinate), \
                            self.assertRaises(MODULE.Incomplete):
                        MODULE.historical_s157_admit_core_equivalence_scope([changed])
        capacity = copy.deepcopy(good)
        capacity['mutant'].update(
            file='crates/time-core/src/principal/chart.rs', replacement='>=',
            name='crates/time-core/src/principal/chart.rs:292:25: replace > with >= in FixturePrincipalState::advance_chart')
        unknown = copy.deepcopy(good); unknown['mutant']['extra'] = 0
        non_json = copy.deepcopy(good); non_json['mutant']['span'] = object()
        for changed in (capacity, unknown, non_json, {**good, 'mutant': []}):
            with self.subTest(mutant=str(changed['mutant'])[:120]), self.assertRaises(MODULE.Incomplete):
                MODULE.historical_s157_admit_core_equivalence_scope([changed])

    def test_each_proof_pin_and_closed_proof_shape_is_required(self):
        good = self.record()
        for key in good['proof']:
            changed = copy.deepcopy(good); changed['proof'][key] = 'unreviewed'
            with self.subTest(key=key), self.assertRaises(MODULE.Incomplete):
                MODULE.historical_s157_admit_core_equivalence_scope([changed])
        for proof in (None, [], {**good['proof'], 'extra': 0},
                      {key: value for key, value in good['proof'].items() if key != 'reviewer'},
                      {**good['proof'], 'sha256': True}):
            with self.subTest(proof=proof), self.assertRaises(MODULE.Incomplete):
                MODULE.historical_s157_admit_core_equivalence_scope([{**good, 'proof': proof}])

    def test_common_admission_checks_actual_current_source_candidate_and_proof(self):
        good = self.record()
        policy = {'maximum_mutants': 512, 'mutation_equivalences': [good]}
        retained = json.loads((ROOT / 'tasks/s1.3-chart-principal-cohort-plan.json').read_text())
        identity = retained['product_inputs_sha256']
        self.assertEqual(MODULE.COMMON.canonical_sha256(identity), good['candidate_sha256'])
        # Only the immutable historical whole-product identity is substituted;
        # the actual principal source and proof bytes are still independently read.
        with patch.object(MODULE.COMMON.PRODUCT, 'admit_product', return_value=({}, identity)):
            admitted = MODULE.COMMON.admit_mutation_equivalences(ROOT, self.historical_scope(), policy)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.COMMON.admit_mutation_equivalences(ROOT, MODULE.SCOPE, policy)
        self.assertEqual(admitted, [good])
        self.assertIsNot(admitted[0], good)
        proof = ROOT / good['proof']['path']
        self.assertEqual(hashlib.sha256(proof.read_bytes()).hexdigest(), good['proof']['sha256'])
        self.assertIn('\n## S157-P-E-002\n', proof.read_text())

    def test_inventory_role_keeps_its_literal_empty_equivalence_list(self):
        helper = InventoryAdmissionTests()
        helper.setUp()
        self.addCleanup(helper.doCleanups)
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); policy, path = helper.fixture(root)
            self.assertEqual(MODULE.INVENTORY_FIXED['mutation_equivalences'], [])
            self.assertEqual(helper.read(root, path, policy), policy)
            self.assertEqual(MODULE.COMMON.admit_mutation_equivalences(root, MODULE.SCOPE, policy), [])
            args = SimpleNamespace(lane='mutation', tool_admission_profile='stable-core')
            scope = MODULE.COMMON.Scope('time-core', tuple(MODULE.INVENTORY_SUBJECTS.items()), MODULE.INVENTORY_ADMISSION)
            self.assertEqual(MODULE.COMMON.admission_profile(args, scope, policy), 'stable-core')
            policy['mutation_equivalences'] = [self.record()]
            path.write_text(json.dumps(policy))
            with self.assertRaises(MODULE.Incomplete):
                helper.read(root, path, policy)
            with self.assertRaises(MODULE.Incomplete):
                MODULE.COMMON.admission_profile(args, scope, policy)

    def synthetic_outcomes(self, caught=3, missed=1):
        """Reuse existing inert phase fixtures, retaining the exact floor descriptor."""
        helper = CoreAnalysisTests(); good = self.record(); filename = good['mutant']['file']
        raw, expected = helper.mutation_report(filename, caught, missed)
        last = raw['outcomes'][-1]['scenario']['Mutant']
        del expected[last['name']]
        raw['outcomes'][-1]['scenario']['Mutant'] = copy.deepcopy(good['mutant'])
        expected[good['mutant']['name']] = copy.deepcopy(good['mutant'])
        return raw, expected

    def test_common_native_gate_keeps_raw_miss_exit_and_full_inventory(self):
        good = self.record(); raw, expected = self.synthetic_outcomes()
        before = copy.deepcopy(raw)
        result = MODULE.COMMON.mutation_counts(
            self.historical_scope(), raw, expected, good['mutant']['file'], native_exit_code=2, equivalences=[good])
        self.assertEqual(raw, before)
        self.assertEqual((result['generated'], result['killed'], result['survived'], result['unviable']),
                         (4, 3, 1, 0))
        self.assertEqual((result['native_exit_code'], result['excluded'], result['raw_denominator'],
                          result['adjusted_denominator']), (2, 1, 4, 3))
        self.assertEqual(result['raw_viable_killed_percent'], 75)
        self.assertEqual(result['adjusted_viable_killed_percent'], 100)
        self.assertTrue(result['gate_passed'])

    def test_common_native_gate_requires_actual_miss_and_rejects_another_survivor(self):
        good = self.record(); helper = CoreAnalysisTests()
        for change in ('caught', 'unviable', 'missing', 'wrong_exit'):
            raw, expected = self.synthetic_outcomes(); native_exit = 2
            if change in ('caught', 'unviable'):
                summary = 'CaughtMutant' if change == 'caught' else 'Unviable'
                raw['outcomes'][-1]['summary'] = summary
                raw['outcomes'][-1]['phase_results'] = helper.phase_results(summary)
                raw['missed'] = 0; raw['caught' if change == 'caught' else 'unviable'] += 1
                native_exit = 0
            elif change == 'missing':
                raw['outcomes'].pop()
            else:
                native_exit = 0
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.COMMON.mutation_counts(self.historical_scope(), raw, expected, good['mutant']['file'],
                                              native_exit_code=native_exit, equivalences=[good])
        raw, expected = self.synthetic_outcomes(caught=20, missed=2)
        result = MODULE.COMMON.mutation_counts(
            self.historical_scope(), raw, expected, good['mutant']['file'], native_exit_code=2,
            equivalences=[good], defer_gate=True)
        self.assertGreaterEqual(result['adjusted_viable_killed_percent'], 95)
        self.assertEqual(len(result['unresolved_survivors']), 1)
        self.assertFalse(result['gate_passed'])


class S158ReceivingEquivalenceControls(HistoricalS14cTestCase):
    """Exact reviewed receiving descriptors, independently copied from native MISS.

    The original receiving61 campaign receipt SHA675cbb9602efb9578007d4061e75
    a798db5c9ecff2a64500745977e64b56acb7 supplies these canonical descriptors.
    Their new candidate binding comes from the consolidated raw-guard test
    candidate; no old-candidate native execution is promoted by these controls.
    """

    CANDIDATE = '5f0a23c12bf5adeae4bacd47592c0b7ec24714b5e862d5d76115483f6f15f0a7'
    SOURCE = '4e0687bd5b3dd1e1ada09af7e17a561f5c1ee322761aae76f8c0c2964ff1c553'
    PROOF = '055046256bf3fa5aca61fb36042f819d2d53407512e0da1c89a801ccd040f085'
    FILE = 'crates/time-core/src/combination/chart/receiving.rs'
    DESCRIPTORS = (
        (87, 23, "ChartTrackingSupporters<'entries, 'chart>::from_decision",
         '-> Result<Self, FixtureTrackingCombinationError>', (61, 5, 111, 6),
         '1a93bdc9751cf7e236dd7c50f70ee6a3905447190c9ad80130cda7c5a11ab07b'),
        (216, 19, 'admit_chart_records', '-> Result<(), FixtureCombinationError>', (199, 1, 279, 2),
         'a071d26acc9e3fd6e6b5e4cf9fcff3b173b4ca4e73a16316634e9db6768c47f7'),
        (216, 45, 'admit_chart_records', '-> Result<(), FixtureCombinationError>', (199, 1, 279, 2),
         '4206a6a897e0ae7b792c2a7c5ac92426e029b7a3e0f458549f0e8f4ca7ffbfad'),
        (228, 9, 'admit_chart_records', '-> Result<(), FixtureCombinationError>', (199, 1, 279, 2),
         'cb45cd39181ec5c28916ad96f782698eaef44f1a579c04a354654028164d074d'),
        (310, 25, 'admit_chart_source',
         '-> Result<Option<AdmittedChartSupporters<\'input, \'entries, \'chart>>, FixtureTrackingCombinationError, >',
         (281, 1, 367, 2),
         '5338d821f9c75801213cd60e68f9f741baac71b2bd30cb360ff263e5f068ca9f'),
    )

    @classmethod
    def records(cls):
        records = []
        for index, (line, column, name, result, span, _digest) in enumerate(cls.DESCRIPTORS, 1):
            identifier = f'S158-R-E-{index:03}'
            records.append({
                'id': identifier, 'candidate_sha256': cls.CANDIDATE, 'source_sha256': cls.SOURCE,
                'mutant': {
                    'name': f'{cls.FILE}:{line}:{column}: replace || with && in {name}',
                    'package': 'time-core', 'file': cls.FILE,
                    'function': {'function_name': name, 'return_type': result,
                                 'span': {'start': {'line': span[0], 'column': span[1]},
                                          'end': {'line': span[2], 'column': span[3]}}},
                    'span': {'start': {'line': line, 'column': column},
                             'end': {'line': line, 'column': column + 2}},
                    'replacement': '&&', 'genre': 'BinaryOperator',
                },
                'requirement': 'CHECK-7/8/14/25',
                'rationale': 'The reviewed opaque decision invariants establish typed-domain equivalence; exact viable native MISS remains required.',
                'proof': {'path': 'tasks/s1.3-chart-combination-equivalence.md',
                          'sha256': cls.PROOF, 'section': identifier,
                          'reviewer': 'root; s157_gate_review',
                          'verdict': 'independently-justified-equivalent'},
            })
        return records

    @classmethod
    def setUpClass(cls):
        """The frozen historical fixture supplies the other reviewed owners."""
        cls.OTHER_RECORDS = [copy.deepcopy(row) for row in S158_POLICY_FIXTURE['mutation_equivalences']
                             if not row['id'].startswith('S158-R-E-')]
        if len(cls.OTHER_RECORDS) != 18:
            raise AssertionError('Expected all 18 independently reviewed companion descriptors.')

    def admit_records(self, receiving):
        """Exercise the complete global guard while varying the five receiving rows."""
        if type(receiving) is list and receiving:
            return MODULE.admit_core_equivalence_scope(
                copy.deepcopy(receiving) + copy.deepcopy(self.OTHER_RECORDS))
        return MODULE.admit_core_equivalence_scope(receiving)

    def test_whole_five_native_descriptors_and_empty_scope_are_admitted(self):
        records = self.records()
        for record, descriptor in zip(records, self.DESCRIPTORS, strict=True):
            self.assertEqual(MODULE.COMMON.canonical_sha256(record['mutant']), descriptor[-1])
        self.assertIsNone(self.admit_records([]))
        self.assertIsNone(self.admit_records(records))
        self.assertIsNone(self.admit_records(list(reversed(records))))

    def test_missing_duplicate_historical_and_non_equivalent_sets_reject(self):
        records = self.records()
        for bad in (None, {}, (), [None] * 5, records[:1], records[:-1], records + records[:1],
                    records[:-1] + records[:1], [S157CurrentFloorEquivalenceControls.record()]):
            with self.subTest(bad=bad), self.assertRaises(MODULE.Incomplete):
                self.admit_records(bad)
        changed = copy.deepcopy(records)
        # Actual line227 remains a testable pointer defect and has no E credit.
        mutant = changed[3]['mutant']
        mutant['name'] = mutant['name'].replace(':228:9:', ':227:9:')
        mutant['span']['start']['line'] = mutant['span']['end']['line'] = 227
        with self.assertRaises(MODULE.Incomplete):
            self.admit_records(changed)

    def test_each_record_requires_current_identity_source_and_bounded_rationale(self):
        records = self.records()
        changes = (
            ('id', 'S158-R-E-006'), ('id', True),
            ('candidate_sha256', '9ad7d1f07c88ec0b86cc9defde6aa7cbf6d356eef27e4372281796dddafd820c'),
            ('candidate_sha256', '0' * 64), ('source_sha256', '0' * 64),
            ('requirement', 'CHECK-25'), ('rationale', ''), ('rationale', ' '),
            ('rationale', 'x' * 4097), ('rationale', True),
        )
        for index in range(5):
            for key, value in changes:
                changed = copy.deepcopy(records); changed[index][key] = value
                with self.subTest(index=index, key=key, value=str(value)[:80]), self.assertRaises(MODULE.Incomplete):
                    self.admit_records(changed)

    def test_every_complete_mutant_component_and_opaque_extra_rejects(self):
        records = self.records()
        for index in range(5):
            for key in ('name', 'package', 'file', 'replacement', 'genre'):
                changed = copy.deepcopy(records); changed[index]['mutant'][key] = 'unreviewed'
                with self.subTest(index=index, key=key), self.assertRaises(MODULE.Incomplete):
                    self.admit_records(changed)
            for key in ('function_name', 'return_type'):
                changed = copy.deepcopy(records); changed[index]['mutant']['function'][key] = 'unreviewed'
                with self.subTest(index=index, key=key), self.assertRaises(MODULE.Incomplete):
                    self.admit_records(changed)
            for scope in ('mutant', 'function'):
                for endpoint in ('start', 'end'):
                    for coordinate in ('line', 'column'):
                        changed = copy.deepcopy(records)
                        owner = changed[index]['mutant']
                        if scope == 'function': owner = owner['function']
                        owner['span'][endpoint][coordinate] += 1
                        with self.subTest(index=index, scope=scope, endpoint=endpoint, coordinate=coordinate), \
                                self.assertRaises(MODULE.Incomplete):
                            self.admit_records(changed)
        for owner in ('record', 'mutant', 'function', 'span', 'proof'):
            changed = copy.deepcopy(records); value = changed[0]
            if owner in ('mutant', 'function', 'span'): value = value['mutant']
            if owner == 'function': value = value['function']
            if owner == 'span': value = value['span']
            if owner == 'proof': value = value['proof']
            value['opaque_extra'] = 0
            with self.subTest(owner=owner), self.assertRaises(MODULE.Incomplete):
                self.admit_records(changed)
        changed = copy.deepcopy(records); changed[0]['mutant']['span'] = object()
        with self.assertRaises(MODULE.Incomplete):
            self.admit_records(changed)

    def test_every_proof_pin_and_exact_record_shape_is_required(self):
        records = self.records()
        for index in range(5):
            for key in records[index]['proof']:
                changed = copy.deepcopy(records); changed[index]['proof'][key] = 'unreviewed'
                with self.subTest(index=index, key=key), self.assertRaises(MODULE.Incomplete):
                    self.admit_records(changed)
            for key in records[index]:
                changed = copy.deepcopy(records); del changed[index][key]
                with self.subTest(index=index, missing=key), self.assertRaises(MODULE.Incomplete):
                    self.admit_records(changed)
        for proof in (None, [], {**records[0]['proof'], 'sha256': True}):
            changed = copy.deepcopy(records); changed[0]['proof'] = proof
            with self.subTest(proof=proof), self.assertRaises(MODULE.Incomplete):
                self.admit_records(changed)

    def test_common_admission_reads_real_current_candidate_source_and_proof(self):
        records = self.records()
        policy = {'maximum_mutants': 512, 'mutation_equivalences': records}
        self.admit_records(records)
        scope = MODULE.COMMON.Scope('time-core', tuple(S158_SUBJECTS.items()), MODULE.POLICY)
        with self.assertRaises(MODULE.COMMON.Incomplete):
            MODULE.COMMON.admit_mutation_equivalences(ROOT, scope, policy)
        with patch.object(MODULE.COMMON.PRODUCT, 'admit_product',
                          return_value=({}, S158_POLICY_FIXTURE['inputs_sha256'])):
            admitted = MODULE.COMMON.admit_mutation_equivalences(ROOT, scope, policy)
        self.assertEqual(admitted, records)
        self.assertIsNot(admitted, records)
        self.assertIsNot(admitted[0]['mutant'], records[0]['mutant'])
        proof = ROOT / 'tasks/s1.3-chart-combination-equivalence.md'
        self.assertEqual(hashlib.sha256(proof.read_bytes()).hexdigest(), self.PROOF)
        self.assertEqual(MODULE.COMMON.canonical_sha256(S158_POLICY_FIXTURE['inputs_sha256']), self.CANDIDATE)
        self.assertNotEqual(MODULE.COMMON.canonical_sha256(MODULE.PRODUCT.admit_product(ROOT)[1]), self.CANDIDATE)
        self.assertEqual(hashlib.sha256((ROOT / self.FILE).read_bytes()).hexdigest(), self.SOURCE)
        for record in records:
            self.assertIn('\n## ' + record['id'] + '\n', proof.read_text())
        for key in ('candidate_sha256', 'source_sha256'):
            changed = copy.deepcopy(records); changed[0][key] = '0' * 64
            with patch.object(MODULE.COMMON.PRODUCT, 'admit_product',
                              return_value=({}, S158_POLICY_FIXTURE['inputs_sha256'])), \
                    self.subTest(pin=key), self.assertRaises(MODULE.Incomplete):
                MODULE.COMMON.admit_mutation_equivalences(ROOT, MODULE.COMMON.Scope('time-core', tuple(S158_SUBJECTS.items()), MODULE.POLICY),
                    {**policy, 'mutation_equivalences': changed})
        for key, value in (('sha256', '0' * 64), ('section', 'S158-R-E-999'),
                           ('path', 'tasks/../s1.3-chart-combination-equivalence.md')):
            changed = copy.deepcopy(records); changed[0]['proof'][key] = value
            with patch.object(MODULE.COMMON.PRODUCT, 'admit_product',
                              return_value=({}, S158_POLICY_FIXTURE['inputs_sha256'])), \
                    self.subTest(proof=key), self.assertRaises(MODULE.Incomplete):
                MODULE.COMMON.admit_mutation_equivalences(ROOT, MODULE.COMMON.Scope('time-core', tuple(S158_SUBJECTS.items()), MODULE.POLICY),
                    {**policy, 'mutation_equivalences': changed})

    def synthetic_current_misses(self):
        """Inert phase fixtures test credit accounting; no native execution is claimed."""
        helper = CoreAnalysisTests()
        raw, expected = helper.mutation_report(self.FILE, 1, 5)
        misses = [row for row in raw['outcomes'] if row['summary'] == 'MissedMutant']
        for row, record in zip(misses, self.records(), strict=True):
            del expected[row['scenario']['Mutant']['name']]
            row['scenario']['Mutant'] = copy.deepcopy(record['mutant'])
            expected[record['mutant']['name']] = copy.deepcopy(record['mutant'])
        return raw, expected

    def test_five_exact_viable_misses_keep_raw_counts_exit_and_inventory(self):
        raw, expected = self.synthetic_current_misses()
        before = copy.deepcopy(raw)
        result = MODULE.COMMON.mutation_counts(MODULE.COMMON.Scope('time-core', tuple(S158_SUBJECTS.items()), MODULE.POLICY), raw, expected, self.FILE,
                                               native_exit_code=2, equivalences=self.records())
        self.assertEqual(raw, before)
        self.assertEqual((result['generated'], result['killed'], result['survived'], result['excluded']),
                         (6, 1, 5, 5))
        self.assertEqual((result['native_exit_code'], result['raw_denominator'], result['adjusted_denominator']),
                         (2, 6, 1))
        self.assertEqual(result['adjusted_viable_killed_percent'], 100)
        self.assertTrue(result['gate_passed'])
        helper = CoreAnalysisTests()
        for change in ('caught', 'unviable', 'missing', 'wrong_exit'):
            raw, expected = self.synthetic_current_misses(); native_exit = 2
            if change in ('caught', 'unviable'):
                summary = 'CaughtMutant' if change == 'caught' else 'Unviable'
                raw['outcomes'][-1]['summary'] = summary
                raw['outcomes'][-1]['phase_results'] = helper.phase_results(summary)
                raw['missed'] -= 1; raw['caught' if change == 'caught' else 'unviable'] += 1
            elif change == 'missing':
                raw['outcomes'].pop()
            else:
                native_exit = 0
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.COMMON.mutation_counts(MODULE.COMMON.Scope('time-core', tuple(S158_SUBJECTS.items()), MODULE.POLICY), raw, expected, self.FILE,
                                               native_exit_code=native_exit, equivalences=self.records())

    def test_unreviewed_line227_survivor_never_receives_one_of_the_five_credits(self):
        raw, expected = self.synthetic_current_misses()
        extra = copy.deepcopy(raw['outcomes'][-1])
        mutant = copy.deepcopy(self.records()[3]['mutant'])
        mutant['name'] = mutant['name'].replace(':228:9:', ':227:9:')
        mutant['span']['start']['line'] = mutant['span']['end']['line'] = 227
        extra['scenario']['Mutant'] = mutant
        raw['outcomes'].append(extra); raw['total_mutants'] += 1; raw['missed'] += 1
        expected[mutant['name']] = copy.deepcopy(mutant)
        result = MODULE.COMMON.mutation_counts(MODULE.COMMON.Scope('time-core', tuple(S158_SUBJECTS.items()), MODULE.POLICY), raw, expected, self.FILE,
            native_exit_code=2, equivalences=self.records(), defer_gate=True)
        self.assertEqual(result['excluded'], 5)
        self.assertEqual(result['unresolved_survivors'], [mutant['name']])
        self.assertFalse(result['gate_passed'])


class S158ConsolidatedEquivalenceControls(HistoricalS14cTestCase):
    """Closed descriptors copied from actual889 native misses, not generated rules.

    Synthetic phases below test receiver accounting only. New native misses,
    source custody and independent domain proofs remain separate obligations.
    """

    OTHER_DESCRIPTOR_SHA256 = {'S158-E-001': '89cdb73c0408322b69c7882cbbd426b774de5a99af6e658e7e8a4676b1f7a639',
 'S158-E-002': 'ec0e00450ec8e7627fa99b2eb46cc00e78e3c3a24640f68bc8a1384598250f0e',
 'S158-E-003': '2a4b9ccd07c2511a39bab3c1d39758ea3acd1219b790c6e727fcafc3c6e947ff',
 'S158-E-004': '891afc8e51a733e78b04100ce2b48390353cc038a07874729aae39aadf0c8a02',
 'S158-E-005': 'adbe07bb70e61b103b5468898e8e7362e30f68d17c4675379b721f3e5158787d',
 'S158-E-006': '6ea68d3cc45afdde63a2a697727af89e45ffc8e9cf66b41687ab4a469c1bb64e',
 'S158-E-007': '4d2fb7768ba894d2fba098f8bebca72f81193bbf4ee9620396f4e9bec085debb',
 'S158-E-008': '1e6705b186a0d7d5be4d1a417861d4599126c89688cbd651d8831f193a77d027',
 'S158-E-009': 'ff0dae6d91312253b2b4cac7b56736c2328f6eebc2a8f6ce6abc630901c7af0a',
 'S158-E-010': 'a855bdff37308d15b237396e0bb974010d4bb22c713abb6eb9a0c4de83966253',
 'S158-E-011': 'c687733f1ab66c5ba4e0b17d4f8833f24c1d6449a555534f72d43d6967480cfb',
 'S158-E-012': '505a650a5393db78e85fcb3976aaaa74ee44353335bc8e8af5b425f16b3fc8d5',
 'S158-E-013': '940698738fe9473cbd823e81aaf0b10a38f9fe84f897cc90d24bd763bb34a8a2',
 'S158-E-014': 'de44909d87fffa04a1041cf6d5d30369cbb6158976828a6f479cc10d078f5a2e',
 'S158-E-015': 'de584b1cfaae4ec86bd65056eee273d9f440192ca8c7848b1e12433ccf1ce0bb',
 'S158-E-016': '1f9cdd61ccfafac2090266b5d5acce56b45795925a187f7f633ca7966afcf79f',
 'S158-E-017': '0c185aaecdd9481f94d4d64f8a8602ce6a2fc0e3d4bcab44e40ea5c5bc572997',
 'S158-E-018': '265b874a6297f038e4a641ffef440f64af1109410b06652361b9765fb5138c00'}

    def records(self):
        """Preserve historical metadata independently constrained by native digests."""
        rows = copy.deepcopy(S158_POLICY_FIXTURE['mutation_equivalences'])
        self.assertEqual(len(rows), 23)
        expected = dict(self.OTHER_DESCRIPTOR_SHA256)
        expected.update({f'S158-R-E-{i:03}': row[-1]
                         for i, row in enumerate(S158ReceivingEquivalenceControls.DESCRIPTORS, 1)})
        self.assertEqual({row['id']: MODULE.COMMON.canonical_sha256(row['mutant']) for row in rows}, expected)
        return rows

    def test_whole_23_and_empty_scope_accept_but_historical_partial_sets_reject(self):
        rows = self.records()
        MODULE.admit_core_equivalence_scope([])
        MODULE.admit_core_equivalence_scope(rows)
        MODULE.admit_core_equivalence_scope(list(reversed(rows)))
        for bad in (None, {}, (), [None] * 23, rows[:1], rows[:-1], rows + rows[:1],
                    rows[:-1] + rows[:1], S158ReceivingEquivalenceControls.records(),
                    [S157CurrentFloorEquivalenceControls.record()]):
            with self.subTest(shape=str(bad)[:100]), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_core_equivalence_scope(bad)

    def test_every_owner_requires_its_exact_candidate_source_descriptor_and_proof(self):
        rows = self.records()
        for index in range(23):
            for key, value in (('id', 'S158-E-999'), ('candidate_sha256', '0' * 64),
                               ('source_sha256', '0' * 64), ('requirement', 'CHECK-25'),
                               ('rationale', ''), ('rationale', ' '), ('rationale', True),
                               ('rationale', 'x' * 4097)):
                bad = copy.deepcopy(rows); bad[index][key] = value
                with self.subTest(index=index, field=key), self.assertRaises(MODULE.Incomplete):
                    MODULE.admit_core_equivalence_scope(bad)
            for key in rows[index]:
                bad = copy.deepcopy(rows); del bad[index][key]
                with self.subTest(index=index, missing=key), self.assertRaises(MODULE.Incomplete):
                    MODULE.admit_core_equivalence_scope(bad)
            for key in ('name', 'file', 'package', 'genre', 'replacement'):
                bad = copy.deepcopy(rows); bad[index]['mutant'][key] = 'unreviewed'
                with self.subTest(index=index, mutant=key), self.assertRaises(MODULE.Incomplete):
                    MODULE.admit_core_equivalence_scope(bad)
            for key in rows[index]['proof']:
                bad = copy.deepcopy(rows); bad[index]['proof'][key] = 'unreviewed'
                with self.subTest(index=index, proof=key), self.assertRaises(MODULE.Incomplete):
                    MODULE.admit_core_equivalence_scope(bad)

    def test_real_common_admission_reconciles_all_current_source_and_proof_bytes(self):
        rows = self.records()
        scope = MODULE.COMMON.Scope('time-core', tuple(S158_SUBJECTS.items()), MODULE.POLICY)
        policy = {'maximum_mutants': 512, 'mutation_equivalences': rows}
        # The real current154-input candidate must reject historical152-input E.
        with self.assertRaises(MODULE.COMMON.Incomplete):
            MODULE.COMMON.admit_mutation_equivalences(ROOT, scope, policy)
        # Isolate that adjacent historical product admission; actual subject/proof
        # bytes and all strict equivalence relationship checks still run here.
        with patch.object(MODULE.COMMON.PRODUCT, 'admit_product',
                          return_value=({}, S158_POLICY_FIXTURE['inputs_sha256'])):
            admitted = MODULE.COMMON.admit_mutation_equivalences(ROOT, scope, policy)
        self.assertEqual(admitted, rows)
        self.assertIsNot(admitted, rows)
        for row in rows:
            self.assertEqual(hashlib.sha256((ROOT / row['mutant']['file']).read_bytes()).hexdigest(),
                             row['source_sha256'])
            self.assertIn('\n## ' + row['id'] + '\n',
                          (ROOT / row['proof']['path']).read_text())

    def test_per_file_exact_misses_retain_native_exit_and_raw_outcomes(self):
        rows = self.records()
        helper = CoreAnalysisTests()
        for filename in sorted({row['mutant']['file'] for row in rows}):
            selected = [row for row in rows if row['mutant']['file'] == filename]
            raw, expected = helper.mutation_report(filename, 1, len(selected))
            misses = [row for row in raw['outcomes'] if row['summary'] == 'MissedMutant']
            for outcome, record in zip(misses, selected, strict=True):
                del expected[outcome['scenario']['Mutant']['name']]
                outcome['scenario']['Mutant'] = copy.deepcopy(record['mutant'])
                expected[record['mutant']['name']] = copy.deepcopy(record['mutant'])
            before = copy.deepcopy(raw)
            result = MODULE.COMMON.mutation_counts(MODULE.COMMON.Scope('time-core', tuple(S158_SUBJECTS.items()), MODULE.POLICY), raw, expected, filename,
                native_exit_code=2, equivalences=selected)
            self.assertEqual(raw, before)
            self.assertEqual((result['killed'], result['survived'], result['excluded']),
                             (1, len(selected), len(selected)))
            self.assertEqual((result['native_exit_code'], result['adjusted_denominator']), (2, 1))
            self.assertTrue(result['gate_passed'])
            foreign = next(row for row in rows if row['mutant']['file'] != filename)
            with self.subTest(filename=filename), self.assertRaises(MODULE.Incomplete):
                MODULE.COMMON.mutation_counts(MODULE.COMMON.Scope('time-core', tuple(S158_SUBJECTS.items()), MODULE.POLICY), raw, expected, filename,
                    native_exit_code=2, equivalences=selected + [foreign])


class S158ActualAssertionTransportControls(HistoricalS14cTestCase):
    """Replay actual formatter bytes; these Python controls execute no product.

    Captured by the configured Session's std-only, exact2KiB formatter fixture:
    s158-assertion-transport-preflight-v2/fault-output.txt, actual exit101.
    Origin receiving_tests.rs952–960 SHA59459fc4bbc1d55adda21a9f53362deef
    d3119a8fa1d87e079525ef6a3604792; assertion token SHA888b5db0b053c432
    14b1eae600a63fc5eed4c1ef83b7e68e5b85fa8bbd32f213. No generated-mutant
    cause, product test result or compiler failure is established by these bytes.
    """

    CAPTURE = (
        "\nthread 'main' (2) panicked at /work/assertion-fixture/probe.rs:13:5:\n"
        "assertion failed: matches!(partial.with_record(second, principal_phase, 1),\n"
        "    Err(ChartReductionError::Combination(FixtureCombinationError::ZeroDenominator\n"
        "    {\n"
        "        index: 1, field:\n"
        "        FixtureCombinationDenominatorField::RateStandardDeviation,\n"
        "    })))\n"
        "note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n"
    )
    CAPTURE_SHA256 = '82755372dc48e19caae89d9e0d484fc3c43b829313e6c1749ed6091859a2fbf5'

    def test_actual_formatter_lf_crlf_is_retained_verbatim_in_test_context(self):
        self.assertEqual(len(self.CAPTURE.encode()), 413)
        self.assertEqual(hashlib.sha256(self.CAPTURE.encode()).hexdigest(), self.CAPTURE_SHA256)
        cargo = '{"reason":"build-finished","success":true}\n'
        for newline in ('\n', '\r\n'):
            raw = self.CAPTURE.replace('\n', newline)
            with self.subTest(newline=repr(newline)):
                self.assertEqual(MODULE.cargo_json_records(raw, test_phase=True), ([], raw))
                self.assertEqual(MODULE.cargo_json_records(cargo.replace('\n', newline) + raw, test_phase=True),
                                 ([{'reason': 'build-finished', 'success': True}], raw))

    def test_actual_formatter_never_relaxes_build_context(self):
        for newline in ('\n', '\r\n'):
            raw = self.CAPTURE.replace('\n', newline)
            for options in ({}, {'test_phase': False}):
                with self.subTest(newline=repr(newline), options=options), self.assertRaises(MODULE.Incomplete):
                    MODULE.cargo_json_records(raw, **options)

    def test_partial_and_near_miss_formatter_blocks_remain_closed(self):
        lines = self.CAPTURE.splitlines(keepends=True)
        for raw in (
                '    {\n',
                ''.join(lines[3:8]),
                ''.join(lines[:7]),
                self.CAPTURE.replace('assertion failed: matches!', 'assertion failed: alien!'),
                self.CAPTURE.replace('partial.with_record', 'other.with_record'),
                self.CAPTURE.replace('index: 1, field:', 'index: 0, field:'),
                self.CAPTURE.replace('RateStandardDeviation', 'PhaseStandardDeviation'),
                self.CAPTURE.replace('    {\n', '   {\n'),
                self.CAPTURE.replace('    })))\n', '    }))\n')):
            with self.subTest(raw=raw), self.assertRaises(MODULE.Incomplete):
                MODULE.cargo_json_records(raw, test_phase=True)

    def test_complete_formatter_does_not_excuse_foreign_or_partial_json(self):
        for raw in (
                '{ foreign assertion }\n',
                '{"reason":"build-finished","success":true\n',
                '{"reason":"build-finished","success":true,"success":false}\n',
                '{"reason":"build-finished","success":NaN}\n',
                '{"reason":"build-finished","success":Infinity}\n',
                '{"reason":"build-finished","success":1e999}\n',
                '{"reason":"unknown"}\n'):
            with self.subTest(raw=raw), self.assertRaises(MODULE.Incomplete):
                MODULE.cargo_json_records(self.CAPTURE + raw, test_phase=True)

    def test_actual_formatter_cannot_establish_or_accompany_unviable_build(self):
        helper = CoreAnalysisTests()
        unviable = {'summary': 'Unviable', 'phase_results': helper.phase_results('Unviable')}
        healthy = helper.semantic_mutation_log()
        MODULE.unviable_compile_failure(unviable, healthy)
        build = healthy.replace(helper.mutation_phase_header('Build'),
                                helper.mutation_phase_header('Build') + self.CAPTURE, 1)
        for output in (self.CAPTURE, build):
            with self.subTest(output=output), self.assertRaises(MODULE.Incomplete):
                MODULE.unviable_compile_failure(unviable, output)


class ChartCombinationIntegrationTests(HistoricalS14cTestCase):
    """Independent S158 source outcomes and pre-effect reservation controls."""

    NAMES = (
        'combination::chart::roundoff::tests::companion_construction_preserves_supported_bits_and_rejects_invalid_fields',
        'combination::chart::roundoff::tests::positive_margin_keeps_ties_and_exact_subnormal_differences',
        'combination::chart::roundoff::tests::positive_division_consumes_the_exact_smallest_subnormal_margin',
        'combination::chart::roundoff::tests::companion_operations_contain_independent_interval_endpoints',
        'combination::chart::roundoff::tests::exact_input_ties_and_cancellation_retain_separate_rounding',
        'combination::chart::roundoff::tests::sixteen_stored_tenths_include_new_rounded_reduction_error',
        'combination::chart::roundoff::tests::reciprocal_rate_weight_retains_the_sd_square_error',
        'combination::chart::roundoff::tests::companion_failures_preserve_actual_point_and_bound_owners',
        'combination::chart::roundoff::tests::companion_diagnostics_are_bounded_and_keep_real_source_chains',
        'combination::admission::chart_receiving::tests::chart_tracking_receiver_keeps_actual_slots_and_complete_witness',
        'combination::admission::chart_receiving::tests::chart_tracking_receiver_rejects_wrong_relationships',
        'combination::admission::chart_receiving::tests::chart_tracking_receiver_requires_excluded_and_retained_tracking',
        'combination::admission::chart_receiving::tests::chart_tracking_receiver_absence_requires_empty_metadata',
        'combination::admission::chart_receiving::tests::chart_tracking_reduction_contains_exact_stored_input_means',
        'combination::admission::chart_receiving::tests::chart_tracking_reduction_failure_preserves_prior_sums',
        'combination::admission::chart_receiving::tests::chart_tracking_singleton_copies_zero_denominator_attachment_bits',
        'chart_combination_retains_real_provenance_and_observation_credit',
        'chart_combination_failure_rolls_back_before_same_event_retry',
        'unequal_original_weights_contain_exact_stored_input_means',
        'sixteen_contributors_bound_new_reduction_roundoff',
        'original_population_controls_singleton_and_filtered_one_retry',
        'bound_only_failure_rolls_back_and_allows_same_observation_retry',
        'combination::lifecycle::chart::guard_tests::chart_raw_pair_guards_preserve_complete_relationships',
        'combination::lifecycle::chart::guard_tests::chart_raw_count_seventeen_cannot_reuse_sixteen_slots',
    )
    NAMES = (*NAMES,
        'reference_input_readiness::genuine_ready_input_is_readonly_and_replay_has_no_new_credit',
        'reference_input_readiness::unavailable_and_waiting_outcomes_reject_without_reviving_readiness',
        'reference_input_readiness::foreign_chart_instances_reject_beside_the_bound_original',
        'reference_input_readiness::feedback_pairs_and_repeats_never_restore_an_invalidated_input')
    PASSED = ''.join('test ' + name + ' ... ok\n' for name in NAMES)
    DOCS = ('test crates/time-core/src/combination/lifecycle/chart.rs - '
            'combination::lifecycle::chart::FixtureChartCombinedEstimate '
            '(line 163) - compile fail ... ok\n')

    def setUp(self):
        fund_synthetic_source_recipes(self)

    def test_complete_twenty_eight_outcomes_are_source_observations_not_operation_counts(self):
        self.assertEqual(MODULE.SUBJECTS, S14C_SUBJECTS)
        self.assertEqual(tuple(MODULE.CHART_COMBINATION.CURRENT_ORDINARY_TESTS), self.NAMES)
        self.assertEqual(len(self.NAMES), 28)
        result = MODULE.chart_combination_test_observation(self.PASSED, require_complete=True)
        self.assertFalse(result['chart_combination_source_only_pending'])
        self.assertEqual(result['chart_combination_test_statuses'], dict.fromkeys(self.NAMES, 'ok'))
        self.assertEqual(result['chart_combination_recorded_source_work_upper'],
                         {'charged_fit_work_upper': 13020, 'point_admissions_upper': 449610})
        self.assertNotIn('chart_combination_observed_operation_counts', result)
        partial = MODULE.chart_combination_test_observation(self.PASSED.splitlines(keepends=True)[0])
        self.assertTrue(partial['chart_combination_source_only_pending'])
        self.assertIsNone(partial['chart_combination_recorded_source_work_upper'])
        self.assertEqual(len(partial['chart_combination_source_test_observation']['missing']), 27)

    def test_missing_failed_ignored_duplicate_and_unknown_outcomes_cannot_complete(self):
        for bad in (self.PASSED.split('\n', 1)[1],
                    self.PASSED.replace(' ... ok', ' ... FAILED', 1),
                    self.PASSED.replace(' ... ok', ' ... ignored', 1),
                    self.PASSED + self.PASSED.splitlines(keepends=True)[0],
                    self.PASSED.replace(self.NAMES[0], 'combination::chart::roundoff::tests::unknown'),
                    self.PASSED.replace(' ... ok', ' ... success', 1)):
            with self.subTest(bad=bad[:90]), self.assertRaises(MODULE.Incomplete):
                MODULE.chart_combination_test_observation(bad, require_complete=True)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.profile_counts(self.PASSED.splitlines(keepends=True)[0], require_fit=False,
                                  require_chart_combination=True)

    def test_actual_literal_doctest_line_is_required_only_for_original_baseline(self):
        result = MODULE.chart_combination_test_observation(
            self.PASSED + self.DOCS, require_complete=True, require_doctests=True)
        self.assertEqual(result['chart_combination_literal_doctests'],
                         [line.removeprefix('test ').removesuffix(' ... ok')
                          for line in self.DOCS.splitlines()])
        for bad in (self.PASSED, self.PASSED + self.DOCS.replace('(line 163)', '(line 164)'),
                    self.PASSED + self.DOCS + self.DOCS,
                    self.PASSED + self.DOCS.replace(' ... ok', ' ... FAILED')):
            with self.subTest(bad=bad[-100:]), self.assertRaises(MODULE.Incomplete):
                MODULE.chart_combination_test_observation(bad, require_complete=True, require_doctests=True)
        self.assertFalse(MODULE.chart_combination_test_observation(
            self.PASSED + self.DOCS.replace('(line 163)', '(line 164)'),
            require_complete=True)['chart_combination_source_only_pending'])
        with self.assertRaises(MODULE.Incomplete):
            MODULE.chart_combination_test_observation(self.PASSED, require_doctests=1)

    def test_each_package_requires_the_complete_486_field_new_reservation(self):
        healthy = {**MODULE.package_charged_upper(), **MODULE.selection_reservation(package=True),
                   'observation_pending': True}
        self.assertEqual(len(healthy['chart_combination_reserved_budget']), 486)
        report = {}
        MODULE.record_counts(report, 'synthetic-s158-package', healthy)
        self.assertEqual(report['core_chart_combination_reserved_totals'], healthy['chart_combination_reserved_budget'])
        self.assertIsNone(report['core_chart_combination_observed_operation_counts'])
        self.assertEqual(report['core_chart_combination_source_test_pending_invocations'], ['synthetic-s158-package'])
        key = next(key for key, amount in healthy['chart_combination_reserved_budget'].items() if amount)
        for change in ('whole', 'field', 'changed', 'boolean', 'negative', 'extra'):
            bad = copy.deepcopy(healthy)
            if change == 'whole': del bad['chart_combination_reserved_budget']
            if change == 'field': del bad['chart_combination_reserved_budget'][key]
            if change == 'changed': bad['chart_combination_reserved_budget'][key] += 1
            if change == 'boolean': bad['chart_combination_reserved_budget'][key] = True
            if change == 'negative': bad['chart_combination_reserved_budget'][key] = -1
            if change == 'extra': bad['chart_combination_reserved_budget']['chart_combination.extra'] = 0
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.record_counts({}, 'invalid-s158-package', bad)

    def test_complete_new_external_reservation_precedes_even_library_command(self):
        class BeforeCommand(Exception):
            """Stop at the first synthetic command boundary."""
        report = {}
        vector = MODULE.chart_combination_budget_vector(external_legal=True)
        def first_command(label, _command, _timeout, **_kwargs):
            self.assertEqual(label, 'contract-library')
            self.assertEqual(len(report['core_invocations']), 1)
            reserved = report['core_invocations'][0]
            self.assertEqual(reserved['chart_combination_external_reserved_budget'], vector)
            self.assertEqual((reserved['charged_fit_work_upper'], reserved['point_admissions_upper']), (685, 23679))
            self.assertTrue(reserved['observation_pending'])
            self.assertNotIn('observed_counts', reserved)
            raise BeforeCommand
        session = SimpleNamespace(run=Mock(side_effect=first_command), stable=Path('/inert-stable'))
        with patch.object(MODULE, 'SUBJECTS', MODULE.S14A_SELECTED_SUBJECTS), self.assertRaises(BeforeCommand):
            MODULE.contracts(session, {'contracts_wall_seconds': 300, 'jobs': 1}, report)
        session.run.assert_called_once()
        key = next(key for key, amount in vector.items() if amount)
        limits = dict(MODULE.CHART_COMBINATION_TELEMETRY['global_limits'])
        limits[key] = vector[key] - 1
        untouched = SimpleNamespace(run=Mock(), stable=Path('/inert-stable'))
        with patch.object(MODULE, 'SUBJECTS', MODULE.S14A_SELECTED_SUBJECTS), \
                patch.dict(MODULE.CHART_COMBINATION_TELEMETRY, {'global_limits': limits}), \
                self.assertRaises(MODULE.Incomplete):
            MODULE.contracts(untouched, {'contracts_wall_seconds': 300, 'jobs': 1}, {})
        untouched.run.assert_not_called()

    def test_complete_current_registry_and_vectors_include_only_the_new_delta(self):
        cases = MODULE.historical_s158_contract_examples()
        self.assertEqual(sum(row['code'] is None for row in cases.values()), 4)
        self.assertEqual(sum(row['code'] is not None for row in cases.values()), 77)
        self.assertEqual(MODULE.EXTERNAL_CONSUMER_LIMITS, {'legal_executions': 1, 'negative_compilations': 6})
        self.assertEqual(cases['legal_chart_combination_and_replay']['chart_combination_calls'],
                         MODULE.CHART_COMBINATION_CONTRACTS.legal_expected())
        package, checks, limits = MODULE.cohort_budget_vectors(require_limits=False)
        self.assertEqual([len(row) for row in (package, *checks.values(), limits)], [2765] * 4)
        self.assertEqual((package['charged_fit_work_upper'], package['point_admissions_upper']), (63129, 2253363))
        self.assertEqual((checks['coverage']['charged_fit_work_upper'], checks['coverage']['point_admissions_upper']),
                         (63129, 2253360))
        self.assertEqual((checks['contracts']['charged_fit_work_upper'], checks['contracts']['point_admissions_upper']),
                         (685, 23679))


class ReferenceInputRegistrationControls(HistoricalS14cTestCase):
    """Retained S1.4a registration controls use their explicit historical profile."""

    def setUp(self):
        for name, value in (
                ('SUBJECTS', MODULE.S14A_SELECTED_SUBJECTS), ('SCOPE', MODULE.S14A_SCOPE),
                ('SOURCE_FILE_COUNT', MODULE.S14A_INVENTORY_SOURCE_FILE_COUNT),
                ('BOUND_INPUTS', MODULE.S14A_INVENTORY_BOUND_INPUTS),
                ('INVENTORY_FIXED', MODULE.S14A_INVENTORY_FIXED),
                ('INVENTORY_SUBJECTS', MODULE.S14A_INVENTORY_SUBJECTS),
                ('INVENTORY_BOUND_INPUTS', MODULE.S14A_INVENTORY_BOUND_INPUTS),
                ('INVENTORY_SOURCE_FILE_COUNT', MODULE.S14A_INVENTORY_SOURCE_FILE_COUNT),
                ('INVENTORY_ADMISSION', MODULE.S14A_INVENTORY_ADMISSION),
                ('FIXED', MODULE.S14A_FIXED),
                ('FINAL_PACKAGE_TEST_COUNTS', MODULE.S14A_FINAL_PACKAGE_TEST_COUNTS),
                ('contract_examples', MODULE.historical_s14a_contract_examples),
                ('cohort_budget_vectors', MODULE.historical_s14a_cohort_budget_vectors),
                ('package_charged_upper', MODULE.historical_s14a_package_charged_upper)):
            scoped = patch.object(MODULE, name, value)
            scoped.start(); self.addCleanup(scoped.stop)

    def test_new_inventory_reader_admits_only_exact_singleton_listing_facts(self):
        factory = InventoryAdmissionTests()
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            policy, path = factory.fixture(root)
            policy.update(copy.deepcopy(MODULE.INVENTORY_FIXED))
            path.write_text(json.dumps(policy))
            admitted = factory.read(root, path, policy)
            self.assertEqual(admitted['source_file_count'], 154)
            self.assertEqual(admitted['subjects'],
                             {'reference_input': 'crates/time-core/src/reference_input.rs'})
            self.assertIs(admitted['campaigns_admitted'], False)
            self.assertEqual(admitted['native_test_invocations'], 0)
            for field, value in (
                    ('schema_version', 13), ('source_file_count', 152),
                    ('listing_maximum_total', 513), ('listing_maximum_per_file', 513),
                    ('maximum_mutants', 513), ('subjects', dict(S158_SUBJECTS)),
                    ('mutation_error_values', None), ('mutation_error_values', {'reference_input': ['panic!()']}),
                    ('mutation_equivalences', [{}]), ('native_test_invocations', 1),
                    ('campaigns_admitted', True)):
                changed = copy.deepcopy(policy)
                changed[field] = value
                path.write_text(json.dumps(changed))
                with self.subTest(field=field), self.assertRaises(MODULE.Incomplete):
                    factory.read(root, path, policy)
            changed = copy.deepcopy(policy)
            del changed['mutation_error_values']
            path.write_text(json.dumps(changed))
            with self.assertRaises(MODULE.Incomplete):
                factory.read(root, path, policy)
            path.write_text(json.dumps(policy))
            self.assertEqual(factory.read(root, path, policy), policy)

    def test_singleton_scope_and_new_recipe_preserve_historical_registration(self):
        self.assertEqual(MODULE.SUBJECTS, {'reference_input': 'crates/time-core/src/reference_input.rs'})
        self.assertEqual(MODULE.S158_SELECTED_SUBJECTS, S158_SUBJECTS)
        self.assertEqual(sum(row['count'] for row in MODULE.S158_MUTATION_INVENTORIES.values()), 881)
        self.assertEqual((MODULE.SOURCE_FILE_COUNT, len(set(MODULE.BOUND_INPUTS))), (154, 154))
        self.assertEqual(len(MODULE.COMPLETE_CRITICAL_SUBJECTS), 24)
        self.assertEqual(MODULE.FIXED['schema_version'], 23)
        self.assertEqual(MODULE.FINAL_PACKAGE_TEST_COUNTS,
                         {'ordinary': 286, 'doctests': 27, 'baseline': 313})
        self.assertEqual((MODULE.INVENTORY_FIXED['schema_version'],
                          MODULE.INVENTORY_FIXED['listing_maximum_total']), (14, 512))
        package, checks, _ = MODULE.cohort_budget_vectors(require_limits=False)
        self.assertEqual([len(row) for row in (package, *checks.values())], [2700] * 3)
        self.assertEqual((package['charged_fit_work_upper'], package['point_admissions_upper']),
                         (61827, 2208402))
        self.assertEqual((checks['coverage']['charged_fit_work_upper'],
                          checks['coverage']['point_admissions_upper']), (61827, 2208399))
        with patch.object(MODULE, 'MUTATION_INVENTORIES', {}), self.assertRaises(MODULE.Incomplete):
            MODULE.checkpoint_source_reservation()

    def test_current_e_is_explicitly_empty_and_only_focused_contracts_are_selected(self):
        MODULE.admit_reference_input_equivalence_scope([])
        for bad in (None, False, {}, (), [{'id': 'S158-R-E-001'}], [False] * 23):
            with self.subTest(bad=bad), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_reference_input_equivalence_scope(bad)
        self.assertEqual(len(MODULE.contract_examples()), 7)
        self.assertEqual(MODULE.EXTERNAL_CONSUMER_LIMITS,
                         {'legal_executions': 1, 'negative_compilations': 6})
        self.assertTrue(all(name.startswith(('reference_input_', 'legal_reference_input_'))
                            for name in MODULE.contract_examples()))
        self.assertIn('legal_principal_checked_producer', MODULE.historical_s158_contract_examples())

    def test_new_outcome_and_doc_reception_is_complete_and_strict(self):
        names = MODULE.CHART_COMBINATION.CURRENT_ORDINARY_TESTS
        reference = [name for name in names if name.startswith('reference_input_readiness::')]
        self.assertEqual(len(reference), 4)
        tests = ''.join('test ' + name + ' ... ok\n' for name in names)
        docs = ('test crates/time-core/src/combination/lifecycle/chart.rs - '
                'combination::lifecycle::chart::FixtureChartCombinedEstimate '
                '(line 163) - compile fail ... ok\n')
        for line in (229, 248, 266):
            docs += ('test crates/time-core/src/reference_input.rs - '
                     'reference_input::FixtureReferenceInput (line ' + str(line) + ')'
                     + (' - compile fail' if line in (229, 248) else '') + ' ... ok\n')
        for text in (tests + docs, (tests + docs).replace('\n', '\r\n'),
                     tests + docs + 'separately received footer without final LF'):
            observed = MODULE.chart_combination_test_observation(
                text, require_complete=True, require_doctests=True)
            self.assertEqual(len(observed['chart_combination_literal_doctests']), 4)
            self.assertFalse(observed['chart_combination_source_only_pending'])
        for name in reference:
            bad = tests.replace('test ' + name + ' ... ok\n', '')
            with self.subTest(name=name), self.assertRaises(MODULE.Incomplete):
                MODULE.chart_combination_test_observation(bad, require_complete=True)
            observed = MODULE.chart_combination_test_observation(bad)
            self.assertTrue(observed['chart_combination_source_only_pending'])
            self.assertIsNone(observed['chart_combination_recorded_source_work_upper'])
        for bad in (tests + docs[:-1], tests + docs + docs,
                    tests + docs.replace('(line 266)', '(line 265)'),
                    tests + docs.replace('... ok', '... FAILED', 1)):
            with self.subTest(bad=bad), self.assertRaises(MODULE.Incomplete):
                MODULE.chart_combination_test_observation(bad, require_complete=True, require_doctests=True)

    def test_focused_legal_recipe_is_precharged_before_library_effect(self):
        fund_synthetic_source_recipes(self)
        report = {}

        def stop_after_precharge(label, _command, _timeout, **_options):
            self.assertEqual(label, 'contract-library')
            row = report['core_invocations'][0]
            self.assertEqual((row['charged_fit_work_upper'], row['point_admissions_upper']), (186, 6423))
            self.assertTrue(row['observation_pending'])
            self.assertEqual(len(row['chart_combination_external_reserved_budget']), 486)
            raise MODULE.Incomplete('synthetic library stop; no native operation')

        session = SimpleNamespace(stable=Path('/synthetic-stable'), run=stop_after_precharge)
        with self.assertRaisesRegex(MODULE.Incomplete, 'synthetic library stop'):
            MODULE.contracts(session, {'contracts_wall_seconds': 600, 'jobs': 1}, report)


S14B_EQUIVALENCE_FIXTURE = [{'candidate_sha256': '6f5471fe21d35455a0be6b8f85800cb44d2b1e2f8effd8aec3492e840f0bf8c6', 'id': 'S14B-R-E-001', 'mutant': {'file': 'crates/time-core/src/regression/chart/roundoff.rs', 'function': {'function_name': 'transform', 'return_type': '-> Result<ChartScalars, FixtureChartError>', 'span': {'end': {'column': 2, 'line': 376}, 'start': {'column': 1, 'line': 310}}}, 'genre': 'BinaryOperator', 'name': 'crates/time-core/src/regression/chart/roundoff.rs:350:33: replace || with && in transform', 'package': 'time-core', 'replacement': '&&', 'span': {'end': {'column': 35, 'line': 350}, 'start': {'column': 33, 'line': 350}}}, 'proof': {'path': 'tasks/s1.4b-chart-composition-equivalence.md', 'reviewer': 'root; roundoff_survivor_review', 'section': 'S14B-R-E-001', 'sha256': '67a3b6eb23ef5f476d7f7af09ffb803863a470d2c9bafd3735e28f1397046231', 'verdict': 'independently-justified-equivalent'}, 'rationale': 'Complete checked chart construction gives both zero or both positive coefficient errors; disjunction and conjunction select the same pure branch for every admitted chart.', 'requirement': 'CHECK-7/8/14/25', 'source_sha256': '1aa537417ef06238d85e7614124e399a301f1e8ecd8e6fdf74b6506e58e395bc'}]

class ChartCompositionRegistrationTests(HistoricalS14cTestCase):
    """Preserve S1.4b facts explicitly while the live profile advances to S1.4c."""

    SUBJECTS = {'chart': 'crates/time-core/src/regression/chart.rs',
                'chart_roundoff': 'crates/time-core/src/regression/chart/roundoff.rs',
                'chart_batch': 'crates/time-core/src/chart_batch.rs'}

    def test_exact_composition_scope_counts_and_predecessor_snapshot(self):
        self.assertEqual(MODULE.S14B_SUBJECTS, self.SUBJECTS)
        self.assertEqual(MODULE.S14B_SCOPE.select('all'), self.SUBJECTS)
        self.assertEqual(MODULE.SOURCE_FILE_COUNT, 156)
        self.assertEqual(MODULE.S14B_FINAL_PACKAGE_TEST_COUNTS,
                         {'ordinary': 290, 'doctests': 27, 'baseline': 317})
        self.assertEqual(MODULE.S14B_COORDINATOR_COHORTS['checkpoint'], 'S1.4b')
        self.assertEqual(MODULE.S14B_COORDINATOR_COHORTS['listing_maximum_total'], 1536)
        self.assertEqual(MODULE.S14A_SELECTED_SUBJECTS,
                         {'reference_input': 'crates/time-core/src/reference_input.rs'})
        self.assertEqual(MODULE.S14A_MUTATION_INVENTORIES['reference_input'], {
            'count': 13, 'sha256': '6f9d1095790c8c1844133d4c5b9cf4ed8f8ee97bf8fd020810ad564f74d5419e'})

    def test_historical_composition_inventory_is_retained_without_current_campaign_credit(self):
        self.assertEqual({name: row['count'] for name, row in MODULE.S14B_MUTATION_INVENTORIES.items()},
                         {'chart': 131, 'chart_roundoff': 78, 'chart_batch': 42})
        package, checks, _ = MODULE.historical_s14b_cohort_budget_vectors(require_limits=False)
        old = MODULE.COHORTS.plan_reservation(
            {'inventories': MODULE.S14B_MUTATION_INVENTORIES, 'remaining_checks': {'coverage': 1, 'contracts': 1}},
            package, checks)
        self.assertEqual((old['mutation_variants'], old['mutation_baselines'], old['mutation_package_invocations']),
                         (251, 3, 254))
        self.assertIsNone(MODULE.S14B_CHECKPOINT_SOURCE_RESERVATION)
        self.assertNotEqual(MODULE.MUTATION_INVENTORIES, MODULE.S14B_MUTATION_INVENTORIES)

    def test_composition_equivalences_are_explicitly_empty_without_history_transfer(self):
        MODULE.admit_chart_composition_equivalence_scope([])
        for entries in (None, {}, [True], [{'proof': 'old-checkpoint'}], ()):
            with self.subTest(entries=entries), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_chart_composition_equivalence_scope(entries)
        self.assertEqual(MODULE.MUTATION_ERROR_VALUES, {})

    def test_historical_composition_equivalence_keeps_exact_source_and_complete_proof(self):
        entries = copy.deepcopy(S14B_EQUIVALENCE_FIXTURE)
        self.assertEqual([entry['id'] for entry in entries], ['S14B-R-E-001'])
        self.assertEqual(entries[0]['mutant']['span']['start'], {'line': 350, 'column': 33})
        MODULE.admit_chart_composition_equivalence_scope(entries)
        self.assertEqual(entries[0]['source_sha256'],
                         '1aa537417ef06238d85e7614124e399a301f1e8ecd8e6fdf74b6506e58e395bc')
        self.assertNotEqual(hashlib.sha256((ROOT / entries[0]['mutant']['file']).read_bytes()).hexdigest(),
                            entries[0]['source_sha256'])  # Historical proof grants no current equivalence.
        self.assertEqual(hashlib.sha256((ROOT / entries[0]['proof']['path']).read_bytes()).hexdigest(),
                         entries[0]['proof']['sha256'])
        with self.assertRaises(MODULE.Incomplete):
            MODULE.admit_reference_input_equivalence_scope(entries)
        bad = [None, {}, entries * 2]
        for key, value in (('candidate_sha256', '0' * 64), ('source_sha256', '0' * 64),
                           ('id', 'S158-R-E-001'), ('requirement', 'CHECK-7'),
                           ('proof', {**entries[0]['proof'], 'sha256': '0' * 64})):
            bad.append([{**entries[0], key: value}])
        for column in (26, 53):
            neighbor = copy.deepcopy(entries)
            neighbor[0]['mutant']['span']['start']['column'] = column
            bad.append(neighbor)
        for changed in bad:
            with self.subTest(entries=changed), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_chart_composition_equivalence_scope(changed)

    def test_current_policy_rejects_changed_equivalence_and_source_before_session(self):
        policy = json.loads((ROOT / MODULE.POLICY).read_text())
        changes = []
        policy['mutation_equivalences'] = copy.deepcopy(S14B_EQUIVALENCE_FIXTURE)
        for field, value in (('candidate_sha256', '0' * 64), ('source_sha256', '0' * 64),
                             ('proof', {**policy['mutation_equivalences'][0]['proof'], 'section': 'S158-R-E-001'})):
            changed = copy.deepcopy(policy)
            changed['mutation_equivalences'][0][field] = value
            changes.append(changed)
        changed = copy.deepcopy(policy)
        changed['inputs_sha256']['crates/time-core/tests/private/chart_composition.rs'] = '0' * 64
        changes.append(changed)
        changed = copy.deepcopy(policy)
        changed['mutation_inventories'] = copy.deepcopy(MODULE.S14B_MUTATION_INVENTORIES)
        changes.append(changed)
        changed = copy.deepcopy(policy)
        changed['mutation_error_values'] = {'chart_roundoff': ['false']}
        changes.append(changed)
        original = MODULE.Path.read_text
        for changed in changes:
            def read(path, *args, **kwargs):
                return json.dumps(changed) if path == ROOT / MODULE.POLICY else original(path, *args, **kwargs)
            with self.subTest(policy=changed), patch.object(MODULE.Path, 'read_text', new=read), \
                 patch.object(MODULE.COMMON, 'Session') as session:
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.read_policy(ROOT, 'contracts')
                session.assert_not_called()

    def test_focused_composition_contract_registry_has_no_numerical_producers(self):
        cases = MODULE.CHART_CONTRACTS.composition_contract_examples()
        legal = [item for item in cases.values() if item['code'] is None]
        self.assertEqual((len(legal), len(cases) - len(legal)), (1, 6))
        self.assertEqual(legal[0]['chart_composition_calls'], {
            'labels': 10, 'monotonic': 3, 'frames': 5, 'domains': 1,
            'charts': 5, 'compositions': 3, 'getters': 28})
        self.assertEqual(legal[0]['source'], MODULE.CHART_CONTRACTS.composition_legal_source())
        self.assertEqual({key: legal[0]['calls'][key] for key in
                          ('direct_fit_calls', 'stats_calls', 'point_admissions')},
                         {'direct_fit_calls': 0, 'stats_calls': 0, 'point_admissions': 0})

    def test_complete_vector_adds_new_fixture_once_and_preserves_historical_width(self):
        prior, old_checks, _ = MODULE.historical_s14a_cohort_budget_vectors(require_limits=False)
        package, checks, _ = MODULE.historical_s14b_cohort_budget_vectors(require_limits=False)
        self.assertEqual(len(prior), 2700)
        self.assertEqual([len(row) for row in (package, *checks.values())], [2740] * 3)
        self.assertEqual((prior['charged_fit_work_upper'], prior['point_admissions_upper']), (61827, 2208402))
        self.assertEqual((package['charged_fit_work_upper'], package['point_admissions_upper']), (62571, 2234094))
        self.assertEqual((checks['coverage']['charged_fit_work_upper'], checks['coverage']['point_admissions_upper']),
                         (62571, 2234091))
        self.assertEqual((old_checks['contracts']['charged_fit_work_upper'],
                          checks['contracts']['charged_fit_work_upper']), (685, 685))
        self.assertEqual(set(package) - set(prior), set(MODULE.CHART.COMPOSITION_FIELDS))
        delta = MODULE.chart_composition_fixture_budget()
        self.assertEqual(MODULE.CHART_COMBINATION.numeric_work(delta),
                         {'charged_fit_work_upper': 744, 'point_admissions_upper': 25692})
        self.assertEqual(MODULE.CHART_COMBINATION.numeric_work(
            MODULE.chart_composition_fixture_budget(external_legal=True)),
                         {'charged_fit_work_upper': 0, 'point_admissions_upper': 0})

    def test_composition_names_require_all_actual_outcomes_without_fabricating_operations(self):
        names = ('chart_composition::composition_preserves_dyadic_values_and_rounding_provenance',
                 'chart_composition::composition_retains_both_operand_errors_through_signed_elapsed',
                 'chart_composition::composition_rejects_relationships_and_ordered_arithmetic_failures',
                 'chart_composition_lifecycle::coefficient_error_identity_precedes_batch_principal_and_combination_updates')
        self.assertEqual(tuple(MODULE.CHART.COMPOSITION_CURRENT_ORDINARY_TESTS), names)
        passed = ''.join('test ' + name + ' ... ok\n' for name in names)
        for text in (passed, passed.replace('\n', '\r\n')):
            observed = MODULE.profile_counts(text, require_fit=False, require_chart_composition=True)
            self.assertEqual((observed['charged_fit_work_upper'], observed['point_admissions_upper']), (744, 25692))
            self.assertFalse(observed['chart_composition_source_only_pending'])
            self.assertEqual(observed['chart_composition_test_statuses'], dict.fromkeys(names, 'ok'))
        for text in (passed.split('\n', 1)[1], passed.replace(' ... ok', ' ... FAILED', 1),
                     passed.replace(' ... ok', ' ... ignored', 1), passed + passed.splitlines(keepends=True)[0],
                     passed.replace(names[0], 'chart_composition::unknown'), passed[:-1]):
            with self.subTest(text=text[:100]), self.assertRaises(MODULE.Incomplete):
                MODULE.profile_counts(text, require_fit=False, require_chart_composition=True)
        missing = MODULE.chart_composition_test_observation('')
        self.assertTrue(missing['chart_composition_source_only_pending'])
        self.assertIsNone(missing['chart_composition_recorded_source_work_upper'])
        with self.assertRaises(MODULE.Incomplete):
            MODULE.profile_counts(MODULE.CHART_CONTRACTS.COMPOSITION_MARKER + '\n', require_fit=False)

    def test_current_reference_legal_profile_precedes_the_first_library_effect(self):
        fund_synthetic_source_recipes(self)
        report = {}
        def stop_after_precharge(label, _command, _timeout, **_options):
            self.assertEqual(label, 'contract-library')
            row = report['core_invocations'][0]
            self.assertEqual((row['charged_fit_work_upper'], row['point_admissions_upper']), (685, 23679))
            self.assertEqual(len(row['achieved_phase_complete_contract_reservation']), 2765)
            self.assertEqual(len(row['achieved_phase_external_reserved_budget']), 25)
            self.assertEqual(len(row['achieved_phase_fixture_reserved_budget']), 486)
            self.assertTrue(row['observation_pending'])
            self.assertIsNone(report['core_chart_composition_legal_observed_calls'])
            self.assertIsNone(report['core_chart_composition_observed_operation_counts'])
            raise MODULE.Incomplete('synthetic composition library stop; no native operation')
        session = SimpleNamespace(stable=Path('/synthetic-stable'), run=stop_after_precharge)
        with self.assertRaisesRegex(MODULE.Incomplete, 'synthetic composition library stop'):
            MODULE.contracts(session, {'contracts_wall_seconds': 600, 'jobs': 1}, report)

    def test_new_source_work_is_removed_once_before_the_unchanged_legacy_doctest_cap(self):
        extra = {'charged_fit_work_upper': 744, 'point_admissions_upper': 25692}
        docs = MODULE.LITERAL_DOCTEST_LIMITS
        baseline = {key: MODULE.LEGACY_PROFILE_LIMITS[key] - docs[
            'stats_calls_upper' if key == 'charged_fit_work_upper' else 'point_admissions_upper']
                    for key in extra}
        counts = {key: baseline[key] + extra[key] for key in extra}
        counts.update({'legacy_' + key: value for key, value in baseline.items()})
        counts['internal_fit_upper_bound'] = counts['charged_fit_work_upper']
        counts['chart_composition_recorded_source_work_upper'] = extra
        observed = MODULE.account_literal_doctests(counts)
        adaptive = MODULE.ADAPTIVE.literal_doctest_reservation()
        self.assertEqual({key: observed[key] for key in extra}, {
            key: MODULE.LEGACY_PROFILE_LIMITS[key] + extra[key] + adaptive[key] for key in extra})
        rejected = dict(counts)
        rejected['charged_fit_work_upper'] += 1
        rejected['legacy_charged_fit_work_upper'] += 1
        with self.assertRaises(MODULE.Incomplete):
            MODULE.account_literal_doctests(rejected)

    def test_package_precharge_rejects_missing_or_changed_composition_families(self):
        fund_synthetic_source_recipes(self)
        good = {**MODULE.package_charged_upper(), **MODULE.selection_reservation(package=True),
                'observation_pending': True}
        healthy = {}
        MODULE.record_counts(healthy, 'synthetic complete source reservation', copy.deepcopy(good))
        self.assertEqual(len(healthy['core_invocations']), 1)
        for field in ('chart_composition_reserved_budget', 'chart_composition_fixture_reserved_budget'):
            for change in ('missing', 'namespace', 'undercharge', 'boolean'):
                bad = copy.deepcopy(good)
                if change == 'missing': del bad[field]
                else:
                    key = next(key for key, value in bad[field].items() if value > 0)
                    if change == 'namespace': del bad[field][key]
                    if change == 'undercharge': bad[field][key] -= 1
                    if change == 'boolean': bad[field][key] = True
                report = {}
                with self.subTest(field=field, change=change), self.assertRaises(MODULE.Incomplete):
                    MODULE.record_counts(report, 'synthetic malformed reservation', bad)
                self.assertNotIn('core_invocations', report)


class AchievedPhaseIntegrationTests(HistoricalS14cTestCase):
    """Independent S1.4c receiving, admission and finite-work controls.

    Synthetic command observations exercise the real Python boundary only.
    The five Rust outcomes are never treated as inner-operation observations.
    """

    SUBJECTS = {'reference_input': 'crates/time-core/src/reference_input.rs'}
    NAMES = (
        'reference_input_readiness::achieved_phase_dyadics_change_coordinates_and_preserve_rate',
        'reference_input_readiness::achieved_phase_context_and_scalar_rejections_preserve_ready_owner',
        'reference_input_readiness::achieved_phase_noeffect_unknown_and_repetition_are_monotone',
        'reference_input_readiness::achieved_phase_arithmetic_failure_is_admitted_and_invalidates_old_owner',
        'reference_input_readiness::achieved_phase_remapped_histories_retain_without_new_estimate',
    )
    PASSED = ''.join('test ' + name + ' ... ok\n' for name in NAMES)
    RECEIVER_FIELDS = (
        'receiver_attempts', 'garde_validations', 'garde_callbacks', 'relationship_passes',
        'readiness_checks', 'chart_identity_checks', 'mapping_label_comparisons',
        'domain_field_comparisons', 'internal_product_getter_requests_upper',
        'effect_chart_constructions_upper', 'endpoint_frame_constructions_upper',
        'coordinate_changed_commits_upper', 'composition_attempts_upper', 'completion_constructions_upper',
        'replacement_reads', 'replacement_moves', 'completion_debug', 'error_source_reads',
        'error_displays', 'error_debug', 'observer_reads_upper', 'additional_doctest_compilations',
        'additional_doctest_compile_fail_cases', 'additional_uncalled_legal_helpers',
        'doctest_executed_examples',
    )
    RECEIVER_VALUES = (
        38, 38, 38, 76, 76, 76, 152, 228, 760, 76, 76, 38, 38, 38,
        1, 21, 1, 2, 1, 1, 40960, 0, 0, 0, 0,
    )
    DOCS = ''.join(
        'test crates/time-core/src/reference_input.rs - reference_input::' + owner
        + ' (line ' + str(line) + ')' + (' - compile fail' if line in (106, 436, 455, 473) else '')
        + ' ... ok\n'
        for line, owner in ((106, 'FixtureAchievedPhaseCompletion'), (436, 'FixtureReferenceInput'),
                            (455, 'FixtureReferenceInput'), (473, 'FixtureReferenceInput'),
                            (495, 'FixtureReferenceInput'), (517, 'FixtureReferenceInput')))

    @classmethod
    def receiver(cls, *, include_doctests=False):
        vector = dict(zip(('achieved_phase.' + key for key in cls.RECEIVER_FIELDS),
                          cls.RECEIVER_VALUES, strict=True))
        if include_doctests:
            vector.update({'achieved_phase.additional_doctest_compilations': 3,
                           'achieved_phase.additional_doctest_compile_fail_cases': 2,
                           'achieved_phase.additional_uncalled_legal_helpers': 1})
        return vector

    def test_live_package_adds_the_new_numerical_fixture_once(self):
        # The old composition helper is a genuine healthy neighboring profile.
        old = MODULE.chart_composition_complete_budget(include_doctests=True)
        self.assertEqual((old['charged_fit_work_upper'], old['point_admissions_upper']),
                         (62571, 2234094))
        live = MODULE.package_charged_upper(include_doctests=True)
        self.assertEqual(live, {'charged_fit_work_upper': 63129,
                                'point_admissions_upper': 2253363})
        self.assertEqual(live['charged_fit_work_upper'] - old['charged_fit_work_upper'], 558)
        self.assertEqual(live['point_admissions_upper'] - old['point_admissions_upper'], 19269)

    def test_complete_2765_vectors_preserve_two_distinct_numerical_additions_once(self):
        prior, old_checks, _ = MODULE.historical_s14b_cohort_budget_vectors(require_limits=False)
        package, checks, limits = MODULE.cohort_budget_vectors(require_limits=False)
        self.assertEqual([len(row) for row in (prior, *old_checks.values())], [2740] * 3)
        self.assertEqual([len(row) for row in (package, *checks.values(), limits)], [2765] * 4)
        self.assertEqual(set(package) - set(prior), set(self.receiver()))
        self.assertEqual({key: package[key] for key in self.receiver()}, self.receiver(include_doctests=True))
        self.assertEqual({key: checks['coverage'][key] for key in self.receiver()}, self.receiver())
        self.assertEqual((package['charged_fit_work_upper'], package['point_admissions_upper']),
                         (63129, 2253363))
        self.assertEqual((checks['coverage']['charged_fit_work_upper'], checks['coverage']['point_admissions_upper']),
                         (63129, 2253360))
        # Both486-field fixture deltas keep their independent source ownership.
        composition = MODULE.chart_composition_fixture_budget()
        phase = MODULE.achieved_phase_fixture_budget()
        self.assertEqual(len(phase), 486)
        self.assertNotEqual(composition, phase)
        self.assertEqual(MODULE.CHART_COMBINATION.numeric_work(composition),
                         {'charged_fit_work_upper': 744, 'point_admissions_upper': 25692})
        self.assertEqual(MODULE.CHART_COMBINATION.numeric_work(phase),
                         {'charged_fit_work_upper': 558, 'point_admissions_upper': 19269})
        for key in ('charged_fit_work_upper', 'point_admissions_upper'):
            self.assertEqual(package[key], prior[key] + MODULE.CHART_COMBINATION.numeric_work(phase)[key])
        for key, value in phase.items():
            self.assertEqual(package[key], prior[key] + value)
        for key, value in MODULE.achieved_phase_composition_delta(include_doctests=True).items():
            self.assertEqual(package[key], prior[key] + value)
        self.assertEqual(MODULE.COMMON.canonical_sha256(package),
                         '781bad30d3f8c39a10d39ce57679a162238eb7683babc4be55fd52bb73c780bd')

    def test_all_five_named_outcomes_are_required_and_no_inner_counts_are_observed(self):
        for text in (self.PASSED, self.PASSED.replace('\n', '\r\n')):
            observed = MODULE.profile_counts(text, require_fit=False, require_achieved_phase=True)
            self.assertEqual(observed['achieved_phase_test_statuses'], dict.fromkeys(self.NAMES, 'ok'))
            self.assertFalse(observed['achieved_phase_source_only_pending'])
            self.assertIsNone(observed['achieved_phase_observed_operation_counts'])
            self.assertEqual(observed['achieved_phase_recorded_source_work_upper'],
                             {'charged_fit_work_upper': 558, 'point_admissions_upper': 19269})
            self.assertEqual((observed['charged_fit_work_upper'], observed['point_admissions_upper']),
                             (558, 19269))
        for name in self.NAMES:
            partial = self.PASSED.replace('test ' + name + ' ... ok\n', '')
            for text in (partial, self.PASSED.replace('test ' + name + ' ... ok\n',
                                                   'test ' + name + ' ... FAILED\n'),
                         self.PASSED.replace('test ' + name + ' ... ok\n',
                                             'test ' + name + ' ... ignored\n')):
                with self.subTest(name=name, text=text[:100]):
                    result = MODULE.achieved_phase_test_observation(text)
                    self.assertTrue(result['achieved_phase_source_only_pending'])
                    self.assertIsNone(result['achieved_phase_recorded_source_work_upper'])
                    self.assertIsNone(result['achieved_phase_observed_operation_counts'])
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.profile_counts(text, require_fit=False, require_achieved_phase=True)
        for malformed in (self.PASSED + self.PASSED.splitlines(keepends=True)[0],
                          self.PASSED.replace(self.NAMES[0], 'reference_input_readiness::achieved_phase_unknown'),
                          self.PASSED.replace(' ... ok', ' ... success', 1), self.PASSED[:-1]):
            with self.subTest(malformed=malformed[:100]), self.assertRaises(MODULE.Incomplete):
                MODULE.achieved_phase_test_observation(malformed)

    def test_six_literal_reference_docs_keep_four_failures_two_uncalled_legal_helpers(self):
        actual = MODULE.achieved_phase_test_observation(
            self.PASSED + self.DOCS, require_complete=True, require_doctests=True)
        self.assertEqual(actual['achieved_phase_literal_doctests'],
                         [line.removeprefix('test ').removesuffix(' ... ok') for line in self.DOCS.splitlines()])
        self.assertEqual(sum(' - compile fail' in line for line in actual['achieved_phase_literal_doctests']), 4)
        self.assertEqual(self.receiver(include_doctests=True)['achieved_phase.doctest_executed_examples'], 0)
        # Original package baseline needs these exact locations. Mutant line
        # shifts cannot fabricate a literal original-baseline observation.
        for bad in (self.PASSED, self.PASSED + self.DOCS.replace('(line 106)', '(line 107)'),
                    self.PASSED + self.DOCS + self.DOCS,
                    self.PASSED + self.DOCS.replace(' ... ok', ' ... FAILED', 1),
                    self.PASSED + self.DOCS[:-1]):
            with self.subTest(bad=bad[-100:]), self.assertRaises(MODULE.Incomplete):
                MODULE.achieved_phase_test_observation(bad, require_complete=True, require_doctests=True)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.achieved_phase_test_observation(self.PASSED, require_doctests=1)

    def test_phase_work_is_removed_once_before_the_unchanged_literal_doctest_cap(self):
        extra = {'charged_fit_work_upper': 558, 'point_admissions_upper': 19269}
        docs = MODULE.LITERAL_DOCTEST_LIMITS
        baseline = {key: MODULE.LEGACY_PROFILE_LIMITS[key] - docs[
            'stats_calls_upper' if key == 'charged_fit_work_upper' else 'point_admissions_upper'] for key in extra}
        counts = {key: baseline[key] + extra[key] for key in extra}
        counts.update({'legacy_' + key: value for key, value in baseline.items()})
        counts['internal_fit_upper_bound'] = counts['charged_fit_work_upper']
        counts['achieved_phase_recorded_source_work_upper'] = extra
        result = MODULE.account_literal_doctests(counts)
        adaptive = MODULE.ADAPTIVE.literal_doctest_reservation()
        self.assertEqual({key: result[key] for key in extra},
                         {key: MODULE.LEGACY_PROFILE_LIMITS[key] + extra[key] + adaptive[key] for key in extra})
        counts['charged_fit_work_upper'] += 1
        counts['legacy_charged_fit_work_upper'] += 1
        with self.assertRaises(MODULE.Incomplete):
            MODULE.account_literal_doctests(counts)

    def test_equal_numerical_uppers_from_two_families_are_each_added_once(self):
        # Deliberately equal synthetic values distinguish ownership from value
        # equality. Deduplicating equal dictionaries loses one family's charge.
        extra = {'charged_fit_work_upper': 558, 'point_admissions_upper': 19269}
        docs = MODULE.LITERAL_DOCTEST_LIMITS
        baseline = {key: MODULE.LEGACY_PROFILE_LIMITS[key] - docs[
            'stats_calls_upper' if key == 'charged_fit_work_upper' else 'point_admissions_upper'] for key in extra}
        counts = {key: baseline[key] + 2 * extra[key] for key in extra}
        counts.update({'legacy_' + key: value for key, value in baseline.items()})
        counts.update({'internal_fit_upper_bound': counts['charged_fit_work_upper'],
                       'chart_composition_recorded_source_work_upper': dict(extra),
                       'achieved_phase_recorded_source_work_upper': dict(extra)})
        result = MODULE.account_literal_doctests(counts)
        adaptive = MODULE.ADAPTIVE.literal_doctest_reservation()
        self.assertEqual({key: result[key] for key in extra}, {
            key: MODULE.LEGACY_PROFILE_LIMITS[key] + 2 * extra[key] + adaptive[key] for key in extra})

    def test_receiver_fixture_and_composition_fields_are_mandatory_before_recording(self):
        fund_synthetic_source_recipes(self)
        good = {**MODULE.package_charged_upper(), **MODULE.selection_reservation(package=True),
                'observation_pending': True}
        self.assertEqual(good['achieved_phase_reserved_budget'], self.receiver())
        self.assertEqual(len(good['achieved_phase_fixture_reserved_budget']), 486)
        self.assertEqual(len(good['achieved_phase_composition_delta_reserved_budget']), 40)
        self.assertEqual(len(good['achieved_phase_complete_package_reservation']), 2765)
        healthy = {}
        MODULE.record_counts(healthy, 'synthetic complete phase source reservation', copy.deepcopy(good))
        self.assertEqual(healthy['core_achieved_phase_reserved_totals'], self.receiver())
        self.assertIsNone(healthy['core_achieved_phase_observed_operation_counts'])
        self.assertEqual(healthy['core_achieved_phase_source_test_pending_invocations'],
                         ['synthetic complete phase source reservation'])
        for field in ('achieved_phase_reserved_budget', 'achieved_phase_fixture_reserved_budget',
                      'achieved_phase_composition_delta_reserved_budget', 'achieved_phase_complete_package_reservation'):
            for change in ('missing', 'field', 'undercharge', 'boolean', 'negative', 'overflow', 'extra'):
                bad = copy.deepcopy(good)
                if change == 'missing':
                    del bad[field]
                else:
                    key = next(key for key, value in bad[field].items() if value > 0)
                    if change == 'field': del bad[field][key]
                    if change == 'undercharge': bad[field][key] -= 1
                    if change == 'boolean': bad[field][key] = True
                    if change == 'negative': bad[field][key] = -1
                    if change == 'overflow': bad[field][key] = 1 << 63
                    if change == 'extra': bad[field]['foreign.unfunded'] = 0
                report = {}
                with self.subTest(field=field, change=change), self.assertRaises(MODULE.Incomplete):
                    MODULE.record_counts(report, 'malformed phase reservation', bad)
                self.assertNotIn('core_invocations', report)
        unowned = {**good}
        del unowned['selection_expected']
        with self.assertRaises(MODULE.Incomplete):
            MODULE.record_counts({}, 'unowned phase source reservation', unowned)

    def test_two_pending_packages_add_each_fixture_once_without_refunds(self):
        fund_synthetic_source_recipes(self)
        report = {}
        good = {**MODULE.package_charged_upper(), **MODULE.selection_reservation(package=True),
                'observation_pending': True}
        for label in ('failed-first-prefix', 'fresh-second-attempt'):
            MODULE.record_counts(report, label, copy.deepcopy(good))
        self.assertEqual(report['core_achieved_phase_reserved_totals'],
                         {key: 2 * value for key, value in self.receiver().items()})
        self.assertEqual(report['core_achieved_phase_fixture_reserved_totals'],
                         {key: 2 * value for key, value in MODULE.achieved_phase_fixture_budget().items()})
        old_fixture = MODULE.chart_composition_fixture_budget()
        phase_fixture = MODULE.achieved_phase_fixture_budget()
        original = MODULE.chart_combination_budget_vector()
        self.assertEqual(report['core_chart_combination_complete_reserved_totals'],
                         {key: 2 * (original[key] + old_fixture[key] + phase_fixture[key]) for key in original})
        self.assertIsNone(report['core_achieved_phase_observed_operation_counts'])

    def test_reference_legal_102_calls_reserve_zero_receivers_before_the_first_command(self):
        fund_synthetic_source_recipes(self)
        legal = [row for row in MODULE.contract_examples().values() if row['code'] is None]
        self.assertEqual(len(legal), 1)
        self.assertEqual(sum(legal[0]['reference_input_calls'].values()), 102)
        self.assertNotIn('receive_achieved_phase', legal[0]['source'])
        vector = MODULE.achieved_phase_complete_budget(external_legal=True)
        self.assertEqual(len(vector), 2765)
        self.assertEqual({key: vector[key] for key in self.receiver()}, dict.fromkeys(self.receiver(), 0))
        self.assertEqual((vector['charged_fit_work_upper'], vector['point_admissions_upper']), (685, 23679))
        self.assertGreater(sum(value for key, value in vector.items() if key.startswith('chart_composition.')), 0)
        report = {}

        def first_command(label, _command, _timeout, **_options):
            self.assertEqual(label, 'contract-library')
            row = report['core_invocations'][0]
            self.assertEqual(row['achieved_phase_complete_contract_reservation'], vector)
            self.assertEqual(row['achieved_phase_external_reserved_budget'], dict.fromkeys(self.receiver(), 0))
            self.assertTrue(row['observation_pending'])
            self.assertIsNone(report['core_achieved_phase_observed_operation_counts'])
            raise MODULE.Incomplete('synthetic reference legal boundary; no native execution')

        session = SimpleNamespace(stable=Path('/inert-stable'), run=Mock(side_effect=first_command))
        with self.assertRaisesRegex(MODULE.Incomplete, 'synthetic reference legal boundary'):
            MODULE.contracts(session, {'contracts_wall_seconds': 300, 'jobs': 1}, report)
        session.run.assert_called_once()

    def test_zero_receiver_external_profile_still_requires_every_exact_field(self):
        fund_synthetic_source_recipes(self)
        complete = MODULE.achieved_phase_complete_budget(external_legal=True)
        good = {
            'charged_fit_work_upper': 685, 'point_admissions_upper': 23679,
            'chart_combination_external_reserved_budget': MODULE.chart_combination_budget_vector(external_legal=True),
            'achieved_phase_external_reserved_budget': dict.fromkeys(self.receiver(), 0),
            'achieved_phase_fixture_reserved_budget': dict.fromkeys(MODULE.CHART_COMBINATION.FIELDS, 0),
            'achieved_phase_composition_delta_reserved_budget': {
                key: value for key, value in complete.items() if key.startswith('chart_composition.')},
            'achieved_phase_complete_contract_reservation': complete,
            'observation_pending': True,
        }
        report = {}
        MODULE.record_counts(report, 'synthetic zero-receiver legal reservation', copy.deepcopy(good))
        self.assertEqual(len(report['core_invocations']), 1)
        for field in ('achieved_phase_external_reserved_budget', 'achieved_phase_fixture_reserved_budget',
                      'achieved_phase_composition_delta_reserved_budget', 'achieved_phase_complete_contract_reservation'):
            for change in ('missing', 'field', 'changed', 'boolean', 'negative', 'overflow'):
                bad = copy.deepcopy(good)
                if change == 'missing': del bad[field]
                else:
                    key = next(iter(bad[field]))
                    if change == 'field': del bad[field][key]
                    if change == 'changed': bad[field][key] += 1
                    if change == 'boolean': bad[field][key] = False
                    if change == 'negative': bad[field][key] = -1
                    if change == 'overflow': bad[field][key] = 1 << 63
                report = {}
                with self.subTest(field=field, change=change), self.assertRaises(MODULE.Incomplete):
                    MODULE.record_counts(report, 'forged zero-receiver legal reservation', bad)
                self.assertNotIn('core_invocations', report)

    def test_historical_23_source_contract_rejects_every_drift(self):
        expected = MODULE.CHART_COMBINATION.ACHIEVED_PHASE_SOURCE_SHA256
        self.assertEqual(len(expected), 23)
        self.assertEqual(expected['crates/time-core/src/reference_input.rs'],
                         'be4a4142672606c3feca43ce36b307eeb2d1b9a56866be203889412f9aa30209')
        self.assertEqual(expected['crates/time-core/src/lib.rs'],
                         'af227606c8f560aa1bdf4d3c99c3e7df1429967b64f276f23562353892f3a629')
        self.assertEqual(expected['crates/time-core/tests/private/reference_input_readiness.rs'],
                         '8dc34c4ac39c09302cd18d0633c0edc2a0aefb7d66cbf157110fbf0c17e3b58c')
        # Frozen contract facts are historical; current bytes have seven reviewed overrides.
        actual = dict(expected)
        self.assertEqual(MODULE.CHART_COMBINATION.admit_achieved_phase_sources(actual), actual)
        for name in expected:
            for kind in ('changed', 'missing'):
                bad = dict(actual)
                if kind == 'changed': bad[name] = '0' * 64
                else: del bad[name]
                with self.subTest(name=name, kind=kind), self.assertRaises(MODULE.CHART_COMBINATION.Incomplete):
                    MODULE.CHART_COMBINATION.admit_achieved_phase_sources(bad)
        with self.assertRaises(MODULE.CHART_COMBINATION.Incomplete):
            MODULE.CHART_COMBINATION.admit_achieved_phase_sources({**actual, 'foreign.rs': '0' * 64})

    def test_fresh_singleton_inventory_and_empty_e_keep_old_results_historical(self):
        self.assertEqual(MODULE.SUBJECTS, self.SUBJECTS)
        self.assertEqual(MODULE.SCOPE.select('all'), self.SUBJECTS)
        self.assertEqual((MODULE.SOURCE_FILE_COUNT, len(MODULE.BOUND_INPUTS)), (156, 156))
        self.assertEqual(MODULE.FINAL_PACKAGE_TEST_COUNTS, {'ordinary': 295, 'doctests': 30, 'baseline': 325})
        self.assertEqual(MODULE.COORDINATOR_COHORTS['checkpoint'], 'S1.4c')
        self.assertEqual(MODULE.COORDINATOR_COHORTS['listing_maximum_total'], 512)
        # Actual inventory-v2/verification.json, receipt SHA174afcc0...76e981.
        # Keep independent listing facts without requiring a private receipt on
        # every checkout; its discovery granted no campaign or test invocation.
        expected_inventory = {'reference_input': {
            'count': 27, 'sha256': 'ec281140c43fcf28fb6d22fe20258a56c2f4205bc2717dfc758634dbbead705a'}}
        self.assertEqual(MODULE.MUTATION_INVENTORIES, expected_inventory)
        reservation = MODULE.checkpoint_source_reservation()
        self.assertEqual(reservation, MODULE.CHECKPOINT_SOURCE_RESERVATION)
        self.assertEqual((reservation['mutation_variants'], reservation['mutation_baselines'],
                          reservation['mutation_package_invocations']), (27, 1, 28))
        self.assertEqual(len(reservation['source_work']), 2765)
        self.assertEqual({key: reservation['source_work'][key] for key in
                          ('charged_fit_work_upper', 'point_admissions_upper')},
                         {'charged_fit_work_upper': 1831426, 'point_admissions_upper': 65371203})
        self.assertEqual(MODULE.S14A_MUTATION_INVENTORIES['reference_input']['count'], 13)
        self.assertEqual(sum(row['count'] for row in MODULE.S14B_MUTATION_INVENTORIES.values()), 251)
        MODULE.admit_reference_input_equivalence_scope([])
        for bad in (None, {}, (), [True], [{'id': 'S14B-R-E-001'}], [{'id': 'S158-R-E-001'}]):
            with self.subTest(entries=bad), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_reference_input_equivalence_scope(bad)
        self.assertEqual(MODULE.MUTATION_ERROR_VALUES, {})
        # Historical metadata never admits the current source tree.
        for lane in ('contracts', 'coverage', 'mutation', 'cohort-union'):
            with self.subTest(lane=lane), patch.object(MODULE.COMMON, 'Session') as session, self.assertRaises(MODULE.Incomplete):
                MODULE.read_policy(ROOT, lane)
            session.assert_not_called()
        invalid_inventories = (
            {}, {'chart': expected_inventory['reference_input']},
            {'reference_input': {'count': 27}},
            {'reference_input': {'sha256': expected_inventory['reference_input']['sha256']}},
            {'reference_input': {**expected_inventory['reference_input'], 'count': 0}},
            {'reference_input': {**expected_inventory['reference_input'], 'count': True}},
            {'reference_input': {**expected_inventory['reference_input'], 'sha256': '0' * 63}},
        )
        for invalid in invalid_inventories:
            # The current admitted policy and cached reservation remain healthy.
            # This isolates rejection of independently missing/incomplete facts.
            with self.subTest(inventory=invalid), patch.object(MODULE, 'MUTATION_INVENTORIES', invalid), \
                 patch.object(MODULE.COMMON, 'Session') as session:
                for lane in ('contracts', 'coverage', 'mutation', 'cohort-union'):
                    with self.subTest(lane=lane), self.assertRaises(MODULE.Incomplete):
                        MODULE.read_policy(ROOT, lane)
                if set(invalid) != set(self.SUBJECTS):
                    with self.assertRaisesRegex(MODULE.Incomplete, 'inventory remains pending'):
                        MODULE.checkpoint_source_reservation()
                session.assert_not_called()

    def test_complete_prospective_vector_checks_every_work_group_before_effects(self):
        fund_synthetic_source_recipes(self)
        with patch.dict(MODULE.FINAL_PACKAGE_TEST_COUNTS, {'ordinary': 295, 'doctests': 30, 'baseline': 325}):
            coverage = MODULE.prospective_scope_reservation('coverage', self.SUBJECTS)
            contracts = MODULE.prospective_scope_reservation('contracts', self.SUBJECTS)
            self.assertEqual(len(coverage['complete_source_work_upper']), 2765)
            self.assertEqual(coverage['package_test_count_upper_each'], 325)
            self.assertEqual(coverage['package_ordinary_test_count_upper_each'], 295)
            self.assertEqual(coverage['package_doctest_count_upper_each'], 30)
            self.assertEqual(coverage['achieved_phase_source_work_upper'], self.receiver())
            self.assertEqual(contracts['achieved_phase_source_work_upper'], dict.fromkeys(self.receiver(), 0))
            groups = (
                ('aggregate', MODULE.GLOBAL_LIMITS, ''),
                ('selection calls', MODULE.SELECTION_GLOBAL_LIMITS, 'selection.call.'),
                ('selection helpers', MODULE.SELECTION_HELPER_GLOBAL_LIMITS, 'selection.helper.'),
                ('selection canonical', MODULE.SELECTION_HELPER_CANONICAL_GLOBAL_LIMITS, 'selection.helper_work.'),
                ('raw private', MODULE.SOURCE_RAW_PRIVATE_GLOBAL_LIMITS, 'source.private.'),
                ('projection', MODULE.PROJECTION_TELEMETRY['global_limits'], ''),
                ('enclosure', MODULE.ENCLOSURE_TELEMETRY['global_limits'], ''),
                ('projected batch', MODULE.PROJECTED_BATCH_TELEMETRY['global_limits'], ''),
                ('principal', MODULE.PRINCIPAL_TELEMETRY['global_limits'], ''),
                ('combination', MODULE.COMBINATION_TELEMETRY['global_limits'], ''),
                ('lifecycle', MODULE.LIFECYCLE_TELEMETRY['global_limits'], ''),
                ('adaptive', MODULE.ADAPTIVE_TELEMETRY['global_limits'], ''),
                ('projection adapter', MODULE.PROJECTION_ADAPTER_TELEMETRY['global_limits'], ''),
                ('standard error', MODULE.STANDARD_ERROR_TELEMETRY['global_limits'], ''),
                ('tracking', MODULE.TRACKING_TELEMETRY['global_limits'], ''),
                ('chart', MODULE.CHART_TELEMETRY['global_limits'], ''),
                ('chart batch', MODULE.CHART_BATCH_TELEMETRY['global_limits'], ''),
                ('chart principal', MODULE.CHART_PRINCIPAL_TELEMETRY['global_limits'], ''),
                ('chart combination', MODULE.CHART_COMBINATION_TELEMETRY['global_limits'], ''),
                ('chart composition', MODULE.CHART_COMPOSITION_TELEMETRY['global_limits'], ''),
                ('achieved phase', MODULE.ACHIEVED_PHASE_TELEMETRY['global_limits'], ''),
            )
            for name, mapping, prefix in groups:
                key = next(key for key in mapping if coverage['complete_source_work_upper'].get(prefix + key, 0) > 0)
                maximum = coverage['complete_source_work_upper'][prefix + key]
                with self.subTest(group=name), patch.dict(mapping, {key: maximum - 1}), \
                     patch.object(MODULE.COMMON, 'Session') as session, self.assertRaises(MODULE.Incomplete):
                    MODULE.prospective_scope_reservation('coverage', self.SUBJECTS)
                session.assert_not_called()

    def test_reference_legal_complete_vector_undercharge_stops_before_library_callback(self):
        fund_synthetic_source_recipes(self)
        required = MODULE.achieved_phase_complete_budget(external_legal=True)
        key = next(key for key in required if key.startswith('chart_composition.') and required[key] > 0)
        limits = dict(MODULE.CHART_COMPOSITION_TELEMETRY['global_limits'])
        limits[key] = required[key] - 1
        session = SimpleNamespace(stable=Path('/inert-stable'), run=Mock())
        with patch.dict(MODULE.CHART_COMPOSITION_TELEMETRY, {'global_limits': limits}), \
             self.assertRaises(MODULE.Incomplete):
            MODULE.contracts(session, {'contracts_wall_seconds': 300, 'jobs': 1}, {})
        session.run.assert_not_called()

    def test_cohort_hook_binds_fresh_325_baseline_complete_vector_and_all_phase_recipe_inputs(self):
        fund_synthetic_source_recipes(self)
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            path = root / MODULE.COHORT_PLAN
            path.parent.mkdir(parents=True)
            path.write_text('{}\n')
            policy_path = root / MODULE.POLICY
            policy_path.parent.mkdir(parents=True)
            policy_path.write_text('{}\n')
            policy = {'inputs_sha256': {'inert.rs': '1' * 64}, 'mutation_equivalences': [],
                      'mutation_inventories': {'reference_input': {'count': 2, 'sha256': '2' * 64}}}
            canonical = MODULE.COMMON.canonical_sha256
            recipe_inputs = []

            def observe_recipe(value):
                if type(value) is dict and 'checkpoint_attempt_scope' in value:
                    recipe_inputs.append(copy.deepcopy(value))
                return canonical(value)

            with patch.object(MODULE, 'read_policy', return_value=policy), \
                 patch.object(MODULE, 'inventory_tool_inputs', return_value=set()), \
                 patch.dict(MODULE.FINAL_PACKAGE_TEST_COUNTS, {'ordinary': 295, 'doctests': 30, 'baseline': 325}), \
                 patch.object(MODULE.COMMON, 'canonical_sha256', side_effect=observe_recipe), \
                 patch.object(MODULE.COHORTS, 'admit_plan', return_value={'inert': True}) as receive:
                MODULE.read_cohort_plan(root, path, policy)
            args = receive.call_args.kwargs
            self.assertEqual(args['baseline_tests'], 325)
            self.assertEqual(len(args['package_budget']), 2765)
            self.assertEqual([len(row) for row in args['check_budgets'].values()], [2765, 2765])
            self.assertEqual(len(args['global_limits']), 2765)
            self.assertEqual(len(recipe_inputs), 1)
            recipe = recipe_inputs[0]
            self.assertEqual(args['recipe_sha256'], canonical(recipe))
            self.assertEqual(recipe['achieved_phase_fixture_recipes'],
                             MODULE.CHART_COMBINATION.achieved_phase_case_recipes())
            self.assertEqual(recipe['achieved_phase_receiver_calls']['receiver_attempts'], 38)
            self.assertEqual(len(recipe['achieved_phase_sources']), 23)
            self.assertEqual(recipe['achieved_phase_reference_doctests'], [106, 436, 455, 473, 495, 517])
            for field in ('package', 'achieved_phase_fixture_recipes', 'achieved_phase_receiver_calls',
                          'achieved_phase_sources', 'achieved_phase_reference_doctests', 'test_counts'):
                bad = copy.deepcopy(recipe)
                del bad[field]
                with self.subTest(field=field):
                    self.assertNotEqual(canonical(bad), args['recipe_sha256'])

    def test_original_mutation_baseline_requires_fresh_325_before_profile_receiving(self):
        fund_synthetic_source_recipes(self)
        helper = CoreAnalysisTests()
        outcome = {'scenario': 'Baseline', 'summary': 'Success', 'log_path': 'baseline.log',
                   'phase_results': helper.phase_results('Success')}
        original = helper.completed_mutation_log('')

        class ReachedProfile(Exception):
            """Observe only Python receiver admission, before synthetic profiles."""

        with tempfile.TemporaryDirectory() as temporary, patch.dict(
                MODULE.FINAL_PACKAGE_TEST_COUNTS, {'ordinary': 295, 'doctests': 30, 'baseline': 325}):
            output = Path(temporary)
            for count in (313, 317, 324, 325):
                (output / 'baseline.log').write_text(original.replace('96 passed', str(count) + ' passed'))
                report = {}
                MODULE.record_counts(report, 'reference_input:mutation-baseline', {
                    **MODULE.package_charged_upper(include_doctests=True),
                    **MODULE.selection_reservation(package=True, include_doctests=True),
                    'observation_pending': True})
                row = report['core_invocations'][0]
                with self.subTest(count=count), patch.object(
                        MODULE, 'profile_counts', side_effect=ReachedProfile) as receive:
                    if count == 325:
                        with self.assertRaises(ReachedProfile):
                            MODULE.observe_mutation_outcome(report, output / 'outcomes.json',
                                'reference_input:mutation-baseline', outcome, row)
                        receive.assert_called_once()
                    else:
                        with self.assertRaisesRegex(MODULE.Incomplete, 'all 325 admitted'):
                            MODULE.observe_mutation_outcome(report, output / 'outcomes.json',
                                'reference_input:mutation-baseline', outcome, row)
                        receive.assert_not_called()
                self.assertTrue(row['observation_pending'])
                self.assertEqual(row['achieved_phase_complete_package_reservation']['achieved_phase.receiver_attempts'], 38)

    def test_every_reached_phase_outcome_needs_the_complete_prepaid_package_scope(self):
        """Actual Python receiving must not turn source outcomes into free work."""
        fund_synthetic_source_recipes(self)
        first = self.PASSED.splitlines(keepends=True)[0]
        for text in (self.PASSED, first, first.replace(' ... ok', ' ... FAILED'),
                     first.replace(' ... ok', ' ... ignored')):
            observed = MODULE.profile_counts(text, require_fit=False)
            # The same actual parsed value is accepted beside full prepayment;
            # a rejected-prefix observation receives no reservation refund.
            good = {**MODULE.package_charged_upper(), **MODULE.selection_reservation(package=True),
                    'observed_counts': copy.deepcopy(observed), 'observation_pending': False}
            healthy = {}
            MODULE.record_counts(healthy, 'funded phase observation', good)
            self.assertEqual(healthy['core_achieved_phase_reserved_totals'], self.receiver())
            self.assertIsNone(healthy['core_achieved_phase_observed_operation_counts'])
            for shape in ('direct', 'nested'):
                unreserved = (copy.deepcopy(observed) if shape == 'direct' else {
                    'charged_fit_work_upper': observed['charged_fit_work_upper'],
                    'point_admissions_upper': observed['point_admissions_upper'],
                    'observed_counts': copy.deepcopy(observed), 'observation_pending': False})
                report = {}
                with self.subTest(text=text[:110], shape=shape):
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.record_counts(report, 'unreserved phase observation', unreserved)
                    self.assertFalse('core_invocations' in report, 'Unfunded phase observation was recorded.')


@contextlib.contextmanager
def historical_settled_source_observations():
    """Supply inert historical hash observations, never admit current Rust bytes.

    Only this nonexistent fixture namespace substitutes the hashing port. The
    real source checker, all frozen pins and every other path keep their original
    behavior. No local receipt, Git revision or historical source cache is read.
    """
    root = Path('/synthetic-settled-rate-source-observations')
    observations = {root / name: digest
                    for name, digest in MODULE.SETTLED_RATE_LIVE_SOURCE_SHA256.items()}
    actual_sha256 = MODULE.PRODUCT.sha256

    def observed_sha256(path):
        return observations[path] if path in observations else actual_sha256(path)

    with patch.object(MODULE.PRODUCT, 'sha256', side_effect=observed_sha256):
        yield root


class HistoricalFixtureIsolationTests(unittest.TestCase):
    """Historical source arithmetic must restore the current admission state."""

    def setUp(self):
        if set(MODULE.MUTATION_INVENTORIES) == set(MODULE.SETTLED_RATE_SUBJECTS):
            MODULE.bind_checkpoint_source_allowances()

    def snapshot(self):
        return {name: MODULE.COMMON.canonical_sha256(getattr(MODULE, name))
                for name in HistoricalS14cTestCase.ALLOWANCE_MAP_NAMES}

    def assert_current_sources_remain_held(self):
        """N1/N2 additions cannot inherit settled-rate source qualification."""
        reason = r'^Settled-rate live chart, receiver or unchanged adapter source differs\.$'
        with self.assertRaisesRegex(MODULE.Incomplete, reason):
            MODULE.admit_settled_rate_sources(ROOT)
        if set(MODULE.MUTATION_INVENTORIES) == set(MODULE.SETTLED_RATE_SUBJECTS):
            with self.assertRaisesRegex(MODULE.Incomplete, reason):
                MODULE.read_policy(ROOT, 'contracts')

    def test_current_allowances_survive_a_healthy_historical_case(self):
        before = self.snapshot()
        class HistoricalProbe(HistoricalS14cTestCase):
            def runTest(self):
                self.assertEqual(MODULE.SOURCE_FILE_COUNT, 156)
                self.assertEqual(MODULE.FIXED['schema_version'], 25)
                self.assertEqual(len(MODULE.cohort_budget_vectors()[0]), 2765)
        result = HistoricalProbe().run(unittest.TestResult())
        self.assertTrue(result.wasSuccessful())
        self.assertEqual(self.snapshot(), before)
        self.assertEqual(MODULE.FIXED['schema_version'], 26)
        self.assertEqual(MODULE.SOURCE_FILE_COUNT, 158)
        self.assertEqual(len(MODULE.cohort_budget_vectors(require_limits=False)[0]), 2820)
        with historical_settled_source_observations() as root:
            MODULE.admit_settled_rate_sources(root)
        self.assert_current_sources_remain_held()

    def test_current_allowances_survive_historical_exception_cleanup(self):
        before = self.snapshot()
        class HistoricalFailure(HistoricalS14cTestCase):
            def runTest(self):
                self.assertEqual(MODULE.FIXED['schema_version'], 25)
                raise RuntimeError('intentional inert historical failure')
        result = HistoricalFailure().run(unittest.TestResult())
        self.assertEqual(len(result.errors), 1)
        self.assertIn('intentional inert historical failure', result.errors[0][1])
        self.assertEqual(self.snapshot(), before)
        self.assertEqual(MODULE.FIXED['schema_version'], 26)
        with historical_settled_source_observations() as root:
            MODULE.admit_settled_rate_sources(root)
        self.assert_current_sources_remain_held()

    def test_current_historical_current_projection_preserves_original_sources(self):
        """Historical fixture projection leaves the held current tree unchanged."""
        before_bytes = (ROOT / MODULE.POLICY).read_bytes()
        before_policy = json.loads(before_bytes)
        before_product = MODULE.PRODUCT.admit_product(ROOT)[1]
        self.assert_current_sources_remain_held()
        with historical_settled_source_observations() as root:
            before_sources = MODULE.admit_settled_rate_sources(root)
        result = CoreAnalysisTests(
            'test_historical_fixture_survives_successor_template_between_healthy_templates'
        ).run(unittest.TestResult())
        after_policy = json.loads((ROOT / MODULE.POLICY).read_bytes())
        self.assertEqual((before_policy['schema_version'], after_policy['schema_version']), (26, 26))
        self.assertEqual(before_policy, after_policy)
        self.assertEqual((ROOT / MODULE.POLICY).read_bytes(), before_bytes)
        self.assertEqual(MODULE.PRODUCT.admit_product(ROOT)[1], before_product)
        with historical_settled_source_observations() as root:
            self.assertEqual(MODULE.admit_settled_rate_sources(root), before_sources)
        self.assert_current_sources_remain_held()
        self.assertTrue(result.wasSuccessful(),
                        'Historical projection changed held current state: ' + str(result.errors))


class SettledRateIntegrationTests(unittest.TestCase):
    """Current finite-source admission is distinct from the qualified S1.4c recipe."""

    def test_current_package_adds_one_genuine_history_fixture(self):
        old = MODULE.achieved_phase_complete_budget(include_doctests=True)
        self.assertEqual((old['charged_fit_work_upper'], old['point_admissions_upper']),
                         (63129, 2253363))
        self.assertEqual(MODULE.package_charged_upper(include_doctests=True),
                         {'charged_fit_work_upper': 63501, 'point_admissions_upper': 2266209})

    def test_current_asymmetric_consumer_adds_no_fit_point_or_test_count(self):
        retained = MODULE.CHART_COMBINATION.fixture_delta_reservation(
            MODULE.chart_combination_work_owners(),
            MODULE.CHART_COMBINATION.achieved_phase_case_recipes()[MODULE.RATE_DELIVERY_FIXTURE_NAME],
            ordinary_outcomes=1)
        current = MODULE.rate_delivery_fixture_budget()
        delta = {key: current[key] - retained[key] for key in current}
        self.assertEqual(MODULE.CHART_COMBINATION.numeric_work(delta),
                         {'charged_fit_work_upper': 0, 'point_admissions_upper': 0})
        self.assertEqual(delta['chart_combination.fixture.charts'], 1)
        self.assertEqual(delta['chart_combination.fixture.prepares'], 2)
        self.assertEqual(delta['chart_combination.fixture.anchor_projections'], 2)
        self.assertEqual(delta['chart_combination.fixture.tracking_projections'], 0)
        self.assertEqual(delta['chart_combination.fixture.histories'], 0)
        self.assertEqual(delta['chart_combination.fixture.appends'], 0)
        self.assertEqual(len(MODULE.RATE_DELIVERY.CURRENT_ORDINARY_TESTS), 12)
        self.assertEqual(MODULE.FINAL_PACKAGE_TEST_COUNTS,
                         {'ordinary': 306, 'doctests': 36, 'baseline': 342})

    def setUp(self):
        fund_synthetic_source_recipes(self)
        counts = patch.dict(MODULE.FINAL_PACKAGE_TEST_COUNTS,
                            {'ordinary': 306, 'doctests': 36, 'baseline': 342})
        counts.start()
        self.addCleanup(counts.stop)

    def test_current_subjects_counts_and_fresh_inventory_are_closed(self):
        self.assertEqual(MODULE.SUBJECTS, {
            'reference_input': 'crates/time-core/src/reference_input.rs',
            'rate_delivery': 'crates/time-core/src/reference_input/rate_delivery.rs',
            'chart': 'crates/time-core/src/regression/chart.rs',
            'chart_roundoff': 'crates/time-core/src/regression/chart/roundoff.rs'})
        self.assertEqual(MODULE.SOURCE_FILE_COUNT, 158)
        self.assertEqual(MODULE.FINAL_PACKAGE_TEST_COUNTS, {'ordinary': 306, 'doctests': 36, 'baseline': 342})
        self.assertEqual(MODULE.FIXED['schema_version'], 26)
        self.assertIn(set(MODULE.MUTATION_INVENTORIES), (set(), set(MODULE.SETTLED_RATE_SUBJECTS)))
        self.assertEqual(MODULE.INVENTORY_ADMISSION, 'tasks/settled-rate-inventory-admission.json')
        self.assertEqual(MODULE.INVENTORY_FIXED['schema_version'], 17)
        self.assertEqual(MODULE.INVENTORY_FIXED['listing_maximum_total'], 2048)
        self.assertEqual(MODULE.COHORT_PLAN, 'tasks/settled-rate-cohort-plan.json')
        if not MODULE.MUTATION_INVENTORIES:
            self.assertIsNone(MODULE.CHECKPOINT_SOURCE_RESERVATION)
        else:
            self.assertEqual(len(MODULE.CHECKPOINT_SOURCE_RESERVATION['source_work']), 2820)

    def test_complete_vector_is_additive_and_legal_rate_namespace_is_zero(self):
        old = MODULE.achieved_phase_complete_budget(include_doctests=True)
        current = MODULE.settled_rate_complete_budget(include_doctests=True)
        fixture = MODULE.rate_delivery_fixture_budget()
        composition = MODULE.rate_delivery_composition_delta(include_doctests=True)
        self.assertEqual(len(current), 2820)
        self.assertEqual(set(current) - set(old), set(MODULE.RATE_DELIVERY.FIELDS))
        self.assertEqual(MODULE.COMMON.canonical_sha256(
            {key: current[key] for key in MODULE.RATE_DELIVERY.FIELDS}),
            '1d3d0f3c40ae37dac50bb333b7810dc0a97ccd7fa35bbef5789576afd8979db6')
        self.assertEqual(MODULE.COMMON.canonical_sha256(fixture),
            '555a3fb02e5e5261eb6f817b539cbd1df91e1aafde5f87e89a17927e6b937f6d')
        self.assertEqual({key: current[key] - old[key] for key in fixture}, fixture)
        self.assertEqual({key: current[key] - old[key] for key in composition}, composition)
        self.assertEqual(current['standard_errors.geometry.logical_bytes_upper']
                         - old['standard_errors.geometry.logical_bytes_upper'], 32 * 372)
        legal = MODULE.settled_rate_complete_budget(external_legal=True)
        self.assertEqual(len(legal), 2820)
        self.assertEqual((legal['charged_fit_work_upper'], legal['point_admissions_upper']), (685, 23679))
        self.assertTrue(all(legal[key] == 0 for key in MODULE.RATE_DELIVERY.FIELDS))

    def test_all_reached_rate_prefixes_need_complete_prepaid_funding(self):
        names = MODULE.RATE_DELIVERY.CURRENT_ORDINARY_TESTS
        streams = [''.join('test ' + name + ' ... ok\n' for name in names)]
        streams += ['test ' + names[0] + ' ... ' + status + '\n' for status in ('ok', 'FAILED', 'ignored')]
        for stream in streams:
            observation = MODULE.profile_counts(stream, require_fit=False, validate_runs_receipt=False)
            self.assertIsNone(observation['observed_operation_counts'])
            for nested in (False, True):
                with self.subTest(stream=stream[:90], nested=nested):
                    counts = ({**{key: observation[key] for key in MODULE.GLOBAL_LIMITS},
                               'observed_counts': observation} if nested else observation)
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.record_counts({}, 'unfunded-rate', counts)
            funded = {**MODULE.package_charged_upper(), **MODULE.selection_reservation(package=True),
                      'observed_counts': observation}
            report = {}
            MODULE.record_counts(report, 'funded-rate', funded)
            self.assertEqual(report['core_rate_delivery_reserved_totals'], MODULE.RATE_DELIVERY.package_overlay())

    def test_every_new_package_reservation_is_required_before_recording(self):
        healthy = {**MODULE.package_charged_upper(), **MODULE.selection_reservation(package=True)}
        MODULE.record_counts({}, 'healthy-prepaid-rate', healthy)
        for name in ('rate_delivery_reserved_budget', 'rate_delivery_fixture_reserved_budget',
                     'rate_delivery_composition_delta_reserved_budget', 'settled_rate_complete_package_reservation'):
            for poison in (None, True, -1, 1 << 63):
                value = dict(healthy)
                if poison is None:
                    value.pop(name)
                else:
                    vector = dict(value[name])
                    vector[next(iter(vector))] = poison
                    value[name] = vector
                with self.subTest(name=name, poison=poison), self.assertRaises(MODULE.Incomplete):
                    MODULE.record_counts({}, 'bad-precharge', value)

    def test_current_source_admission_checks_each_live_union_member(self):
        """Historical pins accept inert observations and reject every changed member."""
        with historical_settled_source_observations() as root:
            MODULE.admit_settled_rate_sources(root)
            for name in MODULE.SETTLED_RATE_LIVE_SOURCE_SHA256:
                original = MODULE.PRODUCT.sha256
                def changed(path, target=name):
                    return '0' * 64 if path == root / target else original(path)
                with self.subTest(name=name), patch.object(MODULE.PRODUCT, 'sha256', side_effect=changed), self.assertRaises(MODULE.Incomplete):
                    MODULE.admit_settled_rate_sources(root)

    def test_current_core_additions_cannot_enter_historical_settled_rate_admission(self):
        with self.assertRaisesRegex(MODULE.Incomplete,
                r'^Settled-rate live chart, receiver or unchanged adapter source differs\.$'):
            MODULE.admit_settled_rate_sources(ROOT)

    def test_literal_docs_and_source_upper_do_not_claim_observed_operations(self):
        output = ''.join('test ' + name + ' ... ok\n' for name in MODULE.RATE_DELIVERY.CURRENT_ORDINARY_TESTS)
        output += ''.join('test ' + name + ' ... ok\n' for name in MODULE.RATE_DELIVERY.DOCTESTS)
        summary = MODULE.profile_counts(output, require_fit=False, validate_runs_receipt=False,
                                        require_rate_delivery=True, require_rate_delivery_doctests=True)
        self.assertEqual(summary['rate_delivery_recorded_source_work_upper'],
                         {'charged_fit_work_upper': 372, 'point_admissions_upper': 12846})
        self.assertEqual(summary['rate_recorded_source_work_upper'], MODULE.RATE_DELIVERY.package_overlay(include_doctests=True))
        funded = {**MODULE.package_charged_upper(include_doctests=True),
                  **MODULE.selection_reservation(package=True, include_doctests=True), 'observed_counts': summary}
        report = {}
        MODULE.record_counts(report, 'literal-rate-docs', funded)
        self.assertEqual(report['core_settled_rate_complete_reserved_totals'],
                         MODULE.settled_rate_complete_budget(include_doctests=True))
        self.assertIsNone(report['core_rate_delivery_observed_operation_counts'])

    def test_reference_legal_consumer_precharges_all_axes_but_invokes_no_rate_type(self):
        report = {}
        def stop(label, _command, _timeout, **_options):
            self.assertEqual(label, 'contract-library')
            row = report['core_invocations'][0]
            self.assertEqual(row['settled_rate_complete_contract_reservation'],
                             MODULE.settled_rate_complete_budget(external_legal=True))
            self.assertEqual(row['rate_delivery_external_reserved_budget'], dict.fromkeys(MODULE.RATE_DELIVERY.FIELDS, 0))
            self.assertEqual((row['charged_fit_work_upper'], row['point_admissions_upper']), (685, 23679))
            raise MODULE.Incomplete('inert library stop')
        session = SimpleNamespace(stable=Path('/synthetic-stable'), run=stop)
        with self.assertRaisesRegex(MODULE.Incomplete, 'inert library stop'):
            MODULE.contracts(session, {'contracts_wall_seconds': 600, 'jobs': 1}, report)

    def test_missing_current_inventory_stops_before_session(self):
        for lane in ('contracts', 'coverage', 'mutation', 'cohort-union'):
            with self.subTest(lane=lane), patch.object(MODULE, 'MUTATION_INVENTORIES', {}), \
                    patch.object(MODULE.COMMON, 'Session') as session, self.assertRaises(MODULE.Incomplete):
                MODULE.read_policy(ROOT, lane)
            session.assert_not_called()

    def test_prospective_four_owner_scope_checks_all_new_axes_and_fresh_baseline(self):
        inventories = {name: {'count': 2, 'sha256': str(index) * 64}
                       for index, name in enumerate(MODULE.SUBJECTS, start=1)}
        with patch.object(MODULE, 'MUTATION_INVENTORIES', inventories), patch.dict(
                MODULE.FINAL_PACKAGE_TEST_COUNTS, {'ordinary': 306, 'doctests': 36, 'baseline': 342}):
            coverage = MODULE.prospective_scope_reservation('coverage', MODULE.SUBJECTS)
            mutation = MODULE.prospective_scope_reservation('mutation', MODULE.SUBJECTS)
            self.assertEqual(len(coverage['complete_source_work_upper']), 2820)
            self.assertEqual(coverage['package_test_count_upper_each'], 342)
            self.assertEqual(coverage['package_ordinary_test_count_upper_each'], 306)
            self.assertEqual(coverage['package_doctest_count_upper_each'], 36)
            self.assertEqual(coverage['charged_upper'], {'charged_fit_work_upper': 63501, 'point_admissions_upper': 2266206})
            self.assertEqual(mutation['package_invocations_upper'], 12)
            self.assertEqual(mutation['charged_upper'], {'charged_fit_work_upper': 12 * 63501, 'point_admissions_upper': 12 * 2266209})
            for mapping, prefix in ((MODULE.RATE_DELIVERY_TELEMETRY['global_limits'], ''),
                                    (MODULE.CHART_COMBINATION_TELEMETRY['global_limits'], ''),
                                    (MODULE.CHART_COMPOSITION_TELEMETRY['global_limits'], ''),
                                    (MODULE.GLOBAL_LIMITS, '')):
                key = next(key for key in mapping if coverage['complete_source_work_upper'].get(prefix + key, 0) > 0)
                with self.subTest(key=key), patch.dict(mapping, {key: coverage['complete_source_work_upper'][key] - 1}), \
                        patch.object(MODULE.COMMON, 'Session') as session, self.assertRaises(MODULE.Incomplete):
                    MODULE.prospective_scope_reservation('coverage', MODULE.SUBJECTS)
                session.assert_not_called()

    def test_rate_numeric_fixture_is_removed_once_before_legacy_doctest_cap(self):
        added = {'charged_fit_work_upper': 372, 'point_admissions_upper': 12846}
        baseline = {key: MODULE.LEGACY_PROFILE_LIMITS[key] - MODULE.LITERAL_DOCTEST_LIMITS[
            'stats_calls_upper' if key == 'charged_fit_work_upper' else 'point_admissions_upper'] for key in added}
        counts = {key: baseline[key] + added[key] for key in added}
        counts.update({'legacy_' + key: value for key, value in baseline.items()})
        counts['internal_fit_upper_bound'] = counts['charged_fit_work_upper']
        counts['rate_delivery_recorded_source_work_upper'] = added
        observed = MODULE.account_literal_doctests(counts)
        docs = MODULE.ADAPTIVE.literal_doctest_reservation()
        for key in added:
            self.assertEqual(observed[key], MODULE.LEGACY_PROFILE_LIMITS[key] + added[key] + docs[key])

    def test_current_cohort_hook_binds_342_and_every_new_recipe_input(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            path = root / MODULE.COHORT_PLAN
            path.parent.mkdir(parents=True)
            path.write_text('{}\n')
            policy_path = root / MODULE.POLICY
            policy_path.parent.mkdir(parents=True)
            policy_path.write_text('{}\n')
            policy = {'inputs_sha256': {'inert.rs': '1' * 64}, 'mutation_equivalences': [],
                      'mutation_inventories': {name: {'count': 2, 'sha256': '2' * 64} for name in MODULE.SUBJECTS}}
            canonical = MODULE.COMMON.canonical_sha256
            recipes = []
            def observe(value):
                if type(value) is dict and 'checkpoint_attempt_scope' in value:
                    recipes.append(copy.deepcopy(value))
                return canonical(value)
            with patch.object(MODULE, 'read_policy', return_value=policy), \
                    patch.object(MODULE, 'inventory_tool_inputs', return_value=set()), \
                    patch.dict(MODULE.FINAL_PACKAGE_TEST_COUNTS, {'ordinary': 306, 'doctests': 36, 'baseline': 342}), \
                    patch.object(MODULE.COMMON, 'canonical_sha256', side_effect=observe), \
                    patch.object(MODULE.COHORTS, 'admit_plan', return_value={'inert': True}) as receiver:
                MODULE.read_cohort_plan(root, path, policy)
            args = receiver.call_args.kwargs
            self.assertEqual(args['baseline_tests'], 342)
            self.assertEqual(args['subjects'], MODULE.SETTLED_RATE_SUBJECTS)
            self.assertEqual([len(row) for row in (args['package_budget'], *args['check_budgets'].values(), args['global_limits'])], [2820] * 4)
            self.assertEqual(len(recipes), 1)
            recipe = recipes[0]
            self.assertEqual(recipe['rate_delivery_case_recipes'], MODULE.RATE_DELIVERY.case_recipes())
            self.assertEqual(recipe['rate_delivery_literal_doctests'], list(MODULE.RATE_DELIVERY.DOCTESTS))
            self.assertEqual(recipe['rate_delivery_sources'], MODULE.RATE_DELIVERY.SOURCE_SHA256)
            self.assertEqual(recipe['settled_rate_live_sources'], MODULE.SETTLED_RATE_LIVE_SOURCE_SHA256)
            self.assertEqual(recipe['rate_delivery_fixture'], MODULE.rate_delivery_fixture_budget())
            self.assertEqual(recipe['settled_rate_package'], MODULE.settled_rate_complete_budget(include_doctests=True))
            for field in ('rate_delivery', 'rate_delivery_case_recipes', 'rate_delivery_literal_doctests',
                          'rate_delivery_sources', 'settled_rate_live_sources', 'rate_delivery_fixture',
                          'rate_delivery_composition_delta', 'settled_rate_package', 'settled_rate_external'):
                bad = dict(recipe)
                del bad[field]
                with self.subTest(field=field):
                    self.assertNotEqual(canonical(bad), args['recipe_sha256'])

    def test_current_mutation_baseline_requires_342_before_profile_receiving(self):
        helper = CoreAnalysisTests()
        outcome = {'scenario': 'Baseline', 'summary': 'Success', 'log_path': 'baseline.log',
                   'phase_results': helper.phase_results('Success')}
        original = helper.completed_mutation_log('')
        class ReachedProfile(Exception):
            """Stop at the actual Python profile receiver; execute no command."""
        with tempfile.TemporaryDirectory() as temporary:
            output = Path(temporary)
            for count in (325, 336, 341, 342):
                (output / 'baseline.log').write_text(original.replace('96 passed', str(count) + ' passed'))
                report = {}
                MODULE.record_counts(report, 'rate_delivery:mutation-baseline', {
                    **MODULE.package_charged_upper(include_doctests=True),
                    **MODULE.selection_reservation(package=True, include_doctests=True),
                    'observation_pending': True})
                row = report['core_invocations'][0]
                with self.subTest(count=count), patch.object(MODULE, 'profile_counts', side_effect=ReachedProfile) as receive:
                    if count == 342:
                        with self.assertRaises(ReachedProfile):
                            MODULE.observe_mutation_outcome(report, output / 'outcomes.json',
                                'rate_delivery:mutation-baseline', outcome, row)
                        self.assertTrue(receive.call_args.kwargs['require_rate_delivery'])
                        self.assertTrue(receive.call_args.kwargs['require_rate_delivery_doctests'])
                    else:
                        with self.assertRaisesRegex(MODULE.Incomplete, 'all 342 admitted'):
                            MODULE.observe_mutation_outcome(report, output / 'outcomes.json',
                                'rate_delivery:mutation-baseline', outcome, row)
                        receive.assert_not_called()
                self.assertTrue(row['observation_pending'])
                self.assertEqual(len(row['settled_rate_complete_package_reservation']), 2820)


class SettledRateCompilerDiagnosticTests(unittest.TestCase):
    """Observed syntax failures are Unviable, without admitting unrelated failures."""

    FILE = 'crates/time-core/src/reference_input/rate_delivery.rs'

    @classmethod
    def null_let_chain_case(cls, line=570):
        """Literal native570/575 diagnostics, relocated to the inert owned worker.

        The V3 Rate campaign emitted code:null for &&→|| in known_relationship.
        These are compiler syntax failures, with no product test execution.
        """
        helper = CoreAnalysisTests()
        source = 'active' if line == 570 else 'effective'
        byte_start = 22350 if line == 570 else 22510
        record = helper.semantic_record()
        record['message'].update(
            code=None, message='`||` operators are not supported in let chain conditions',
            rendered='error: `||` operators are not supported in let chain conditions',
            spans=[{'file_name': cls.FILE, 'byte_start': byte_start, 'byte_end': byte_start + 2,
                    'line_start': line, 'line_end': line, 'column_start': 9, 'column_end': 11,
                    'is_primary': True, 'text': [{
                        'text': '        || /* ~ changed by cargo-mutants ~ */ interval.span.' + source + '.0 < last',
                        'highlight_start': 9, 'highlight_end': 11}],
                    'label': None, 'suggested_replacement': None,
                    'suggestion_applicability': None, 'expansion': None}])
        mutant = {
            'file': cls.FILE, 'package': 'time-core', 'genre': 'BinaryOperator', 'replacement': '||',
            'name': cls.FILE + ':' + str(line) + ':9: replace && with || in known_relationship',
            'function': {'function_name': 'known_relationship',
                         'return_type': '-> Result<(), FixtureRateDeliveryError>',
                         'span': {'start': {'line': 550, 'column': 1}, 'end': {'line': 580, 'column': 2}}},
            'span': {'start': {'line': line, 'column': 9}, 'end': {'line': line, 'column': 11}},
        }
        outcome = {'scenario': {'Mutant': mutant}, 'summary': 'Unviable',
                   'phase_results': helper.phase_results('Unviable')}
        return outcome, record

    @staticmethod
    def compiler_log(record, *, repeated=False):
        """Keep one actual Build segment;575 emitted the same error twice."""
        diagnostics = (json.dumps(record) + '\n') * (2 if repeated else 1)
        return (CoreAnalysisTests.mutation_phase_header('Build') + diagnostics
                + '{"reason":"build-finished","success":false}\n')

    def test_observed_null_code_let_chain_compile_errors_are_unviable(self):
        for line in (570, 575):
            outcome, record = self.null_let_chain_case(line)
            with self.subTest(line=line):
                MODULE.unviable_compile_failure(
                    outcome, self.compiler_log(record, repeated=line == 575))

    def test_null_code_let_chain_keeps_exact_mutant_diagnostic_and_phase_ownership(self):
        for change in ('message', 'unknown_code', 'foreign_file', 'wrong_function', 'wrong_genre',
                       'wrong_replacement', 'wrong_mutant_line', 'wrong_mutant_column',
                       'wrong_mutant_end', 'wrong_primary_line', 'wrong_primary_column',
                       'wrong_primary_end', 'foreign_primary', 'nonprimary', 'wrong_source_clause', 'wrong_return_type', 'wrong_function_span',
                       'package', 'manifest', 'target', 'warning', 'nested_error',
                       'infrastructure', 'build_success', 'success_phase', 'test_phase'):
            outcome, record = self.null_let_chain_case()
            mutant = outcome['scenario']['Mutant']
            span = record['message']['spans'][0]
            if change == 'message':
                record['message']['message'] += ' unrelated cause'
            elif change == 'unknown_code':
                record['message']['code'] = {'code': 'unreviewed_let_chain', 'explanation': None}
            elif change == 'foreign_file':
                mutant['file'] = 'crates/time-core/src/reference_input.rs'
            elif change == 'wrong_function':
                mutant['function']['function_name'] = 'settlement_parts'
            elif change == 'wrong_genre':
                mutant['genre'] = 'FnValue'
            elif change == 'wrong_replacement':
                mutant['replacement'] = '&&'
            elif change == 'wrong_mutant_line':
                mutant['span']['start']['line'] = 571
            elif change == 'wrong_mutant_column':
                mutant['span']['start']['column'] = 8
            elif change == 'wrong_mutant_end':
                mutant['span']['end']['column'] = 12
            elif change == 'wrong_primary_line':
                span['line_start'] = span['line_end'] = 575
            elif change == 'wrong_primary_column':
                span['column_start'] = 8
            elif change == 'wrong_primary_end':
                span['column_end'] = 12
            elif change == 'foreign_primary':
                span['file_name'] = 'crates/time-core/src/reference_input.rs'
            elif change == 'nonprimary':
                span['is_primary'] = False
            elif change == 'wrong_source_clause':
                span['text'][0]['text'] = span['text'][0]['text'].replace('active', 'effective')
            elif change == 'wrong_return_type':
                mutant['function']['return_type'] = '-> bool'
            elif change == 'wrong_function_span':
                mutant['function']['span']['end']['line'] = 581
            elif change == 'package':
                record['package_id'] = 'registry+https://example.invalid/dependency#1.0.0'
            elif change == 'manifest':
                record['manifest_path'] = '/work/tmp/cargo-mutants-source-v2TRlL.tmp/foreign/Cargo.toml'
            elif change == 'target':
                record['target']['kind'] = ['test']
            elif change == 'warning':
                record['message']['level'] = 'warning'
            elif change == 'nested_error':
                record['message']['children'] = [{'message': 'unrelated compiler error',
                    'level': 'error', 'children': [], 'code': None, 'spans': []}]
            elif change == 'success_phase':
                outcome['phase_results'][0]['process_status'] = 'Success'
            elif change == 'test_phase':
                outcome['phase_results'][0]['phase'] = 'Test'
            output = self.compiler_log(record)
            if change == 'infrastructure':
                output += 'error: could not execute process: Resource temporarily unavailable (os error 11)\n'
            elif change == 'build_success':
                output = output.replace('"success":false', '"success":true')
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.unviable_compile_failure(outcome, output)

    def test_unknown_null_and_coded_compiler_neighbors_keep_their_prior_rules(self):
        helper = CoreAnalysisTests()
        outcome = {'summary': 'Unviable', 'phase_results': helper.phase_results('Unviable')}
        # Existing owned E0308 remains accepted beside the new narrow null-code case.
        MODULE.unviable_compile_failure(outcome, helper.semantic_mutation_log())
        for code, message in ((None, 'an unrelated compiler syntax error'),
                              (None, '`||` operators are not supported in unrelated conditions'),
                              ({'code': 'unreviewed_error', 'explanation': None}, 'unrelated error')):
            record = helper.semantic_record()
            record['message'].update(code=code, message=message)
            with self.subTest(code=code, message=message), self.assertRaises(MODULE.Incomplete):
                MODULE.unviable_compile_failure(outcome, helper.semantic_mutation_log(record))


class SettledRateAccountingRefreshTests(unittest.TestCase):
    """Per-refresh source arithmetic must preserve totals and observe changed inputs."""

    @staticmethod
    def report_fixture():
        """Three inert schema-complete rows isolate arithmetic, not source admission.

        Each row reserves two direct composition attempts, three delegated
        achieved-phase attempts and one receiver/fit unit. Two rows request
        the documented doctest deltas. No process or actual work is observed.
        """
        complete = dict.fromkeys(MODULE.achieved_phase_complete_budget(), 0)
        complete['charged_fit_work_upper'] = 1
        rows = []
        for label, docs in (('doc-first', True), ('ordinary', False), ('doc-last', True)):
            scalar = dict.fromkeys(MODULE.CHART.COMPOSITION_FIELDS, 0)
            scalar['chart_composition.composition_attempts'] = 2
            phase_scalar = dict.fromkeys(MODULE.CHART.COMPOSITION_FIELDS, 0)
            phase_scalar['chart_composition.composition_attempts'] = 3
            receiver = dict.fromkeys(MODULE.CHART.ACHIEVED_PHASE_FIELDS, 0)
            receiver['achieved_phase.receiver_attempts'] = 1
            rows.append({
                'invocation': label, 'selection_expected': 'package', 'literal_doctests_reserved': docs,
                'chart_composition_reserved_budget': scalar,
                'chart_composition_fixture_reserved_budget': dict.fromkeys(MODULE.CHART_COMBINATION.FIELDS, 0),
                'achieved_phase_reserved_budget': receiver,
                'achieved_phase_fixture_reserved_budget': dict.fromkeys(MODULE.CHART_COMBINATION.FIELDS, 0),
                'achieved_phase_composition_delta_reserved_budget': phase_scalar,
                'achieved_phase_complete_package_reservation': dict(complete),
            })
        return {'core_invocations': rows,
                'core_chart_combination_reserved_totals': dict.fromkeys(MODULE.CHART_COMBINATION.FIELDS, 0)}

    def test_refresh_preserves_disjoint_row_sums_doc_deltas_and_idempotence(self):
        report = self.report_fixture()
        inputs = copy.deepcopy(report['core_invocations'])
        MODULE.refresh_chart_composition_accounting(report)
        scalar = dict.fromkeys(MODULE.CHART.COMPOSITION_FIELDS, 0)
        scalar.update({'chart_composition.composition_attempts': 6,
                       'chart_composition.old_affine_additional_callbacks': 4,
                       'chart_composition.old_affine_additional_scalar_predicates': 8})
        phase_scalar = dict.fromkeys(MODULE.CHART.COMPOSITION_FIELDS, 0)
        phase_scalar['chart_composition.composition_attempts'] = 9
        receiver = dict.fromkeys(MODULE.CHART.ACHIEVED_PHASE_FIELDS, 0)
        receiver.update({'achieved_phase.receiver_attempts': 3,
                         'achieved_phase.additional_doctest_compilations': 6,
                         'achieved_phase.additional_doctest_compile_fail_cases': 4,
                         'achieved_phase.additional_uncalled_legal_helpers': 2})
        self.assertEqual(report['core_chart_composition_reserved_totals'], scalar)
        self.assertEqual(report['core_achieved_phase_composition_delta_reserved_totals'], phase_scalar)
        self.assertEqual(report['core_achieved_phase_reserved_totals'], receiver)
        self.assertEqual(report['core_chart_composition_complete_reserved_totals'],
                         {key: scalar[key] + phase_scalar[key] for key in scalar})
        complete = report['core_achieved_phase_complete_reserved_totals']
        self.assertEqual({key: value for key, value in complete.items() if value}, {'charged_fit_work_upper': 3})
        self.assertEqual(report['core_chart_combination_complete_reserved_totals'],
                         dict.fromkeys(MODULE.CHART_COMBINATION.FIELDS, 0))
        self.assertEqual(report['core_achieved_phase_source_test_pending_invocations'],
                         ['doc-first', 'ordinary', 'doc-last'])
        self.assertIsNone(report['core_chart_composition_legal_observed_calls'])
        self.assertIsNone(report['core_achieved_phase_observed_operation_counts'])
        self.assertIsNone(report['core_rate_delivery_observed_operation_counts'])
        self.assertEqual(report['core_invocations'], inputs)
        first = copy.deepcopy(report)
        MODULE.refresh_chart_composition_accounting(report)
        self.assertEqual(report, first)

    def test_each_refresh_rebuilds_source_recipe_and_reads_current_allowances(self):
        report = self.report_fixture()
        source = MODULE.CHART.achieved_phase_package_reservation
        current = {'extra_doc_compilations': 1}
        def changed_source(*, include_doctests=False):
            result = source(include_doctests=include_doctests)
            if include_doctests:
                result['achieved_phase.additional_doctest_compilations'] += current['extra_doc_compilations']
            return result
        key = 'achieved_phase.additional_doctest_compilations'
        # A prospective source recipe changes between calls through the same
        # helper identity. A persistent memoized tuple would hide this change.
        with patch.object(MODULE.CHART, 'achieved_phase_package_reservation', side_effect=changed_source):
            MODULE.refresh_chart_composition_accounting(report)
            self.assertEqual(report['core_achieved_phase_reserved_totals'][key], 8)
            current['extra_doc_compilations'] = 2
            MODULE.refresh_chart_composition_accounting(report)
            self.assertEqual(report['core_achieved_phase_reserved_totals'][key], 10)
            limits = MODULE.ACHIEVED_PHASE_TELEMETRY['global_limits']
            with patch.dict(limits, {key: 9}), self.assertRaisesRegex(
                    MODULE.Incomplete, 'Achieved-phase cumulative receiver source reservation is exceeded'):
                MODULE.refresh_chart_composition_accounting(report)
            MODULE.refresh_chart_composition_accounting(report)
            self.assertEqual(report['core_achieved_phase_reserved_totals'][key], 10)
            current['extra_doc_compilations'] = 1
            MODULE.refresh_chart_composition_accounting(report)
            self.assertEqual(report['core_achieved_phase_reserved_totals'][key], 8)


class NativeReceiptVectorTransportTests(unittest.TestCase):
    """Lossless transport controls on synthetic reports; no native-result claims."""

    FIELDS = ('settled_rate_complete_package_reservation',
              'achieved_phase_complete_package_reservation')
    MARKER = 'nevrotick_settled_rate_source_vectors'
    REFERENCE = 'source_vector_sha256'
    CAP = 64 * 1024 * 1024

    @classmethod
    def setUpClass(cls):
        # Actual finite source recipes give realistic key/value widths. These
        # reports are transport fixtures, never admitted campaign evidence.
        cls.rate = MODULE.settled_rate_complete_budget(include_doctests=True)
        cls.phase = MODULE.achieved_phase_complete_budget(include_doctests=True)
        cls.rate_plain = MODULE.settled_rate_complete_budget()
        cls.phase_plain = MODULE.achieved_phase_complete_budget()
        cls.extra = MODULE.rate_delivery_fixture_budget()
        cls.rate_totals = MODULE.RATE_DELIVERY.package_overlay()
        assert len(cls.rate) == 2820 and len(cls.phase) == 2765 and len(cls.extra) == 486

    @classmethod
    def report(cls, count=1, *, alternate=False, extra_fixture=False):
        rows = []
        for index in range(count):
            docs = not alternate or index % 2 == 0
            row = {
                'invocation': 'synthetic-transport-' + str(index),
                cls.FIELDS[0]: cls.rate if docs else cls.rate_plain,
                cls.FIELDS[1]: cls.phase if docs else cls.phase_plain,
                'observation_pending': True,
                'observed_counts': {'synthetic_transport_only': True, 'observed_operation_counts': None},
            }
            if extra_fixture:
                row['rate_delivery_fixture_reserved_budget'] = cls.extra
            rows.append(row)
        return {'status': 'synthetic-transport-fixture', 'selected_lane': 'mutation',
                'qualification_scope': 'Synthetic bytes only; no native or application evidence.',
                'core_accounting_schema_version': 3,
                'core_rate_delivery_reserved_totals': dict(cls.rate_totals),
                'core_invocations': rows, 'observed_operation_counts': None}

    @staticmethod
    def digest(value):
        return hashlib.sha256(json.dumps(
            value, sort_keys=True, separators=(',', ':'), allow_nan=False).encode()).hexdigest()

    @staticmethod
    def read_wire(wire):
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / 'verification.json'
            path.write_text(json.dumps(wire, separators=(',', ':'), allow_nan=False) + '\n')
            return MODULE.native_receipt_document(path)

    def test_actual_writer_and_file_reader_restore_200_source_vector_rows(self):
        report = self.report(200)
        original = copy.deepcopy(report)
        vector_bytes = sum(len(json.dumps(vector, separators=(',', ':')).encode())
                           for vector in (self.rate, self.phase))
        self.assertGreater(200 * vector_bytes, self.CAP)
        text = MODULE.native_receipt_text(report)
        self.assertLessEqual(len(text.encode()), self.CAP)
        wire = json.loads(text)
        self.assertEqual(set(wire), {self.MARKER, 'vectors', 'receipt'})
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / 'verification.json'
            path.write_text(text)
            restored = MODULE.native_receipt_document(path)
        self.assertTrue(MODULE.exact(restored, original))
        self.assertTrue(MODULE.exact(report, original))
        first, second = restored['core_invocations'][:2]
        self.assertIsNot(first[self.FIELDS[0]], second[self.FIELDS[0]])
        self.assertIsNot(first[self.FIELDS[0]], wire['vectors'][wire['receipt']['core_invocations'][0][self.FIELDS[0]][self.REFERENCE]])
        key = next(iter(first[self.FIELDS[0]]))
        first[self.FIELDS[0]][key] += 1
        self.assertEqual(second[self.FIELDS[0]], self.rate)
        self.assertEqual(report['core_invocations'][0][self.FIELDS[0]], self.rate)

    def test_representative_134_current_rows_fit_before_any_native_work(self):
        # Match a current full row's source-field shape, without dispatching
        # accounting, command execution, test observations or policy admission.
        template = {**MODULE.package_charged_upper(include_doctests=True),
                    **MODULE.selection_reservation(package=True, include_doctests=True),
                    'observation_pending': True,
                    'execution': 'synthetic prospective transport preflight only'}
        row_bytes = len(json.dumps(template, separators=(',', ':')).encode())
        self.assertGreater(134 * row_bytes, self.CAP)
        report = self.report(134)
        report['core_invocations'] = [
            {**template, 'invocation': 'synthetic-full-row-' + str(index)} for index in range(134)]
        text = MODULE.native_receipt_text(report)
        self.assertLessEqual(len(text.encode()), self.CAP)
        self.assertIn(self.MARKER, json.loads(text))
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / 'verification.json'
            path.write_text(text)
            restored = MODULE.native_receipt_document(path)
        self.assertTrue(MODULE.exact(restored, report))
        self.assertTrue(all(row['observation_pending'] for row in restored['core_invocations']))

    def test_small_legacy_bytes_and_unrelated_oversize_guard_are_preserved(self):
        legacy = {'status': 'incomplete', 'legacy': {'count': 1, 'flag': True,
                  'value': 0.5, 'nothing': None}, 'rows': []}
        expected = '{"status":"incomplete","legacy":{"count":1,"flag":true,"value":0.5,"nothing":null},"rows":[]}\n'
        self.assertEqual(MODULE.native_receipt_text(legacy), expected)
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / 'legacy.json'
            path.write_text(expected)
            self.assertTrue(MODULE.exact(MODULE.native_receipt_document(path), legacy))
        report = self.report()
        report['uncompacted_payload'] = 'x' * self.CAP
        with self.assertRaisesRegex(MODULE.Incomplete, 'fixed byte allowance'):
            MODULE.native_receipt_text(report)

    def test_four_vector_table_is_self_contained_and_only_two_fields_are_references(self):
        report = self.report(2, alternate=True, extra_fixture=True)
        original = copy.deepcopy(report)
        wire = MODULE.pack_receipt_vectors(report)
        self.assertEqual(set(wire), {self.MARKER, 'vectors', 'receipt'})
        self.assertIs(type(wire[self.MARKER]), int)
        self.assertEqual(wire[self.MARKER], 1)
        self.assertEqual(len(wire['vectors']), 4)
        self.assertTrue(MODULE.exact(report, original))
        for source, row in zip(original['core_invocations'], wire['receipt']['core_invocations'], strict=True):
            for field in self.FIELDS:
                self.assertEqual(row[field], {self.REFERENCE: self.digest(source[field])})
                self.assertEqual(wire['vectors'][row[field][self.REFERENCE]], source[field])
            self.assertEqual(row['rate_delivery_fixture_reserved_budget'], source['rate_delivery_fixture_reserved_budget'])
            self.assertEqual(row['observed_counts'], source['observed_counts'])
        # A historical receipt must not be rebuilt from mutable current recipes.
        with patch.object(MODULE, 'settled_rate_complete_budget', side_effect=AssertionError('current recipe consulted')), \
             patch.object(MODULE, 'achieved_phase_complete_budget', side_effect=AssertionError('current recipe consulted')), \
             patch.object(MODULE.RATE_DELIVERY, 'package_overlay', side_effect=AssertionError('current recipe consulted')):
            restored = self.read_wire(wire)
        self.assertTrue(MODULE.exact(restored, original))

    def test_file_reader_rejects_closed_schema_reference_table_and_digest_tampering(self):
        original = MODULE.pack_receipt_vectors(self.report())
        for change in ('extra_wrapper', 'missing_vectors', 'missing_receipt', 'boolean_schema',
                       'unknown_schema', 'unknown_ref', 'missing_ref', 'extra_ref_key',
                       'bad_digest', 'unused_vector', 'fifth_vector', 'wrong_width', 'outside_ref'):
            wire = copy.deepcopy(original)
            row = wire['receipt']['core_invocations'][0]
            digest = row[self.FIELDS[0]][self.REFERENCE]
            if change == 'extra_wrapper':
                wire['unreviewed'] = True
            elif change == 'missing_vectors':
                del wire['vectors']
            elif change == 'missing_receipt':
                del wire['receipt']
            elif change == 'boolean_schema':
                wire[self.MARKER] = True
            elif change == 'unknown_schema':
                wire[self.MARKER] = 2
            elif change == 'unknown_ref':
                row[self.FIELDS[0]][self.REFERENCE] = '0' * 64
            elif change == 'missing_ref':
                row[self.FIELDS[0]] = {}
            elif change == 'extra_ref_key':
                row[self.FIELDS[0]]['unreviewed'] = 0
            elif change == 'bad_digest':
                vector = wire['vectors'][digest]
                vector[next(iter(vector))] += 1
            elif change in ('unused_vector', 'fifth_vector'):
                for amount in range(1, 2 if change == 'unused_vector' else 4):
                    extra = dict(wire['vectors'][digest])
                    extra[next(iter(extra))] += amount
                    wire['vectors'][self.digest(extra)] = extra
            elif change == 'wrong_width':
                row[self.FIELDS[0]] = dict(row[self.FIELDS[1]])
                wire['receipt']['core_invocations'].append({
                    'invocation': 'synthetic-reversed-width',
                    self.FIELDS[0]: {self.REFERENCE: digest},
                    self.FIELDS[1]: {self.REFERENCE: digest}})
            elif change == 'outside_ref':
                row['unreviewed_package_reservation'] = {self.REFERENCE: digest}
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                self.read_wire(wire)

    def test_vector_integer_key_and_width_rules_have_valid_neighbor_boundaries(self):
        for bad in (True, False, -1, 1 << 63, 1.0, '1', None):
            report = copy.deepcopy(self.report())
            report['core_invocations'][0][self.FIELDS[0]][next(iter(self.rate))] = bad
            with self.subTest(pack_bad=bad), self.assertRaises(MODULE.Incomplete):
                MODULE.pack_receipt_vectors(report)
            # Recompute the forged table digest: rejection must reach the exact
            # integer rule rather than succeed merely because of a stale SHA.
            wire = MODULE.pack_receipt_vectors(self.report())
            row = wire['receipt']['core_invocations'][0]
            old = row[self.FIELDS[0]][self.REFERENCE]
            vector = wire['vectors'].pop(old)
            vector[next(iter(vector))] = bad
            digest = self.digest(vector)
            wire['vectors'][digest] = vector
            row[self.FIELDS[0]][self.REFERENCE] = digest
            with self.subTest(decoded_bad=bad), self.assertRaises(MODULE.Incomplete):
                self.read_wire(wire)
        for change in ('long_key', 'nonascii_key', 'integer_key', 'missing_field', 'extra_field'):
            report = copy.deepcopy(self.report())
            vector = report['core_invocations'][0][self.FIELDS[0]]
            key = next(iter(vector))
            if change == 'long_key':
                vector['a' * 257] = vector.pop(key)
            elif change == 'nonascii_key':
                vector['unit.non-ascii-é'] = vector.pop(key)
            elif change == 'integer_key':
                vector[17] = vector.pop(key)
            elif change == 'missing_field':
                del vector[key]
            elif change == 'extra_field':
                vector['unit.unreviewed'] = 0
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.pack_receipt_vectors(report)
        neighbor = copy.deepcopy(self.report())
        vector = neighbor['core_invocations'][0][self.FIELDS[0]]
        key = next(iter(vector))
        vector['a' * 256] = (1 << 63) - 1
        del vector[key]
        self.assertTrue(MODULE.exact(self.read_wire(MODULE.pack_receipt_vectors(neighbor)), neighbor))

    def test_row_and_whole_expansion_limits_count_uncompacted_fixture_entries(self):
        healthy = MODULE.pack_receipt_vectors(self.report(516))
        self.assertEqual(len(healthy['receipt']['core_invocations']), 516)
        with self.assertRaisesRegex(MODULE.Incomplete, 'invocation count'):
            MODULE.pack_receipt_vectors(self.report(517))
        too_many = copy.deepcopy(MODULE.pack_receipt_vectors(self.report()))
        too_many['receipt']['core_invocations'] *= 517
        with self.assertRaisesRegex(MODULE.Incomplete, 'invocation count'):
            self.read_wire(too_many)
        with self.assertRaisesRegex(MODULE.Incomplete, 'expanded JSON-entry allowance'):
            MODULE.pack_receipt_vectors(self.report(516, extra_fixture=True))

    def test_json_depth_and_nested_envelope_bounds_keep_a_healthy_neighbor(self):
        for depth in (64, 65):
            report = self.report()
            nested = 0
            for _ in range(depth):
                nested = [nested]
            report['synthetic_nested_metadata'] = nested
            if depth == 64:
                self.assertTrue(MODULE.exact(self.read_wire(MODULE.pack_receipt_vectors(report)), report))
            else:
                with self.assertRaisesRegex(MODULE.Incomplete, 'JSON depth'):
                    MODULE.pack_receipt_vectors(report)
                wire = MODULE.pack_receipt_vectors(self.report())
                wire['receipt']['synthetic_nested_metadata'] = nested
                with self.assertRaisesRegex(MODULE.Incomplete, 'JSON depth'):
                    self.read_wire(wire)
        wire = MODULE.pack_receipt_vectors(self.report())
        wire['receipt']['synthetic_nested_metadata'] = {self.MARKER: 1}
        with self.assertRaisesRegex(MODULE.Incomplete, 'Nested source-vector envelope'):
            self.read_wire(wire)

    def test_only_current_mutation_role_uses_envelopes_and_inventory_role_stays_separate(self):
        for change in ('lane', 'boolean_schema', 'unknown_schema', 'missing_rate_totals', 'missing_rows'):
            report = self.report()
            if change == 'lane':
                report['selected_lane'] = 'contracts'
            elif change == 'boolean_schema':
                report['core_accounting_schema_version'] = True
            elif change == 'unknown_schema':
                report['core_accounting_schema_version'] = 4
            elif change == 'missing_rate_totals':
                del report['core_rate_delivery_reserved_totals']
            elif change == 'missing_rows':
                del report['core_invocations']
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.pack_receipt_vectors(report)
        wire = MODULE.pack_receipt_vectors(self.report())
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / 'transport.json'
            path.write_text(json.dumps(wire, separators=(',', ':')) + '\n')
            self.assertLess(path.stat().st_size, 512 * 1024)
            self.assertTrue(MODULE.exact(MODULE.inventory_document(path), wire))
            self.assertTrue(MODULE.exact(MODULE.native_receipt_document(path), self.report()))
        path = Mock()
        with self.assertRaises(MODULE.Incomplete):
            MODULE._core_json_document(path, role='native')
        path.resolve.assert_not_called()
        path.open.assert_not_called()


class SettledRateAssertionTransportTests(unittest.TestCase):
    """Source-owned Rust panic rendering is raw Test text, never Cargo evidence."""

    # Captured from the real chart mutant at79:42. The complete assertion is
    # owned by tests/private/chart_roundoff.rs:471; no retained file is required
    # to execute these independently expected transport controls.
    ASSERTION = '\n'.join((
        'assertion failed: matches!(settled_effect(rate, 0, duration),',
        '    Err(FixtureChartError::Arithmetic',
        '    {',
        '        step: FixtureChartStep::RateTimesElapsed, source: FitError::Arithmetic',
        '        {',
        '            stage: ArithmeticStage::Response, operation:',
        '            ArithmeticOperation::Multiply, issue: actual',
        '        }',
        '    }) if actual == issue)',
    )) + '\n'
    FINISHED = '{"reason":"build-finished","success":true}\n'

    def test_complete_settled_rate_assertion_preserves_test_bytes_and_build_stays_strict(self):
        for newline in ('\n', '\r\n'):
            assertion = self.ASSERTION.replace('\n', newline)
            context = ("thread 'settled_rate_case' panicked at tests/private/chart_roundoff.rs:471:9:"
                       + newline + assertion)
            records, raw = MODULE.cargo_json_records(
                self.FINISHED.replace('\n', newline) + context, test_phase=True)
            self.assertEqual(records, [{'reason': 'build-finished', 'success': True}])
            self.assertEqual(raw, context)
            for mode in ({}, {'test_phase': False}):
                with self.subTest(newline=newline, mode=mode), self.assertRaises(MODULE.Incomplete):
                    MODULE.cargo_json_records(self.FINISHED + assertion, **mode)

    def test_partial_near_miss_and_malformed_json_neighbors_remain_rejected(self):
        cases = {
            'missing_assertion_start': '\n'.join(self.ASSERTION.splitlines()[1:]) + '\n',
            'missing_last_line': '\n'.join(self.ASSERTION.splitlines()[:-1]) + '\n',
            'unterminated_complete_block': self.ASSERTION[:-1],
            'changed_guard': self.ASSERTION.replace('actual == issue', 'actual != issue'),
            'changed_step': self.ASSERTION.replace('FixtureChartStep::RateTimesElapsed',
                                                   'FixtureChartStep::SecondsDivide'),
            'changed_indent': self.ASSERTION.replace('    {\n', '   {\n', 1),
            'inserted_cargo_record': self.ASSERTION.replace(
                '        {\n', '        {"reason":"build-finished","success":true}\n', 1),
            'bare_brace': '    {\n',
            'unknown_object_after_block': self.ASSERTION + '{}\n',
            'malformed_cargo_after_block': self.ASSERTION + '{"reason":"build-finished","success":tru}\n',
            'duplicate_cargo_field_after_block': self.ASSERTION
                + '{"reason":"build-finished","success":true,"success":false}\n',
            'nonfinite_cargo_after_block': self.ASSERTION + '{"reason":"build-finished","success":NaN}\n',
            'unknown_cargo_after_block': self.ASSERTION + '{"reason":"unreviewed","success":true}\n',
        }
        for name, body in cases.items():
            with self.subTest(case=name), self.assertRaises(MODULE.Incomplete):
                MODULE.cargo_json_records(self.FINISHED + body, test_phase=True)
        self.assertEqual(MODULE.cargo_json_records(self.FINISHED, test_phase=True),
                         ([{'reason': 'build-finished', 'success': True}], ''))

    def test_human_assertion_supplies_neither_compile_cause_nor_native_kill_alone(self):
        helper = CoreAnalysisTests()
        unviable = {'summary': 'Unviable', 'phase_results': helper.phase_results('Unviable')}
        MODULE.unviable_compile_failure(unviable, helper.semantic_mutation_log())
        with self.assertRaises(MODULE.Incomplete):
            MODULE.unviable_compile_failure(unviable,
                helper.mutation_phase_header('Build') + self.ASSERTION
                + '{"reason":"build-finished","success":false}\n')
        caught = {'summary': 'CaughtMutant', 'phase_results': helper.phase_results('CaughtMutant')}
        with self.assertRaisesRegex(MODULE.Incomplete, 'no complete actual native failed-test evidence'):
            MODULE.caught_native_test_failure(caught, self.ASSERTION)
        # This is synthetic adjacent phase/failed-test-parser evidence only;
        # complete actual descriptors/outcomes remain the separate kill gate.
        complete_failure = (self.ASSERTION + '\nfailures:\n'
            '    regression::chart::roundoff::tests::settled_rate_rejects_unsupported_rates_and_reachable_results\n'
            '\ntest result: FAILED. 0 passed; 1 failed; 0 ignored;\n')
        MODULE.caught_native_test_failure(caught, complete_failure)


if __name__ == '__main__':
    unittest.main()
