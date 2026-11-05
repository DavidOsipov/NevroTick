'use strict';
// Requires Playwright and Chromium; run separately with node tests/input-fix.browser.cjs.
const { chromium } = require('playwright');
const http = require('http');
const fs = require('fs');
const path = require('path');
const assert = require('node:assert/strict');
const root = path.resolve(__dirname, '..');
(async () => {
  const server = http.createServer((req, res) => {
    const file = path.join(root, decodeURIComponent(new URL(req.url, 'http://localhost').pathname));
    if (!file.startsWith(root + path.sep)) return res.writeHead(403).end();
    try { res.end(fs.readFileSync(file)); } catch { res.writeHead(404).end(); }
  });
  await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
  const browser = await chromium.launch({executablePath: process.env.CHROMIUM_PATH || '/usr/bin/chromium', headless: true, args: ['--no-sandbox']});
  try {
    const page = await browser.newPage();
    for (const file of ['index.html', 'NevroTick-UI-v0.4.html']) {
      await page.goto(`http://127.0.0.1:${server.address().port}/${file}`);
      await page.locator('nav').getByRole('button',{name:'Settings',exact:true}).click();
      await page.locator('#frequency-mode').selectOption('custom');
      await page.locator('#frequency-interval').selectOption('own');
      const input = page.locator('#frequency-own-minutes');
      const tip = page.locator('#frequency-form details.help-tip');
      await tip.locator('summary').click();
      await input.fill('');
      await input.pressSequentially('123');
      assert.equal(await input.inputValue(),'123');
      assert.equal(await tip.evaluate(e=>e.open),true);
      await input.press('Control+z');
      assert.notEqual(await input.inputValue(),'123','native undo must change typed digits');
      await input.press('Control+Shift+z');
      assert.equal(await input.inputValue(),'123','native redo must restore typed digits');
      assert.equal(await page.evaluate(() => document.activeElement.id),'frequency-own-minutes');
      await page.evaluate(() => { window.inputBeforeComposition = document.getElementById('frequency-own-minutes'); });
      await input.evaluate(e => { e.dispatchEvent(new CompositionEvent('compositionstart',{bubbles:true,data:''})); e.dispatchEvent(new InputEvent('input',{bubbles:true,data:'3',isComposing:true})); e.dispatchEvent(new CompositionEvent('compositionend',{bubbles:true,data:'3'})); });
      assert.equal(await page.evaluate(() => window.inputBeforeComposition === document.getElementById('frequency-own-minutes')),true,'composition must not replace input');
      await input.fill('0');
      await page.locator('#frequency-form').getByRole('button',{name:'Update preview'}).click();
      assert.equal(await input.getAttribute('aria-invalid'),'true');
      assert.match(await page.locator('#frequency-error').innerText(),/whole number.*at least 1/i);
      await input.fill('7');
      assert.equal(await input.getAttribute('aria-invalid'),null);
      assert.equal(await page.locator('#frequency-error').isHidden(),true);
      await page.locator('#frequency-form').getByRole('button',{name:'Update preview'}).click();
      assert.match(await page.locator('.frequency-current').innerText(),/7 minutes/i);
      await page.getByRole('button',{name:'Power user',exact:true}).click();
      await page.locator('nav').getByRole('button',{name:'Sources'}).click();
      const status = page.locator('#source-results-status');
      await page.evaluate(() => { window.originalResultsStatus = document.getElementById('source-results-status'); });
      await page.locator('#source-search').fill('zzzzzz');
      assert.equal(await status.innerText(),'No matching sources.');
      await page.locator('#source-search').fill('');
      assert.match(await status.innerText(),/\d+ sources found\./);
      await page.getByRole('button',{name:'In use',exact:true}).click();
      assert.equal(await page.evaluate(() => window.originalResultsStatus === document.getElementById('source-results-status')),true,'live status must remain mounted after a filter change');
      assert.match(await status.innerText(),/\d+ sources? found\./);
      await page.getByRole('button',{name:'Add source preview'}).click();
      assert.match(await page.locator('#help-name').innerText(),/Preview limit: 1–64 printable ASCII characters.*without leading or trailing spaces/i);
      console.log(`${file}: undo/redo, composition node, validation, live results and preview note passed`);
    }
  } finally { await browser.close(); server.close(); }
})().catch(error => { console.error(error); process.exitCode = 1; });
