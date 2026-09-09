import IdentusDid
import Testing

@Test
func bindingContractExecutesInIosSimulator() throws {
    #expect(bindingApiVersion() == 1)

    let did = try parseDid(value: "did:example:123")
    #expect(did.value == "did:example:123")
    #expect(did.method == "example")
    #expect(did.methodSpecificId == "123")

    let didUrl = try parseDidUrl(value: "did:example:123/path?service=agent#key-1")
    #expect(didUrl.value == "did:example:123/path?service=agent#key-1")
    #expect(didUrl.did == "did:example:123")
    #expect(didUrl.path == "/path")
    #expect(didUrl.query == "service=agent")
    #expect(didUrl.fragment == "key-1")

    let callerText = "did:EXAMPLE:do-not-reflect"
    do {
        _ = try parseDid(value: callerText)
        Issue.record("invalid DID was accepted")
    } catch let error as DidBindingError {
        guard case let .InvalidDid(code) = error else {
            Issue.record("unexpected DID error")
            return
        }
        #expect(code == "did.invalid_did")
        #expect(!error.localizedDescription.contains(callerText))
    }

    let oversized = "did:example:" + String(repeating: "a", count: 4_096)
    do {
        _ = try parseDidUrl(value: oversized)
        Issue.record("oversized DID URL was accepted")
    } catch let error as DidBindingError {
        guard case let .InvalidDidUrl(code) = error else {
            Issue.record("unexpected DID URL error")
            return
        }
        #expect(code == "did.invalid_did_url")
    }
}
