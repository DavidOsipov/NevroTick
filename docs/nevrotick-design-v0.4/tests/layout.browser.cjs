'use strict';
// Run after node build.cjs, with Playwright and Chromium available.
const { chromium } = require('playwright');
const http = require('node:http');
const fs = require('node:fs');
const path = require('node:path');
const assert = require('node:assert/strict');
const M = require('../src/model.js');
const root = path.resolve(__dirname, '..');
const screenshotDirectory = process.env.LAYOUT_SCREENSHOTS;
const types = { '.html':'text/html', '.css':'text/css', '.js':'text/javascript', '.avif':'image/avif' };

(async () => {
  const server = http.createServer((req, res) => {
    const file = path.join(root, decodeURIComponent(new URL(req.url, 'http://localhost').pathname));
    if (!file.startsWith(root + path.sep)) return res.writeHead(403).end();
    try {
      res.writeHead(200, { 'Content-Type':types[path.extname(file)] || 'application/octet-stream' });
      res.end(fs.readFileSync(file));
    } catch { res.writeHead(404).end(); }
  });
  await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
  const browser = await chromium.launch({
    executablePath:process.env.CHROMIUM_PATH || '/usr/bin/chromium',
    headless:true, args:['--no-sandbox']
  });
  let statusChecks = 0, aboutChecks = 0;
  if (screenshotDirectory) fs.mkdirSync(screenshotDirectory, { recursive:true });
  try {
    for (const file of ['index.html', 'NevroTick-UI-v0.4.html']) {
      for (const width of [360, 480, 1280]) {
        const page = await browser.newPage({ viewport:{ width, height:720 } });
        const errors = [];
        page.on('pageerror', error => errors.push(error.message));
        const button = name => page.getByRole('button', { name, exact:true });
        const screenshot = async name => {
          if (screenshotDirectory) await page.screenshot({
            path:path.join(screenshotDirectory, `${path.basename(file, '.html')}-${name}-${width}.png`),
            fullPage:true
          });
        };

        for (const scene of M.SCENARIOS) {
          await page.goto(`http://127.0.0.1:${server.address().port}/${file}?scene=${scene.id}`);
          for (const mode of ['simple', 'power']) {
            if (mode === 'power') await button('Power user').click();
            const context = `${file} ${width}px ${mode} ${scene.id}`;
            const geometry = await page.evaluate(() => {
              const clipped = [];
              // A clipped ancestor can hide overflow without widening the document.
              for (const badge of document.querySelectorAll('.status-top .badge')) {
                const rect = badge.getBoundingClientRect();
                const row = badge.parentElement.getBoundingClientRect();
                if (rect.left < row.left - 1 || rect.right > row.right + 1) clipped.push(badge.textContent);
                for (let ancestor = badge.parentElement; ancestor; ancestor = ancestor.parentElement) {
                  const style = getComputedStyle(ancestor);
                  if (!['hidden', 'clip'].includes(style.overflowX)) continue;
                  const bounds = ancestor.getBoundingClientRect();
                  if (rect.left < bounds.left - 1 || rect.right > bounds.right + 1) clipped.push(badge.textContent);
                }
              }
              return { clipped, documentWidth:document.documentElement.scrollWidth };
            });
            assert.deepEqual(geometry.clipped, [], `${context}: status badge is clipped`);
            assert.ok(geometry.documentWidth <= width, `${context}: document overflows`);
            statusChecks++;
            if (scene.id === 'stale' || (width === 360 && ['reserve', 'recovered'].includes(scene.id))) {
              await screenshot(`${mode}-${scene.id}`);
            }
          }
        }

        for (const mode of ['simple', 'power']) {
          await page.goto(`http://127.0.0.1:${server.address().port}/${file}`);
          if (mode === 'power') await button('Power user').click();
          await page.locator('.app-nav').getByRole('button', { name:'About', exact:true }).click();
          const context = `${file} ${width}px ${mode} About`;
          const geometry = await page.evaluate(() => {
            const copy = document.querySelector('.about-copy').getBoundingClientRect();
            const panel = document.querySelector('.about-panel').getBoundingClientRect();
            const logo = document.querySelector('.about-logo');
            return {
              copyWidth:copy.width, panelHeight:panel.height,
              logoLoaded:logo.complete && logo.naturalWidth > 0,
              narrowValues:[...document.querySelectorAll('.about-copy .compact-facts dd')]
                .filter(value => value.getBoundingClientRect().width > 0 && value.getBoundingClientRect().width < 150)
                .map(value => value.textContent),
              documentWidth:document.documentElement.scrollWidth
            };
          });
          assert.ok(geometry.logoLoaded, `${context}: logo loads`);
          assert.ok(geometry.documentWidth <= width, `${context}: document overflows`);
          assert.deepEqual(geometry.narrowValues, [], `${context}: fact values have a readable column`);
          if (mode === 'simple' && width === 1280) {
            assert.ok(geometry.copyWidth >= 400, `${context}: copy needs at least 400px`);
            assert.ok(geometry.panelHeight < 1100, `${context}: avoid an unnecessarily tall card`);
          }
          await screenshot(`${mode}-about`);
          await page.getByText('License and artwork', { exact:true }).click();
          await page.getByText('Technologies and libraries', { exact:true }).click();
          await page.getByText('Privacy and connections', { exact:true }).click();
          assert.equal(await page.locator('.about-repo').count(), 2, `${context}: repository and release links remain`);
          await page.getByText('© David Osipov.', { exact:false }).waitFor({ state:'visible' });
          await screenshot(`${mode}-about-expanded`);
          aboutChecks++;
        }
        assert.deepEqual(errors, [], `${file} ${width}px: browser errors`);
        await page.close();
      }
    }
    console.log(`PASS: ${statusChecks} Status scene/mode/width checks; ${aboutChecks} About layouts with content and licensing retained.`);
  } finally {
    await browser.close();
    await new Promise(resolve => server.close(resolve));
  }
})().catch(error => { console.error(error); process.exitCode = 1; });
