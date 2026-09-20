function installPasteFallback() {
  document.addEventListener('keydown', (event) => {
    const isPasteShortcut = (event.ctrlKey || event.metaKey)
      && event.key.toLowerCase() === 'v';
    if (isPasteShortcut) {
      // Let the browser produce the paste event, but keep GPUI's key handler
      // from consuming the shortcut before the browser reads the clipboard.
      event.stopPropagation();
    }
  }, true);

  document.addEventListener('paste', (event) => {
    const mirror = document.querySelector('textarea');
    const text = event.clipboardData?.getData('text/plain');
    if (!mirror || !text) {
      return;
    }

    event.preventDefault();
    event.stopPropagation();
    mirror.focus();

    // Use the browser's native edit path so gpui-pre-web receives a normal
    // input event and can import the mirror diff into the GPUI text state.
    const inserted = document.execCommand('insertText', false, text);
    if (!inserted) {
      const start = mirror.selectionStart ?? mirror.value.length;
      const end = mirror.selectionEnd ?? start;
      mirror.setRangeText(text, start, end, 'end');
      mirror.dispatchEvent(new InputEvent('input', {
        bubbles: true,
        inputType: 'insertText',
        data: text,
      }));
    }
  }, true);
}

async function init() {
  const loadingEl = document.getElementById('loading');
  const appEl = document.getElementById('app');

  try {
    // Import the WASM module
    const wasm = await import('../pkg/protocol_viewer_web.js');
    await wasm.default();

    // Initialize the application
    await wasm.run();
    installPasteFallback();

    // Hide loading indicator
    if (loadingEl) {
      loadingEl.remove();
    }
  } catch (error) {
    console.error('Failed to initialize:', error);

    // Show error message
    if (loadingEl) {
      loadingEl.innerHTML = `
        <div class="error">
          <h2>Failed to load the application</h2>
          <p>${error.message || error}</p>
          <p style="margin-top: 10px; font-size: 14px;">
            Please check the console for more details.
          </p>
        </div>
      `;
    }
  }
}

init();
