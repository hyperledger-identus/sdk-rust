package org.hyperledger.identus.did

fun main() {
    check(bindingApiVersion() == 1u)

    val did = parseDid("did:example:123")
    check(did.value == "did:example:123")
    check(did.method == "example")
    check(did.methodSpecificId == "123")

    val didUrl = parseDidUrl("did:example:123/path?service=agent#key-1")
    check(didUrl.value == "did:example:123/path?service=agent#key-1")
    check(didUrl.did == "did:example:123")
    check(didUrl.path == "/path")
    check(didUrl.query == "service=agent")
    check(didUrl.fragment == "key-1")

    val callerText = "did:EXAMPLE:do-not-reflect"
    try {
        parseDid(callerText)
        error("invalid DID was accepted")
    } catch (error: DidBindingException.InvalidDid) {
        check(error.code == "did.invalid_did")
        check(!error.message.orEmpty().contains(callerText))
    }

    // Exercise the independent 8 KiB DID URL suffix bound; the global ceiling
    // is larger so portable PRISM long-form DIDs remain representable.
    val oversized = "did:example:1/" + "a".repeat(8_192)
    try {
        parseDidUrl(oversized)
        error("oversized DID URL was accepted")
    } catch (error: DidBindingException.InvalidDidUrl) {
        check(error.code == "did.invalid_did_url")
    }

    println("kotlin-smoke: passed")
}
