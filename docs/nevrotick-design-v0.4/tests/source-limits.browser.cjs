'use strict';
const {chromium} = require('playwright');
const http = require('node:http');
const fs = require('node:fs');
const path = require('node:path');
const assert = require('node:assert/strict');
const root = path.resolve(__dirname, '..');
const types = {'.html':'text/html', '.js':'text/javascript', '.css':'text/css', '.avif':'image/avif'};

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
    for (const file of ['index.html','NevroTick-UI-v0.4.html']) {
      for (const width of [360,480,1280]) {
        const page = await browser.newPage({viewport:{width,height:1000}});
        const errors = [];
        page.on('pageerror', error => errors.push(error.message));
        for (const [scene,id,code] of [['server-rate-limited','delta','RATE'], ['server-access-refused','beta','DENY']]) {
          await page.goto(`http://127.0.0.1:${server.address().port}/${file}?scene=${scene}`);
          await page.locator('[data-action="sync-now"]').click();
          assert.match(await page.locator('#sync-response').innerText(),id === 'delta' ? /Checking/ : /Source access refused/);
          await page.locator('#sync-response').getByRole('button',{name:'Review source in Power user',exact:true}).click();
          assert.equal(await page.locator('dialog .source-detail-title code').innerText(),`${id}.example`);
          await page.locator('dialog').getByRole('button',{name:'Review date recovery',exact:true}).click();
          await page.getByRole('button',{name:'Back to sources',exact:true}).click();
          assert.equal(await page.locator('dialog .source-detail-title code').innerText(),`${id}.example`);
          assert.equal(await page.locator('dialog').getByRole('button',{name:'Review date recovery',exact:true}).evaluate(e=>e===document.activeElement),true);
          await page.keyboard.press('Escape');
          await page.goto(`http://127.0.0.1:${server.address().port}/${file}?scene=${scene}`);
          if (id === 'beta') {
            await page.getByRole('button',{name:'Review source in Power user',exact:true}).focus();
            await page.keyboard.press('Enter');
            assert.equal(await page.locator('dialog .source-detail-title code').innerText(),'beta.example');
            await page.keyboard.press('Escape');
            assert.equal(await page.evaluate(() => document.activeElement.dataset.value),'beta');
          } else {
            assert.match(await page.locator('h1').innerText(), /Time sync active/);
            await page.getByRole('button',{name:'Power user',exact:true}).click();
          }
          await page.locator('nav').getByRole('button',{name:'Sources',exact:true}).click();
          await page.locator('#source-search').fill('alpha');
          await page.getByRole('button',{name:'In use',exact:true}).click();
          await page.locator('nav').getByRole('button',{name:'Diagnostics',exact:true}).click();
          await page.getByRole('button',{name:'View affected source',exact:true}).focus();
          await page.keyboard.press('Enter');
          assert.equal(await page.locator('dialog[open]').count(),1);
          assert.equal(await page.locator('dialog .source-detail-title code').innerText(),`${id}.example`);
          assert.match(await page.locator('dialog').innerText(), /Filters cleared to show the affected source/);
          assert.match(await page.locator('dialog').innerText(), /This response contains no time measurement/);
          await page.locator('dialog details').getByText('Endpoint & identity details').click();
          assert.match(await page.locator('dialog details').innerText(), new RegExp(code));
          await page.keyboard.press('Escape');
          assert.equal(await page.locator('#source-search').inputValue(),'');
          assert.equal(await page.getByRole('button',{name:'All sources',exact:true}).getAttribute('aria-pressed'),'true');
          assert.equal(await page.locator(`#source-row-${id}`).getAttribute('aria-describedby'),'affected-source-note');
          assert.equal(await page.locator('#affected-source-note').innerText(),'Affected source');
          assert.equal(await page.evaluate(() => document.activeElement.dataset.action),'source-details');
          assert.equal(await page.evaluate(() => document.activeElement.dataset.value),id);
          assert.equal(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth),true,`${file}/${scene}/${width} viewport overflow`);
        }
        await page.goto(`http://127.0.0.1:${server.address().port}/${file}?scene=all-rate-limited`);
        await page.locator('[data-action="sync-now"]').click();
        assert.match(await page.locator('#sync-response').innerText(),/Checks delayed by server limits/);
        await page.getByRole('button',{name:'Power user',exact:true}).click();
        await page.locator('nav').getByRole('button',{name:'Sources',exact:true}).click();
        await page.getByRole('button',{name:'Details for Alpha',exact:true}).click();
        assert.match(await page.locator('dialog').innerText(),/Checks delayed by server/);
        assert.match(await page.locator('dialog').innerText(),/12 s ago/);
        assert.match(await page.locator('dialog').innerText(),/4 min ago/);
        assert.doesNotMatch(await page.locator('dialog').innerText(),/No rate-limit observation|No source restriction observed/);
        await page.keyboard.press('Escape');
        assert.deepEqual(errors,[],`${file}/${width} browser errors`);
        await page.close();
      }
    }
    console.log('Source restriction routes, keyboard focus, visible filter notice and 360/480/1280 px layouts passed in local and standalone HTML');
  } finally { await browser.close(); server.close(); }
})().catch(error => {console.error(error);process.exitCode=1;});
