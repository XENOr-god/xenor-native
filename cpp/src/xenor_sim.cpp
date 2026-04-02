#include "xenor_internal.hpp"

#include <new>

namespace xenor::native {

namespace {

void run_input_phase(State& state, const XenorInputFrame& input) {
  state.phase = XENOR_PHASE_INPUT;

  uint64_t input_mix = mix_input_frame(input, state.tick);
  input_mix ^= splitmix64(state.seed + state.tick);
  input_mix ^= splitmix64(state.accumulator ^ static_cast<uint64_t>(state.velocity));

  state.last_input_mix = splitmix64(input_mix);
  state.accumulator = splitmix64(state.accumulator ^ state.last_input_mix);
  state.energy += 1U + static_cast<uint64_t>(input.throttle & 0x3fU);
}

void run_simulation_phase(State& state, const XenorInputFrame& input) {
  state.phase = XENOR_PHASE_SIMULATE;

  const int64_t thrust = static_cast<int64_t>(input.throttle) - 24;
  const int64_t steering = static_cast<int64_t>(input.steer) * 5;
  const int64_t entropy_bias = static_cast<int64_t>((state.last_input_mix >> 61U) & 0x7U) - 3;
  const int64_t assist = (input.action_mask & 0x1U) != 0U ? 9 : 0;
  const int64_t brake = (input.action_mask & 0x2U) != 0U ? 13 : 0;
  const int64_t damping = state.velocity / 6;
  const int64_t acceleration = thrust + steering + entropy_bias + assist - brake - damping;

  state.velocity += acceleration;
  state.position += state.velocity + static_cast<int64_t>((state.last_input_mix >> 17U) & 0x1fU) - 15;
  state.energy += 1U + magnitude64(state.velocity % 97);
}

void run_finalize_phase(State& state, const XenorInputFrame& input) {
  state.phase = XENOR_PHASE_FINALIZE;

  const uint64_t tick_mix =
      splitmix64(state.last_input_mix ^ state.accumulator ^ state.seed ^ (state.tick + 1U));

  state.accumulator = splitmix64(state.accumulator + tick_mix + static_cast<uint64_t>(state.position));
  state.energy = (state.energy ^ (tick_mix & 0x3ffU)) + static_cast<uint64_t>(state.tick + 1U) +
                 static_cast<uint64_t>(input.action_mask & 0x3U);
  state.tick += 1U;
}

}  // namespace

}  // namespace xenor::native

extern "C" {

XenorSimulation* xenor_sim_create(XenorSeed seed) {
  XenorSimulation* simulation = new (std::nothrow) XenorSimulation{};
  if (simulation == nullptr) {
    return nullptr;
  }

  simulation->state = xenor::native::make_initial_state(seed);
  return simulation;
}

void xenor_sim_destroy(XenorSimulation* simulation) {
  delete simulation;
}

int32_t xenor_sim_reset(XenorSimulation* simulation, XenorSeed seed) {
  if (simulation == nullptr) {
    return XENOR_STATUS_NULL;
  }

  simulation->state = xenor::native::make_initial_state(seed);
  return XENOR_STATUS_OK;
}

int32_t xenor_sim_step(XenorSimulation* simulation, const XenorInputFrame* input) {
  if (simulation == nullptr || input == nullptr) {
    return XENOR_STATUS_NULL;
  }

  xenor::native::run_input_phase(simulation->state, *input);
  xenor::native::run_simulation_phase(simulation->state, *input);
  xenor::native::run_finalize_phase(simulation->state, *input);
  return XENOR_STATUS_OK;
}

int32_t xenor_sim_snapshot(const XenorSimulation* simulation, XenorStateSnapshot* out_snapshot) {
  if (simulation == nullptr || out_snapshot == nullptr) {
    return XENOR_STATUS_NULL;
  }

  xenor::native::write_snapshot(simulation->state, out_snapshot);
  return XENOR_STATUS_OK;
}

int32_t xenor_sim_checksum(const XenorSimulation* simulation, XenorChecksum* out_checksum) {
  if (simulation == nullptr || out_checksum == nullptr) {
    return XENOR_STATUS_NULL;
  }

  *out_checksum = xenor::native::checksum_from_state(simulation->state);
  return XENOR_STATUS_OK;
}

uint64_t xenor_sim_tick(const XenorSimulation* simulation) {
  if (simulation == nullptr) {
    return 0;
  }

  return simulation->state.tick;
}

}  // extern "C"
