#![forbid(unsafe_code)]

#[cfg(test)]
mod tests {
    use identity_did::{CoreDID as IotaDid, DIDUrl as IotaDidUrl};
    use identus_did::{Did, DidUrl, MAX_DID_BYTES, MAX_DID_URL_BYTES};

    const CORPUS: &str = include_str!("../corpus.tsv");

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Outcome {
        Accept,
        Reject,
    }

    impl Outcome {
        fn parse(value: &str) -> Self {
            match value {
                "accept" => Self::Accept,
                "reject" => Self::Reject,
                _ => panic!("unknown outcome spelling"),
            }
        }

        const fn from_bool(accepted: bool) -> Self {
            if accepted { Self::Accept } else { Self::Reject }
        }
    }

    #[derive(Debug)]
    struct Case<'a> {
        id: &'a str,
        source: &'a str,
        revision: &'a str,
        profile: &'a str,
        input: String,
        normative: Outcome,
        current: Outcome,
        candidate: Outcome,
        mismatch: &'a str,
    }

    impl<'a> Case<'a> {
        fn parse(line: &'a str) -> Self {
            let columns: Vec<_> = line.split('\t').collect();
            assert_eq!(columns.len(), 9, "invalid corpus row shape");
            assert!(!columns[0].is_empty(), "case identifier is required");
            assert!(!columns[1].is_empty(), "case source is required");
            assert!(!columns[2].is_empty(), "source revision is required");
            assert!(matches!(columns[3], "did" | "did-url"));
            Self {
                id: columns[0],
                source: columns[1],
                revision: columns[2],
                profile: columns[3],
                input: expand_input(columns[4]),
                normative: Outcome::parse(columns[5]),
                current: Outcome::parse(columns[6]),
                candidate: Outcome::parse(columns[7]),
                mismatch: columns[8],
            }
        }
    }

    fn current_outcome(profile: &str, input: &str) -> Outcome {
        Outcome::from_bool(match profile {
            "did" => Did::parse(input).is_ok(),
            "did-url" => DidUrl::parse(input).is_ok(),
            _ => false,
        })
    }

    fn candidate_outcome(profile: &str, input: &str) -> Outcome {
        Outcome::from_bool(match profile {
            "did" => input.parse::<IotaDid>().is_ok(),
            "did-url" => input.parse::<IotaDidUrl>().is_ok(),
            _ => false,
        })
    }

    #[allow(clippy::cmp_owned)] // Deliberately inspect the candidate's serialized spelling.
    fn candidate_preserves_spelling(profile: &str, input: &str) -> bool {
        match profile {
            "did" => {
                let current = Did::parse(input).expect("accepted Identus DID");
                let candidate = input.parse::<IotaDid>().expect("accepted IOTA DID");
                assert_eq!(current.as_str(), input);
                candidate.to_string() == input
            }
            "did-url" => {
                let current = DidUrl::parse(input).expect("accepted Identus DID URL");
                let candidate = input.parse::<IotaDidUrl>().expect("accepted IOTA DID URL");
                assert_eq!(current.as_str(), input);
                candidate.to_string() == input
            }
            _ => unreachable!(),
        }
    }

    fn expand_input(value: &str) -> String {
        match value {
            "<DID_2048>" => format!("did:x:{}", "a".repeat(MAX_DID_BYTES - "did:x:".len())),
            "<DID_2049>" => format!("did:x:{}", "a".repeat(MAX_DID_BYTES + 1 - "did:x:".len())),
            "<URL_4096>" => format!(
                "did:x:a/{}",
                "b".repeat(MAX_DID_URL_BYTES - "did:x:a/".len())
            ),
            "<URL_4097>" => format!(
                "did:x:a/{}",
                "b".repeat(MAX_DID_URL_BYTES + 1 - "did:x:a/".len())
            ),
            _ => value
                .replace("<SP>", " ")
                .replace("<LF>", "\n")
                .replace("<CR>", "\r"),
        }
    }

    #[test]
    fn attributed_corpus_matches_pinned_normative_and_oracle_classes() {
        let mut compared = 0_usize;
        let mut divergences = 0_usize;

        for line in CORPUS.lines().skip(1) {
            let case = Case::parse(line);
            assert!(!case.id.is_empty());
            assert!(!case.source.is_empty());
            assert!(!case.revision.is_empty());

            let current = current_outcome(case.profile, &case.input);
            let candidate = candidate_outcome(case.profile, &case.input);
            assert_eq!(current, case.normative, "Identus drift for {}", case.id);
            assert_eq!(
                current, case.current,
                "stale Identus result for {}",
                case.id
            );
            assert_eq!(
                candidate, case.candidate,
                "IOTA oracle drift for {}",
                case.id
            );

            if current == candidate {
                if current == Outcome::Accept {
                    let exact = candidate_preserves_spelling(case.profile, &case.input);
                    if exact {
                        assert_eq!(case.mismatch, "none", "spurious class for {}", case.id);
                    } else {
                        assert_eq!(
                            case.mismatch, "representation-normalization",
                            "unclassified spelling divergence for {}",
                            case.id
                        );
                        divergences += 1;
                    }
                } else {
                    assert_eq!(case.mismatch, "none", "spurious class for {}", case.id);
                }
            } else {
                assert_ne!(case.mismatch, "none", "unclassified divergence");
                divergences += 1;
            }
            compared += 1;
        }

        assert_eq!(compared, 30);
        assert_eq!(divergences, 7);
    }

    #[test]
    fn candidate_diagnostics_are_never_part_of_the_oracle_result() {
        let secret = "private-caller-marker";
        let input = format!("did:example:{secret} space");
        let result = candidate_outcome("did", &input);
        assert_eq!(result, Outcome::Reject);
        assert!(!format!("{result:?}").contains(secret));
    }
}
