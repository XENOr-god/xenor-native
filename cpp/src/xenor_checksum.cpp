#include "xenor_internal.hpp"

namespace xenor::native {

namespace {

uint64_t fold_word(uint64_t accumulator, uint64_t word) {
  accumulator ^= word + 0x9e3779b97f4a7c15ULL + (accumulator << 6U) + (accumulator >> 2U);
  return splitmix64(accumulator);
}

}  // namespace

XenorChecksum checksum_from_state(const State& state) {
  uint64_t accumulator = 0x6eed0e9da4d94a4fULL;
  accumulator = fold_word(accumulator, state.seed);
  accumulator = fold_word(accumulator, state.tick);
  accumulator = fold_word(accumulator, static_cast<uint64_t>(state.position));
  accumulator = fold_word(accumulator, static_cast<uint64_t>(state.velocity));
  accumulator = fold_word(accumulator, state.energy);
  accumulator = fold_word(accumulator, state.accumulator);
  accumulator = fold_word(accumulator, state.last_input_mix);
  accumulator = fold_word(accumulator, state.phase);
  return XenorChecksum{accumulator};
}

}  // namespace xenor::native
