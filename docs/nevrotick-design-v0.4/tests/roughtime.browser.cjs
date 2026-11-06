'use strict';
// Build first. Uses only a loopback server, sample scenes and browser state.
const {chromium} = require('playwright');
const http = require('node:http');
const fs = require('node:fs');
const path = require('node:path');
const assert = require('node:assert/strict');
const M = require('../src/model.js');
const root = path.resolve(__dirname,'..');
const output = process.env.ROUGHTIME_REVIEW_DIR || '/tmp/nevrotick-roughtime-browser';
const scenes = ['healthy','roughtime','roughtime-date-ready','roughtime-recovered','roughtime-unavailable','roughtime-rejected','signed-conflict','reserve','recovered'];
const types = {'.html':'text/html','.css':'text/css','.js':'text/javascript','.avif':'image/avif'};
(async () => {
  fs.mkdirSync(output,{recursive:true});
  const server=http.createServer((req,res) => {
    const file=path.join(root,decodeURIComponent(new URL(req.url,'http://localhost').pathname));
    if (!file.startsWith(root+path.sep)) return res.writeHead(403).end();
    try {res.writeHead(200,{'Content-Type':types[path.extname(file)] || 'application/octet-stream'}).end(fs.readFileSync(file));}
    catch {res.writeHead(404).end();}
  });
  await new Promise(resolve => server.listen(0,'127.0.0.1',resolve));
  const browser=await chromium.launch({executablePath:process.env.CHROMIUM_PATH || '/usr/bin/chromium',headless:true,args:['--no-sandbox']});
  const results=[], screenshots=[];
  try {
    for (const file of ['index.html','NevroTick-UI-v0.4.html']) for (const width of [360,480,1280]) {
      const page=await browser.newPage({viewport:{width,height:900}});
      const errors=[], remoteRequests=[];
      page.on('pageerror',e=>errors.push(e.message));
      page.on('request',req=>{if (!req.url().startsWith('http://127.0.0.1:') && !req.url().startsWith('data:')) remoteRequests.push(req.url());});
      const button=name=>page.getByRole('button',{name,exact:true});
      const nav=name=>page.locator('.app-nav').getByRole('button',{name,exact:true});
      const overflow=async label=>assert.equal(await page.evaluate(()=>document.documentElement.scrollWidth>innerWidth),false,`${file} ${width}: ${label} overflow`);
      const recoveryBounds=async label=>{
        const escaped=await page.locator('.recovery-path').evaluate(grid=>{
          const checks=[...grid.querySelectorAll('.recovery-steps,.steps,.steps li,.steps li>div,.steps .badge')].filter(child=>child.getClientRects().length);
          return checks.flatMap(child=>{
            const parent=child.parentElement, box=child.getBoundingClientRect(), container=parent.getBoundingClientRect();
            return box.left < container.left-1 || box.right > container.right+1 || child.scrollWidth > child.clientWidth+1
              ? [{element:child.className || child.tagName,parent:parent.className || parent.tagName,left:box.left,right:box.right,parentLeft:container.left,parentRight:container.right,clientWidth:child.clientWidth,scrollWidth:child.scrollWidth}]
              : [];
          });
        });
        assert.deepEqual(escaped,[],`${file} ${width}: ${label} recovery content escapes its container even if an ancestor clips document overflow`);
      };
      for (const scene of scenes) {
        const s=M.getScenario(scene), evidence=s.roughtimeEvidence;
        const capture=async label=>{
          await overflow(scene+'/'+label);
          if ((file !== 'index.html' && width !== 480) || (file === 'index.html' && width === 480 && scene === 'roughtime-date-ready')) {
            const name=`${file === 'index.html' ? 'index' : 'standalone'}-${width}-${scene}-${label}.png`;
            // Capture from the document origin so fixed offscreen skip links
            // are not painted into the middle of a full-page screenshot.
            await page.evaluate(()=>window.scrollTo(0,0));
            await page.screenshot({path:path.join(output,name),fullPage:true}); screenshots.push(name);
          }
        };
        await page.goto(`http://127.0.0.1:${server.address().port}/${file}?scene=${scene}`);
        assert.equal(await page.locator('h1').innerText(),s.title);
        assert.equal(await page.locator('.simple-future').count(),s.future ? 1 : 0);
        assert.equal(await page.locator('.simple-status section').count(),0,'no diagnostic card inside Simple status');
        if (scene === 'roughtime-recovered') {
          assert.match(await page.locator('.simple-trust').innerText(),/Authenticated NTS · sources agree/);
          await button('Date recovery details').click();
          await page.locator('.roughtime-history > summary').click();
          assert.match(await page.locator('.roughtime-evidence').innerText(),/Signed Roughtime date basis/);
          await button('Back to status').click();
          assert.equal(await button('Simple').getAttribute('aria-pressed'),'true');
        }
        await button('Sync now').click();
        assert.match(await page.locator('#sync-response').innerText(),new RegExp(M.SYNC_RESPONSES[s.syncResponse].label));
        assert.equal(await page.locator('dialog[open]').count(),0,'Sync now stays inline');
        await capture('simple-status');

        await nav('Settings').click();
        assert.match(await page.locator('.date-recovery-settings').innerText(),/Roughtime when needed/);
        assert.equal(await page.locator('.date-recovery-settings input').count(),0,'no invented Roughtime toggle');
        await capture('simple-settings');
        await button('Review date recovery in Power user').focus();
        const originY=await page.evaluate(()=>window.scrollY);
        await page.keyboard.press('Enter');
        assert.equal(await button('Power user').getAttribute('aria-pressed'),'true');
        assert.equal(await page.locator('h1').innerText(),'Date recovery');
        assert.match(await page.locator('.recovery-current').innerText(),/Roughtime supplies a signed approximate date/);
        assert.doesNotMatch(await page.locator('.recovery-current').innerText(),/Future design/);
        const panel=page.locator('.roughtime-evidence');
        if (evidence.state === 'not-needed') assert.equal(await panel.count(),0);
        else {
          if (await page.locator('.roughtime-history').count()) await page.locator('.roughtime-history > summary').click();
          for (const fact of [evidence.reason,evidence.signatureStatus,evidence.ntsHandoffState,evidence.ancestry]) assert.ok((await panel.innerText()).includes(fact));
          assert.equal(await panel.getByRole('checkbox').count(),0);
          if (evidence.approximateDateInterval) assert.ok((await panel.innerText()).includes(evidence.approximateDateInterval));
          else assert.match(await panel.innerText(),/No usable interval accepted/);
          await panel.locator('summary').click();
          for (const x of evidence.sources) for (const field of ['host','operatorGroup','keyIdentity']) assert.ok((await panel.innerText()).includes(x[field]));
        }
        assert.equal(await page.locator('.future-reserve').count(),s.future ? 1 : 0);
        assert.equal(await page.locator('.recovery-path').evaluate(el=>el.open),evidence.scope === 'active-bootstrap' || scene === 'reserve');
        if (!(await page.locator('.recovery-path').evaluate(el=>el.open))) await page.locator('.recovery-path > summary').click();
        assert.doesNotMatch(await page.locator('.steps').innerText(),/Khronos|Ananke|reserve/);
        await recoveryBounds(scene);
        if (scene === 'roughtime-date-ready') {
          const stageBadge=page.locator('.steps .badge');
          const originalLabel=await stageBadge.textContent();
          await stageBadge.evaluate(element=>{element.textContent='Signed approximate date handed to isolated NTS verification · waiting for fresh authenticated observations and separate correction authority';});
          await recoveryBounds('long stage badge');
          await stageBadge.evaluate((element,label)=>{element.textContent=label;},originalLabel);
        }
        await capture('recovery');
        await button('Back to settings').click();
        assert.equal(await button('Simple').getAttribute('aria-pressed'),'true');
        assert.equal(await page.locator('h1').innerText(),'Settings');
        assert.equal(await page.evaluate(()=>document.activeElement.textContent),'Review date recovery in Power user');
        assert.ok(Math.abs(await page.evaluate(()=>window.scrollY)-originY)<=1,'return restores scroll');

        await button('Power user').click();
        await button('Time policy').click();
        assert.match(await page.locator('.date-recovery-settings').innerText(),/Roughtime when needed/);
        await capture('power-settings');
        await button('Review date recovery').click(); await button('Back to time policy').click();
        assert.equal(await button('Time policy').getAttribute('aria-current'),'page');
        assert.equal(await page.evaluate(()=>document.activeElement.textContent),'Review date recovery');

        await nav('Sources').click();
        assert.equal(await page.locator('.source-table tbody tr').count(),4);
        assert.doesNotMatch(await page.locator('.source-table').innerText(),/date-[ab]\.example/);
        assert.match(await page.locator('.date-recovery-sources').innerText(),/no synchronization votes/);
        if (evidence.scope === 'active-bootstrap') {
          assert.equal(await page.locator('.source-table .badge').filter({hasText:'Waiting'}).count(),4);
          assert.doesNotMatch(await page.locator('.source-table').innerText(),/Authenticated/);
        }
        await capture('sources');
        await button('Explore recovery').click(); await button('Back to sources').click();
        await nav('Statistics').click();
        assert.doesNotMatch(await page.locator('main').textContent(),/date-[ab]\.example|Roughtime/);
        await overflow('statistics');
        await nav('Status').click();
        assert.equal(await page.locator('.status-top').innerText().then(t=>t.includes('Future recovery design')),s.future);
        if (evidence.scope === 'active-bootstrap') assert.match(await page.locator('.status-summary').innerText(),/0 of 4 identities/);
        await capture('power-status');

        await nav('Diagnostics').click();
        const diagnostics=page.locator('.roughtime-evidence');
        if (evidence.state === 'not-needed') assert.equal(await diagnostics.count(),0);
        else {
          if (await page.locator('.roughtime-history').count()) await page.locator('.roughtime-history > summary').click();
          assert.ok((await diagnostics.innerText()).includes(evidence.reason));
          await diagnostics.locator('summary').click();
          for (const x of evidence.sources) assert.ok((await diagnostics.innerText()).includes(x.keyIdentity));
          if (scene === 'roughtime-recovered') assert.match(await diagnostics.innerText(),/Signed Roughtime date basis/);
        }
        await capture('diagnostics');
        await nav('About').click();
        assert.match(await page.locator('.about-copy').innerText(),/Intended v1 capabilities.*Roughtime/);
        await capture('about-power');
        await button('Simple').click();
        assert.equal(await page.locator('h1').innerText(),'About NevroTick');
        await capture('about-simple');
        results.push({file,width,scene,result:'passed'});
      }
      // New settings entry must retain source drafts and obey the existing scene guard.
      await page.goto(`http://127.0.0.1:${server.address().port}/${file}`);
      await button('Power user').click(); await nav('Sources').click();
      await button('Add source preview').click();
      await page.locator('#field-name').fill('Retained draft'); await page.locator('#field-host').fill('draft.example');
      await page.locator('dialog').getByRole('button',{name:'Update preview',exact:true}).click();
      await button('Simple').click(); await nav('Settings').click(); await button('Review date recovery in Power user').click();
      await button('Back to settings').click();
      assert.match(await page.locator('.simple-settings-note').innerText(),/1 source preview change is retained/);
      await page.locator('#reviewer-controls > summary').click();
      await page.locator('#scenario').selectOption('roughtime-rejected');
      assert.equal(await button('Clear local previews and continue').count(),1);
      await button('Cancel').click();
      assert.equal(await page.locator('#scenario').inputValue(),'healthy');
      assert.deepEqual(errors,[]); assert.deepEqual(remoteRequests,[]);
      console.log(`${file} ${width}px: all nine scenes, six surfaces, ancestry, return context and draft guard passed`);
      await page.close();
    }
    fs.writeFileSync(path.join(output,'results.json'),JSON.stringify({cases:results.length,results,screenshots,remoteRequests:0,browserErrors:0},null,2));
    console.log(`54 matrix cases passed; ${screenshots.length} screenshots: ${output}`);
  } finally {await browser.close(); await new Promise(resolve=>server.close(resolve));}
})().catch(e=>{console.error(e);process.exitCode=1;});
