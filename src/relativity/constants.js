// U-Time v0.8 relativistic constants.
//
// These constants follow the IAU SOFA conventions:
// L_G = 1 - d(TT)/d(TCG)
// L_B = 1 - d(TDB)/d(TCB)
// TDB0 = TDB offset (seconds) at the 1977 reference epoch.
//
// This is a derived implementation. It is NOT software provided by,
// or endorsed by, the IAU SOFA Board. See docs/sofa-derived-work.md.

export const REL_DAY_SECONDS = 86_400;
export const REL_JD_MJD0 = 2_400_000.5;
export const REL_MJD_1977 = 43_144.0;
export const REL_TT_MINUS_TAI_SECONDS = 32.184;
export const REL_LG = 6.969290134e-10;
export const REL_LB = 1.550519768e-8;
export const REL_TDB0_SECONDS = -6.55e-5;
export const REL_J2000_JD = 2_451_545.0;
