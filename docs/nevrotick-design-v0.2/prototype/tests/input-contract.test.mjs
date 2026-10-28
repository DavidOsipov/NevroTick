import test from 'node:test';
import assert from 'node:assert/strict';
import {
  canonicalHost, validateSource, validateConfiguration, newSource,
  newRecovery, copyRecovery, validateRecovery,
  sourceReviewChanges, recoveryReviewChanges,
} from '../src/model.js';

const sources = protocol => [
  {...newSource(protocol),id:'a',host:'a.example'},
  {...newSource(protocol),id:'b',host:'b.example'},
];

test('hostname and IPv6 normalization preserve valid identities', () => {
  assert.equal(canonicalHost('Time.Example.'),'time.example');
  assert.equal(canonicalHost('bücher.example'),'xn--bcher-kva.example');
  assert.equal(canonicalHost('192.0.2.10'),'192.0.2.10');
  assert.equal(canonicalHost('2001:0db8:0:0:0:0:0:10'),'2001:db8::10');
  assert.equal(canonicalHost('[2001:db8::10]'),'2001:db8::10');
});

test('URL components and address/port ambiguity cannot become server identities', () => {
  for (const value of [
    '', 'https://time.example/', 'user@time.example', 'time.example/path',
    'time.example?port=123', 'time.example#fragment', 'time.example:4460',
    '[2001:db8::10]:4460', '2001:db8::10%eth0', 'time.example\\other',
  ]) assert.throws(()=>canonicalHost(value),undefined,value);
  assert.equal(canonicalHost('time.example'),'time.example');
});

test('legacy IPv4 variants stay rejected after URL hostname normalization', () => {
  for (const value of [
    '0x7f000001','0x7f.0.0.1','0177.0.0.1','2130706433',
    '127.1','127.00.0.1','999.0.0.1','１２７.０.０.１',
  ]) assert.throws(()=>canonicalHost(value),undefined,value);
  // A valid DNS label beginning with 0x remains a DNS label, not an IP.
  assert.equal(canonicalHost('0x7f.example'),'0x7f.example');
  assert.equal(canonicalHost('127.0.0.1'),'127.0.0.1');
});

test('source ports reject adjacent invalid bounds and non-integer spellings', () => {
  const form={...newSource('nts'),host:'time.example'};
  for (const port of ['1','4460','65535']) {
    assert.equal(validateSource({...form,port},[],'nts').errors.port,undefined,port);
  }
  for (const port of ['0','65536','-1','44.60','1e2','+123',' 123','']) {
    assert.ok(validateSource({...form,port},[],'nts').errors.port,port);
  }
});

test('source identity duplicates are compared after normalization', () => {
  const existing=[{...newSource('nts'),id:'saved',host:'time.example'}];
  assert.ok(validateSource({...newSource('nts'),host:'TIME.EXAMPLE.'},existing,'nts').errors.host);
  assert.equal(validateSource({...existing[0],host:'TIME.EXAMPLE.'},existing,'nts').errors.host,undefined);
});

test('profile change remains blocked until every normal source matches the draft', () => {
  const ntpForm={...newSource('ntp'),host:'new.example'};
  assert.ok(validateSource(ntpForm,[],'nts').errors.protocol);
  assert.equal(validateSource(ntpForm,[],'ntp').errors.protocol,undefined);
  assert.ok(validateConfiguration(sources('nts'),'ntp').some(x=>x.includes('match')));
  const mixed=[sources('nts')[0],sources('ntp')[1]];
  assert.ok(validateConfiguration(mixed,'nts').some(x=>x.includes('match')));
  assert.deepEqual(validateConfiguration(sources('ntp'),'ntp'),[]);
  assert.deepEqual(validateConfiguration(sources('nts'),'nts'),[]);
});

test('example minimum source policy does not silently weaken after removal', () => {
  assert.ok(validateConfiguration(sources('nts').slice(0,1),'nts').some(x=>x.includes('at least 2')));
  assert.deepEqual(validateConfiguration(sources('nts'),'nts'),[]);
});

