/* Dependency-free build. Run: node build.cjs */
'use strict';
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const M = require('./src/model.js');
const V = require('./src/views.js');
const root = __dirname;
const read = p => fs.readFileSync(path.join(root,p),'utf8');
const css = read('src/styles.css');
const scriptFiles = ['src/model.js','src/views.js','src/app.js'];
const script = scriptFiles.map(read).join('\n');
const hash = content => crypto.createHash('sha256').update(content).digest('base64');
const template = read('src/template.html');
const initial = V.render(M.initialState());
const baseCsp = "default-src 'none'; base-uri 'none'; form-action 'none'; connect-src 'none'; object-src 'none'; frame-src 'none'; worker-src 'none';";
const local = template.replace('__CSP__',baseCsp+" script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:;")
  .replace('__STYLE__','<link rel="stylesheet" href="src/styles.css">')
  .replace('__INITIAL__',initial)
  .replace('__SCRIPTS__',scriptFiles.map(f=>'<script src="'+f+'"></script>').join('\n'));
fs.writeFileSync(path.join(root,'index.html'),local);
let standalone = template.replace('__CSP__',baseCsp+` script-src 'sha256-${hash(script)}'; style-src 'unsafe-inline'; img-src data:;`)
  .replace('__STYLE__','<style>'+css+'</style>')
  .replace('__INITIAL__',initial)
  .replace('__SCRIPTS__','<script>'+script+'</script>');
for (const file of ['logo.avif','mascot.avif','icon.avif']) {
  const uri='data:image/avif;base64,'+fs.readFileSync(path.join(root,'assets',file)).toString('base64');
  standalone=standalone.split('assets/'+file).join(uri);
}
// Asset substitution also affects the script bytes, so the CSP binds the final script.
const finalScript = standalone.match(/<script>([\s\S]*?)<\/script>/)[1];
standalone=standalone.replace(/script-src 'sha256-[^']+'/,'script-src \'sha256-'+hash(finalScript)+'\'');
fs.writeFileSync(path.join(root,'NevroTick-UI-v0.4.html'),standalone);
console.log(JSON.stringify({scenarios:M.SCENARIOS.length,standaloneBytes:Buffer.byteLength(standalone),sourceCommit:M.SOURCE_COMMIT}));
