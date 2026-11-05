'use strict';
const test = require('node:test');
const assert = require('node:assert/strict');
const M = require('../src/model.js');
const V = require('../src/views.js');

function help(view, name) {
  const pattern = new RegExp(`<details class="help-tip"><summary aria-label="About ${name}">[\\s\\S]*?</summary><span class="help-tip-content">([\\s\\S]*?)</span></details>`, 'i');
  const match = view.match(pattern);
  assert.ok(match, `Missing accessible help: ${name}`);
  return match[1];
}

test('status help distinguishes authentication, agreement and estimated uncertainty', () => {
  const view = V.status({...M.initialState(), mode:'power'});
  assert.match(help(view,'authentication'), /authenticates.*does not guarantee.*accura(?:cy|te)/i);
  assert.match(help(view,'source agreement'), /minimum.*majority/i);
  assert.match(help(view,'estimated uncertainty'), /estimate.*not.*guarantee/i);
  assert.match(view, /Model estimate, not a precision guarantee/);
});

test('time check help explains request scheduling without changing custom interval validation', () => {
  const state = M.initialState();
  assert.match(help(V.simpleSettings(state), 'time check interval'), /measurement.*adjustment.*delay/i);
  const custom = M.reduce(state,{type:'frequency-draft',value:'own:0'});
  assert.ok(M.reduce(custom,{type:'frequency-save'}).frequencyError);
});

test('profile help preserves explicit selection and clock ownership distinctions', () => {
  const view = V.settings({...M.initialState(),mode:'power',settingsSection:'policy'});
  assert.match(help(view,'active profile'), /NTP.*automatic.*observe.only.*Windows Time/i);
  assert.match(view,/No mixed authenticated\/plain voting/);
});

test('source, chart and diagnostics help explain interpretation in context', () => {
  const state = {...M.initialState(),mode:'power'};
  assert.match(help(V.sources(state),'source roles'), /main.*supporting.*not used/i);
  assert.match(help(V.statistics(state),'source comparison'), /uncertainty.*estimate/i);
  assert.match(help(V.diagnostics(state),'evidence freshness'), /reply.*estimate.*correction.*snapshot/i);
});

test('recovery helper help does not replace visible safety warnings', () => {
  const state = {...M.initialState(),mode:'power',modal:'recovery-editor'};
  const view = V.modal(state);
  assert.match(help(view,'plain-NTP helpers'), /certificate.*unauthenticated/i);
  assert.match(view,/Two-helper mode requires both and never silently falls back to one/);
  assert.match(view,/Helpers neither vote in normal synchronization nor change Windows time/);
});
