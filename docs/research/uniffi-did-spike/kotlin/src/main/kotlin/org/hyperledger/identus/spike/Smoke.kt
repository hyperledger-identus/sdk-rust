package org.hyperledger.identus.spike

fun main() {
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
        check(!error.message.orEmpty().contains(callerText))
    }

    val oversized = "did:example:" + "a".repeat(4_096)
    try {
        parseDidUrl(oversized)
        error("oversized DID URL was accepted")
    } catch (_: DidBindingException.InvalidDidUrl) {
    }

    println("kotlin-smoke: passed")
}
