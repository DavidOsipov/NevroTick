'use strict';
// Run after node build.cjs, with Playwright and Chromium available.
const {chromium} = require('playwright');
const http = require('http');
const fs = require('fs');
const path = require('path');
const assert = require('node:assert/strict');
const root = path.resolve(__dirname,'..');
const types={'.html':'text/html','.css':'text/css','.js':'text/javascript','.avif':'image/avif'};
(async()=>{
  const server=http.createServer((req,res)=>{
    const file=path.join(root,decodeURIComponent(new URL(req.url,'http://localhost').pathname));
    if(!file.startsWith(root+path.sep))return res.writeHead(403).end();
    try{res.writeHead(200,{'Content-Type':types[path.extname(file)]||'application/octet-stream'}).end(fs.readFileSync(file));}
    catch{res.writeHead(404).end();}
  });
  await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve));
  const browser=await chromium.launch({executablePath:process.env.CHROMIUM_PATH||'/usr/bin/chromium',headless:true,args:['--no-sandbox']});
  try{
    for(const file of ['index.html','NevroTick-UI-v0.4.html'])for(const width of [360,480,1280]){
      const page=await browser.newPage({viewport:{width,height:800}});
      const errors=[];page.on('pageerror',e=>errors.push(e.message));
      const go=scene=>page.goto(`http://127.0.0.1:${server.address().port}/${file}${scene?'?scene='+scene:''}`);
      const button=name=>page.getByRole('button',{name,exact:true});
      const nav=name=>page.locator('.app-nav').getByRole('button',{name,exact:true});
      const focus=()=>page.evaluate(()=>({id:document.activeElement.id,action:document.activeElement.dataset.action,value:document.activeElement.dataset.value}));
      await go();
      await button('Power user').click();await nav('Sources').click();
      await button('Waiting / not used').click();await page.locator('#source-search').fill('delta');
      await page.locator('[aria-label="Sources table"]').evaluate(x=>x.scrollLeft=80);
      await button('Explore recovery').focus();await page.evaluate(()=>window.scrollTo(0,240));
      const originY=await page.evaluate(()=>window.scrollY);
      const tableX=await page.locator('[aria-label="Sources table"]').evaluate(x=>x.scrollLeft);
      await page.keyboard.press('Enter');
      await button('Back to sources').click();
      assert.equal(await page.locator('#source-search').inputValue(),'delta');
      assert.equal(await button('Waiting / not used').getAttribute('aria-pressed'),'true');
      assert.equal((await focus()).value,'recovery');
      assert.ok(Math.abs(await page.evaluate(()=>window.scrollY)-originY)<=1,'restore document scroll');
      assert.equal(await page.locator('[aria-label="Sources table"]').evaluate(x=>x.scrollLeft),tableX);

      await button('Details for Delta').click();
      const recoveryLink=page.locator('dialog').getByRole('button',{name:'Review date recovery',exact:true});
      await page.locator('dialog details summary').click();
      await recoveryLink.focus();
      const dialogY=await page.locator('dialog').evaluate(x=>x.scrollTop);
      await page.keyboard.press('Enter');await button('Back to sources').click();
      assert.equal(await page.locator('dialog[open]').count(),1);
      assert.match(await page.locator('dialog').innerText(),/delta.example/);
      assert.equal(await page.locator('dialog details').evaluate(x=>x.open),true);
      assert.equal((await focus()).value,'recovery');
      assert.ok(Math.abs(await page.locator('dialog').evaluate(x=>x.scrollTop)-dialogY)<=1,'restore source dialog scroll');
      await page.keyboard.press('Escape');
      assert.equal((await focus()).value,'delta','closing restored details focuses its source');

      await nav('Diagnostics').click();
      await page.locator('#event-filter').selectOption('attention');
      const technical=page.locator('main details').filter({has:page.locator('[data-action="page"][data-value="recovery"]')});
      await technical.locator('summary').click();
      await button('View date recovery').click();await button('Back to diagnostics').click();
      assert.equal(await page.locator('#event-filter').inputValue(),'attention');
      assert.equal(await technical.evaluate(x=>x.open),true,'restore expanded origin so its button can receive focus');
      assert.equal((await focus()).value,'recovery');

      await nav('Settings').click();await button('Time policy').click();
      await button('Date recovery').click();await button('Back to time policy').click();
      assert.equal(await button('Time policy').getAttribute('aria-current'),'page');
      assert.equal((await focus()).value,'recovery');
      await button('Date recovery').click();
      await button('Simple').click();await button('Power user').click();
      assert.equal(await page.locator('h1').innerText(),'Date recovery');
      await button('Simple').click();await nav('About').click();await button('Power user').click();
      assert.equal(await page.locator('h1').innerText(),'About NevroTick');

      await nav('Sources').click();await button('Explore recovery').click();
      await button('Configure helper preview').click();
      await page.locator('#recovery-count').selectOption('2');
      await page.locator('#recovery-identity-1').fill('one.example');
      await page.locator('#recovery-identity-2').fill('unfinished /');
      const tip=page.locator('#recovery-form details.help-tip');
      await tip.locator('summary').click();await page.keyboard.press('Escape');
      assert.equal(await tip.evaluate(x=>x.open),false);
      assert.equal(await page.locator('#recovery-form').count(),1);
      await page.evaluate(()=>window.helperEditorDialog=document.getElementById('dialog'));
      await page.keyboard.press('Escape');
      assert.equal(await page.locator('dialog[open]').count(),1);
      assert.equal(await page.evaluate(()=>window.helperEditorDialog===document.getElementById('dialog')),true,'leave warning uses the same dialog node');
      assert.equal(await button('Keep draft and leave').count(),1);
      await page.keyboard.press('Escape');
      assert.equal(await page.locator('#recovery-identity-2').inputValue(),'unfinished /');
      await page.locator('dialog').getByRole('button',{name:'Cancel',exact:true}).click();
      await button('Keep draft and leave').click();
      assert.equal(await page.locator('dialog[open]').count(),0);
      assert.match(await page.locator('.helper-draft-status').innerText(),/Helper draft not applied/);
      await button('Review helper preview').click();
      assert.equal(await page.locator('#recovery-identity-2').getAttribute('aria-invalid'),'true','review must validate kept partial input');
      assert.equal(await page.locator('#recovery-identity-2').inputValue(),'unfinished /');
      await page.locator('#recovery-identity-2').fill('two.example');
      await page.locator('dialog').getByRole('button',{name:'Review helper preview',exact:true}).click();
      assert.match(await page.locator('dialog').innerText(),/one.example/);
      await page.keyboard.press('Escape');
      assert.equal(await page.locator('dialog[open]').count(),0,'saved preview closes without leave warning');
      await button('Configure helper preview').click();
      await page.locator('#recovery-identity-1').fill('edited.example');
      await page.keyboard.press('Escape');await button('Discard edits').click();
      await button('Configure helper preview').click();
      assert.equal(await page.locator('#recovery-identity-1').inputValue(),'one.example');
      await page.keyboard.press('Escape');
      assert.equal(await page.locator('dialog[open]').count(),0);

      await page.locator('#reviewer-controls > summary').click();
      await page.locator('#scenario').selectOption('reserve');
      await button('Cancel').click();
      assert.equal(await page.locator('#scenario').inputValue(),'healthy');
      await button('Review helper preview').click();
      assert.match(await page.locator('dialog').innerText(),/one.example/);
      await page.keyboard.press('Escape');
      await page.locator('#scenario').selectOption('reserve');await button('Clear local previews and continue').click();
      assert.equal(await page.locator('dialog[open]').count(),0);
      assert.equal(await page.locator('.helper-draft-status').count(),0);
      assert.equal(await page.locator('h1').innerText(),'Clock status');

      await go('reserve');await button('What can I do?').click();
      await button('Review date recovery in Power user').click();
      assert.equal(await button('Power user').getAttribute('aria-pressed'),'true');
      await button('Back to status').click();
      assert.equal(await button('Simple').getAttribute('aria-pressed'),'true');
      assert.equal((await focus()).value,'status-help');
      await nav('About').click();assert.equal(await page.locator('h1').innerText(),'About NevroTick');
      await button('Power user').click();await nav('Sources').click();
      await page.evaluate(()=>{const x=document.createElement('button');x.id='temporary-recovery-entry';x.dataset.action='page';x.dataset.value='recovery';x.textContent='Temporary recovery entry';document.querySelector('main').append(x);});
      await button('Temporary recovery entry').click();await button('Back to sources').click();
      assert.equal((await focus()).id,'page-title','missing origin falls back to the page heading');
      assert.equal(await page.evaluate(()=>document.documentElement.scrollWidth>innerWidth),false,'no viewport overflow');
      assert.deepEqual(errors,[]);
      console.log(`${file} ${width}px: recovery context, drafts, guards, keyboard and About passed`);
      await page.close();
    }
  }finally{await browser.close();server.close();}
})().catch(e=>{console.error(e);process.exitCode=1;});
