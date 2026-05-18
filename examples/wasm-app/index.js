// Dynamically import the WASM glue module (bundled by wasm-bindgen with --target web).
// We must call the default export (__wbg_init) to initialize the WASM binary
// before using the exported functions.
let wasm;

try {
  const wasmModule = await import('./identus_crypto_wasm.js');
  await wasmModule.default();  // Initialize WASM binary
  wasm = wasmModule;
} catch (err) {
  console.error('Failed to load WASM module:', err);
  document.body.innerHTML = `
    <div class="container">
      <h1>❌ WASM Load Error</h1>
      <p>Could not load <code>identus_crypto_wasm.js</code>.</p>
      <p>Error: ${err.message}</p>
      <p>Make sure the demo is served from the correct directory (e.g. via <code>nix develop -c just run-demo-web</code>).</p>
    </div>`;
}

// ---- Tab switching ------------------------------------------------------

const tabButtons = document.querySelectorAll('.tab-btn');
const tabContents = document.querySelectorAll('.tab-content');

tabButtons.forEach(btn => {
  btn.addEventListener('click', () => {
    const tabId = btn.dataset.tab;

    // Deactivate all tabs
    tabButtons.forEach(b => b.classList.remove('active'));
    tabContents.forEach(c => c.classList.remove('active'));

    // Activate the selected tab
    btn.classList.add('active');
    document.getElementById('tab-' + tabId).classList.add('active');
  });
});

// ---- DOM refs - Generate Key tab ----------------------------------------

const btnGenerate  = document.getElementById('btn-generate');
const secretKeyEl  = document.getElementById('secret-key');
const publicKeyEl  = document.getElementById('public-key');

// ---- DOM refs - Sign tab ------------------------------------------------

const signMessage    = document.getElementById('sign-message');
const signSecretKey  = document.getElementById('sign-secret-key');
const btnSign        = document.getElementById('btn-sign');
const signatureEl    = document.getElementById('signature');
const signError      = document.getElementById('sign-error');

// ---- DOM refs - Verify tab ----------------------------------------------

const verifyMessage    = document.getElementById('verify-message');
const verifySignature  = document.getElementById('verify-signature');
const verifyPublicKey  = document.getElementById('verify-public-key');
const btnVerify        = document.getElementById('btn-verify');
const verifyResult     = document.getElementById('verify-result');

// ---- Helpers ------------------------------------------------------------

function setVerifyBadge(valid, message) {
  if (message) {
    verifyResult.textContent = message;
  } else {
    verifyResult.textContent = valid ? '✅ Valid signature' : '❌ Invalid signature';
  }
  verifyResult.className = 'verify-badge ' + (valid ? 'valid' : 'invalid');
}

// ---- Generate Key handler -----------------------------------------------

btnGenerate.addEventListener('click', () => {
  if (!wasm) return;

  const kp = wasm.generate_key();
  secretKeyEl.value  = kp.secret_key_hex;
  publicKeyEl.value  = kp.public_key_hex;
});

// ---- Sign handler -------------------------------------------------------

btnSign.addEventListener('click', () => {
  if (!wasm) return;

  const msgBytes = new TextEncoder().encode(signMessage.value || '');
  const sk = signSecretKey.value.trim();
  signError.textContent = '';

  if (!sk) {
    signError.textContent = 'Please enter a secret key.';
    return;
  }

  try {
    const sig = wasm.sign(msgBytes, sk);
    signatureEl.value = sig;
  } catch (err) {
    signError.textContent = err.message || err;
  }
});

// ---- Verify handler -----------------------------------------------------

btnVerify.addEventListener('click', () => {
  if (!wasm) return;

  const msgBytes = new TextEncoder().encode(verifyMessage.value || '');
  const sig = verifySignature.value.trim();
  const pk = verifyPublicKey.value.trim();

  if (!sig || !pk) {
    setVerifyBadge(false, 'Please fill in all fields.');
    return;
  }

  try {
    const valid = wasm.verify(msgBytes, sig, pk);
    setVerifyBadge(valid);
  } catch (err) {
    setVerifyBadge(false, 'Error: ' + (err.message || err));
  }
});
