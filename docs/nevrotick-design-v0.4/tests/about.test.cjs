'use strict';
const test = require('node:test');
const assert = require('node:assert/strict');
const V = require('../src/views.js');
const M = require('../src/model.js');

test('About is reachable and stays open when switching between both modes', () => {
  let state = M.initialState();
  const nav = V.render(state).match(/<nav class="app-nav"[\s\S]*?<\/nav>/)[0];
  assert.match(nav, /data-action="page" data-value="about"/);
  state = M.reduce(state, {type:'page',page:'about'});
  assert.equal(state.page, 'about');
  assert.match(V.render(state), /<h1[^>]*>About NevroTick<\/h1>/);
  state = M.reduce(state, {type:'view-mode',mode:'power'});
  assert.equal(state.page, 'about');
  state = M.reduce(state, {type:'view-mode',mode:'simple'});
  assert.equal(state.page, 'about');
  assert.match(V.render(state), /data-value="about" aria-current="page"/);
});

test('About describes the product, its author and its current development status', () => {
  const html = V.about();
  assert.match(html, /NevroTick is a Windows timekeeping project/i);
  assert.match(html, /David Osipov/);
  assert.match(html, /synchronization is not yet available/i);
  assert.doesNotMatch(html, /UI review v0\.4|Source snapshot|What runs here|Local design fixtures/i);
});

test('About provides a real user-initiated repository link', () => {
  const html = V.about();
  assert.match(html, /<a\b[^>]*href="https:\/\/github\.com\/DavidOsipov\/NevroTick"[^>]*>[^<]*GitHub[^<]*<\/a>/i);
});

test('About identifies development status and offers release information without implying installation', () => {
  const html = V.about();
  assert.match(html, /Version status/);
  assert.match(html, /no stable release declared/i);
  assert.match(html, /<a\b[^>]*href="https:\/\/github\.com\/DavidOsipov\/NevroTick\/releases"[^>]*>[^<]*release/i);
  assert.match(html, /opening (?:the|this) page does not install/i);
});

test('About describes planned network and privacy behavior without presenting it as implemented', () => {
  const html = V.about();
  assert.match(html, /<summary>Privacy and connections<\/summary>/);
  assert.match(html, /planned/i);
  assert.match(html, /configured time servers/i);
  assert.match(html, /DNS.*certificate/i);
  assert.match(html, /no automatic telemetry or crash upload/i);
});

test('About attributes the technologies and distinguishes code and artwork rights', () => {
  const html = V.about();
  for (const name of ['chrony', 'Tokio', 'garde', 'getrandom', 'windows-sys']) assert.match(html, new RegExp(name, 'i'));
  assert.match(html, /GPL-2\.0-only/);
  assert.match(html, /all rights reserved/i);
  assert.match(html, /independent|separate/i);
});
