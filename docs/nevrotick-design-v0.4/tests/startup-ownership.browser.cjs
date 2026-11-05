'use strict';
// Only deterministic in-memory fixture commands. HTTP serves local build artifacts.
const {chromium}=require('playwright'),http=require('node:http'),fs=require('node:fs'),path=require('node:path'),assert=require('node:assert/strict');
const root=path.resolve(__dirname,'..'),screenshots=process.env.OWNERSHIP_SCREENSHOT_DIR;
(async()=>{
  const server=http.createServer((req,res)=>{
    const file=path.join(root,decodeURIComponent(new URL(req.url,'http://localhost').pathname));
    if(!file.startsWith(root+path.sep))return res.writeHead(403).end();
    try{res.end(fs.readFileSync(file));}catch{res.writeHead(404).end();}
  });
  await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve));
  const browser=await chromium.launch({executablePath:process.env.CHROMIUM_PATH||'/usr/bin/chromium',headless:true,args:['--no-sandbox']});
  const records=[];
  try{
    for(const file of ['index.html','NevroTick-UI-v0.4.html'])for(const width of [360,1280]){
      const page=await browser.newPage({viewport:{width,height:1000}}),errors=[],external=[];
      page.on('pageerror',e=>errors.push(e.message));
      page.on('request',r=>{if(!r.url().startsWith('http://127.0.0.1:')&&!r.url().startsWith('data:'))external.push(r.url());});
      const nav=async section=>page.locator(`.app-nav [data-value="${section}"]`).click();
      const settings=async()=>{await nav('settings');await page.locator('.settings-nav [data-value="service"]').click();};
      const open=async(scene='healthy')=>{
        await page.goto(`http://127.0.0.1:${server.address().port}/${file}?scene=${scene}`);
        await page.getByRole('button',{name:'Power user',exact:true}).click();await settings();
      };
      const reviewer=async()=>{if(!(await page.locator('#reviewer-controls').evaluate(e=>e.open)))await page.locator('#reviewer-controls > summary').click();};
      const op=value=>page.locator(`#background-service [data-action="lifecycle-request"][data-value="${value}"]`);
      const runtime=value=>page.locator(`#background-service [data-action="service-request"][data-value="${value}"]`);
      const submit=async value=>{
        await op(value).click();assert.equal(await page.locator('dialog select').count(),0);
        await page.locator('dialog [data-action="lifecycle-confirm"]').focus();await page.keyboard.press('Enter');
        assert.equal(await page.locator('dialog').count(),0);
        assert.match(await page.locator('#lifecycle-response').innerText(),/pending/i);
        assert.equal(await page.locator('#lifecycle-response').evaluate(e=>e===document.activeElement),true,'confirmation retains focus in local feedback');
      };
      const deliver=async(value='confirmed')=>{
        await reviewer();await page.locator('#sample-lifecycle-response').selectOption(value);
        await page.getByRole('button',{name:'Deliver sample lifecycle response',exact:true}).click();
        assert.notEqual(await page.evaluate(()=>document.activeElement.tagName),'BODY');
      };
      const run=async(value,response)=>{await submit(value);await deliver(response);};
      const shot=async label=>{
        assert.equal(await page.locator('body').evaluate(e=>e.scrollWidth<=innerWidth),true,`${label}: no overflow at ${width}`);
        if(screenshots){fs.mkdirSync(screenshots,{recursive:true});await page.locator('.app-window').screenshot({style:'.skip-link:not(:focus){visibility:hidden!important}',path:path.join(screenshots,`${file.startsWith('index')?'local':'standalone'}-${label}-${width}.png`)});}
      };
      const exportText=async()=>{
        await nav('diagnostics');await page.getByRole('button',{name:'Export preview',exact:true}).click();
        const text=await page.locator('.export-preview').innerText();await page.keyboard.press('Escape');return text;
      };
      await open('roughtime-recovered');
      await nav('diagnostics');
      const originalJournal=await page.locator('.event-list').innerText();
      assert.equal(await page.locator('.events-panel h2').innerText(),'Recent events');
      await page.getByRole('button',{name:'Preview temporary collection',exact:true}).click();
      await page.locator('#sample-log-scope').selectOption('both');await page.getByRole('button',{name:'Start example',exact:true}).click();
      await page.keyboard.press('Escape');await settings();
      await op('startup-off').click();await page.keyboard.press('Escape');
      assert.equal(await op('startup-off').evaluate(e=>e===document.activeElement),true);
      assert.equal(await page.getByRole('switch',{name:'Start automatically with Windows'}).getAttribute('aria-checked'),'true');
      await submit('startup-off');
      assert.equal(await page.locator('#background-service [data-action="service-request"]').evaluateAll(es=>es.every(e=>e.disabled)),true);
      await nav('status');assert.match(await page.locator('#state-title').innerText(),/Time sync active/);
      await nav('diagnostics');assert.match(await page.locator('.sample-log-summary').innerText(),/Sample response · Active/);
      assert.equal(await page.locator('.events-panel h2').innerText(),'Recent events');
      assert.doesNotMatch(await page.locator('.events-panel').innerText(),/previous snapshot/);
      assert.equal(await page.locator('.event-list').innerText(),originalJournal);
      assert.doesNotMatch(await page.locator('.sample-log-summary').innerText(),/Unknown|Prior session ended/);
      await settings();await deliver();
      assert.equal(await page.getByRole('switch',{name:'Start automatically with Windows'}).getAttribute('aria-checked'),'false');
      assert.match(await page.locator('.service-facts').innerText(),/Both services running/);
      await shot('autostart-off-running');
      await run('startup-on');assert.match(await page.locator('#lifecycle-response').innerText(),/Autostart On/);
      await run('startup-off','unknown');
      assert.match(await page.locator('#lifecycle-response').innerText(),/Autostart outcome unknown/);
      assert.equal(await page.getByRole('switch',{name:'Start automatically with Windows'}).count(),0);
      assert.equal(await runtime('stop').isEnabled(),true,'unknown configuration does not erase runtime/owner facts');
      await nav('status');assert.match(await page.locator('#state-title').innerText(),/Time sync active/);
      await nav('diagnostics');assert.match(await page.locator('.sample-log-summary').innerText(),/Sample response · Active/);
      await open('service-stopped');await run('startup-off');await run('startup-on');
      assert.match(await page.locator('.service-facts').innerText(),/Both services stopped/);
      await open();await op('disable').click();assert.match(await page.locator('dialog').innerText(),/Automatic synchronization will stop/);
      await page.keyboard.press('Escape');assert.equal(await op('disable').evaluate(e=>e===document.activeElement),true);
      await submit('disable');await reviewer();await page.getByRole('button',{name:'Сбросить сцену и настройки',exact:true}).click();
      assert.match(await page.locator('dialog').innerText(),/Pending or unknown service examples/);
      await page.getByRole('button',{name:'Cancel',exact:true}).click();await deliver();
      assert.match(await page.locator('#lifecycle-response').innerText(),/NevroTick fully disabled/);
      assert.match(await page.locator('.time-management').innerText(),/No active synchronizer/);
      await shot('fully-disabled');
      await runtime('start').click();assert.match(await page.locator('dialog').innerText(),/Enable and start/);
      assert.match(await page.locator('dialog').innerText(),/Manual for this session. Autostart stays Off/);
      await page.locator('dialog [data-action="service-confirm"]').click();await reviewer();
      await page.getByRole('button',{name:'Deliver sample service response',exact:true}).click();
      assert.equal(await page.getByRole('switch',{name:'Start automatically with Windows'}).getAttribute('aria-checked'),'false');
      assert.match(await page.locator('.service-facts').innerText(),/Both services running/);
      assert.doesNotMatch(await page.locator('#lifecycle-response').innerText(),/NevroTick fully disabled/);
      await run('use-nevrotick');await nav('status');assert.match(await page.locator('#state-title').innerText(),/Reacquiring time/);
      assert.doesNotMatch(await page.locator('.status-summary').innerText(),/3 of 4 identities/);
      for(const condition of ['policy-blocked','dependency-unavailable','dependency-unknown','ownership-uncertain']){
        await open();await reviewer();await page.locator('#sample-lifecycle-preflight').selectOption(condition);
        await op('use-windows').click();assert.match(await page.locator('dialog').innerText(),/Cannot continue/);
        assert.equal(await page.locator('dialog [data-action="lifecycle-confirm"]').count(),0);
        assert.equal(await page.getByRole('button',{name:/^Continue\./}).isDisabled(),true);
        if(condition==='policy-blocked')await shot('policy-blocked');
        await page.keyboard.press('Escape');assert.match(await page.locator('.service-facts').innerText(),/Both services running/);
        assert.equal(await page.getByRole('switch',{name:'Start automatically with Windows'}).getAttribute('aria-checked'),'true');
      }
      for(const outcome of ['failed','uac-cancelled','cleanup-unconfirmed','windows-unavailable','unknown']){
        await open();await run('use-windows',outcome);
        const text=await page.locator('#lifecycle-response').innerText();
        if(['failed','uac-cancelled'].includes(outcome)){
          assert.match(await page.locator('.service-facts').innerText(),/Both services running/);
          assert.equal(await op('use-windows').isEnabled(),true);
        }else if(outcome==='cleanup-unconfirmed'){
          assert.match(text,/Partial · cleanup unconfirmed/);assert.match(text,/full disabling is not confirmed/);
          assert.match(await page.locator('.service-facts').innerText(),/Both services stopped/);
          assert.equal(await op('use-windows').isDisabled(),true);await shot('cleanup-partial');
        }else if(outcome==='windows-unavailable'){
          assert.match(text,/Automatic synchronization is off/);assert.match(await page.locator('.time-management').innerText(),/No active synchronizer/);
        }else{
          assert.match(text,/Outcome unknown/);assert.equal(await runtime('start').isDisabled(),true);
          assert.equal(await op('use-windows').isDisabled(),true);await shot('unknown');
        }
      }
      await open('roughtime-recovered');await reviewer();await page.locator('#sample-lifecycle-preflight').selectOption('managed-allowed');
      await op('use-windows').click();assert.match(await page.locator('dialog').innerText(),/Manual · trigger-start/);
      assert.match(await page.locator('dialog').innerText(),/Existing peers, domain settings and organization policy are preserved/);
      await page.keyboard.press('Escape');await run('use-windows');
      assert.match(await page.locator('#lifecycle-response').innerText(),/independently confirmed/);
      await shot('windows-managed-settings');
      assert.match(await page.locator('main').innerText(),/Applies to NevroTick; Windows Time keeps its existing settings/);
      await page.getByRole('button',{name:'Simple',exact:true}).click();
      assert.match(await page.locator('.simple-service-summary').innerText(),/Windows Time continues to manage time/);
      assert.doesNotMatch(await page.locator('.simple-service-summary').innerText(),/Automatic synchronization is paused/);
      await page.getByRole('button',{name:'Power user',exact:true}).click();
      await nav('status');assert.match(await page.locator('#state-title').innerText(),/Windows Time manages time/);
      assert.match(await page.locator('.status-summary').innerText(),/Historical NevroTick evidence only/);
      assert.equal(await page.locator('.app-window [data-action="sync-now"]').count(),0);
      assert.match(await page.locator('.app-footer').innerText(),/Windows Time/);await shot('windows-managed-status');
      for(const section of ['sources','statistics']){
        await nav(section);assert.match(await page.locator('.time-manager-summary').innerText(),/Windows Time/);
        assert.match(await page.locator('main').innerText(),/Historical/);
      }
      await nav('sources');await page.locator('[data-action="source-details"][data-value="alpha"]').first().click();
      assert.match(await page.locator('dialog').innerText(),/Current time manager: Windows Time/);
      assert.match(await page.locator('dialog').innerText(),/NTS · prior evidence only/);await page.keyboard.press('Escape');
      const exported=await exportText();
      assert.equal(await page.locator('.events-panel h2').innerText(),'Retained events · before service operation');
      assert.match(await page.locator('.events-panel').innerText(),/Event ages and status wording are from the previous snapshot; they do not describe current synchronization/);
      assert.equal(await page.locator('.event-list').innerText(),originalJournal);
      assert.match(exported,/Clock ownership: Windows Time/);assert.match(exported,/Stored NevroTick profile \(inactive\): NTS only/);
      assert.match(exported,/Windows startup: manual-trigger/);assert.match(exported,/Windows synchronization: Not yet verified/);
      assert.match(exported,/Recovery ancestry: Signed Roughtime date basis/);
      await settings();await runtime('start').click();assert.match(await page.locator('dialog').innerText(),/observation only/);
      await page.locator('dialog [data-action="service-confirm"]').click();await reviewer();
      await page.getByRole('button',{name:'Deliver sample service response',exact:true}).click();
      await nav('status');assert.match(await page.locator('#state-title').innerText(),/Windows Time manages time/);
      await page.getByRole('button',{name:'Check sources',exact:true}).click();
      assert.match(await page.locator('#sync-response').innerText(),/observation only/);assert.match(await page.locator('#sync-response').innerText(),/no Windows resync or clock write/);
      await settings();await runtime('stop').click();
      assert.match(await page.locator('dialog').innerText(),/Windows Time continues to manage time/);
      assert.doesNotMatch(await page.locator('dialog').innerText(),/clock may drift|NevroTick starts for observation/);
      await page.keyboard.press('Escape');await run('disable');assert.match(await page.locator('#lifecycle-response').innerText(),/Windows Time remains the clock manager/);
      await run('use-nevrotick');assert.match(await page.locator('.service-facts').innerText(),/Both services running/);
      assert.match(await page.locator('.service-facts').innerText(),/stopped · Disabled/);
      assert.equal(await page.getByRole('switch',{name:'Start automatically with Windows'}).getAttribute('aria-checked'),'false');
      await nav('status');assert.match(await page.locator('#state-title').innerText(),/Reacquiring time/);await shot('reverse-reacquiring');
      assert.match(await exportText(),/Last NevroTick ownership generation: 13/);
      await open();await run('disable');await runtime('start').click();
      await page.locator('dialog [data-action="service-confirm"]').click();await reviewer();
      await page.locator('#sample-service-response').selectOption('unknown');
      await page.getByRole('button',{name:'Deliver sample service response',exact:true}).click();
      assert.match(await page.locator('.service-facts').innerText(),/Time measurements: Unknown. Clock control: Unknown/);
      assert.match(await exportText(),/Fully disabled: false/);
      assert.deepEqual(errors,[]);assert.deepEqual(external,[]);
      records.push({file,width,checks:'startup pending/success/unknown + DEBUG; full disable/cancel/enable; external preflight blockers; partial/unknown/rejection; Windows in all views/export; observation-only Start; explicit reverse; retained journal scope/raw facts; focus/reset/overflow',pageErrors:0,externalRequests:0});
      await page.close();
    }
    console.log(JSON.stringify(records,null,2));
  }finally{await browser.close();server.close();}
})().catch(error=>{console.error(error);process.exitCode=1;});
