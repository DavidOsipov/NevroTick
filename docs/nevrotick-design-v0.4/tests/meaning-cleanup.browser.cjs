'use strict';
// Build first. All navigation and examples remain local to the prototype.
const {chromium}=require('playwright');
const http=require('node:http');
const fs=require('node:fs');
const path=require('node:path');
const assert=require('node:assert/strict');
const root=path.resolve(__dirname,'..');
const output=process.env.MEANING_REVIEW_DIR || '/tmp/nevrotick-audit-meaning-evidence';
const types={'.html':'text/html','.css':'text/css','.js':'text/javascript','.avif':'image/avif'};

(async()=>{
  fs.mkdirSync(output,{recursive:true});
  const server=http.createServer((req,res)=>{
    const file=path.join(root,decodeURIComponent(new URL(req.url,'http://localhost').pathname));
    if (!file.startsWith(root+path.sep)) return res.writeHead(403).end();
    try {res.writeHead(200,{'Content-Type':types[path.extname(file)] || 'application/octet-stream'}).end(fs.readFileSync(file));}
    catch {res.writeHead(404).end();}
  });
  await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve));
  const browser=await chromium.launch({executablePath:process.env.CHROMIUM_PATH || '/usr/bin/chromium',headless:true,args:['--no-sandbox']});
  const results=[];
  try {
    for (const file of ['index.html','NevroTick-UI-v0.4.html']) for (const width of [360,1280]) {
      const page=await browser.newPage({viewport:{width,height:900}});
      const errors=[],remote=[];
      page.on('pageerror',error=>errors.push(error.message));
      page.on('request',request=>{if (!request.url().startsWith('http://127.0.0.1:') && !request.url().startsWith('data:')) remote.push(request.url());});
      const button=name=>page.getByRole('button',{name,exact:true});
      const nav=name=>page.locator('.app-nav').getByRole('button',{name,exact:true});
      const load=scene=>page.goto(`http://127.0.0.1:${server.address().port}/${file}?scene=${scene}`);
      const capture=async name=>{
        assert.equal(await page.evaluate(()=>document.documentElement.scrollWidth>innerWidth),false,`${file} ${width}: ${name} overflow`);
        await page.screenshot({path:path.join(output,`${file==='index.html'?'index':'standalone'}-${width}-${name}.png`),fullPage:true});
      };
      await load('healthy');
      await button('Power user').click(); await nav('Settings').click();
      assert.equal(await page.locator('.schedule-details').count(),0);
      assert.match(await page.locator('#frequency-help').innerText(),/Recovery and server limits/);
      await button('Time policy').click();
      assert.doesNotMatch(await page.locator('main').innerText(),/Provider presets|exact correction bounds|unqualified universal threshold/);
      await button('Your preferences').click();
      assert.equal(await page.getByLabel('Appearance',{exact:true}).count(),0);
      assert.equal(await page.getByLabel('Start NevroTick in the tray at sign-in',{exact:true}).isEnabled(),false);
      await page.getByLabel('Keep NevroTick in the tray',{exact:true}).check();
      await page.getByLabel('Start NevroTick in the tray at sign-in',{exact:true}).check();
      assert.equal(await page.locator('#sample-signin').isChecked(),true);
      await capture('preferences');
      await page.getByLabel('Keep NevroTick in the tray',{exact:true}).uncheck();
      assert.equal(await page.locator('#sample-signin').isEnabled(),false);
      await nav('Sources').click();
      assert.equal(await page.getByText('Catalogue & admission',{exact:true}).count(),0);
      await button('Add source preview').click();
      assert.match(await page.locator('#help-name').innerText(),/Preview limit: 1–64 printable ASCII/);
      assert.doesNotMatch(await page.locator('#dialog').innerText(),/UI-M0|production protocol rule/);
      await button('Cancel').click(); await button('Explore recovery').click();
      assert.equal(await page.locator('.recovery-path').evaluate(el=>el.open),false);
      assert.equal(await page.locator('.roughtime-evidence,.recovery-summary,.future-reserve').count(),0);
      assert.equal(await page.getByText('Implementation scope',{exact:true}).count(),0);
      await capture('healthy-recovery');
      await page.locator('.recovery-path > summary').focus(); await page.keyboard.press('Enter');
      assert.equal(await page.locator('.steps').isVisible(),true);
      await nav('About').click(); await page.getByText('Technologies and libraries',{exact:true}).click();
      const about=await page.locator('.about-copy').innerText();
      for (const name of ['chrony','Tokio','garde','getrandom','windows-sys']) assert.ok(about.includes(name));
      assert.doesNotMatch(about,/loopback|Engine experiment/);

      for (const scene of ['server-access-refused','holdover','roughtime-unavailable','roughtime-rejected','signed-conflict']) {
        await load(scene);
        const simple=await page.locator('.simple-status').innerText();
        assert.doesNotMatch(simple,/previous model|previous time model|valid time model|Agreement not met|NTS not yet established|separate correction authority|certificate-time basis|active.*bootstrap/);
        if (scene==='server-access-refused') assert.match(simple,/Beta refused access.*Not enough time sources agree/s);
        if (scene==='holdover') {
          assert.match(simple,/Based on the last valid time estimate/);
          await capture('simple-last-valid-estimate');
        }
        if (scene.startsWith('roughtime-') || scene==='signed-conflict') {
          assert.match(simple,/Date recovery has not changed your clock/);
          assert.match(simple,/NTS checks must finish.*permission is still required/s);
        }
      }
      await nav('Settings').click();
      await button('Review time policy in Power user').click();
      assert.equal(await button('Time policy').getAttribute('aria-current'),'page');

      await load('healthy'); await button('Power user').click(); await nav('Sources').click();
      assert.equal(await page.locator('.source-table thead th').nth(3).innerText(),'Last measurement reply');
      await nav('Statistics').click();
      const headers=await page.locator('.statistics-table thead th').allInnerTexts();
      assert.deepEqual(headers.slice(-2),['Last measurement reply','Last accepted measurement']);
      await page.getByText('Measurement context',{exact:true}).click();
      assert.doesNotMatch(await page.locator('main').innerText(),/Preview check preference|The sample schedule is unchanged/);
      await button('delta.example').click();
      const fact=label=>page.locator('.dialog-facts .fact').filter({has:page.locator('dt',{hasText:new RegExp('^'+label+'$')})}).locator('dd').innerText();
      assert.equal(await fact('Last measurement reply'),'8 s ago');
      assert.equal(await fact('Last accepted measurement'),'Not accepted');
      assert.equal(await fact('Last authenticated time measurement'),'8 s ago');
      await page.locator('.dialog-header').getByRole('button',{name:'Close dialog',exact:true}).click();

      await load('ntp'); await nav('Settings').click();
      assert.match(await page.locator('.date-recovery-applicability').innerText(),/not used by the current plain-NTP profile/);
      await button('Review date recovery in Power user').click();
      assert.match(await page.locator('.notice').innerText(),/not used by the current plain-NTP profile/);
      await nav('Sources').click(); await button('alpha.example').click();
      assert.equal(await fact('Last authenticated time measurement'),'Not applicable — plain NTP');
      await page.locator('.dialog-header').getByRole('button',{name:'Close dialog',exact:true}).click();
      await nav('Settings').click(); await button('Time policy').click();
      assert.match(await page.locator('.date-recovery-applicability').innerText(),/used for secure NTS checks/);

      await load('roughtime-date-ready'); await nav('Settings').click(); await button('Review date recovery in Power user').click();
      assert.equal(await page.locator('.recovery-path').evaluate(el=>el.open),true);
      assert.equal(await page.locator('.roughtime-evidence').isVisible(),true);
      assert.equal(await page.locator('.future-reserve').count(),0);
      assert.match(await page.locator('.roughtime-evidence').innerText(),/Date basis handed to isolated NTS verification/);
      await capture('active-recovery');

      await load('roughtime-recovered'); await button('Date recovery details').click();
      assert.equal(await page.locator('.roughtime-history').evaluate(el=>el.open),false);
      assert.equal(await page.locator('.roughtime-evidence').isVisible(),false);
      await page.locator('.roughtime-history > summary').focus(); await page.keyboard.press('Enter');
      assert.match(await page.locator('.roughtime-evidence').innerText(),/Signed Roughtime date basis/);
      await capture('retained-history');
      await nav('Diagnostics').click();
      assert.equal(await page.locator('.roughtime-history').evaluate(el=>el.open),false);

      for (const [scene,heading] of [['stale','Service collection status is unknown'],['service-stopped','Service collection is unavailable']]) {
        await load(scene); await button('Power user').click(); await nav('Diagnostics').click();
        assert.equal(await page.locator('.logging-panel h2').innerText(),heading);
        assert.equal(await page.locator('.roughtime-evidence').count(),0);
        await capture(`${scene}-diagnostics`);
      }
      await load('conflict'); await button('Power user').click(); await nav('Diagnostics').click();
      await page.locator('#event-filter').selectOption('attention');
      assert.equal(await page.locator('#event-filter option:checked').innerText(),'Notices, warnings and errors');
      assert.ok(await page.locator('.event-level.error').count()>0);
      await load('reserve'); await nav('Settings').click(); await button('Review date recovery in Power user').click();
      assert.equal(await page.locator('.future-reserve').isVisible(),true);
      assert.match(await page.locator('.app-footer').innerText(),/Prototype — sample data/);
      assert.deepEqual(errors,[]); assert.deepEqual(remote,[]);
      results.push({file,width,result:'passed'});
      console.log(`${file} ${width}px: Simple copy, measurement labels, plain-NTP applicability, settings, disclosures, collection status, filter and About passed`);
      await page.close();
    }
    fs.writeFileSync(path.join(output,'verification-summary.json'),JSON.stringify({results},null,2)+'\n');
  } finally {await browser.close(); await new Promise(resolve=>server.close(resolve));}
})().catch(error=>{console.error(error);process.exitCode=1;});
