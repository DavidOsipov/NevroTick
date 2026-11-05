'use strict';
// Run separately with Playwright and Chromium available: node tests/tooltips.browser.cjs
const {chromium} = require('playwright');
const http = require('http');
const fs = require('fs');
const path = require('path');
const assert = require('node:assert/strict');
const root = path.resolve(__dirname, '..');
const types = {'.html':'text/html', '.css':'text/css', '.js':'text/javascript', '.avif':'image/avif'};
(async () => {
  const server = http.createServer((req,res) => {
    const file = path.join(root, decodeURIComponent(new URL(req.url, 'http://localhost').pathname));
    if (!file.startsWith(root + path.sep)) return res.writeHead(403).end();
    try { res.writeHead(200, {'Content-Type':types[path.extname(file)] || 'application/octet-stream'}).end(fs.readFileSync(file)); }
    catch { res.writeHead(404).end(); }
  });
  await new Promise(resolve => server.listen(0,'127.0.0.1',resolve));
  const browser = await chromium.launch({executablePath:process.env.CHROMIUM_PATH || '/usr/bin/chromium',headless:true,args:['--no-sandbox']});
  try {
    const page = await browser.newPage();
    const go = () => page.goto(`http://127.0.0.1:${server.address().port}/index.html`);
    const tip = name => page.locator('details.help-tip').filter({has:page.locator(`summary[aria-label="About ${name}"]`)});
    await go();
    await page.locator('nav').getByRole('button',{name:'Settings',exact:true}).click();
    await tip('time check interval').locator('summary').focus();
    await page.keyboard.press('Enter');
    await page.keyboard.press('Tab');
    assert.equal(await page.evaluate(() => document.activeElement.id),'frequency-mode');
    await page.keyboard.press('ArrowDown');
    assert.equal(await tip('time check interval').evaluate(el => el.open),true,'keyboard selection must preserve open help');
    await page.locator('#frequency-interval').selectOption('own');
    assert.equal(await tip('time check interval').evaluate(el => el.open),true,'subsequent form update must preserve help');
    await page.locator('#frequency-own-minutes').fill('12');
    assert.equal(await tip('time check interval').evaluate(el => el.open),true,'typing must preserve help');
    await page.locator('main h1').click();
    assert.equal(await tip('time check interval').evaluate(el => el.open),false,'outside click must still close help');
    await go();
    await page.getByRole('button',{name:'Power user',exact:true}).click();
    await page.locator('nav').getByRole('button',{name:'Sources',exact:true}).click();
    await page.getByRole('button',{name:'Explore recovery'}).click();
    await page.getByRole('button',{name:'Configure helper preview'}).click();
    await page.locator('#recovery-count').selectOption('1');
    await page.locator('#recovery-identity-1').fill('alpha.example');
    await tip('plain-NTP helpers').locator('summary').focus();
    await page.keyboard.press('Enter');
    await page.keyboard.press('Tab');
    assert.equal(await page.evaluate(() => document.activeElement.id),'recovery-count');
    await page.keyboard.press('Escape');
    assert.equal(await page.locator('dialog[open]').count(),1,'first Escape must keep dialog open');
    assert.equal(await tip('plain-NTP helpers').evaluate(el => el.open),false,'first Escape must close help');
    assert.equal(await page.locator('#recovery-identity-1').inputValue(),'alpha.example','unsaved text must survive');
    await page.keyboard.press('Escape');
    assert.equal(await page.locator('dialog[open]').count(),1,'second Escape keeps the editor dialog for its leave warning');
    assert.equal(await page.getByRole('button',{name:'Keep draft and leave',exact:true}).count(),1);
    await page.keyboard.press('Escape');
    assert.equal(await page.locator('#recovery-identity-1').inputValue(),'alpha.example','warning Escape returns to editing');
    console.log('Tooltip form rerender and modal Escape checks passed');
  } finally { await browser.close(); server.close(); }
})().catch(error => { console.error(error); process.exitCode = 1; });
