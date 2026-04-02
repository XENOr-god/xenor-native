#ifndef XENOR_SIM_H
#define XENOR_SIM_H

#include <stdint.h>

#include "xenor_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct XenorSimulation XenorSimulation;

XenorSimulation *xenor_sim_create(XenorSeed seed);
void xenor_sim_destroy(XenorSimulation *simulation);

int32_t xenor_sim_reset(XenorSimulation *simulation, XenorSeed seed);
int32_t xenor_sim_step(XenorSimulation *simulation, const XenorInputFrame *input);
int32_t xenor_sim_snapshot(const XenorSimulation *simulation, XenorStateSnapshot *out_snapshot);
int32_t xenor_sim_checksum(const XenorSimulation *simulation, XenorChecksum *out_checksum);
uint64_t xenor_sim_tick(const XenorSimulation *simulation);

#ifdef __cplusplus
}
#endif

#endif
