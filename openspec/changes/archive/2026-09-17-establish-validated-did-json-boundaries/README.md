# establish-validated-did-json-boundaries

Issues #315 and #297 are resolved together by making JSON-LD context objects
valid by construction, applying one private iterative rejection mechanism to
the complete native DID JSON family, and moving the 128-item policy out of the
generic `OneOrMany<T>` representation.
