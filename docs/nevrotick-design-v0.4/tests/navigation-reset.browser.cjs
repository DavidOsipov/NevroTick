'use strict';
// Local source entry by default. Run against a built standalone too with NAV_TEST_FILES=index.html,NevroTick-UI-v0.4.html.
const {chromium}=require('playwright');
const http=require('node:http'),fs=require('node:fs'),path=require('node:path'),assert=require('node:assert/strict');
const root=path.resolve(__dirname,'..'),screens=process.env.NAV_SCREENSHOT_DIR;
(async()=>{
  const server=http.createServer((req,res)=>{
    const file=path.join(root,decodeURIComponent(new URL(req.url,'http://localhost').pathname));
    if(!file.startsWith(root+path.sep))return res.writeHead(403).end();
    try{res.end(fs.readFileSync(file));}catch{res.writeHead(404).end();}
  });
  await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve));
  const browser=await chromium.launch({executablePath:process.env.CHROMIUM_PATH||'/usr/bin/chromium',headless:true,args:['--no-sandbox']});
  try{
    for(const file of (process.env.NAV_TEST_FILES||'index.html').split(','))for(const width of [360,980]){
      const page=await browser.newPage({viewport:{width,height:760}}),errors=[];page.on('pageerror',e=>errors.push(e.message));
      const go=scene=>page.goto(`http://127.0.0.1:${server.address().port}/${file}?scene=${scene||'healthy'}`);
      const button=name=>page.getByRole('button',{name,exact:true});
      const nav=name=>page.locator('.app-nav').getByRole('button',{name,exact:true});
      const active=()=>page.evaluate(()=>({id:document.activeElement.id,tag:document.activeElement.tagName,action:document.activeElement.dataset?.action,value:document.activeElement.dataset?.value,y:scrollY}));
      const screenshot=async name=>{if(screens){fs.mkdirSync(screens,{recursive:true});await page.screenshot({path:path.join(screens,`${file.startsWith('index')?'local':'standalone'}-${width}-${name}.png`)});}};
      async function add(name){await button('Add source preview').click();await page.locator('#field-name').fill(name);await page.locator('#field-host').fill(name+'.example');await page.locator('#source-form button[type="submit"]').click();}
      async function assertViewport(before){const max=await page.evaluate(()=>Math.max(0,document.documentElement.scrollHeight-innerHeight));assert.ok(Math.abs((await active()).y-Math.min(before,max))<=1,'action preserves the current viewport within the new document bounds');}
      await go();await button('Power user').click();await nav('Sources').click();
      await page.locator('#source-search').fill('nomatch');await button('Clear filters').focus();const clearY=(await active()).y;await page.keyboard.press('Enter');
      assert.equal((await active()).id,'source-search');await assertViewport(clearY);await page.keyboard.press('Tab');assert.equal((await active()).action,'source-filter');await screenshot('clear-filter-focus');
      await add('one');await add('two');const undo=page.locator('[data-action="undo-draft"]');const secondId=await undo.nth(1).getAttribute('data-value');await undo.first().focus();const twoY=(await active()).y;await page.keyboard.press('Enter');
      assert.equal((await active()).action,'undo-draft');assert.equal((await active()).value,secondId);await assertViewport(twoY);
      await page.locator('[data-action="undo-draft"]').focus();const lastY=(await active()).y;await page.keyboard.press('Enter');assert.equal((await active()).action,'modal');assert.equal((await active()).value,'add-source');await assertViewport(lastY);await screenshot('undo-last-focus');
      for(const kind of ['active','partial','unconfirmed'])for(const trigger of ['reset','scenario']){
        await go(kind==='unconfirmed'?'stale':'healthy');await button('Power user').click();await page.locator('#reviewer-controls > summary').click();
        if(kind==='partial')await page.locator('#sample-log-service-response').selectOption('refused');
        await nav('Diagnostics').click();await button('Preview temporary collection').click();await page.locator('#sample-log-scope').selectOption(kind==='active'?'ui':'both');await button('Start example').click();await page.keyboard.press('Escape');
        const retained=await page.locator('.sample-log-summary').innerText();
        assert.match(retained,new RegExp(kind==='unconfirmed'?'Unconfirmed':kind==='partial'?'Partial':'Active'));
        const originalScene=kind==='unconfirmed'?'stale':'healthy';
        async function attempt(){if(trigger==='scenario'){await page.locator('#scenario').focus();await page.locator('#scenario').selectOption('holdover');}else{await page.locator('[data-action="reset"]').focus();await page.keyboard.press('Enter');}}
        await attempt();assert.equal(await page.locator('dialog[open]').count(),1);assert.equal((await active()).action,'cancel-reset');
        assert.match(await page.locator('dialog').innerText(),/does not stop or confirm.*real service/i);await screenshot(`${kind}-${trigger}-confirmation`);
        await page.keyboard.press('Enter');assert.equal(await page.locator('dialog[open]').count(),0);
        if(trigger==='scenario')assert.equal((await active()).id,'scenario');else assert.equal((await active()).action,'reset');
        assert.equal(await page.locator('#scenario').inputValue(),originalScene);assert.equal(await page.locator('.sample-log-summary').innerText(),retained);
        await button('Preview temporary collection').click();assert.match(await page.locator('#sample-log-result').innerText(),new RegExp(kind==='unconfirmed'?'Unconfirmed':kind==='partial'?'Partial':'Active'));await page.keyboard.press('Escape');
        await attempt();await page.locator('[data-action="confirm-reset"]').focus();await page.keyboard.press('Enter');assert.equal(await page.locator('dialog[open]').count(),0);
        assert.equal(await page.locator('#scenario').inputValue(),trigger==='scenario'?'holdover':originalScene);
        await nav('Diagnostics').click();await button('Preview temporary collection').click();const cleared=await page.locator('#sample-log-result').innerText();assert.match(cleared,/Sample response · Not started/);assert.doesNotMatch(cleared,/Sample DEBUG active|start and stop unconfirmed/);await page.keyboard.press('Escape');
      }
      await go('stale');await button('Power user').click();await nav('Diagnostics').click();await button('Preview temporary collection').click();
      await page.locator('#sample-log-scope').selectOption('both');await button('Start example').click();await button('Stop example').focus();await page.keyboard.press('Enter');
      const pending=await page.locator('#sample-log-result').innerText();assert.match(pending,/Stop requested · service stop remains unconfirmed/);assert.match(pending,/UI component\s+Stopped in sample/);assert.match(pending,/Service component\s+Unknown/);
      assert.equal(await page.getByRole('button',{name:/^Stop requested/}).isDisabled(),true);assert.equal(await button('Stop example').count(),0);assert.equal((await active()).action,'close');
      assert.equal(await page.evaluate(()=>{const box=document.activeElement.getBoundingClientRect();return box.top>=0&&box.bottom<=innerHeight;}),true,'the enabled Close action remains visible when the stop explanation expands');await screenshot('unknown-stop-requested');
      await page.keyboard.press('Escape');assert.match(await page.locator('.sample-log-summary').innerText(),/Stop requested · service stop remains unconfirmed/);await button('Preview temporary collection').click();assert.equal(await page.locator('#sample-log-result').innerText(),pending);assert.equal(await page.getByRole('button',{name:/^Stop requested/}).isDisabled(),true);
      assert.deepEqual(errors,[]);console.log(`${file} ${width}px: source keyboard focus, viewport, active/partial/unknown reset confirmation and pending Stop passed`);await page.close();
    }
  }finally{await browser.close();server.close();}
})().catch(e=>{console.error(e);process.exitCode=1});
