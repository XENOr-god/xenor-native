use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use xenor_native::{InputFrame, ReplayTrace, SimulationSeed};

#[derive(Debug)]
struct CliOptions {
    seed: u64,
    input_file: Option<PathBuf>,
    emit_replay: Option<PathBuf>,
    emit_snapshot: bool,
    repeat: usize,
    benchmark_out: Option<PathBuf>,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let options = parse_args(env::args().skip(1))?;
    let seed = SimulationSeed::new(options.seed);
    let frames = match &options.input_file {
        Some(path) => load_frames(path)?,
        None => ReplayTrace::sample(seed).frames().to_vec(),
    };

    let trace = ReplayTrace::new(seed, frames);
    let report = trace.execute()?;

    println!("seed: {}", trace.seed().value());
    println!("ticks: {}", report.steps.len());
    println!("final checksum: {}", report.final_checksum);
    if options.emit_snapshot {
        println!("final snapshot: {}", report.final_snapshot.summary());
    }

    if let Some(path) = &options.emit_replay {
        fs::write(path, report.to_json_pretty())?;
        println!("replay artifact: {}", path.display());
    }

    if options.repeat > 1 {
        let samples = benchmark(&trace, options.repeat)?;
        if let Some(path) = &options.benchmark_out {
            fs::write(path, render_benchmark_csv(&samples))?;
            println!("benchmark artifact: {}", path.display());
        }

        let total: u128 = samples.iter().map(|sample| sample.duration_ns).sum();
        let average = total / samples.len() as u128;
        let min = samples
            .iter()
            .map(|sample| sample.duration_ns)
            .min()
            .unwrap_or(0);
        let max = samples
            .iter()
            .map(|sample| sample.duration_ns)
            .max()
            .unwrap_or(0);
        println!("benchmark repeat: {}", samples.len());
        println!("benchmark mean(ns): {}", average);
        println!("benchmark min(ns): {}", min);
        println!("benchmark max(ns): {}", max);
    }

    Ok(())
}

fn parse_args<I>(args: I) -> Result<CliOptions, Box<dyn std::error::Error>>
where
    I: IntoIterator<Item = String>,
{
    let mut options = CliOptions {
        seed: 17,
        input_file: None,
        emit_replay: None,
        emit_snapshot: false,
        repeat: 1,
        benchmark_out: None,
    };

    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--seed" => {
                options.seed = next_value(&mut args, "--seed")?.parse()?;
            }
            "--input-file" => {
                options.input_file = Some(PathBuf::from(next_value(&mut args, "--input-file")?));
            }
            "--emit-replay" => {
                options.emit_replay = Some(PathBuf::from(next_value(&mut args, "--emit-replay")?));
            }
            "--snapshot" => {
                options.emit_snapshot = true;
            }
            "--repeat" => {
                options.repeat = next_value(&mut args, "--repeat")?.parse()?;
                if options.repeat == 0 {
                    return Err("--repeat must be greater than zero".into());
                }
            }
            "--benchmark-out" => {
                options.benchmark_out =
                    Some(PathBuf::from(next_value(&mut args, "--benchmark-out")?));
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            _ => return Err(format!("unknown argument: {arg}").into()),
        }
    }

    Ok(options)
}

fn next_value<I>(args: &mut I, flag: &str) -> Result<String, Box<dyn std::error::Error>>
where
    I: Iterator<Item = String>,
{
    args.next()
        .ok_or_else(|| format!("missing value for {flag}").into())
}

fn print_help() {
    println!("xenor-cli");
    println!("  --seed <u64>            simulation seed");
    println!("  --input-file <path>     optional input replay file");
    println!("  --emit-replay <path>    write replay report JSON");
    println!("  --snapshot              print final snapshot summary");
    println!("  --repeat <count>        rerun the replay for benchmark sampling");
    println!("  --benchmark-out <path>  write benchmark CSV");
}

fn load_frames(path: &Path) -> Result<Vec<InputFrame>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let mut frames = Vec::new();
    let mut saw_header = false;

    for (index, raw_line) in content.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<_> = line.split(',').map(str::trim).collect();
        if !saw_header
            && parts.len() == 3
            && parts[0].eq_ignore_ascii_case("throttle")
            && parts[1].eq_ignore_ascii_case("steer")
            && parts[2].eq_ignore_ascii_case("action_mask")
        {
            saw_header = true;
            continue;
        }

        if parts.len() != 3 {
            return Err(format!("invalid frame at line {}", index + 1).into());
        }

        let throttle: u32 = parts[0].parse()?;
        let steer: i32 = parts[1].parse()?;
        let action_mask: u8 = parts[2].parse()?;
        frames.push(InputFrame::new(throttle, steer, action_mask));
    }

    if frames.is_empty() {
        return Err("input file did not contain any frames".into());
    }

    Ok(frames)
}

#[derive(Clone, Copy, Debug)]
struct BenchmarkSample {
    iteration: usize,
    duration_ns: u128,
    checksum: u64,
}

fn benchmark(
    trace: &ReplayTrace,
    repeat: usize,
) -> Result<Vec<BenchmarkSample>, Box<dyn std::error::Error>> {
    let mut samples = Vec::with_capacity(repeat);
    let mut expected_checksum = None;

    for iteration in 0..repeat {
        let start = Instant::now();
        let report = trace.execute()?;
        let duration_ns = start.elapsed().as_nanos();
        let checksum = report.final_checksum.value();

        if let Some(expected) = expected_checksum {
            if checksum != expected {
                return Err(format!(
                    "non-deterministic checksum detected at iteration {iteration}: expected {expected:016x}, got {checksum:016x}"
                )
                .into());
            }
        } else {
            expected_checksum = Some(checksum);
        }

        samples.push(BenchmarkSample {
            iteration,
            duration_ns,
            checksum,
        });
    }

    Ok(samples)
}

fn render_benchmark_csv(samples: &[BenchmarkSample]) -> String {
    let mut output = String::from("iteration,duration_ns,checksum\n");
    for sample in samples {
        output.push_str(&format!(
            "{},{},{:016x}\n",
            sample.iteration, sample.duration_ns, sample.checksum
        ));
    }
    output
}
