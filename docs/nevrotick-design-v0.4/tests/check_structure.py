"""Check rendered HTML and standalone packaging. Uses lxml; not browser evidence."""
import base64
import hashlib
import json
import re
import sys
from collections import Counter
from pathlib import Path
from lxml import html

root = Path(__file__).resolve().parents[1]
records = json.loads(Path(sys.argv[1]).read_text())
errors = []
for item in records:
    doc = html.fragment_fromstring(item['html'], create_parent='div')
    ids = [e.get('id') for e in doc.xpath('//*[@id]')]
    duplicate = [k for k,v in Counter(ids).items() if v>1]
    if duplicate: errors.append((item['name'],'duplicate IDs',duplicate))
    for e in doc.xpath('//*[@aria-labelledby or @aria-describedby]'):
        for attr in ('aria-labelledby','aria-describedby'):
            for target in (e.get(attr) or '').split():
                if target not in ids: errors.append((item['name'],'missing ARIA target',target))
    for e in doc.xpath('//label[@for]'):
        if e.get('for') not in ids: errors.append((item['name'],'missing label target',e.get('for')))
    for e in doc.xpath('//button'):
        if not (e.get('aria-label') or ''.join(e.itertext()).strip()): errors.append((item['name'],'unnamed button',''))
        if not e.get('type'): errors.append((item['name'],'button without explicit type',''))
    for e in doc.xpath('//*[local-name()="svg"]'):
        if e.get('role') == 'img' and not e.get('aria-labelledby'): errors.append((item['name'],'unnamed graphic',''))
    if len(doc.xpath('//main//h1')) != 1: errors.append((item['name'],'main must contain one heading',''))
    for e in doc.xpath('//img'):
        if e.get('alt') is None: errors.append((item['name'],'image without alt',e.get('src')))
    for e in doc.xpath('//input|//select'):
        eid=e.get('id'); labels=doc.xpath('//label[@for=$v]',v=eid or 'NOID')
        if not (e.get('aria-label') or labels or e.xpath('ancestor::label')): errors.append((item['name'],'unnamed input',eid))

single = (root/'NevroTick-UI-v0.4.html').read_text()
doc = html.fromstring(single)
script = doc.xpath('//script')[0].text
csp = doc.xpath('//meta[@http-equiv="Content-Security-Policy"]')[0].get('content')
expected = base64.b64encode(hashlib.sha256(script.encode()).digest()).decode()
assert "script-src 'sha256-"+expected+"'" in csp, 'CSP script hash does not match bundled bytes'
assert "connect-src 'none'" in csp
assert not doc.xpath('//script[@src]|//link[@rel="stylesheet"]'), 'Standalone depends on external script or CSS'
for e in doc.xpath('//img|//link[@rel="icon"]'):
    src=e.get('src') or e.get('href')
    assert src.startswith('data:image/avif;base64,'), src
assert '__CSP__' not in single and '__INITIAL__' not in single
assert 'assets/' not in script, 'Dynamic views contain unresolved asset paths'
for asset in ('logo.avif','mascot.avif','icon.avif'):
    encoded=base64.b64encode((root/'assets'/asset).read_bytes()).decode()
    assert encoded in single, 'AVIF bytes were not preserved: '+asset
scripts='\n'.join((root/'src'/p).read_text() for p in ('model.js','views.js','app.js'))
for forbidden in (r'\bfetch\s*\(',r'\bXMLHttpRequest\b',r'\bWebSocket\b',r'\blocalStorage\b',r'\bsessionStorage\b',r'\bindexedDB\b',r'\bserviceWorker\b',r'\beval\s*\(',r'new\s+Function\b'):
    assert not re.search(forbidden,scripts), 'Unexpected active capability: '+forbidden
assert not re.search(r'\son(?:click|error|load)\s*=',single[:single.index('<script>')]), 'Inline event handler in HTML'
if errors:
    print(json.dumps(errors,indent=2));sys.exit(1)
print(json.dumps({'rendered_documents_checked':len(records),'structural_errors':0,'CSP_script_hash':'matches final bundled script','external_dependencies':0,'original_AVIF_bytes':'preserved','browser_validation':'not performed by this structural check'},indent=2))
