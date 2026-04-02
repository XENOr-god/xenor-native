#ifndef XENOR_TYPES_H
#define XENOR_TYPES_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

enum XenorStatus {
  XENOR_STATUS_OK = 0,
  XENOR_STATUS_NULL = 1,
  XENOR_STATUS_BAD_ARGUMENT = 2,
};

enum XenorPhase {
  XENOR_PHASE_INPUT = 0,
  XENOR_PHASE_SIMULATE = 1,
  XENOR_PHASE_FINALIZE = 2,
};

typedef struct XenorSeed {
  uint64_t value;
} XenorSeed;

typedef struct XenorInputFrame {
  uint32_t throttle;
  int32_t steer;
  uint8_t action_mask;
  uint8_t reserved[3];
} XenorInputFrame;

typedef struct XenorStateSnapshot {
  uint64_t tick;
  int64_t position;
  int64_t velocity;
  uint64_t energy;
  uint64_t accumulator;
  uint64_t last_input_mix;
  uint32_t phase;
  uint32_t flags;
} XenorStateSnapshot;

typedef struct XenorChecksum {
  uint64_t value;
} XenorChecksum;

#ifdef __cplusplus
}
#endif

#endif
