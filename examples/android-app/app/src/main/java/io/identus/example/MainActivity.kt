package io.identus.example

import android.os.Bundle
import android.widget.Button
import android.widget.EditText
import android.widget.TextView
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import uniffi.identus_crypto_uniffi.KeyInfo
import uniffi.identus_crypto_uniffi.SignException
import uniffi.identus_crypto_uniffi.generateKey
import uniffi.identus_crypto_uniffi.sign
import uniffi.identus_crypto_uniffi.verify

/**
 * Minimal Android demo app that calls the Identus crypto UniFFI bindings.
 *
 * UI layout: activity_main.xml
 * - Generate Key: calls generateKey() and displays secret/public keys
 * - Sign: calls sign(secretKey, message.toByteArray()) and displays signature
 * - Verify: calls verify(publicKey, message.toByteArray(), signature) and shows result
 */
class MainActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        // ── Generate Key ─────────────────────────────────────────────
        val tvSecretKey = findViewById<TextView>(R.id.tvSecretKey)
        val tvPublicKey = findViewById<TextView>(R.id.tvPublicKey)

        findViewById<Button>(R.id.btnGenerateKey).setOnClickListener {
            try {
                val keyInfo: KeyInfo = generateKey()
                tvSecretKey.text = keyInfo.secretKey
                tvPublicKey.text = keyInfo.publicKey
            } catch (e: Exception) {
                showError("Key generation failed: ${e.message}")
            }
        }

        // ── Sign ─────────────────────────────────────────────────────
        val etSignSecretKey = findViewById<EditText>(R.id.etSignSecretKey)
        val etSignMessage = findViewById<EditText>(R.id.etSignMessage)
        val tvSignature = findViewById<TextView>(R.id.tvSignature)

        findViewById<Button>(R.id.btnSign).setOnClickListener {
            val secretKey = etSignSecretKey.text.toString().trim()
            if (secretKey.isEmpty()) {
                showError("Enter a secret key")
                return@setOnClickListener
            }

            val messageText = etSignMessage.text.toString()
            // Convert message text to ByteArray using UTF-8 encoding,
            // matching the expected Vec<u8> parameter in the Rust API.
            val messageBytes: ByteArray = messageText.toByteArray(Charsets.UTF_8)

            try {
                val signature: String = sign(secretKey, messageBytes)
                tvSignature.text = signature
            } catch (e: SignException) {
                showError("Signing failed: ${e.message}")
            } catch (e: Exception) {
                showError("Unexpected error: ${e.message}")
            }
        }

        // ── Verify ───────────────────────────────────────────────────
        val etVerifyPublicKey = findViewById<EditText>(R.id.etVerifyPublicKey)
        val etVerifyMessage = findViewById<EditText>(R.id.etVerifyMessage)
        val etVerifySignature = findViewById<EditText>(R.id.etVerifySignature)
        val tvVerifyResult = findViewById<TextView>(R.id.tvVerifyResult)

        findViewById<Button>(R.id.btnVerify).setOnClickListener {
            val publicKey = etVerifyPublicKey.text.toString().trim()
            if (publicKey.isEmpty()) {
                showError("Enter a public key")
                return@setOnClickListener
            }

            val messageText = etVerifyMessage.text.toString()
            val signatureValue = etVerifySignature.text.toString().trim()
            if (signatureValue.isEmpty()) {
                showError("Enter a signature")
                return@setOnClickListener
            }

            // Convert message text to ByteArray using UTF-8 encoding,
            // matching the expected Vec<u8> parameter in the Rust API.
            val messageBytes: ByteArray = messageText.toByteArray(Charsets.UTF_8)

            try {
                val isValid: Boolean = verify(publicKey, messageBytes, signatureValue)
                tvVerifyResult.text = if (isValid) "✅ Valid" else "❌ Invalid"
            } catch (e: Exception) {
                showError("Verification failed: ${e.message}")
            }
        }
    }

    private fun showError(message: String) {
        Toast.makeText(this, message, Toast.LENGTH_SHORT).show()
    }
}
