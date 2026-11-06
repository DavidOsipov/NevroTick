"""Bounded read-only gate over the verifier's frozen source snapshot."""
from pathlib import Path
import argparse, json, sys
import check_ui_text_coverage as coverage
import protected_json
from ui_source_io import read_regular

def regular(root, relative):
    path = root / relative
    if path.is_symlink() or not path.is_file() or not path.resolve().is_relative_to(root):
        raise ValueError('regular protected source required')
    payload = read_regular(path, 1_000_000)
    return protected_json.read(payload)

def main():
    parser = argparse.ArgumentParser(description='Check independent GUI text sinks and exact protected contract coverage.')
    parser.add_argument('--root', required=True, type=Path)
    arguments = parser.parse_args()
    root = arguments.root.resolve(strict=True)
    folder = 'crates/nevrotick-ui/src/localization/'
    try:
        contract = regular(root, folder + 'contract-source.json')
        baseline = regular(root, folder + 'text-contract-v1-baseline.json')
        manifest = regular(root, folder + 'runtime-text-sinks.json')
        if set(manifest) != {'schema_version', 'scope', 'optional_empty_slots', 'runtime_sinks'} or type(manifest['schema_version']) is not int or manifest['schema_version'] != 1:
            raise ValueError('closed receiver manifest schema')
        for document in [contract, baseline]:
            if any(type(document[name]) is not int or document[name] < 1 for name in ['catalog_schema_version','gui_build_version','text_contract_version']):
                raise ValueError('exact positive integer versions')
        files = coverage.snapshot(root)
        if coverage.PRODUCT_BUILD_SOURCE not in files or coverage.PRODUCT_HOST_SOURCE not in files:
            raise ValueError('required product build/host entry is missing')
        report = coverage.check(contract, files, baseline, manifest['runtime_sinks'])
        result = {'status':'passed' if report['ok'] else 'failed', 'active_ids':report['active_ids'],
                  'used_ids':report['used_ids'], 'catalog_schema_version':contract['catalog_schema_version'],
                  'text_contract_version':contract['text_contract_version'], 'gui_build_version':contract['gui_build_version'],
                  'runtime_sink_map_provided':report['runtime_sink_map_provided'],
                  'baseline_provided':report['baseline_provided'],
                  'semantic_fingerprint':report['semantic_fingerprint'],
                  'error_count':len(report['errors']), 'errors':report['errors'][:100]}
    except (ValueError, KeyError, TypeError, OSError, UnicodeError, RecursionError):
        result = {'status':'failed','error_count':1,'errors':[{'kind':'invalid-or-incomplete-protected-source'}]}
    print(json.dumps(result, ensure_ascii=True, separators=(',',':')))
    return 0 if result['status'] == 'passed' else 1

if __name__ == '__main__':
    sys.exit(main())
