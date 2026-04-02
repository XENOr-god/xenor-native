#include "xenor_internal.hpp"

namespace xenor::native {

uint64_t splitmix64(uint64_t value) {
  value += 0x9e3779b97f4a7c15ULL;
  value = (value ^ (value >> 30U)) * 0xbf58476d1ce4e5b9ULL;
  value = (value ^ (value >> 27U)) * 0x94d049bb133111ebULL;
  return value ^ (value >> 31U);
}

uint64_t mix_input_frame(const XenorInputFrame& input, uint64_t tick) {
  uint64_t mix = splitmix64(tick ^ 0x517cc1b727220a95ULL);
  mix ^= splitmix64(static_cast<uint64_t>(input.throttle) << 32U);
  mix ^= splitmix64(static_cast<uint64_t>(static_cast<uint32_t>(input.steer)));
  mix ^= splitmix64(static_cast<uint64_t>(input.action_mask) << 56U);
  return splitmix64(mix);
}

uint64_t magnitude64(int64_t value) {
  if (value >= 0) {
    return static_cast<uint64_t>(value);
  }

  return static_cast<uint64_t>(-(value + 1)) + 1U;
}

}  // namespace xenor::native