test('enabled recovery requires its selected number of valid helpers at save and apply', () => {
  const off=newRecovery();
  assert.deepEqual(validateRecovery(off).errors,[]);
  const incomplete={...off,enabled:true,helpers:['a.example','']};
  assert.ok(validateRecovery(incomplete).errors.some(x=>x.includes('helper 2')));
  assert.ok(validateConfiguration(sources('nts'),'nts',incomplete).some(x=>x.includes('helper 2')));
  const one={...incomplete,mode:'one'};
  assert.deepEqual(validateRecovery(one).errors,[]);
  assert.deepEqual(validateConfiguration(sources('nts'),'nts',one),[]);
  const two={...incomplete,helpers:['a.example','b.example']};
  assert.deepEqual(validateConfiguration(sources('nts'),'nts',two),[]);
});

test('two recovery helpers cannot be aliases of the same literal identity', () => {
  for (const helpers of [
    ['A.EXAMPLE.','a.example'],
    ['2001:db8::1','2001:0db8:0:0:0:0:0:1'],
    ['bücher.example','xn--bcher-kva.example'],
  ]) {
    const result=validateRecovery({enabled:true,mode:'two',helpers});
    assert.ok(result.errors.some(x=>x.includes('distinct')),helpers.join(' / '));
  }
  assert.ok(validateRecovery({enabled:true,mode:'two',helpers:['0x7f000001','b.example']}).errors.length);
});

test('recovery editor copies and validation cannot mutate the saved draft', () => {
  const draft={enabled:true,mode:'two',helpers:['A.EXAMPLE.','b.example']};
  const snapshot=structuredClone(draft);
  const editor=copyRecovery(draft);
  editor.helpers[0]='changed.example';
  editor.mode='one';
  assert.deepEqual(draft,snapshot);
  const checked=validateRecovery(draft);
  assert.deepEqual(checked.errors,[]);
  assert.equal(checked.recovery.helpers[0],'a.example');
  assert.deepEqual(draft,snapshot);
  assert.deepEqual(copyRecovery(draft),snapshot);
  assert.ok(validateRecovery({enabled:true,mode:'auto',helpers:['a.example','b.example']}).errors.length);
});

test('review exposes a port-only change with its transport and purpose', () => {
  const before={...newSource('nts'),host:'time.example'};
  const rows=sourceReviewChanges(before,{...before,port:'4443'});
  assert.equal(rows.length,1);
  assert.match(rows[0].kind,/Port/);
  assert.equal(rows[0].before,'4460 · TCP · NTS-KE');
  assert.equal(rows[0].after,'4443 · TCP · NTS-KE');
});

test('review retains all edited source parameters and includes new-source connection details', () => {
  const before={...newSource('nts'),host:'a.example',name:'Old name',group:'Operator A'};
  const after={...before,host:'b.example',protocol:'ntp',port:'8123',name:'New name',group:'Operator B',scale:'other'};
  const rows=sourceReviewChanges(before,after);
  for (const value of ['b.example','NTPv4 · unauthenticated','8123 · UDP · NTP','New name','Operator B','other']) {
    assert.ok(rows.some(r=>r.after===value),value);
  }
  assert.ok(sourceReviewChanges(undefined,after).some(r=>r.after==='8123 · UDP · NTP'));
});

test('recovery review includes helper and policy changes even when enabled stays true', () => {
  const before={enabled:true,mode:'two',helpers:['a.example','b.example']};
  const after={enabled:true,mode:'one',helpers:['c.example','b.example']};
  const rows=recoveryReviewChanges(before,after);
  assert.ok(rows.some(r=>r.kind==='Bootstrap policy'&&r.after.includes('One helper')));
  assert.ok(rows.some(r=>r.kind==='Bootstrap helper 1'&&r.before==='a.example'&&r.after==='c.example'));
  assert.deepEqual(recoveryReviewChanges(before,copyRecovery(before)),[]);
});
