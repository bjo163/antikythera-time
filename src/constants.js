export const NS_PER_SECOND = 1_000_000_000n;
export const SECONDS_PER_DAY = 86_400n;
export const NS_PER_DAY = SECONDS_PER_DAY * NS_PER_SECOND;
export const JULIAN_YEAR_SECONDS = 31_557_600n;
export const JULIAN_YEAR_NS = JULIAN_YEAR_SECONDS * NS_PER_SECOND;

// J2000.0 = JD 2451545.0 TT. This is a coordinate epoch, not "the beginning of time".
export const J2000_JD_TT = 2_451_545.0;

// Mean synodic month used only by the cycle-model layer (29.530588853 d).
// It is not a claim that every observed lunation has exactly this duration.
export const MEAN_SYNODIC_MONTH_NS = 2_551_442_876_899_200n;
