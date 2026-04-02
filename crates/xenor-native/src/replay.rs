use core::fmt::Write;

use crate::checksum::Checksum;
use crate::runtime::{RuntimeError, SimulationRuntime};
use crate::snapshot::{InputFrame, SimulationSeed, StateSnapshot};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayTrace {
    seed: SimulationSeed,
    frames: Vec<InputFrame>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayStep {
    pub input: InputFrame,
    pub snapshot: StateSnapshot,
    pub checksum: Checksum,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayReport {
    pub seed: SimulationSeed,
    pub steps: Vec<ReplayStep>,
    pub final_snapshot: StateSnapshot,
    pub final_checksum: Checksum,
}

impl ReplayTrace {
    pub fn new(seed: SimulationSeed, frames: Vec<InputFrame>) -> Self {
        Self { seed, frames }
    }

    pub fn sample(seed: SimulationSeed) -> Self {
        let frames = vec![
            InputFrame::new(14, 1, 0),
            InputFrame::new(22, -2, 1),
            InputFrame::new(30, 0, 0),
            InputFrame::new(18, 3, 0),
            InputFrame::new(26, -1, 2),
            InputFrame::new(28, 1, 1),
        ];
        Self { seed, frames }
    }

    pub fn seed(&self) -> SimulationSeed {
        self.seed
    }

    pub fn frames(&self) -> &[InputFrame] {
        &self.frames
    }

    pub fn execute(&self) -> Result<ReplayReport, RuntimeError> {
        let mut runtime = SimulationRuntime::new(self.seed)?;
        let mut steps = Vec::with_capacity(self.frames.len());

        for input in &self.frames {
            let snapshot = runtime.step(*input)?;
            let checksum = runtime.checksum()?;
            steps.push(ReplayStep {
                input: *input,
                snapshot,
                checksum,
            });
        }

        let final_snapshot = runtime.snapshot()?;
        let final_checksum = runtime.checksum()?;

        Ok(ReplayReport {
            seed: self.seed,
            steps,
            final_snapshot,
            final_checksum,
        })
    }
}

impl ReplayReport {
    pub fn to_json_pretty(&self) -> String {
        let mut output = String::new();
        writeln!(&mut output, "{{").unwrap();
        writeln!(&mut output, "  \"seed\": {},", self.seed.value()).unwrap();
        writeln!(
            &mut output,
            "  \"final_checksum\": \"{}\",",
            self.final_checksum
        )
        .unwrap();
        writeln!(&mut output, "  \"final_snapshot\": {{").unwrap();
        append_snapshot(&mut output, &self.final_snapshot, 4, true);
        writeln!(&mut output, "  }},").unwrap();
        writeln!(&mut output, "  \"steps\": [").unwrap();

        for (index, step) in self.steps.iter().enumerate() {
            writeln!(&mut output, "    {{").unwrap();
            writeln!(
                &mut output,
                "      \"input\": {{ \"throttle\": {}, \"steer\": {}, \"action_mask\": {} }},",
                step.input.throttle, step.input.steer, step.input.action_mask
            )
            .unwrap();
            writeln!(&mut output, "      \"checksum\": \"{}\",", step.checksum).unwrap();
            writeln!(&mut output, "      \"snapshot\": {{").unwrap();
            append_snapshot(&mut output, &step.snapshot, 8, true);
            writeln!(&mut output, "      }}").unwrap();
            if index + 1 == self.steps.len() {
                writeln!(&mut output, "    }}").unwrap();
            } else {
                writeln!(&mut output, "    }},").unwrap();
            }
        }

        writeln!(&mut output, "  ]").unwrap();
        writeln!(&mut output, "}}").unwrap();
        output
    }
}

fn append_snapshot(output: &mut String, snapshot: &StateSnapshot, indent: usize, trailing: bool) {
    let pad = " ".repeat(indent);
    writeln!(output, "{pad}\"tick\": {},", snapshot.tick).unwrap();
    writeln!(output, "{pad}\"phase\": \"{}\",", snapshot.phase).unwrap();
    writeln!(output, "{pad}\"position\": {},", snapshot.position).unwrap();
    writeln!(output, "{pad}\"velocity\": {},", snapshot.velocity).unwrap();
    writeln!(output, "{pad}\"energy\": {},", snapshot.energy).unwrap();
    writeln!(
        output,
        "{pad}\"accumulator\": \"{:016x}\",",
        snapshot.accumulator
    )
    .unwrap();
    writeln!(
        output,
        "{pad}\"last_input_mix\": \"{:016x}\",",
        snapshot.last_input_mix
    )
    .unwrap();
    if trailing {
        writeln!(output, "{pad}\"flags\": {}", snapshot.flags).unwrap();
    } else {
        write!(output, "{pad}\"flags\": {}", snapshot.flags).unwrap();
    }
}
