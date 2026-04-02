use xenor_native::{InputFrame, ReplayTrace, SimulationSeed};

fn reference_frames() -> Vec<InputFrame> {
    vec![
        InputFrame::new(12, 1, 0),
        InputFrame::new(24, -1, 1),
        InputFrame::new(18, 2, 0),
        InputFrame::new(26, 0, 2),
        InputFrame::new(20, -3, 1),
    ]
}

#[test]
fn same_seed_and_inputs_yield_same_terminal_state() {
    let seed = SimulationSeed::new(0xfeed_beef);
    let trace = ReplayTrace::new(seed, reference_frames());

    let first = trace.execute().expect("first replay should succeed");
    let second = trace.execute().expect("second replay should succeed");

    assert_eq!(first.final_snapshot, second.final_snapshot);
    assert_eq!(first.final_checksum, second.final_checksum);
}

#[test]
fn changed_input_changes_terminal_checksum() {
    let seed = SimulationSeed::new(0xfeed_beef);
    let baseline = ReplayTrace::new(seed, reference_frames())
        .execute()
        .expect("baseline replay should succeed");

    let mut mutated_frames = reference_frames();
    mutated_frames[2] = InputFrame::new(19, 2, 1);
    let mutated = ReplayTrace::new(seed, mutated_frames)
        .execute()
        .expect("mutated replay should succeed");

    assert_ne!(baseline.final_checksum, mutated.final_checksum);
}
