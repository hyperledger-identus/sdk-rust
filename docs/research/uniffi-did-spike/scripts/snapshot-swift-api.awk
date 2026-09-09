/^public struct DidUrlView:/ {
    section = "record"
    print "record DidUrlView"
    next
}
/^public struct DidView:/ {
    section = "record"
    print "record DidView"
    next
}
/^enum DidBindingError:/ {
    section = "error"
    print "error DidBindingError"
    next
}
/^public func parseDid\(value:/ {
    section = ""
    print "function parseDid(value:String)throws->DidView"
    next
}
/^public func parseDidUrl\(value:/ {
    section = ""
    print "function parseDidUrl(value:String)throws->DidUrlView"
    next
}
section == "record" && /^    public var / {
    line = $0
    sub(/^    public var /, "", line)
    gsub(/: /, ":", line)
    print "property " line
    next
}
section == "error" && /^    case InvalidDid$/ {
    print "case InvalidDid"
    next
}
section == "error" && /^    case InvalidDidUrl$/ {
    print "case InvalidDidUrl"
    next
}
section != "" && /^}$/ {
    section = ""
}
