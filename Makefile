.PHONY: build test native-build smoke

build:
	cargo build

test:
	cargo test

native-build:
	cmake -S cpp -B cpp/build
	cmake --build cpp/build

smoke:
	mkdir -p target
	cargo run --bin xenor-cli -- --seed 17 --snapshot --emit-replay target/ci-replay.json --repeat 3 --benchmark-out target/ci-benchmark.csv
	python3 python/tools/analyze_replay.py target/ci-replay.json
	python3 python/tools/benchmark_report.py target/ci-benchmark.csv
