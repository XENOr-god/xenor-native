#include "xenor_internal.hpp"

namespace xenor::native {

State make_initial_state(XenorSeed seed) {
  State state{};
  state.seed = seed.value;
  state.tick = 0;
  state.position = static_cast<int64_t>(splitmix64(seed.value ^ 0x243f6a8885a308d3ULL) & 0x3fffULL) -
                   0x1fff;
  state.velocity = static_cast<int64_t>(splitmix64(seed.value ^ 0x13198a2e03707344ULL) & 0x1ffULL) -
                   0x100;
  state.energy = 256U + (splitmix64(seed.value ^ 0xa4093822299f31d0ULL) & 0x7ffULL);
  state.accumulator = splitmix64(seed.value ^ 0x082efa98ec4e6c89ULL);
  state.last_input_mix = splitmix64(seed.value ^ 0x452821e638d01377ULL);
  state.phase = XENOR_PHASE_FINALIZE;
  return state;
}

void write_snapshot(const State& state, XenorStateSnapshot* out_snapshot) {
  out_snapshot->tick = state.tick;
  out_snapshot->position = state.position;
  out_snapshot->velocity = state.velocity;
  out_snapshot->energy = state.energy;
  out_snapshot->accumulator = state.accumulator;
  out_snapshot->last_input_mix = state.last_input_mix;
  out_snapshot->phase = state.phase;
  out_snapshot->flags = (state.velocity < 0 ? 0x1U : 0U) | ((state.energy & 0x1U) ? 0x2U : 0U);
}

}  // namespace xenor::native
