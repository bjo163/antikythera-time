use mtime_astro::{topocentric_horizon_iau2006, Body};
use mtime_core::{EarthObserver, QualityClass, TemporalError, SECONDS_PER_DAY};
use mtime_eop::{interpolate, utc_jd_to_ut1_jd, EopRecord};
use mtime_hijri::{GeometrySemantics, HijriAstronomicalState};
use mtime_spk::SpkEphemeris;
use mtime_timescales::{tt_to_tdb, DtrProvider, UtcInstant, utc_to_tt};

pub const JD_UNIX_EPOCH: f64 = 2_440_587.5;

#[derive(Debug, Clone, PartialEq)]
pub enum HilalEngineError {
    Temporal(TemporalError),
    MissingEop,
    InvalidBracket,
    IterationLimit,
}

impl From<TemporalError> for HilalEngineError {
    fn from(value: TemporalError) -> Self {
        Self::Temporal(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SunsetDefinition {
    /// Target geometric Sun-center altitude in degrees.
    /// The caller is responsible for choosing the calendrical/astronomical
    /// definition; M-Time does not silently hard-code -0.833° or another rule.
    pub sun_center_altitude_deg: f64,
}

impl SunsetDefinition {
    #[must_use]
    pub const fn geometric_center_horizon() -> Self {
        Self {
            sun_center_altitude_deg: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SunsetSolution {
    pub jd_utc: f64,
    pub jd_ut1: f64,
    pub target_sun_altitude_deg: f64,
    pub residual_deg: f64,
}

pub struct HilalEngine<'a> {
    ephemeris: SpkEphemeris<'a>,
}

impl<'a> HilalEngine<'a> {
    pub fn from_spk_bytes(bytes: &'a [u8]) -> Result<Self, HilalEngineError> {
        Ok(Self {
            ephemeris: SpkEphemeris::from_bytes(bytes)?,
        })
    }

    pub fn state_at_utc<P: DtrProvider>(
        &self,
        utc: UtcInstant,
        observer: EarthObserver,
        site_id: impl Into<String>,
        eop_rows: &[EopRecord],
        dtr_provider: &P,
    ) -> Result<HijriAstronomicalState, HilalEngineError> {
        let jd_utc = utc_to_jd(utc);
        let mjd_utc = jd_utc - 2_400_000.5;
        let eop = interpolate(eop_rows, mjd_utc).map_err(|_| HilalEngineError::MissingEop)?;
        let jd_ut1 = utc_jd_to_ut1_jd(jd_utc, eop);

        let tt = utc_to_tt(utc)?;
        let dtr = dtr_provider.dtr(&tt)?;
        let tdb = tt_to_tdb(&tt, dtr)?;
        let tdbp = tdb.jd_parts();

        let moon = self
            .ephemeris
            .geocentric_vector_km(Body::Moon, (tdbp.d1, tdbp.d2))?;
        let horizon = topocentric_horizon_iau2006(
            moon,
            observer,
            (tt.jd_parts().d1, tt.jd_parts().d2),
            (2_400_000.5, jd_ut1 - 2_400_000.5),
            eop.xp_arcsec,
            eop.yp_arcsec,
        )?;
        let elongation = self
            .ephemeris
            .geometric_elongation_deg((tdbp.d1, tdbp.d2))?;

        Ok(HijriAstronomicalState {
            conjunction_jd_tt: None,
            sunset_jd_ut1: None,
            moon_altitude_topocentric_deg: horizon.altitude_deg,
            elongation_geocentric_deg: elongation,
            geometry_semantics: GeometrySemantics::mabims_required(),
            moon_age_hours: None,
            moon_lag_minutes: None,
            site_id: site_id.into(),
            ephemeris_source:
                "JPL DE440 SPK / ICRF + IAU 2006/2000A + IERS EOP; airless geometric"
                    .into(),
            quality: QualityClass::HighPrecision,
        })
    }

    pub fn find_sunset_utc<P: DtrProvider>(
        &self,
        start_utc_jd: f64,
        end_utc_jd: f64,
        observer: EarthObserver,
        eop_rows: &[EopRecord],
        dtr_provider: &P,
        definition: SunsetDefinition,
        tolerance_seconds: f64,
    ) -> Result<SunsetSolution, HilalEngineError> {
        if !start_utc_jd.is_finite()
            || !end_utc_jd.is_finite()
            || start_utc_jd >= end_utc_jd
            || !definition.sun_center_altitude_deg.is_finite()
            || !tolerance_seconds.is_finite()
            || tolerance_seconds <= 0.0
        {
            return Err(HilalEngineError::InvalidBracket);
        }

        let mut lo = start_utc_jd;
        let mut hi = end_utc_jd;
        let mut flo =
            self.sun_altitude_utc(lo, observer, eop_rows, dtr_provider)?
                - definition.sun_center_altitude_deg;
        let fhi =
            self.sun_altitude_utc(hi, observer, eop_rows, dtr_provider)?
                - definition.sun_center_altitude_deg;

        if flo < 0.0 || fhi > 0.0 || flo.signum() == fhi.signum() {
            return Err(HilalEngineError::InvalidBracket);
        }

        for _ in 0..80 {
            let mid = (lo + hi) / 2.0;
            let fm =
                self.sun_altitude_utc(mid, observer, eop_rows, dtr_provider)?
                    - definition.sun_center_altitude_deg;
            if (hi - lo) * SECONDS_PER_DAY <= tolerance_seconds {
                let eop = interpolate(eop_rows, mid - 2_400_000.5)
                    .map_err(|_| HilalEngineError::MissingEop)?;
                return Ok(SunsetSolution {
                    jd_utc: mid,
                    jd_ut1: utc_jd_to_ut1_jd(mid, eop),
                    target_sun_altitude_deg: definition.sun_center_altitude_deg,
                    residual_deg: fm,
                });
            }
            if fm > 0.0 {
                lo = mid;
                flo = fm;
            } else {
                hi = mid;
            }
        }
        let _ = flo;
        Err(HilalEngineError::IterationLimit)
    }

    pub fn state_at_sunset<P: DtrProvider>(
        &self,
        start_utc_jd: f64,
        end_utc_jd: f64,
        observer: EarthObserver,
        site_id: impl Into<String>,
        eop_rows: &[EopRecord],
        dtr_provider: &P,
        definition: SunsetDefinition,
        tolerance_seconds: f64,
    ) -> Result<(SunsetSolution, HijriAstronomicalState), HilalEngineError> {
        let sunset = self.find_sunset_utc(
            start_utc_jd,
            end_utc_jd,
            observer,
            eop_rows,
            dtr_provider,
            definition,
            tolerance_seconds,
        )?;
        let utc = jd_to_utc(sunset.jd_utc)?;
        let mut state =
            self.state_at_utc(utc, observer, site_id, eop_rows, dtr_provider)?;
        state.sunset_jd_ut1 = Some(sunset.jd_ut1);
        Ok((sunset, state))
    }

    fn sun_altitude_utc<P: DtrProvider>(
        &self,
        jd_utc: f64,
        observer: EarthObserver,
        eop_rows: &[EopRecord],
        dtr_provider: &P,
    ) -> Result<f64, HilalEngineError> {
        let utc = jd_to_utc(jd_utc)?;
        let mjd = jd_utc - 2_400_000.5;
        let eop = interpolate(eop_rows, mjd).map_err(|_| HilalEngineError::MissingEop)?;
        let jd_ut1 = utc_jd_to_ut1_jd(jd_utc, eop);
        let tt = utc_to_tt(utc)?;
        let dtr = dtr_provider.dtr(&tt)?;
        let tdb = tt_to_tdb(&tt, dtr)?;
        let p = tdb.jd_parts();
        let sun = self
            .ephemeris
            .geocentric_vector_km(Body::Sun, (p.d1, p.d2))?;
        Ok(topocentric_horizon_iau2006(
            sun,
            observer,
            (tt.jd_parts().d1, tt.jd_parts().d2),
            (2_400_000.5, jd_ut1 - 2_400_000.5),
            eop.xp_arcsec,
            eop.yp_arcsec,
        )?
        .altitude_deg)
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolarCrossingDirection {
    Rising,
    Setting,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SolarAltitudeSolution {
    pub jd_utc: f64,
    pub jd_ut1: f64,
    pub target_sun_altitude_deg: f64,
    pub residual_deg: f64,
    pub direction: SolarCrossingDirection,
}

pub fn find_solar_altitude_crossing_utc<P: DtrProvider>(
    engine: &HilalEngine<'_>,
    start_utc_jd: f64,
    end_utc_jd: f64,
    observer: EarthObserver,
    eop_rows: &[EopRecord],
    dtr_provider: &P,
    target_sun_altitude_deg: f64,
    direction: SolarCrossingDirection,
    tolerance_seconds: f64,
) -> Result<SolarAltitudeSolution, HilalEngineError> {
    if !start_utc_jd.is_finite()
        || !end_utc_jd.is_finite()
        || start_utc_jd >= end_utc_jd
        || !target_sun_altitude_deg.is_finite()
        || !tolerance_seconds.is_finite()
        || tolerance_seconds <= 0.0
    {
        return Err(HilalEngineError::InvalidBracket);
    }

    let mut lo = start_utc_jd;
    let mut hi = end_utc_jd;
    let mut flo =
        engine.sun_altitude_utc(lo, observer, eop_rows, dtr_provider)? - target_sun_altitude_deg;
    let fhi =
        engine.sun_altitude_utc(hi, observer, eop_rows, dtr_provider)? - target_sun_altitude_deg;

    let valid_direction = match direction {
        SolarCrossingDirection::Rising => flo <= 0.0 && fhi >= 0.0,
        SolarCrossingDirection::Setting => flo >= 0.0 && fhi <= 0.0,
    };
    if !valid_direction || flo.signum() == fhi.signum() {
        return Err(HilalEngineError::InvalidBracket);
    }

    for _ in 0..80 {
        let mid = (lo + hi) / 2.0;
        let fm =
            engine.sun_altitude_utc(mid, observer, eop_rows, dtr_provider)?
                - target_sun_altitude_deg;
        if (hi - lo) * SECONDS_PER_DAY <= tolerance_seconds {
            let eop = interpolate(eop_rows, mid - 2_400_000.5)
                .map_err(|_| HilalEngineError::MissingEop)?;
            return Ok(SolarAltitudeSolution {
                jd_utc: mid,
                jd_ut1: utc_jd_to_ut1_jd(mid, eop),
                target_sun_altitude_deg,
                residual_deg: fm,
                direction,
            });
        }

        match direction {
            SolarCrossingDirection::Rising => {
                if fm < 0.0 {
                    lo = mid;
                    flo = fm;
                } else {
                    hi = mid;
                }
            }
            SolarCrossingDirection::Setting => {
                if fm > 0.0 {
                    lo = mid;
                    flo = fm;
                } else {
                    hi = mid;
                }
            }
        }
    }
    let _ = flo;
    Err(HilalEngineError::IterationLimit)
}

#[must_use]
pub fn utc_to_jd(utc: UtcInstant) -> f64 {
    JD_UNIX_EPOCH
        + (utc.unix_seconds as f64 + f64::from(utc.nanoseconds) / 1e9) / SECONDS_PER_DAY
}

pub fn jd_to_utc(jd_utc: f64) -> Result<UtcInstant, HilalEngineError> {
    if !jd_utc.is_finite() {
        return Err(HilalEngineError::InvalidBracket);
    }
    let total = (jd_utc - JD_UNIX_EPOCH) * SECONDS_PER_DAY;
    let mut seconds = total.floor() as i64;
    let mut nanoseconds = ((total - seconds as f64) * 1e9).round() as i64;
    if nanoseconds >= 1_000_000_000 {
        seconds += 1;
        nanoseconds -= 1_000_000_000;
    }
    if nanoseconds < 0 {
        seconds -= 1;
        nanoseconds += 1_000_000_000;
    }
    Ok(UtcInstant {
        unix_seconds: seconds,
        nanoseconds: nanoseconds as u32,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unix_epoch_roundtrip() {
        let u = UtcInstant {
            unix_seconds: 0,
            nanoseconds: 0,
        };
        assert_eq!(utc_to_jd(u), JD_UNIX_EPOCH);
        let r = jd_to_utc(JD_UNIX_EPOCH).unwrap();
        assert_eq!(r.unix_seconds, 0);
        assert_eq!(r.nanoseconds, 0);
    }

    #[test]
    fn fractional_second_roundtrip_is_close() {
        let u = UtcInstant {
            unix_seconds: 1_773_914_400,
            nanoseconds: 123_000_000,
        };
        let r = jd_to_utc(utc_to_jd(u)).unwrap();
        assert_eq!(r.unix_seconds, u.unix_seconds);
        assert!((i64::from(r.nanoseconds) - i64::from(u.nanoseconds)).abs() < 50_000);
    }
}
