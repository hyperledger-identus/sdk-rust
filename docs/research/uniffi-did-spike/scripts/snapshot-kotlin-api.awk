/^data class DidUrlView \(/ {
    section = "record"
    print "record DidUrlView"
    next
}
/^data class DidView \(/ {
    section = "record"
    print "record DidView"
    next
}
/^sealed class DidBindingException:/ {
    section = "error"
    print "error DidBindingException"
    next
}
/fun `parseDid`\(`/ {
    section = ""
    print "function parseDid(value:String):DidView throws DidBindingException"
    next
}
/fun `parseDidUrl`\(`/ {
    section = ""
    print "function parseDidUrl(value:String):DidUrlView throws DidBindingException"
    next
}
section == "record" && /var `/ {
    line = $0
    sub(/^    var `/, "", line)
    sub(/`: kotlin\./, ":", line)
    sub(/,$/, "", line)
    gsub(/[[:space:]]/, "", line)
    print "property " line
    next
}
section == "error" && /class InvalidDid\(/ {
    print "case InvalidDid"
    next
}
section == "error" && /class InvalidDidUrl\(/ {
    print "case InvalidDidUrl"
    next
}
