# Source provenance

| Source | Revision/version | Paths or role | License | Disposition |
| --- | --- | --- | --- | --- |
| RFC Editor | RFC 9901, text SHA-256 `fd4cb79184162fa7ef95f4f63abab8ee6d03c9a04c19b8f83758946464b284eb` | normative compact, disclosure, presentation, and key-binding behavior plus examples | RFC legal provisions; extracted code components use the RFC Trust license notice | normative and official vectors |
| `affinidi/affinidi-rs` | `affinidi-sd-jwt` 0.1.8 / `95fde42bd338` | independent complete-base behavior | Apache-2.0 | frozen differential oracle only |
| `iotaledger/product-core` | `sd-jwt-payload` 0.5.1 / `c47e52c31545` | independent payload/disclosure behavior | Apache-2.0 | frozen differential oracle only |
| `reallyme/identity` | `reallyme-sd-jwt` 0.4.1 / `60f42c536d6e` | independent bounded/staged behavior | Apache-2.0 | frozen architecture/differential oracle only |
| `hyperledger-identus/sdk-ts` | `4bf86ebf69d5e96616a148e4c973f831f95fa38e` | consumer-regression fixtures | Apache-2.0 | migration evidence only |

No donor production source file is copied wholesale and no oracle is a
workspace dependency. Every adapted official or donor fixture records exact
path/section, transformation, expected result, and source identifier beside the
vector packet.
