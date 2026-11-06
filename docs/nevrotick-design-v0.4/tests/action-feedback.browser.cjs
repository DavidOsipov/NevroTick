'use strict';
// Chromium evidence for action responses; no network, clock or service operation is simulated.
const { chromium } = require('playwright');
const http = require('node:http');
const fs = require('node:fs');
const path = require('node:path');
const assert = require('node:assert/strict');
const root = path.resolve(__dirname,'..');
const screenshotDir = process.env.ACTION_SCREENSHOT_DIR;
(async () => {
  const server = http.createServer((req,res) => {
    const file = path.join(root,decodeURIComponent(new URL(req.url,'http://localhost').pathname));
    if (!file.startsWith(root + path.sep)) return res.writeHead(403).end();
    try { res.end(fs.readFileSync(file)); } catch { res.writeHead(404).end(); }
  });
  await new Promise(resolve => server.listen(0,'127.0.0.1',resolve));
  const browser=await chromium.launch({executablePath:process.env.CHROMIUM_PATH || '/usr/bin/chromium',headless:true,args:['--no-sandbox']});
  const results=[];
  try {
    for (const file of ['index.html','NevroTick-UI-v0.4.html']) for (const width of [360,480,1280]) {
      const page=await browser.newPage({viewport:{width,height:1000}});
      const errors=[];page.on('pageerror',error=>errors.push(error.message));
      await page.goto(`http://127.0.0.1:${server.address().port}/${file}`);
      await page.locator('#reviewer-controls > summary').click();
      assert.equal(await page.locator('.app-window #sample-sync-result').count(),0);
      assert.deepEqual(await page.locator('#sample-sync-result option').evaluateAll(nodes=>nodes.map(n=>n.value)),['checking','already']);
      const sync=page.locator('[data-action="sync-now"]');
      await page.evaluate(()=>{window.actionStatus=document.getElementById('sync-response');window.initialStatusTitle=document.querySelector('.simple-status h1').textContent;window.initialMeasurementAge=document.querySelector('.simple-freshness').textContent;});
      await sync.focus();await page.keyboard.press('Enter');
      assert.match(await page.locator('#sync-response').innerText(),/Sample response\s+Checking/);
      assert.equal(await page.locator('dialog').count(),0);
      assert.equal(await page.evaluate(()=>window.actionStatus===document.getElementById('sync-response')),true);
      assert.equal(await sync.evaluate(e=>e===document.activeElement),true);
      await page.keyboard.press('Space');
      assert.match(await page.locator('#sync-response').innerText(),/Already checking/);
      assert.equal(await page.evaluate(()=>window.initialStatusTitle===document.querySelector('.simple-status h1').textContent),true);
      assert.equal(await page.evaluate(()=>window.initialMeasurementAge===document.querySelector('.simple-freshness').textContent),true);
      await page.getByRole('button',{name:'Power user',exact:true}).click();
      assert.match(await page.locator('#sync-response').innerText(),/Already checking/);
      assert.equal(await page.locator('body').evaluate(e=>e.scrollWidth<=innerWidth),true,`page overflow at ${width}`);
      if(screenshotDir){fs.mkdirSync(screenshotDir,{recursive:true});await page.screenshot({path:path.join(screenshotDir,`${file.startsWith('index')?'local':'standalone'}-sync-${width}.png`),fullPage:true});}
      for(const [scene,expected] of [['conflict','Waiting for enough agreeing sources'],['slewing','Correction in progress'],['stale','Status unknown'],['service-stopped','Check unavailable'],['all-rate-limited','Checks delayed by server limits'],['blocked','Checking']]) {
        await page.locator('#scenario').selectOption(scene);
        await page.locator('[data-action="sync-now"]').click();
        assert.match(await page.locator('#sync-response').innerText(),new RegExp(expected));
        assert.equal(await page.locator('dialog').count(),0);
      }
      await page.locator('#scenario').selectOption('healthy');
      await page.locator('#sample-log-service-response').selectOption('refused');
      await page.locator('nav').getByRole('button',{name:'Diagnostics',exact:true}).click();
      await page.getByRole('button',{name:'Preview temporary collection'}).click();
      assert.equal(await page.locator('dialog select').count(),2,'only scope and duration belong in the action dialog');
      await page.locator('#sample-log-duration').selectOption('15');
      await page.evaluate(()=>{window.collectionStatus=document.getElementById('sample-log-result');});
      await page.getByRole('button',{name:'Start example',exact:true}).click();
      assert.equal(await page.evaluate(()=>window.collectionStatus===document.getElementById('sample-log-result')),true);
      assert.match(await page.locator('#sample-log-result').innerText(),/Sample response · Active/);
      assert.doesNotMatch(await page.locator('#sample-log-result').innerText(),/permission refused|collection failed/i);
      assert.match(await page.locator('#sample-log-result').innerText(),/15 minutes · frozen sample; no timer/);
      assert.equal(await page.getByRole('button',{name:'Stop example',exact:true}).evaluate(e=>e===document.activeElement),true);
      await page.getByRole('button',{name:'Stop example',exact:true}).click();
      await page.locator('#sample-log-scope').selectOption('both');
      await page.getByRole('button',{name:'Start example',exact:true}).click();
      assert.match(await page.locator('#sample-log-result').innerText(),/Sample response · Partial/);
      assert.match(await page.locator('#sample-log-result').innerText(),/permission refused/i);
      await page.keyboard.press('Escape');
      assert.match(await page.locator('.sample-log-summary').innerText(),/Partial/);
      await page.getByRole('button',{name:'Preview temporary collection'}).click();
      assert.match(await page.locator('#sample-log-result').innerText(),/Partial/);
      assert.equal(await page.locator('#sample-log-duration').inputValue(),'15');
      await page.getByRole('button',{name:'Stop example',exact:true}).click();
      await page.locator('#sample-log-scope').selectOption('service');
      await page.getByRole('button',{name:'Start example',exact:true}).click();
      assert.match(await page.locator('#sample-log-result').innerText(),/Sample response · Not started/);
      assert.doesNotMatch(await page.locator('#sample-log-result').innerText(),/Partial|Remaining duration/);
      await page.keyboard.press('Escape');
      await page.locator('#sample-log-ui-response').selectOption('failed');
      await page.getByRole('button',{name:'Preview temporary collection'}).click();
      await page.locator('#sample-log-scope').selectOption('ui');
      await page.getByRole('button',{name:'Start example',exact:true}).click();
      assert.match(await page.locator('#sample-log-result').innerText(),/Sample response · Failed/);
      await page.locator('#sample-log-duration').selectOption('30');
      assert.match(await page.locator('#sample-log-result').innerText(),/Sample response · Not started/);
      assert.doesNotMatch(await page.locator('#sample-log-result').innerText(),/Sample collection failed/);
      await page.getByRole('button',{name:'Start example',exact:true}).click();
      assert.match(await page.locator('#sample-log-result').innerText(),/Sample response · Failed/);
      assert.match(await page.locator('#sample-log-result').innerText(),/30 minutes \(illustrative\)/);
      await page.keyboard.press('Escape');
      await page.locator('#scenario').selectOption('stale');
      await page.locator('nav').getByRole('button',{name:'Diagnostics',exact:true}).click();
      await page.getByRole('button',{name:'Preview temporary collection'}).click();
      await page.locator('#sample-log-scope').selectOption('service');
      assert.equal(await page.getByRole('button',{name:/^Start example/}).isDisabled(),true);
      await page.locator('#sample-log-scope').selectOption('both');
      await page.getByRole('button',{name:'Start example',exact:true}).click();
      assert.match(await page.locator('#sample-log-result').innerText(),/Sample response · Unconfirmed/);
      assert.match(await page.locator('#sample-log-result').innerText(),/Remaining duration\s+Unknown/);
      await page.getByRole('button',{name:'Stop example',exact:true}).click();
      assert.match(await page.locator('#sample-log-result').innerText(),/Sample response · Unconfirmed/);
      assert.doesNotMatch(await page.locator('#sample-log-result').innerText(),/Sample response · (Stopped|Not started|Partial)/);
      if(screenshotDir) await page.screenshot({path:path.join(screenshotDir,`${file.startsWith('index')?'local':'standalone'}-diagnostics-${width}.png`),fullPage:true});
      await page.keyboard.press('Escape');
      if (width === 1280) {
        await page.setViewportSize({width,height:720});
        await page.goto(`http://127.0.0.1:${server.address().port}/${file}?scene=server-rate-limited`);
        const primary = page.getByRole('button',{name:'Sync now',exact:true});
        await primary.focus(); await page.keyboard.press('Enter');
        const box = await page.locator('#sync-response').boundingBox();
        assert.ok(box.y >= 0 && box.y + box.height <= 721,'the complete inline response must be visible in a 720px viewport');
        assert.equal(await primary.evaluate(e=>e===document.activeElement),true,'revealing feedback must preserve the action focus');
      }
      assert.deepEqual(errors,[]);
      results.push({file,width,checks:'keyboard inline status; fixed scene outcomes; component scopes; close/reopen; active/partial/not started/failed/unconfirmed; duration reset; frozen duration',pageErrors:0});
      await page.close();
    }
    console.log(JSON.stringify(results,null,2));
  } finally {await browser.close();server.close();}
})().catch(error=>{console.error(error);process.exitCode=1;});
