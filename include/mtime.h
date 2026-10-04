#ifndef MTIME_H
#define MTIME_H
#include <stdint.h>
#ifdef __cplusplus
extern "C" {
#endif
typedef struct {
  uint8_t valid;
  uint32_t profile;
  int64_t linear_ns_hi;
  uint64_t linear_ns_lo;
  double jd_tt;
  double sun_deg;
  double moon_deg;
  double phase_deg;
  double node_deg;
  double solar_year_phase;
  double synodic_phase;
  double sidereal_phase;
  double anomalistic_phase;
  double draconic_phase;
  double metonic_phase;
  double saros_phase;
  double exeligmos_phase;
} MTimeCStateV1;
MTimeCStateV1 mtime_state_v1_from_tt(double jd_tt, uint8_t experimental_v2);
#ifdef __cplusplus
}
#endif
#endif
