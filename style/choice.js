(() => {
if ('CrownChoice' in window) return; window.CrownChoice = true;
const selector = '[data-ui-choice]';
function sync(choice) {
  const option = choice.querySelector('.ui-choice__input:checked')?.closest('.ui-choice__option') || choice.querySelector('.ui-choice__option');
  const current = choice.querySelector('[data-ui-choice-current]');
  if (!option || !current) return;
  const icon = option.querySelector('.ui-choice__icon');
  const title = option.querySelector('.ui-choice__title');
  if (!icon || !title) return;
  const oldIcon = current.querySelector('.ui-choice__icon');
  const oldTitle = current.querySelector('.ui-choice__title');
  if (oldIcon?.className === icon.className && oldTitle?.textContent === title.textContent && current.children.length === 2) return;
  current.replaceChildren(icon.cloneNode(true), title.cloneNode(true));
}
function scan(root) {
  if (root instanceof Element && root.matches(selector)) sync(root);
  root.querySelectorAll?.(selector).forEach(sync);
}
scan(document);
new MutationObserver(records => records.forEach(record => {
  if (record.target instanceof Element) {
    const choice = record.target.closest(selector);
    if (choice) sync(choice);
  }
  record.addedNodes.forEach(node => { if (node instanceof Element) scan(node); });
})).observe(document.documentElement, {childList: true, subtree: true});
document.addEventListener('change', event => {
  const input = event.target;
  if (!(input instanceof HTMLInputElement) || !input.matches('.ui-choice__input[type="radio"]')) return;
  const choice = input.closest(selector);
  if (!choice) return;
  sync(choice);
});
document.addEventListener('click', event => {
  const option = event.target instanceof Element ? event.target.closest('.ui-choice__option') : null;
  const choice = option?.closest(selector);
  if (!choice) return;
  // Label activation and the radio's change complete before the next task.
  setTimeout(() => {
    sync(choice);
    choice.open = false;
    choice.querySelector('[data-ui-choice-current]')?.focus();
  }, 0);
});
document.addEventListener('reset', event => {
  const form = event.target;
  if (form instanceof HTMLFormElement) setTimeout(() => scan(form), 0);
});
document.addEventListener('pointerdown', event => {
  document.querySelectorAll('details[data-ui-choice][open]').forEach(choice => {
    if (!choice.contains(event.target)) choice.open = false;
  });
});
document.addEventListener('keydown', event => {
  if (event.key === 'Escape') document.querySelectorAll('details[data-ui-choice][open]').forEach(choice => { choice.open = false; });
  const input = event.target;
  if ((event.key === 'Enter' || event.key === ' ') && input instanceof HTMLInputElement && input.matches('.ui-choice__option .ui-choice__input[type="radio"]') && input.closest(selector)) {
    event.preventDefault();
    input.click();
  }
});
})();
