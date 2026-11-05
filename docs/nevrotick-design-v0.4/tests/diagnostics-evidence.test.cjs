'use strict';
const test = require('node:test');
const assert = require('node:assert/strict');
const M = require('../src/model.js');
const V = require('../src/views.js');

test('diagnostics does not invent component compatibility or the last failure', () => {
  for (const scenario of M.SCENARIOS) {
    const view = V.diagnostics({ ...M.initialState(scenario.id), mode: 'power' });
    assert.match(view, /Component compatibility<\/dt><dd>Not established in this fixture/);
    assert.match(view, /Last failure<\/dt><dd>Not available in this fixture/);
    assert.match(view, /does not establish that none occurred/);
  }
});
