use std::{collections::BTreeMap, hint::black_box, time::Instant};

use did_url_parser::DID as Candidate;
use identus_did::{Did, DidUrl, MAX_DID_BYTES, MAX_DID_URL_BYTES};

const CORPUS: &str = include_str!("../corpus.tsv");

fn main() {
    let curated = compare_curated();
    let exhaustive = compare_exhaustive();
    demonstrate_facade_mismatches();
    if std::env::var_os("DID_PARSER_BENCH").is_some() {
        benchmark();
    }

    println!(
        "result\tcurated={}\tcurated_mismatches={}\texhaustive={}\texhaustive_mismatches={}",
        curated.compared, curated.mismatches, exhaustive.compared, exhaustive.mismatches
    );
}

#[derive(Default)]
struct Counts {
    compared: usize,
    mismatches: usize,
}

fn compare_curated() -> Counts {
    let mut counts = Counts::default();
    for (line_number, line) in CORPUS.lines().enumerate().skip(1) {
        let columns: Vec<_> = line.split('\t').collect();
        assert_eq!(columns.len(), 9, "invalid corpus row {}", line_number + 1);
        let id = columns[0];
        assert!(!id.is_empty(), "missing case id at row {}", line_number + 1);
        assert!(!columns[1].is_empty(), "missing source for {id}");
        assert!(!columns[2].is_empty(), "missing revision for {id}");
        let profile = columns[3];
        let input = expand_input(columns[4]);
        assert!(
            matches!(columns[5], "accept" | "reject"),
            "invalid expected outcome for {id}"
        );
        assert!(
            matches!(columns[6], "accept" | "reject"),
            "invalid current outcome for {id}"
        );
        assert!(
            matches!(columns[7], "accept" | "reject"),
            "invalid candidate outcome for {id}"
        );
        let current = current_accepts(profile, &input);
        let candidate = candidate_accepts(profile, &input);
        let expected = columns[5] == "accept";
        let expected_current = columns[6] == "accept";
        let expected_candidate = columns[7] == "accept";
        assert_eq!(
            current, expected,
            "SDK/normative expectation drift for {id}"
        );
        assert_eq!(current, expected_current, "stale current outcome for {id}");
        assert_eq!(
            candidate, expected_candidate,
            "stale candidate outcome for {id}"
        );

        counts.compared += 1;
        if current != candidate {
            assert_ne!(columns[8], "none", "unclassified mismatch for {id}");
            counts.mismatches += 1;
            println!(
                "curated-mismatch\tid={id}\tclass={}\tcurrent={current}\tcandidate={candidate}",
                columns[8]
            );
        } else if current {
            assert_eq!(columns[8], "none", "spurious mismatch class for {id}");
            assert_equivalent_components(profile, &input);
        } else {
            assert_eq!(columns[8], "none", "spurious mismatch class for {id}");
        }
    }
    counts
}

fn compare_exhaustive() -> Counts {
    let mut counts = Counts::default();
    let mut classes = BTreeMap::<&str, usize>::new();

    for byte in 0_u8..=127 {
        let ch = char::from(byte);
        compare_generated(
            "method-byte",
            "did",
            &format!("did:a{ch}b:value"),
            &mut counts,
            &mut classes,
        );
        compare_generated(
            "msi-byte",
            "did",
            &format!("did:example:a{ch}b"),
            &mut counts,
            &mut classes,
        );
        compare_generated(
            "path-byte",
            "did-url",
            &format!("did:example:123/a{ch}b"),
            &mut counts,
            &mut classes,
        );
        compare_generated(
            "query-byte",
            "did-url",
            &format!("did:example:123?a{ch}b"),
            &mut counts,
            &mut classes,
        );
        compare_generated(
            "fragment-byte",
            "did-url",
            &format!("did:example:123#a{ch}b"),
            &mut counts,
            &mut classes,
        );
        compare_generated(
            "leading-byte",
            "did-url",
            &format!("{ch}did:example:123"),
            &mut counts,
            &mut classes,
        );
        compare_generated(
            "trailing-byte",
            "did-url",
            &format!("did:example:123{ch}"),
            &mut counts,
            &mut classes,
        );
    }

    for first in 0_u8..=127 {
        for second in 0_u8..=127 {
            compare_generated(
                "percent-pair",
                "did",
                &format!("did:example:a%{}{}b", char::from(first), char::from(second)),
                &mut counts,
                &mut classes,
            );
        }
    }

    for input in [
        "did:example:a:",
        "did:example::",
        "did:example:a::",
        "did:example:a:b:",
    ] {
        compare_generated("terminal-colon", "did", input, &mut counts, &mut classes);
    }

    for (class, count) in classes {
        println!("exhaustive-mismatch\tclass={class}\tcount={count}");
    }
    counts
}

