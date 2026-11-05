'use strict';
// Only recovery history after simulated service operations; no Windows effects.
const {chromium}=require('playwright');
const http=require('node:http');
const fs=require('node:fs');
const path=require('node:path');
const assert=require('node:assert/strict');
const root=path.resolve(__dirname,'..');
const M=require('../src/model.js');
const scenes=['healthy','roughtime','roughtime-date-ready','roughtime-recovered'];
(async()=>{
  const server=http.createServer((req,res)=>{
    const file=path.join(root,decodeURIComponent(new URL(req.url,'http://localhost').pathname));
    if(!file.startsWith(root+path.sep))return res.writeHead(403).end();
    try{res.end(fs.readFileSync(file));}catch{res.writeHead(404).end();}
  });
  await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve));
  const browser=await chromium.launch({executablePath:process.env.CHROMIUM_PATH||'/usr/bin/chromium',headless:true,args:['--no-sandbox']});
  const results=[];
  try{
    for(const file of ['index.html','NevroTick-UI-v0.4.html'])for(const width of [360,1280])for(const scene of scenes){
      const page=await browser.newPage({viewport:{width,height:1000}});
      const errors=[],external=[];
      page.on('pageerror',e=>errors.push(e.message));
      page.on('request',req=>{if(!req.url().startsWith('http://127.0.0.1:')&&!req.url().startsWith('data:'))external.push(req.url());});
      const url=`http://127.0.0.1:${server.address().port}/${file}?scene=${scene}`;
      const raw=M.getScenario(scene).roughtimeEvidence;
      const openSettings=async()=>{
        await page.locator('.app-nav [data-value="settings"]').click();
        const simple=page.getByRole('button',{name:'Manage service in Power user',exact:true});
        if(await simple.count())await simple.click();
        else await page.locator('.settings-nav [data-value="service"]').click();
      };
      const submit=async op=>{
        await page.locator(`[data-action="service-request"][data-value="${op}"]`).click();
        await page.locator('dialog [data-action="service-confirm"]').click();
        assert.match(await page.locator('#service-response').innerText(),/Pending/);
      };
      const deliver=async()=>{
        if(!(await page.locator('#reviewer-controls').evaluate(e=>e.open)))await page.locator('#reviewer-controls > summary').click();
        await page.getByRole('button',{name:'Deliver sample service response',exact:true}).click();
      };
      const checkHistory=async()=>{
        const history=page.locator('.roughtime-history');
        if(scene==='healthy'){
          assert.equal(await history.count(),0,'no invented Roughtime evidence for a previously healthy path');
          return;
        }
        assert.equal(await history.count(),1);
        assert.equal(await history.evaluate(e=>e.open),false,'previous recovery starts folded as history');
        assert.match(await history.locator('.section-heading .badge').textContent(),/^Previous /);
        assert.doesNotMatch(await history.locator('.section-heading .badge').textContent(),/NTS tracking active|verification pending|Checking signatures/);
        await history.locator(':scope > summary').click();
        const facts=await history.innerText();
        assert.match(facts,/Previous observation before service operation:/);
        assert.match(facts,/Previous handoff before service operation:/);
        assert.match(facts,/Current handoff is unconfirmed/);
        assert.match(facts,/previous observation|previous service observation/);
        assert.ok(facts.includes(raw.signatureStatus));
        assert.ok(facts.includes(raw.ancestry));
        if(raw.approximateDateInterval)assert.ok(facts.includes(raw.approximateDateInterval));
      };
      const checkSurfaces=async(stage)=>{
        await page.locator('.app-nav [data-value="diagnostics"]').click();
        await checkHistory();
        const technical=page.locator('main details').filter({has:page.locator('[data-action="page"][data-value="recovery"]')});
        await technical.locator(':scope > summary').click();
        await page.getByRole('button',{name:'View date recovery',exact:true}).click();
        assert.match(await page.locator('.recovery-current h2').innerText(),/Current recovery stage unavailable after service operation/);
        assert.equal(await page.locator('.recovery-path').evaluate(e=>e.open),false);
        assert.equal(await page.locator('.steps li.current').count(),0);
        assert.doesNotMatch(await page.locator('.recovery-current').innerText(),/Healthy NTS stops here|NTS tracking resumed|NTS tracking active|Checking signatures/);
        await checkHistory();
        assert.equal(await page.locator('body').evaluate(e=>e.scrollWidth<=innerWidth),true);
        results.push({file,width,scene,stage,diagnostics:'history',currentRecovery:'unavailable',pageErrors:0});
      };
      await page.goto(url);
      const fixture=await page.evaluate(id=>JSON.stringify(window.NevroTickModel.getScenario(id)),scene);
      await openSettings();await submit('stop');
      await checkSurfaces('pending');
      await deliver();await checkSurfaces('stopped');
      assert.equal(await page.evaluate(id=>JSON.stringify(window.NevroTickModel.getScenario(id)),scene),fixture);
      await page.goto(url);await openSettings();await submit('restart');await deliver();
      await checkSurfaces('restarted');
      assert.equal(await page.evaluate(id=>JSON.stringify(window.NevroTickModel.getScenario(id)),scene),fixture);
      assert.deepEqual(errors,[]);assert.deepEqual(external,[]);
      await page.close();
    }
    console.log(JSON.stringify({stageCases:results.length,surfacesChecked:results.length*2,results},null,2));
  }finally{await browser.close();server.close();}
})().catch(error=>{console.error(error);process.exitCode=1;});
