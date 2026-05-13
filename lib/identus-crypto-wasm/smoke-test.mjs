import * as wasm from './pkg/identus_crypto_wasm.js';
import fs from 'fs';

// Read the .wasm binary directly and initialize synchronously
const wasmBytes = fs.readFileSync('./pkg/identus_crypto_wasm_bg.wasm');
wasm.initSync({ module: wasmBytes });

console.log('=== WASM Identus Crypto Smoke Test ===\n');

// Test 1: generate_key
console.log('1. generate_key()...');
const keypair = wasm.generate_key();
console.log(`   secret_key_hex: ${keypair.secret_key_hex}`);
console.log(`   public_key_hex: ${keypair.public_key_hex}`);
console.log(`   secret key length: ${keypair.secret_key_hex.length} hex chars (${keypair.secret_key_hex.length / 2} bytes)`);
console.log(`   public key length: ${keypair.public_key_hex.length} hex chars (${keypair.public_key_hex.length / 2} bytes)`);

// Verify expected byte lengths
if (keypair.secret_key_hex.length !== 64) throw new Error(`Expected 64 hex chars for secret key, got ${keypair.secret_key_hex.length}`);
if (keypair.public_key_hex.length !== 64) throw new Error(`Expected 64 hex chars for public key, got ${keypair.public_key_hex.length}`);
console.log('   ✓ Key lengths correct (32 bytes each)\n');

// Test 2: sign + verify round-trip
console.log('2. sign/verify round-trip...');
const message = new TextEncoder().encode('Hello, Identus WASM!');
const signature = wasm.sign(message, keypair.secret_key_hex);
console.log(`   message: 'Hello, Identus WASM!'`);
console.log(`   signature: ${signature}`);
console.log(`   signature length: ${signature.length} hex chars (${signature.length / 2} bytes)`);

if (signature.length !== 128) throw new Error(`Expected 128 hex chars for signature, got ${signature.length}`);
console.log('   ✓ Signature length correct (64 bytes)\n');

// Verify with correct key
console.log('3. verify() with correct key...');
const valid = wasm.verify(message, signature, keypair.public_key_hex);
console.log(`   result: ${valid}`);
if (!valid) throw new Error('Signature should be valid with correct key');
console.log('   ✓ Signature verified\n');

// Verify with wrong key
console.log('4. verify() with wrong key...');
const wrongKeypair = wasm.generate_key();
const invalid = wasm.verify(message, signature, wrongKeypair.public_key_hex);
console.log(`   result: ${invalid}`);
if (invalid) throw new Error('Signature should be invalid with wrong key');
console.log('   ✓ Wrong key correctly rejected\n');

// Verify with wrong message
console.log('5. verify() with wrong message...');
const wrongMessage = new TextEncoder().encode('Wrong message');
const wrongMsgResult = wasm.verify(wrongMessage, signature, keypair.public_key_hex);
console.log(`   result: ${wrongMsgResult}`);
if (wrongMsgResult) throw new Error('Signature should be invalid with wrong message');
console.log('   ✓ Wrong message correctly rejected\n');

// Test 6: Empty message
console.log('6. sign/verify with empty message...');
const emptyMsg = new Uint8Array(0);
const emptySig = wasm.sign(emptyMsg, keypair.secret_key_hex);
const emptyValid = wasm.verify(emptyMsg, emptySig, keypair.public_key_hex);
console.log(`   result: ${emptyValid}`);
if (!emptyValid) throw new Error('Empty message signature should verify');
console.log('   ✓ Empty message round-trip\n');

// Test 7: Multiple generates produce different keys
console.log('7. Multiple generate_key() calls produce unique keys...');
const k1 = wasm.generate_key();
const k2 = wasm.generate_key();
const k3 = wasm.generate_key();
const allUnique = new Set([k1.secret_key_hex, k2.secret_key_hex, k3.secret_key_hex]).size === 3;
console.log(`   all unique: ${allUnique}`);
if (!allUnique) throw new Error('Generated keys should be unique');
console.log('   ✓ Keys are randomized\n');

console.log('=== All smoke tests passed! ===');