fn compare_generated(
    class: &'static str,
    profile: &str,
    input: &str,
    counts: &mut Counts,
    classes: &mut BTreeMap<&'static str, usize>,
) {
    counts.compared += 1;
    if current_accepts(profile, input) != candidate_accepts(profile, input) {
        counts.mismatches += 1;
        *classes.entry(class).or_default() += 1;
    }
}

fn current_accepts(profile: &str, input: &str) -> bool {
    match profile {
        "did" => Did::parse(input).is_ok(),
        "did-url" => DidUrl::parse(input).is_ok(),
        _ => panic!("unknown profile {profile}"),
    }
}

fn candidate_accepts(profile: &str, input: &str) -> bool {
    Candidate::parse(input).is_ok_and(|value| {
        profile == "did-url"
            || (value.path().is_empty() && value.query().is_none() && value.fragment().is_none())
    })
}

fn assert_equivalent_components(profile: &str, input: &str) {
    let candidate = Candidate::parse(input).expect("candidate accepted corpus value");
    assert_eq!(candidate.as_str(), input);
    match profile {
        "did" => {
            let current = Did::parse(input).expect("current accepted corpus DID");
            assert_eq!(candidate.method(), current.method());
            assert_eq!(candidate.method_id(), current.method_specific_id());
        }
        "did-url" => {
            let current = DidUrl::parse(input).expect("current accepted corpus DID URL");
            assert_eq!(candidate.method(), current.method());
            assert_eq!(candidate.method_id(), current.method_specific_id());
            assert_eq!(candidate.path(), current.path());
            assert_eq!(candidate.query(), current.query());
            assert_eq!(candidate.fragment(), current.fragment());
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

fn demonstrate_facade_mismatches() {
    let current_owned = "did:example:owned".to_owned();
    let current_pointer = current_owned.as_ptr();
    let current = Did::try_from(current_owned).expect("current owned DID");
    let candidate_owned = "did:example:owned".to_owned();
    let candidate_pointer = candidate_owned.as_ptr();
    let candidate = Candidate::try_from(candidate_owned).expect("candidate owned DID");
    println!(
        "owned-pointer\tcurrent_reused={}\tcandidate_reused={}",
        current_pointer == current.as_str().as_ptr(),
        candidate_pointer == candidate.as_str().as_ptr()
    );

    let mut mutable = Candidate::parse("did:example:valid").expect("candidate valid DID");
    mutable.set_method("INVALID");
    mutable.set_method_id("");
    mutable.set_path("not/absolute");
    println!(
        "unchecked-mutation\treparse={}\tstored={:?}",
        Candidate::parse(mutable.as_str()).is_ok(),
        mutable.as_str()
    );

    println!(
        "type-size\tcurrent_did={}\tcurrent_url={}\tcandidate={}",
        size_of::<Did>(),
        size_of::<DidUrl>(),
        size_of::<Candidate>()
    );
}

fn benchmark() {
    const ITERATIONS: usize = 500_000;
    const VALID: &str =
        "did:midnight:undeployed:network:holder_1/credentials/active?limit=20#key-1";
    const INVALID: &str = "did:midnight:undeployed:network:holder_1/invalid path";

    let current_valid = measure(|| DidUrl::parse(black_box(VALID)).is_ok(), ITERATIONS);
    let candidate_valid = measure(|| Candidate::parse(black_box(VALID)).is_ok(), ITERATIONS);
    let current_invalid = measure(|| DidUrl::parse(black_box(INVALID)).is_err(), ITERATIONS);
    let candidate_invalid = measure(|| Candidate::parse(black_box(INVALID)).is_err(), ITERATIONS);
    println!(
        concat!(
            "throughput\titerations={}\tcurrent_valid={:?}\t",
            "candidate_valid={:?}\tcurrent_invalid={:?}\tcandidate_invalid={:?}"
        ),
        ITERATIONS, current_valid, candidate_valid, current_invalid, candidate_invalid
    );
}

fn measure(mut operation: impl FnMut() -> bool, iterations: usize) -> std::time::Duration {
    let started = Instant::now();
    for _ in 0..iterations {
        assert!(black_box(operation()));
    }
    started.elapsed()
}
