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
      <p>Make sure the demo is served from the correct directory (e.g. via <code>nix run .#example-web</code>).</p>
    </div>`;
}

// ---- DOM refs -----------------------------------------------------------

const messageEl     = document.getElementById('message');
const btnGenerate   = document.getElementById('btn-generate');
const btnSign       = document.getElementById('btn-sign');
const btnVerify     = document.getElementById('btn-verify');
const btnVerifyWrg  = document.getElementById('btn-verify-wrong');
const secretKeyEl   = document.getElementById('secret-key');
const publicKeyEl   = document.getElementById('public-key');
const signatureEl   = document.getElementById('signature');
const verifyResult  = document.getElementById('verify-result');

// ---- State --------------------------------------------------------------

let currentKeypair = null;   // { secret_key_hex, public_key_hex }
let currentSignature = null; // hex string
let wrongKeypair = null;     // for "verify with wrong key"

// ---- Helpers ------------------------------------------------------------

function msgBytes() {
  return new TextEncoder().encode(messageEl.value || '');
}

function updateKeyDisplay(kp) {
  secretKeyEl.textContent  = kp.secret_key_hex;
  publicKeyEl.textContent  = kp.public_key_hex;
}

function enableSignVerify(enabled) {
  btnSign.disabled    = !enabled;
  btnVerify.disabled  = !enabled;
  btnVerifyWrg.disabled = !enabled;
}

function setVerifyBadge(valid) {
  verifyResult.textContent = valid ? '✅ Valid signature' : '❌ Invalid signature';
  verifyResult.className = 'verify-badge ' + (valid ? 'valid' : 'invalid');
}

// ---- Generate Key -------------------------------------------------------

btnGenerate.addEventListener('click', () => {
  if (!wasm) return;

  currentKeypair = wasm.generate_key();
  // Also generate a second keypair to use as the "wrong key" for verification
  wrongKeypair = wasm.generate_key();
  currentSignature = null;

  updateKeyDisplay(currentKeypair);
  signatureEl.textContent  = '—';
  verifyResult.textContent = '—';
  verifyResult.className   = 'verify-badge';

  enableSignVerify(true);
});

// ---- Sign ---------------------------------------------------------------

btnSign.addEventListener('click', () => {
  if (!wasm || !currentKeypair) return;

  currentSignature = wasm.sign(msgBytes(), currentKeypair.secret_key_hex);
  signatureEl.textContent = currentSignature;
  verifyResult.textContent = '—';
  verifyResult.className   = 'verify-badge';
});

// ---- Verify (correct key) -----------------------------------------------

btnVerify.addEventListener('click', () => {
  if (!wasm || !currentKeypair || !currentSignature) return;

  const valid = wasm.verify(msgBytes(), currentSignature, currentKeypair.public_key_hex);
  setVerifyBadge(valid);
});

// ---- Verify (wrong key) -------------------------------------------------

btnVerifyWrg.addEventListener('click', () => {
  if (!wasm || !wrongKeypair || !currentSignature) return;

  const valid = wasm.verify(msgBytes(), currentSignature, wrongKeypair.public_key_hex);
  setVerifyBadge(valid);
});
