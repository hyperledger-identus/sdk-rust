use identus_siopv2::{EventId, SiopFlow};

#[test]
fn siopv2_property_event_ids_accept_every_exact_boundary_length() {
    for length in 1..=128 {
        assert!(EventId::new(&"a".repeat(length), 128).is_ok());
    }
    assert!(EventId::new("", 128).is_err());
    assert!(EventId::new(&"a".repeat(129), 128).is_err());
    assert!(EventId::new("line\nbreak", 128).is_err());
}

#[test]
fn siopv2_property_terminal_events_are_idempotent_and_conflicts_fail() {
    let mut terminal = SiopFlow::Refused {
        event_id: EventId::new("refusal-1", 128).unwrap(),
    };
    terminal
        .refuse(EventId::new("refusal-1", 128).unwrap())
        .expect("same event is idempotent");
    assert!(matches!(terminal, SiopFlow::Refused { .. }));
    assert_eq!(
        terminal
            .refuse(EventId::new("refusal-2", 128).unwrap())
            .unwrap_err(),
        identus_siopv2::Siopv2Error::EventConflict,
    );
    assert!(matches!(terminal, SiopFlow::Refused { .. }));
}
