import Foundation

@main
struct Smoke {
    static func main() throws {
        precondition(bindingApiVersion() == 1)

        let did = try parseDid(value: "did:example:123")
        precondition(did.value == "did:example:123")
        precondition(did.method == "example")
        precondition(did.methodSpecificId == "123")

        let didUrl = try parseDidUrl(value: "did:example:123/path?service=agent#key-1")
        precondition(didUrl.value == "did:example:123/path?service=agent#key-1")
        precondition(didUrl.did == "did:example:123")
        precondition(didUrl.path == "/path")
        precondition(didUrl.query == "service=agent")
        precondition(didUrl.fragment == "key-1")

        let callerText = "did:EXAMPLE:do-not-reflect"
        do {
            _ = try parseDid(value: callerText)
            preconditionFailure("invalid DID was accepted")
        } catch let error as DidBindingError {
            guard case let .InvalidDid(code) = error else {
                preconditionFailure("unexpected DID error")
            }
            precondition(code == "did.invalid_did")
            precondition(!error.localizedDescription.contains(callerText))
        }

        // The PRISM-capable domain ceiling is intentionally large. Exercise the
        // independent 8 KiB DID URL suffix bound without allocating 139 KiB.
        let oversized = "did:example:1/" + String(repeating: "a", count: 8_192)
        do {
            _ = try parseDidUrl(value: oversized)
            preconditionFailure("oversized DID URL was accepted")
        } catch let error as DidBindingError {
            guard case let .InvalidDidUrl(code) = error else {
                preconditionFailure("unexpected DID URL error")
            }
            precondition(code == "did.invalid_did_url")
        }

        print("swift-smoke: passed")
    }
}
