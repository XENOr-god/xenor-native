use xenor_native::{Phase, ReplayTrace, SimulationRuntime, SimulationSeed};

#[test]
fn replay_steps_are_monotonic_and_finalize_each_tick() {
    let trace = ReplayTrace::sample(SimulationSeed::new(42));
    let report = trace.execute().expect("replay should succeed");

    assert_eq!(report.steps.len(), trace.frames().len());
    for (index, step) in report.steps.iter().enumerate() {
        assert_eq!(step.snapshot.tick, index as u64 + 1);
        assert_eq!(step.snapshot.phase, Phase::Finalize);
    }
}

#[test]
fn replay_report_matches_direct_runtime_execution() {
    let trace = ReplayTrace::sample(SimulationSeed::new(77));
    let report = trace.execute().expect("replay should succeed");

    let mut runtime = SimulationRuntime::new(trace.seed()).expect("runtime should initialize");
    let snapshot = runtime
        .run(trace.frames())
        .expect("direct runtime execution should succeed");
    let checksum = runtime.checksum().expect("checksum should be available");

    assert_eq!(snapshot, report.final_snapshot);
    assert_eq!(checksum, report.final_checksum);
    assert_eq!(runtime.tick(), trace.frames().len() as u64);
}
