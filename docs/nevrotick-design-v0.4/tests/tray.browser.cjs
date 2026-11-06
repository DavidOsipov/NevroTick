'use strict';
const { chromium } = require('playwright');
const http=require('node:http'),fs=require('node:fs'),path=require('node:path'),assert=require('node:assert/strict');
const M=require('../src/model.js');
const root=path.resolve(__dirname,'..');
const types={'.html':'text/html','.js':'text/javascript','.css':'text/css','.avif':'image/avif'};

(async()=>{
  const server=http.createServer((request,response)=>{
    const file=path.resolve(root,'.'+decodeURIComponent(new URL(request.url,'http://localhost').pathname));
    if(!file.startsWith(root+path.sep))return response.writeHead(403).end();
    try{response.writeHead(200,{'Content-Type':types[path.extname(file)]||'application/octet-stream'});response.end(fs.readFileSync(file));}
    catch{response.writeHead(404).end();}
  });
  await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve));
  let browser;let journeys=0,scenes=0;
  try{
    browser=await chromium.launch({executablePath:process.env.CHROMIUM_PATH||'/usr/bin/chromium',headless:true,args:['--no-sandbox']});
    const screenshotDir=process.env.TRAY_SCREENSHOTS;
    if(screenshotDir)fs.mkdirSync(screenshotDir,{recursive:true});
    for(const file of ['index.html','NevroTick-UI-v0.4.html'])for(const width of [360,480,1280]){
      const page=await browser.newPage({viewport:{width,height:720}}),errors=[],remote=[];
      page.on('pageerror',e=>errors.push(e.message));
      page.on('request',r=>{if(!r.url().startsWith('http://127.0.0.1:')&&!r.url().startsWith('data:'))remote.push(r.url());});
      const url=`http://127.0.0.1:${server.address().port}/${file}`;
      await page.goto(url);
      assert.equal(await page.locator('#tray-trigger').count(),0,'tray is opt-in');
      await page.locator('nav').getByRole('button',{name:'Settings',exact:true}).click();
      await page.locator('#sample-tray').check();
      await page.locator('#sample-signin').check();
      await page.locator('#tray-trigger').click();
      await page.locator('#tray-menu').waitFor();
      assert.equal(await page.locator('#tray-menu [role=menuitem]').first().evaluate(el=>el===document.activeElement),true);
      await page.keyboard.press('ArrowDown');
      assert.equal(await page.locator(':focus').getAttribute('data-action'),'tray-check');
      await page.keyboard.press('Home');
      assert.equal(await page.locator(':focus').getAttribute('data-action'),'tray-open-page');
      await page.keyboard.press('End');
      assert.match(await page.locator(':focus').textContent(),/Quit UI/);
      await page.keyboard.press('Escape');
      assert.equal(await page.locator('#tray-menu').isVisible(),false);
      assert.equal(await page.locator(':focus').getAttribute('id'),'tray-trigger');
      assert.equal(await page.locator('#tray-tooltip').getAttribute('aria-hidden'),'true');
      await page.locator('#tray-trigger').click({button:'right'});
      await page.locator('#tray-menu').getByRole('menuitem',{name:'About NevroTick',exact:true}).click();
      assert.equal(await page.locator('h1').textContent(),'About NevroTick');
      await page.locator('#tray-trigger').click();
      if(screenshotDir&&file==='NevroTick-UI-v0.4.html')await page.screenshot({path:path.join(screenshotDir,`tray-normal-${width}.png`)});
      await page.keyboard.press('Escape');
      await page.locator('#tray-contrast-preview').check();
      await page.locator('#tray-trigger').focus();await page.keyboard.press('Shift+F10');
      assert.equal(await page.locator('#tray-contrast-preview').isChecked(),true);
      const bounds=await page.locator('#tray-menu-popup').boundingBox();
      assert.ok(bounds.x>=0 && bounds.x+bounds.width<=width+1,`menu fits ${width}px`);
      assert.ok(bounds.y>=0 && bounds.y+bounds.height<=721,`menu fits viewport height at ${width}px`);
      assert.equal(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth),true);
      if(screenshotDir&&file==='NevroTick-UI-v0.4.html')await page.screenshot({path:path.join(screenshotDir,`tray-menu-${width}.png`)});
      await page.locator('#tray-menu').getByRole('menuitem',{name:/Quit UI/}).click();
      await page.getByRole('dialog').waitFor();
      assert.match(await page.getByRole('dialog').textContent(),/background services will keep running/i);
      assert.equal(await page.getByRole('button',{name:'Keep NevroTick open',exact:true}).evaluate(el=>el===document.activeElement),true);
      if(screenshotDir&&file==='NevroTick-UI-v0.4.html')await page.screenshot({path:path.join(screenshotDir,`tray-quit-${width}.png`)});
      await page.getByRole('button',{name:'Manage background service',exact:true}).click();
      assert.equal(await page.getByRole('dialog').count(),0);
      assert.match(await page.locator('main').textContent(),/Background service/);
      assert.equal(await page.locator('#tray-trigger').count(),1,'management route does not quit observer');
      await page.locator('#tray-trigger').click();
      await page.locator('#tray-menu').getByRole('menuitem',{name:/Quit UI/}).click();
      await page.keyboard.press('Escape');
      assert.equal(await page.locator('#tray-trigger').count(),1,'cancel keeps observer');
      await page.locator('#tray-trigger').click();
      await page.locator('#tray-menu').getByRole('menuitem',{name:/Quit UI/}).click();
      await page.getByRole('button',{name:'Quit UI only',exact:true}).click();
      assert.equal(await page.locator('#tray-trigger').count(),0);
      await page.locator('[data-action=tray-observer-open]').click();
      assert.equal(await page.locator('#tray-trigger').count(),1);
      await page.locator('#tray-trigger').click();
      await page.locator('#tray-menu').getByRole('menuitem',{name:'Settings',exact:true}).click();
      await page.getByRole('button',{name:'Your preferences',exact:true}).click();
      assert.equal(await page.locator('#sample-signin').isChecked(),true,'quitting retains sign-in choice');
      assert.equal(await page.locator('#sample-tray').isChecked(),true);
      await page.getByRole('button',{name:'About the tray example',exact:true}).click();
      await page.getByRole('button',{name:'Open tray menu preview',exact:true}).click();
      assert.equal(await page.locator('#tray-menu').isVisible(),true,'tray help opens the menu');
      assert.equal(await page.locator('#tray-menu [role=menuitem]').first().evaluate(el=>el===document.activeElement),true);
      await page.keyboard.press('Tab');
      assert.equal(await page.locator('#tray-menu').isVisible(),false,'Tab exits the menu');
      await page.locator('nav').getByRole('button',{name:'Sources',exact:true}).click();
      await page.getByRole('button',{name:'Add source preview',exact:true}).click();
      await page.locator('#field-name').fill('Unsubmitted source');
      assert.equal(await page.locator('#tray-trigger').isDisabled(),true,'tray cannot replace editor');
      await page.locator('#tray-trigger').evaluate(el=>el.click());
      assert.equal(await page.locator('#field-name').inputValue(),'Unsubmitted source');
      await page.getByRole('button',{name:'Cancel',exact:true}).click();
      assert.deepEqual(errors,[]);assert.deepEqual(remote,[]);
      journeys++;await page.close();
    }
    for(const width of [360,1280]){
      const page=await browser.newPage({viewport:{width,height:720}}),errors=[];
      page.on('pageerror',e=>errors.push(e.message));
      for(const scene of M.SCENARIOS){
        await page.goto(`http://127.0.0.1:${server.address().port}/NevroTick-UI-v0.4.html?scene=${scene.id}`);
        await page.locator('[data-action=tray-preview-enable]').click();
        await page.locator('#tray-menu').waitFor();
        assert.equal(await page.locator('#tray-trigger .tray-icon').getAttribute('data-kind'),M.trayPresentation(M.initialState(scene.id)).kind);
        const bounds=await page.locator('#tray-menu-popup').boundingBox();
        assert.ok(bounds.x>=0&&bounds.x+bounds.width<=width+1,scene.id+' menu fits');
        assert.ok(bounds.y>=0&&bounds.y+bounds.height<=721,scene.id+' menu fits height');
        assert.equal(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth),true,scene.id+' no overflow');
        assert.deepEqual(errors,[]);scenes++;
        if(['service-stopped','stale'].includes(scene.id)){
          await page.locator('#tray-menu').getByRole('menuitem',{name:/Quit UI/}).click();
          const text=await page.getByRole('dialog').textContent();
          assert.match(text,scene.id==='service-stopped'?/already stopped/:/unconfirmed/);
          assert.doesNotMatch(text,/background services will keep running/i);
        }
      }
      await page.close();
    }
    console.log(JSON.stringify({journeys,sceneViews:scenes,browser:await browser.version(),screenshots:screenshotDir||null}));
  }finally{if(browser)await browser.close();await new Promise(resolve=>server.close(resolve));}
})().catch(error=>{console.error(error);process.exitCode=1;});
