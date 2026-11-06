'use strict';
// Deterministic HTML samples only. The browser notification API is trapped, not called.
const {chromium}=require('playwright'),http=require('node:http'),fs=require('node:fs'),path=require('node:path'),assert=require('node:assert/strict');
const M=require('../src/model.js'),root=path.resolve(__dirname,'..'),shots=process.env.NOTIFICATION_SCREENSHOT_DIR;
const immediate=new Set(['expired','conflict','service-stopped','blocked','signed-conflict','correction','outcome-unknown']);
const persistent=new Set(['server-access-refused','holdover','offline','stale','leap-guard','roughtime-unavailable','roughtime-rejected','all-rate-limited']);
(async()=>{
 const server=http.createServer((req,res)=>{const file=path.join(root,decodeURIComponent(new URL(req.url,'http://localhost').pathname));if(!file.startsWith(root+path.sep))return res.writeHead(403).end();try{res.end(fs.readFileSync(file))}catch{res.writeHead(404).end()}});
 await new Promise(r=>server.listen(0,'127.0.0.1',r));
 const browser=await chromium.launch({executablePath:process.env.CHROMIUM_PATH||'/usr/bin/chromium',headless:true,args:['--no-sandbox']});
 const records=[];
 try{for(const file of ['index.html','NevroTick-UI-v0.4.html'])for(const width of [360,1280]){
  const page=await browser.newPage({viewport:{width,height:1000}}),errors=[],external=[];
  page.on('pageerror',e=>errors.push(e.message));page.on('request',r=>{if(!r.url().startsWith('http://127.0.0.1:')&&!r.url().startsWith('data:'))external.push(r.url())});
  await page.addInitScript(()=>{window.__notificationAttempts=[];Object.defineProperty(window,'Notification',{configurable:true,value:class{constructor(){window.__notificationAttempts.push('construct');throw new Error('Native browser notification forbidden in HTML preview')}static requestPermission(){window.__notificationAttempts.push('permission');throw new Error('Browser permission forbidden in HTML preview')}}})});
  const fresh=async(scene='healthy')=>page.goto(`http://127.0.0.1:${server.address().port}/${file}?scene=${scene}`);
  const reviewer=async()=>{if(!await page.locator('#reviewer-controls').evaluate(e=>e.open))await page.locator('#reviewer-controls > summary').click()};
  const pub=async()=>{await reviewer();await page.locator('[data-action="notification-observe"]').click()};
  const power=async()=>{if(await page.locator('[data-action="view-mode"][data-value="power"]').getAttribute('aria-pressed')==='false')await page.getByRole('button',{name:'Power user',exact:true}).click()};
  const nav=async value=>page.locator(`.app-nav [data-value="${value}"]`).click();
  const settings=async(section='service')=>{await power();await nav('settings');await page.locator(`.settings-nav [data-value="${section}"]`).click()};
  const life=async(op,response='confirmed')=>{await settings();await page.locator(`#background-service [data-action="lifecycle-request"][data-value="${op}"]`).click();await page.locator('dialog [data-action="lifecycle-confirm"]').click();await reviewer();await page.locator('#sample-lifecycle-response').selectOption(response);await page.locator('[data-action="sample-lifecycle-deliver"]').click()};
  const runtime=async(op,response='confirmed')=>{await settings();await page.locator(`#background-service [data-action="service-request"][data-value="${op}"]`).click();await page.locator('dialog [data-action="service-confirm"]').click();await reviewer();await page.locator('#sample-service-response').selectOption(response);await page.locator('[data-action="sample-service-deliver"]').click()};
  const count=()=>page.locator('.windows-toast').count();
  const key=()=>page.locator('.windows-toast [data-action="notification-open"]').getAttribute('data-value');
  const shot=async(label,selector='.app-window')=>{assert.equal(await page.locator('body').evaluate(e=>e.scrollWidth<=innerWidth),true,label+' overflow');if(shots){fs.mkdirSync(shots,{recursive:true});await page.locator(selector).screenshot({path:path.join(shots,`${file.startsWith('index')?'local':'standalone'}-${label}-${width}.png`)})}};

  await fresh();await nav('settings');assert.equal(await page.locator('#notify-problems').isChecked(),true);
  await page.getByRole('button',{name:'Send test notification',exact:true}).click();assert.equal(await count(),1);
  assert.match(await page.locator('.windows-toast').innerText(),/Test notification.*does not verify clock synchronization/s);
  assert.equal(await page.evaluate(()=>document.activeElement.id),'notification-sample-status');
  await page.locator('.windows-toast [data-action="notification-open"]').click();assert.equal(await page.locator('.settings-nav [aria-current="page"]').getAttribute('data-value'),'notifications');
  await shot('notification-settings');
  await page.locator('#notify-problems').uncheck();await page.getByRole('button',{name:'Send test notification',exact:true}).click();assert.equal(await count(),0);assert.match(await page.locator('#notification-sample-status').innerText(),/notifications are off/);
  await page.getByRole('button',{name:'Simple',exact:true}).click();assert.equal(await page.locator('#notify-problems').isChecked(),false);await page.locator('#notify-problems').check();
  await page.getByRole('button',{name:'Notification details in Power user',exact:true}).click();assert.equal(await page.locator('#notify-problems').isChecked(),true);
  await page.getByRole('button',{name:'Windows notification settings preview',exact:true}).click();assert.match(await page.locator('dialog').innerText(),/Windows Settings → System → Notifications → NevroTick/);await page.keyboard.press('Escape');

  await fresh('conflict');await pub();assert.equal(await count(),1);assert.match(await page.locator('.windows-toast').innerText(),/Time sources disagree/);const incident=await key();
  await shot('notification-conflict','.windows-notification-preview');
  await pub();assert.equal(await key(),incident);await page.locator('[data-action="notification-dismiss"]').click();assert.equal(await count(),0);assert.equal(await page.evaluate(()=>document.activeElement.id),'notification-sample-status');
  await pub();assert.equal(await count(),0);assert.match(await page.locator('#notification-sample-status').innerText(),/No duplicate/);
  await fresh('conflict');await pub();await page.locator('.windows-toast [data-action="notification-open"]').click();assert.equal(await page.locator('.app-nav [aria-current="page"]').getAttribute('data-value'),'statistics');assert.equal(await page.locator('dialog').count(),0);

  await fresh('offline');await pub();await pub();assert.equal(await count(),0);await page.locator('#sample-notification-persistent').check();await pub();assert.equal(await count(),1);assert.match(await page.locator('.windows-toast').innerText(),/not replying/);await shot('notification-persistent','.windows-notification-preview');
  for(const os of ['blocked-app','blocked-policy','suppressed','unknown','unsupported']){
   await fresh('expired');await reviewer();await page.locator('#sample-notification-os').selectOption(os);await pub();assert.equal(await count(),0,os);
   await page.locator('#sample-notification-os').selectOption('allowed');await pub();assert.equal(await count(),1,os+' readiness restored');assert.equal(await key(),'nt-1');
   await page.locator('#sample-notification-os').selectOption(os);await pub();assert.equal(await count(),0);await page.locator('#sample-notification-os').selectOption('allowed');await pub();assert.equal(await count(),0,os+' no repeat once already shown');
  }
  await fresh('expired');await reviewer();await page.locator('#sample-notification-presenter').selectOption('missing');await pub();assert.equal(await count(),0);await settings('notifications');assert.match(await page.locator('.notification-settings').innerText(),/App and tray are not running/);await shot('notifications-no-presenter');await reviewer();await page.locator('#sample-notification-presenter').selectOption('active');await pub();assert.equal(await count(),1);

  await fresh('conflict');await pub();const beforeCancelled=await key();await settings();await page.locator('#background-service [data-action="service-request"][data-value="stop"]').click();await page.locator('dialog [data-action="service-confirm"]').click();await pub();assert.equal(await count(),0);await page.locator('#sample-service-response').selectOption('uac-cancelled');await page.locator('[data-action="sample-service-deliver"]').click();await pub();assert.equal(await count(),0,'a pending gap and cancelled attempt do not reannounce the old clock incident');assert.match(await page.locator('#notification-sample-status').innerText(),/already previewed/);
  await fresh();await life('use-windows','windows-unavailable');await reviewer();await page.locator('#sample-notification-os').selectOption('blocked-app');await pub();assert.equal(await count(),0);await life('use-windows','uac-cancelled');await reviewer();await page.locator('#sample-notification-os').selectOption('allowed');await pub();assert.equal(await count(),1);assert.match(await page.locator('.windows-toast').innerText(),/No active synchronizer/);
  await fresh();await life('use-windows','windows-unavailable');await pub();const partialId=await key();await life('use-windows','failed');await pub();assert.equal(await key(),partialId,'a rejected retry retains the same incomplete handover incident');
  await fresh();await life('startup-off','unknown');await pub();const startupId=await key();await runtime('stop','uac-cancelled');await pub();assert.equal(await key(),startupId,'an unrelated rejected runtime attempt retains the original autostart issue scope');

  for(const scene of M.SCENARIOS){
   await fresh(scene.id);await pub();const expected=immediate.has(scene.id);
   assert.equal(!!await count(),expected,scene.id+' immediate');
   if(persistent.has(scene.id)){await page.locator('#sample-notification-persistent').check();await pub();assert.equal(await count(),1,scene.id+' persistent')}
  }

  await fresh('service-stopped');await power();assert.equal(await page.locator('.status-actions [data-action="sync-now"]').count(),0);await page.locator('.status-actions [data-action="service-settings"]').click();assert.equal(await page.locator('.settings-nav [aria-current="page"]').getAttribute('data-value'),'service');
  await fresh('observe');await power();assert.equal(await page.locator('.status-actions [data-action="sync-now"]').innerText(),'Check sources');assert.match(await page.locator('.status-actions').innerText(),/does not apply a clock change/);
  await fresh('healthy');await runtime('stop');await pub();assert.equal(await count(),0,'planned Stop stays quiet');await nav('status');assert.equal(await page.locator('.status-actions [data-action="sync-now"]').count(),0);
  await fresh('healthy');await life('disable');await pub();assert.equal(await count(),0,'planned Disable stays quiet');
  await fresh('healthy');await life('startup-off','unknown');await pub();assert.match(await page.locator('.windows-toast').innerText(),/Autostart change is unconfirmed/);await nav('status');assert.equal(await page.locator('.status-actions [data-action="sync-now"]').innerText(),'Sync now');
  await fresh('healthy');await life('use-windows');await runtime('start','unknown');await pub();assert.match(await page.locator('.windows-toast').innerText(),/Windows Time remains.*manager/);await nav('status');assert.match(await page.locator('#state-title').innerText(),/Windows Time manages time/);await shot('windows-observer-unknown');

  await fresh('correction');await pub();await settings();await page.locator('#background-service [data-action="service-request"][data-value="stop"]').click();await page.locator('dialog [data-action="service-confirm"]').click();assert.match(await page.locator('.notification-earlier').innerText(),/Earlier notification/);await page.locator('.windows-toast [data-action="notification-open"]').click();assert.equal(await page.locator('dialog').count(),0);assert.match(await page.locator('#state-title').innerText(),/pending/);assert.equal(await page.locator('.status-actions button[data-action="sync-now"]').count(),0);

  await fresh('conflict');await pub();const oldId=await key();await reviewer();await page.locator('#scenario').selectOption('expired');assert.equal(await page.locator('dialog [data-action="confirm-reset"]').count(),1);await page.locator('dialog [data-action="cancel-reset"]').click();assert.equal(await key(),oldId);await page.locator('#scenario').selectOption('expired');await page.locator('dialog [data-action="confirm-reset"]').click();await pub();const newId=await key();assert.notEqual(newId,oldId);
  await page.locator('.windows-toast [data-action="notification-open"]').evaluate((e,id)=>e.dataset.value=id,oldId);await page.locator('.windows-toast [data-action="notification-open"]').click();assert.equal(await page.locator('dialog').count(),0);assert.equal(await count(),1,'retired identity cannot activate new contents');
  await page.locator('.windows-toast [data-action="notification-open"]').evaluate((e,id)=>e.dataset.value=id,newId);await page.locator('.windows-toast [data-action="notification-open"]').click();assert.equal(await page.locator('.app-nav [aria-current="page"]').getAttribute('data-value'),'diagnostics');

  // Source-level tests exercise native-style activation during dirty recovery
  // editing; the HTML dialog intentionally makes outside preview controls inert.
  assert.equal(await page.locator('body').evaluate(e=>e.scrollWidth<=innerWidth),true);
  assert.deepEqual(await page.evaluate(()=>window.__notificationAttempts),[]);assert.deepEqual(errors,[]);assert.deepEqual(external,[]);
  records.push({file,width,scenes:30,checks:'settings both modes/test/OS/DND/presenter; all30 policies; persistence/dedupe/dismiss; safe/current/stale/invalid activation; planned transitions; unknown scope; focus/reset/overflow',pageErrors:errors.length,externalRequests:external.length,browserNotificationCalls:0});
  await page.close();
 }}finally{await browser.close();await new Promise(r=>server.close(r))}
 console.log(JSON.stringify(records,null,2));
})().catch(e=>{console.error(e);process.exitCode=1});
