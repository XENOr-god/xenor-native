#pragma once

#include <cstdint>

#include "xenor_sim.h"

namespace xenor::native {

struct State {
  uint64_t seed;
  uint64_t tick;
  int64_t position;
  int64_t velocity;
  uint64_t energy;
  uint64_t accumulator;
  uint64_t last_input_mix;
  uint32_t phase;
};

uint64_t splitmix64(uint64_t value);
uint64_t mix_input_frame(const XenorInputFrame& input, uint64_t tick);
uint64_t magnitude64(int64_t value);

State make_initial_state(XenorSeed seed);
void write_snapshot(const State& state, XenorStateSnapshot* out_snapshot);
XenorChecksum checksum_from_state(const State& state);

}  // namespace xenor::native

struct XenorSimulation {
  xenor::native::State state;
};
