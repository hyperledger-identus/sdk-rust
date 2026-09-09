package org.hyperledger.identus.did.consumer

import android.app.Activity
import android.os.Bundle
import android.util.Log
import org.hyperledger.identus.did.DidBindingException
import org.hyperledger.identus.did.bindingApiVersion
import org.hyperledger.identus.did.parseDid
import org.hyperledger.identus.did.parseDidUrl

class MainActivity : Activity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        try {
            checkContract()
            Log.i(TAG, SUCCESS)
        } catch (error: Throwable) {
            Log.e(TAG, "$FAILURE:${error.javaClass.simpleName}:${error.message}")
        } finally {
            finish()
        }
    }

    private fun checkContract() {
        check(bindingApiVersion() == 1u)

        val did = parseDid("did:example:123")
        check(did.value == "did:example:123")
        check(did.method == "example")
        check(did.methodSpecificId == "123")

        val url = parseDidUrl("did:example:123/path?service=agent#key-1")
        check(url.did == "did:example:123")
        check(url.path == "/path")
        check(url.query == "service=agent")
        check(url.fragment == "key-1")

        val callerText = "did:EXAMPLE:do-not-reflect"
        val invalid = runCatching { parseDid(callerText) }.exceptionOrNull()
        check(invalid is DidBindingException.InvalidDid)
        check(invalid.code == "did.invalid_did")
        check(callerText !in invalid.toString())

        val oversized = "did:example:" + "a".repeat(4_096)
        val tooLarge = runCatching { parseDidUrl(oversized) }.exceptionOrNull()
        check(tooLarge is DidBindingException.InvalidDidUrl)
        check(tooLarge.code == "did.invalid_did_url")
    }

    private companion object {
        const val TAG = "IdentusDidProof"
        const val SUCCESS = "IDENTUS_DID_ANDROID_OK"
        const val FAILURE = "IDENTUS_DID_ANDROID_FAIL"
    }
}
