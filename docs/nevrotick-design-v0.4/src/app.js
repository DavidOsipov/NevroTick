(function () {
  'use strict';
  const M = window.NevroTickModel;
  const V = window.NevroTickViews;
  const mount = document.getElementById('root');
  const requested = new URLSearchParams(window.location.search).get('scene');
  let state = M.initialState(requested || 'healthy');
  let returnFocus = null;
  let guardFocus = null;

  function focusKey(element) {
    return element ? { id:element.id, action:element.dataset?.action, value:element.dataset?.value, label:element.getAttribute('aria-label'), dialog:!!element.closest('dialog') } : null;
  }
  function findFocus(key) {
    if (!key) return null;
    if (key.id) return document.getElementById(key.id);
    return [...document.querySelectorAll('[data-action]')].find(x => x.dataset.action === key.action && x.dataset.value === key.value && (!key.label || x.getAttribute('aria-label') === key.label) && (key.dialog === undefined || !!x.closest('dialog') === key.dialog));
  }
  function recoveryOrigin() {
    return {
      focusKey:state.modal && state.modal !== 'source-details' ? returnFocus : focusKey(document.activeElement),
      scroll:{x:window.scrollX,y:window.scrollY},
      dialogScroll:document.getElementById('dialog')?.scrollTop || 0,
      dialogReturnFocus:state.modal === 'source-details' ? returnFocus : null,
      dialogDetails:[...document.querySelectorAll('dialog details[open]:not(.help-tip)')].map(x => x.querySelector('summary')?.textContent),
      openDetails:[...document.querySelectorAll('main details[open]:not(.help-tip)')].map(x => x.querySelector('summary')?.textContent),
      scrollRegions:[...document.querySelectorAll('.table-scroll,.plot-scroll,.app-nav')].map(x => ({label:x.getAttribute('aria-label'),left:x.scrollLeft,top:x.scrollTop}))
    };
  }
  function restoreRecoveryOrigin(context) {
    for (const details of document.querySelectorAll('main details:not(.help-tip)')) {
      if (context.openDetails?.includes(details.querySelector('summary')?.textContent)) details.open = true;
    }
    for (const details of document.querySelectorAll('dialog details:not(.help-tip)')) {
      if (context.dialogDetails?.includes(details.querySelector('summary')?.textContent)) details.open = true;
    }
    for (const saved of context.scrollRegions || []) {
      const region = [...document.querySelectorAll('.table-scroll,.plot-scroll,.app-nav')].find(x => x.getAttribute('aria-label') === saved.label);
      region?.scrollTo(saved.left,saved.top);
    }
    const target = findFocus(context.focusKey) || document.getElementById('page-title');
    target?.focus({preventScroll:true});
    document.getElementById('dialog')?.scrollTo(0,context.dialogScroll || 0);
    window.scrollTo(context.scroll?.x || 0,context.scroll?.y || 0);
    if (state.modal === 'source-details') returnFocus = context.dialogReturnFocus;
  }
  function readRecoveryInput() {
    const form = document.getElementById('recovery-form');
    if (!form) return;
    state = M.reduce(state,{type:'recovery-input',count:form.elements.namedItem('count').value,helpers:[form.elements.namedItem('helper1').value,form.elements.namedItem('helper2').value]});
  }

  function announceSourceResults() {
    const count = M.visibleSources(state).length;
    const status = document.getElementById('source-results-status');
    if (status) status.textContent = count === 0 ? 'No matching sources.' : `${count} ${count === 1 ? 'source' : 'sources'} found.`;
  }

  function openRenderedDialog() {
    const d = document.getElementById('dialog');
    if (d) {
      d.showModal();
      d.addEventListener('cancel', event => { event.preventDefault(); dispatch({ type: 'close' }); });
    }
  }
  function updateTableScrollHints() {
    for (const region of mount.querySelectorAll('.table-scroll[data-scroll-hint]')) {
      const hint = document.getElementById(region.dataset.scrollHint);
      if (!hint) continue;
      const overflowing = region.scrollWidth > region.clientWidth + 1;
      hint.hidden = !overflowing;
      if (overflowing) region.setAttribute('aria-describedby', hint.id);
      else region.removeAttribute('aria-describedby');
    }
  }
  window.addEventListener('resize', updateTableScrollHints);
  function render(focusPage = false, preserveDialog = false) {
    const trayContrast = !!document.getElementById('tray-contrast-preview')?.checked;
    const responseRegions = ['service-response','lifecycle-response','notification-sample-status'].map(id=>document.getElementById(id)).filter(Boolean);
    const existing = preserveDialog && document.querySelector('#dialog[open]');
    if (existing) {
      // Change the editor body to its leave warning in the same open dialog.
      const next = document.createElement('div');
      next.innerHTML = V.render(state);
      existing.innerHTML = next.querySelector('#dialog').innerHTML;
      for (const child of [...mount.children]) if (child !== existing) child.remove();
      for (const child of [...next.children]) if (child.id !== 'dialog') mount.insertBefore(child,existing);
    } else {
      mount.innerHTML = V.render(state);
      for (const current of responseRegions) {
        const nextResponse = document.getElementById(current.id);
        if (!nextResponse) continue;
        current.className = nextResponse.className;
        if (current.id !== 'notification-sample-status' || current.innerHTML !== nextResponse.innerHTML) current.innerHTML = nextResponse.innerHTML;
        nextResponse.replaceWith(current);
      }
      openRenderedDialog();
    }
    updateTableScrollHints();
    const trayContrastControl = document.getElementById('tray-contrast-preview');
    if (trayContrastControl) trayContrastControl.checked = trayContrast;
    if (focusPage) document.getElementById('page-title')?.focus({ preventScroll: true });
  }
  function dispatch(action) {
    const reviewer = document.getElementById('reviewer-controls');
    if (reviewer && reviewer.open !== state.reviewerOpen) state = {...state,reviewerOpen:reviewer.open};
    if (state.modal === 'recovery-editor' && !state.leaveGuard) readRecoveryInput();
    if (['page','power-page'].includes(action.type) && action.page === 'recovery') action = {...action,context:recoveryOrigin()};
    const wasModal = state.modal;
    const previousGuard = state.leaveGuard;
    const resolved = previousGuard && ['confirm-reset','recovery-keep-leave','recovery-discard-edits'].includes(action.type) ? previousGuard.action : action;
    const returningContext = resolved.type === 'recovery-back' ? state.returnContext : null;
    const focused = document.activeElement;
    const focusedKey = focusKey(focused);
    const sourceAction = ['clear-source-filter','undo-draft'].includes(action.type);
    const sourceViewport = sourceAction ? {x:window.scrollX,y:window.scrollY} : null;
    const undoneDraftIndex = action.type === 'undo-draft' ? state.drafts.findIndex(x => x.id === action.id) : -1;
    // Changing the interval rebuilds the form. Preserve its open help while
    // editing, without carrying it across navigation or other actions.
    const intervalHelpOpen = action.type === 'frequency-draft' && !!document.querySelector('#frequency-form details.help-tip[open]');
    const next = M.reduce(state, action);
    if (next === state) return;
    if (!previousGuard && next.leaveGuard) guardFocus = action.type === 'scenario' ? {id:'scenario'} : focusedKey;
    state = next;
    if (['tray-menu-open','tray-menu-toggle','tray-menu-close'].includes(action.type) && !wasModal) {
      const contrast = !!document.getElementById('tray-contrast-preview')?.checked;
      const preview = document.createElement('div');
      preview.innerHTML = V.render(state);
      document.getElementById('tray-preview')?.replaceWith(preview.querySelector('#tray-preview'));
      const contrastControl = document.getElementById('tray-contrast-preview');
      if (contrastControl) contrastControl.checked = contrast;
      if (state.trayMenuOpen) {
        document.getElementById('tray-preview')?.scrollIntoView({block:'end',inline:'nearest'});
        document.querySelector('#tray-menu [role="menuitem"]:not(:disabled)')?.focus({preventScroll:true});
      }
      else if (action.type === 'tray-menu-toggle') document.getElementById('tray-trigger')?.focus({preventScroll:true});
      return;
    }
    // Keep the status region mounted while announcing a command response.
    if (action.type === 'sync-now' && !wasModal && document.getElementById('sync-response')) {
      const response = document.createElement('div');
      response.innerHTML = V.syncFeedback(state);
      const current = document.getElementById('sync-response');
      const updated = response.firstElementChild;
      current.className = updated.className;
      current.innerHTML = updated.innerHTML;
      const responseBounds = current.getBoundingClientRect();
      if (responseBounds.top < 0 || responseBounds.bottom > window.innerHeight) {
        current.scrollIntoView({block:'nearest',inline:'nearest'});
      }
      return;
    }
    if (['sample-log-start','sample-log-stop'].includes(action.type) && state.modal === 'debug-sample') {
      const preview = document.createElement('div');
      preview.innerHTML = V.modal(state);
      const result = document.getElementById('sample-log-result');
      result.innerHTML = preview.querySelector('#sample-log-result').innerHTML;
      for (const id of ['sample-log-scope','sample-log-duration','sample-log-ui-response','sample-log-service-response']) {
        document.getElementById(id).disabled = M.logRunning(M.serviceLog(state));
      }
      document.querySelector('#dialog .dialog-bottom-actions').replaceWith(preview.querySelector('.dialog-bottom-actions'));
      const nextControl = state.sampleLog.stopRequested
        ? document.querySelector('#dialog .dialog-bottom-actions [data-action="close"]')
        : document.querySelector(`[data-action="${M.logRunning(M.serviceLog(state)) ? 'sample-log-stop' : 'sample-log-start'}"]`);
      nextControl?.focus({preventScroll:!state.sampleLog.stopRequested});
      return;
    }
    const navigating = !state.leaveGuard && ['page','power-page','policy-page','service-settings','service-diagnose','scenario','reset','recovery-back','notification-settings','notification-open','tray-open-page','tray-check','tray-quit-manage'].includes(resolved.type);
    render(navigating,wasModal === 'recovery-editor' && state.modal === 'recovery-editor');
    if (['source-filter','clear-source-filter'].includes(action.type)) announceSourceResults();
    if (intervalHelpOpen) {
      const tip = document.querySelector('#frequency-form details.help-tip');
      if (tip) tip.open = true;
    }
    if (returningContext && !state.leaveGuard) {
      restoreRecoveryOrigin(returningContext);
    } else if (sourceAction && !state.modal) {
      const remainingDraft = state.drafts[undoneDraftIndex] || state.drafts[undoneDraftIndex - 1];
      const target = action.type === 'clear-source-filter' ? document.getElementById('source-search')
        : (remainingDraft && findFocus({action:'undo-draft',value:remainingDraft.id})) || findFocus({action:'modal',value:'add-source'});
      (target || document.getElementById('page-title'))?.focus({preventScroll:true});
      window.scrollTo(sourceViewport.x,sourceViewport.y);
    } else if (state.leaveGuard) {
      document.querySelector('[data-action="recovery-stay"],[data-action="cancel-reset"]')?.focus({preventScroll:true});
    } else if (previousGuard && ['close','recovery-stay','cancel-reset'].includes(action.type)) {
      (findFocus(guardFocus) || document.getElementById('recovery-count') || document.getElementById('page-title'))?.focus({preventScroll:true});
    } else if (navigating) {
      window.scrollTo(0,0);
      if (action.type === 'service-diagnose') {
        const evidence = document.getElementById('service-diagnosis-title');
        evidence?.focus({preventScroll:true});
        evidence?.scrollIntoView({block:'start'});
      }
    } else if (wasModal === 'edit-source' && state.modal === 'source-details') {
      document.querySelector('dialog [data-action="edit-source"]')?.focus({preventScroll:true});
    } else if (wasModal && !state.modal && returnFocus && !state.trayMenuOpen) {
      const target = findFocus(returnFocus);
      (target && !target.disabled ? target : document.getElementById(action.type.startsWith('lifecycle-') || action.type === 'sample-lifecycle-deliver' || wasModal === 'lifecycle-confirm' ? 'lifecycle-response' : 'service-response') || document.getElementById('page-title'))?.focus({ preventScroll: true });
    } else if (['notification-dismiss','notification-test'].includes(action.type)) {
      document.getElementById('notification-sample-status')?.focus({preventScroll:true});
    } else if (['sample-service-deliver','sample-lifecycle-deliver'].includes(action.type)) {
      const target = findFocus(focusedKey);
      (target && !target.disabled ? target : document.getElementById(action.type.startsWith('lifecycle-') || action.type === 'sample-lifecycle-deliver' || wasModal === 'lifecycle-confirm' ? 'lifecycle-response' : 'service-response') || document.getElementById('page-title'))?.focus({preventScroll:true});
    } else if (state.modal && wasModal === state.modal && focusedKey?.id) {
      findFocus(focusedKey)?.focus({ preventScroll: true });
    } else if (!state.modal && focusedKey) {
      findFocus(focusedKey)?.focus({ preventScroll: true });
    }
    if (state.trayMenuOpen && ['tray-menu-open','tray-menu-toggle','tray-preview-enable'].includes(action.type)) {
      document.getElementById('tray-preview')?.scrollIntoView({block:'end',inline:'nearest'});
      document.querySelector('#tray-menu [role="menuitem"]:not(:disabled)')?.focus({preventScroll:true});
    }
    if (action.type === 'notification-test') document.querySelector('.windows-notification-preview')?.scrollIntoView({block:'nearest'});
    if (['service-confirm','sample-service-deliver','lifecycle-confirm','sample-lifecycle-deliver'].includes(action.type)) document.getElementById(action.type.includes('lifecycle') ? 'lifecycle-response' : 'service-response')?.scrollIntoView({block:'nearest'});
  }
  function showEditorErrors(raw, errors) {
    const form = document.getElementById('source-form');
    for (const key of ['name', 'host', 'protocol', 'port']) {
      const field = form.elements.namedItem(key);
      field.value = raw[key];
      field.setAttribute('aria-invalid', errors[key] ? 'true' : 'false');
      field.closest('.field').classList.toggle('invalid', Boolean(errors[key]));
      const node = document.getElementById('error-' + key);
      node.textContent = errors[key] || '';
      node.hidden = !errors[key];
    }
    form.elements.namedItem(Object.keys(errors)[0])?.focus();
  }
  function feedback(text) { const target = document.getElementById('dialog-feedback'); if (target) target.textContent = text; }
  function downloadSummary() {
    const blob = new Blob([M.summary(state) + '\n'], { type: 'text/plain;charset=utf-8' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url; a.download = 'NevroTick-sample-diagnostics.txt';
    document.body.append(a); a.click(); a.remove();
    window.setTimeout(() => URL.revokeObjectURL(url), 1000);
    feedback('Sample report prepared locally. Your browser controls the download.');
  }
  document.addEventListener('click', event => {
    for (const tip of document.querySelectorAll('details.help-tip[open]')) {
      if (!tip.contains(event.target)) tip.open = false;
    }
  });
  document.addEventListener('keydown', event => {
    if (event.key !== 'Escape') return;
    const scope = document.querySelector('dialog[open]') || document;
    const tip = scope.querySelector('details.help-tip[open]');
    if (!tip) return;
    event.preventDefault();
    event.stopPropagation();
    const focusedInside = tip.contains(document.activeElement);
    tip.open = false;
    if (focusedInside) tip.querySelector('summary')?.focus();
  });
  const trayItems = () => [...document.querySelectorAll('#tray-menu [role="menuitem"]:not(:disabled)')];
  mount.addEventListener('contextmenu', event => {
    const trigger=event.target.closest('#tray-trigger');
    if (!trigger || trigger.disabled) return;
    event.preventDefault();dispatch({type:'tray-menu-open'});
  });
  mount.addEventListener('keydown', event => {
    const trigger=event.target.closest('#tray-trigger');
    if (trigger && !trigger.disabled && (['ContextMenu','ArrowDown','ArrowUp'].includes(event.key) || event.key==='F10' && event.shiftKey)) {
      event.preventDefault();dispatch({type:'tray-menu-open'});
      if(event.key==='ArrowUp')trayItems().at(-1)?.focus();
      return;
    }
    if (event.target.closest('#tray-menu')) {
      const items=trayItems(),index=items.indexOf(document.activeElement);
      if (['ArrowDown','ArrowUp','Home','End'].includes(event.key)) {
        event.preventDefault();
        const next=event.key==='Home'?0:event.key==='End'?items.length-1:(index+(event.key==='ArrowDown'?1:-1)+items.length)%items.length;
        items[next]?.focus();return;
      }
      if (event.key==='Escape' || event.key==='Tab') {
        if(event.key==='Escape'){event.preventDefault();event.stopPropagation();}
        dispatch({type:'tray-menu-close'});document.getElementById('tray-trigger')?.focus({preventScroll:true});
        if(event.key==='Escape')document.getElementById('tray-tooltip')?.setAttribute('aria-hidden','true');
        return;
      }
    }
    if (trigger && event.key==='Escape') document.getElementById('tray-tooltip')?.setAttribute('aria-hidden','true');
  });
  for (const eventName of ['pointermove','focusin']) mount.addEventListener(eventName,event=>{
    if(event.target.closest('#tray-trigger') && (event.type==='focusin' || event.movementX || event.movementY))document.getElementById('tray-tooltip')?.removeAttribute('aria-hidden');
  });
  document.addEventListener('pointerdown',event=>{
    if(state.trayMenuOpen && !event.target.closest('#tray-preview'))dispatch({type:'tray-menu-close'});
  });
  document.addEventListener('focusin',event=>{
    if(state.trayMenuOpen && !event.target.closest('#tray-preview'))dispatch({type:'tray-menu-close'});
  });
  window.addEventListener('blur',()=>{if(state.trayMenuOpen)dispatch({type:'tray-menu-close'});});
  mount.addEventListener('click', async event => {
    const target = event.target.closest('[data-action]');
    if (!target || target.disabled) return;
    const action = target.dataset.action, value = target.dataset.value;
    if ((['modal','source-details','edit-source','service-request','lifecycle-request','notification-open'].includes(action) && !state.modal) || (action === 'recovery-open' && !state.modal)) returnFocus = focusKey(target);
    if (action === 'page') dispatch({ type:'page', page:value });
    else if (action === 'view-mode') dispatch({ type:'view-mode', mode:value });
    else if (action === 'tray-open-page') dispatch({type:action,page:value});
    else if (['tray-menu-open','tray-menu-toggle','tray-menu-close','tray-preview-enable','tray-observer-open','tray-check','sample-tray-quit-confirm','tray-quit-manage'].includes(action)) dispatch({type:action});
    else if (action === 'power-page') dispatch({ type:'power-page', page:value });
    else if (['notification-settings','notification-test','notification-observe'].includes(action)) dispatch({type:action});
    else if (action === 'notification-open' || action === 'notification-dismiss') dispatch({type:action,id:value});
    else if (action === 'policy-page') dispatch({ type:'policy-page' });
    else if (action === 'service-settings' || action === 'service-diagnose' || action === 'service-confirm' || action === 'sample-service-deliver') dispatch({type:action});
    else if (action === 'lifecycle-confirm' || action === 'sample-lifecycle-deliver') dispatch({type:action});
    else if (action === 'lifecycle-request') dispatch({type:'lifecycle-request',operation:value});
    else if (action === 'service-request') dispatch({type:'service-request',operation:value});
    else if (action === 'modal') dispatch({ type:'modal', modal:value });
    else if (action === 'source-details') dispatch({ type:'modal', modal:'source-details', id:value });
    else if (action === 'affected-source') {
      returnFocus = {action:'source-details', value:value || state.scenario.affectedSource};
      dispatch({type:'affected-source', id:value});
      document.getElementById('source-row-' + state.selectedSource)?.scrollIntoView({block:'nearest'});
    }
    else if (action === 'edit-source') dispatch({ type:'modal', modal:'edit-source', id:value });
    else if (action === 'close') dispatch({ type:'close' });
    else if (action === 'reset') dispatch({ type:'reset' });
    else if (action === 'compact') dispatch({ type:'compact' });
    else if (action === 'sample-tray-sync' || action === 'sync-now') dispatch({ type:'sync-now' });
    else if (action === 'sample-tray-quit') { returnFocus={id:'tray-trigger'};dispatch({ type:'sample-tray-quit' }); }
    else if (action === 'sample-log-start') dispatch({ type:'sample-log-start' });
    else if (action === 'sample-log-stop') dispatch({ type:'sample-log-stop' });
    else if (action === 'source-filter') dispatch({ type:'source-filter', value });
    else if (action === 'clear-source-filter') dispatch({type:'clear-source-filter'});
    else if (action === 'remove-source') dispatch({ type:'remove-source', id:value });
    else if (action === 'discard-drafts') dispatch({ type:'discard-drafts' });
    else if (action === 'policy-discard') dispatch({ type:'policy-discard' });
    else if (action === 'recovery-open') dispatch({ type:'recovery-edit' });
    else if (action === 'recovery-cancel') dispatch({ type:'recovery-cancel' });
    else if (action === 'recovery-discard') dispatch({ type:'recovery-discard' });
    else if (['recovery-back','recovery-keep-leave','recovery-discard-edits','recovery-stay','confirm-reset','cancel-reset'].includes(action)) dispatch({type:action});
    else if (action === 'undo-draft') dispatch({type:'undo-draft',id:value});
    else if (action === 'settings') dispatch({type:'settings',value});
    else if (action === 'frequency-cancel') {
      dispatch({type:'frequency-cancel'});
      document.getElementById('frequency-mode')?.focus({preventScroll:true});
    }
    else if (action === 'related-events') dispatch({type:'page',page:'diagnostics'});
    else if (action === 'download-summary') downloadSummary();
    else if (action === 'copy-summary') {
      try {
        if (!navigator.clipboard || !navigator.clipboard.writeText) throw new Error('Clipboard unavailable');
        await navigator.clipboard.writeText(M.summary(state));
        feedback('Fixture summary copied. No system data was collected.');
      } catch (_) { feedback('Your browser did not allow clipboard access. Select the text above, or save the sample report.'); }
    }
  });
  mount.addEventListener('change', event => {
    const target = event.target;
    if (target.id === 'scenario') dispatch({type:'scenario',id:target.value});
    else if (target.id === 'frequency-mode') dispatch({type:'frequency-draft',value:target.value==='auto'?'auto':state.frequencyCustom});
    else if (target.id === 'frequency-interval') dispatch({type:'frequency-draft',value:target.value === 'own' ? 'own:' + state.frequencyOwn : target.value});
    else if (target.id === 'event-filter') dispatch({type:'event-filter',value:target.value});
    else if (target.id === 'recovery-count') {
      readRecoveryInput();
      document.getElementById('recovery-helper-1').hidden = target.value === '0';
      document.getElementById('recovery-helper-2').hidden = target.value !== '2';
    }
    else if (target.id === 'sample-sync-result') dispatch({type:'sample-sync',value:target.value});
    else if (target.id === 'sample-lifecycle-preflight') dispatch({type:'sample-lifecycle-preflight',value:target.value});
    else if (target.id === 'sample-lifecycle-response') dispatch({type:'sample-lifecycle-response',value:target.value});
    else if (target.id === 'sample-service-response') dispatch({type:'sample-service-response',value:target.value});
    else if (target.id === 'sample-log-scope') dispatch({type:'sample-log-scope',value:target.value});
    else if (target.id === 'sample-log-duration') dispatch({type:'sample-log-duration',value:target.value});
    else if (['sample-log-ui-response','sample-log-service-response'].includes(target.id)) dispatch({type:'sample-log-response',component:target.id === 'sample-log-ui-response' ? 'ui' : 'service',value:target.value});
    else if (target.id === 'notify-problems') dispatch({type:'notifications-enabled',value:target.checked});
    else if (target.id === 'sample-notification-os') dispatch({type:'sample-notification-os',value:target.value});
    else if (target.id === 'sample-notification-presenter') dispatch({type:'sample-notification-presenter',value:target.value});
    else if (target.id === 'sample-notification-persistent') dispatch({type:'sample-notification-persistent',value:target.checked});
    else if (target.id === 'sample-tray') dispatch({type:'sample-tray',value:target.checked});
    else if (target.id === 'sample-signin') dispatch({type:'sample-signin',value:target.checked});
    else if (target.id === 'field-protocol') {
      const port = document.getElementById('field-port');
      if (['123','4460'].includes(port.value)) port.value = target.value === 'NTS' ? '4460' : '123';
      document.querySelector('label[for="field-port"]').textContent = target.value === 'NTS' ? 'NTS-KE port' : 'NTP port';
      document.querySelector('.protocol-notice').hidden = target.value !== 'NTP';
    }
  });
  mount.addEventListener('input', event => {
    if (event.target.closest('#recovery-form')) {
      // Retain the native input node, undo history and incomplete fields.
      readRecoveryInput();
      return;
    }
    if (event.target.id === 'frequency-own-minutes') {
      // Keep the input node mounted so browser undo and IME composition survive.
      state = M.reduce(state, {type:'frequency-draft', value:'own:' + event.target.value});
      const next = document.createElement('div');
      next.innerHTML = V.frequencySettings(state);
      for (const selector of ['#frequency-error', '.frequency-current', '.frequency-actions']) {
        document.querySelector(selector)?.replaceWith(next.querySelector(selector));
      }
      event.target.removeAttribute('aria-invalid');
      document.querySelector('main > .toast-inline')?.remove();
      return;
    }
    if (event.target.id !== 'source-search') return;
    const value = event.target.value;
    state = M.reduce(state,{type:'source-query',value});
    const section = document.createElement('div');
    section.innerHTML = V.sources(M.presentState(state));
    document.querySelector('.source-panel').replaceWith(section.querySelector('.source-panel'));
    updateTableScrollHints();
    announceSourceResults();
  });
  mount.addEventListener('submit', event => {
    if (event.target.id === 'policy-form') {
      event.preventDefault();
      dispatch({type:'policy-draft',value:event.target.elements.namedItem('profile').value});
      return;
    }
    if (event.target.id === 'recovery-form') {
      event.preventDefault();
      const form = event.target;
      dispatch({type:'recovery-preview',count:form.elements.namedItem('count').value,helpers:[form.elements.namedItem('helper1').value,form.elements.namedItem('helper2').value]});
      if (state.recoveryErrors.helper1) document.getElementById('recovery-identity-1')?.focus();
      else if (state.recoveryErrors.helper2) document.getElementById('recovery-identity-2')?.focus();
      return;
    }
    if (event.target.id === 'frequency-form') {
      event.preventDefault();
      dispatch({type:'frequency-save'});
      document.getElementById(state.frequencyError ? 'frequency-own-minutes' : 'frequency-mode')?.focus({preventScroll:true});
      return;
    }
    if (event.target.id !== 'source-form') return;
    event.preventDefault();
    const raw = Object.fromEntries(new FormData(event.target).entries());
    const result = M.reduce(state,{type:'add-draft',raw,id:state.selectedSource});
    if (Object.keys(result.editorError).length) { state = result; showEditorErrors(raw,result.editorError); }
    else { state = {...result,mode:'power',page:'sources',powerPage:'sources'}; render(); document.getElementById('page-title')?.focus(); }
  });
  mount.addEventListener('toggle', event => {
    if (event.target.id === 'reviewer-controls' && event.target.isConnected) state = {...state,reviewerOpen:event.target.open};
  }, true);
  render();
})();
