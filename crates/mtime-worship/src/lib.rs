use mtime_core::{EvidenceState, Provenance, QualityClass};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolarEventKind {
    FajrThreshold,
    Sunrise,
    SolarNoon,
    Sunset,
    IshaThreshold,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SolarThresholdProfile {
    pub id: &'static str,
    pub version: &'static str,
    pub event: SolarEventKind,
    pub sun_altitude_deg: f64,
    pub evidence: EvidenceState,
    pub quality: QualityClass,
    pub provenance: Provenance,
}

impl SolarThresholdProfile {
    #[must_use]
    pub fn indonesia_kemenag_fajr_20() -> Self {
        Self {
            id: "ID_KEMENAG_FAJR_MINUS_20",
            version: "observational-policy-current-2026",
            event: SolarEventKind::FajrThreshold,
            sun_altitude_deg: -20.0,
            evidence: EvidenceState::Modeled,
            quality: QualityClass::Reference,
            provenance: Provenance {
                source: "Kementerian Agama RI — kriteria Subuh Matahari sekitar -20 derajat".into(),
                source_version: Some("reviewed through 2026 public statements".into()),
                retrieved_at: None,
            },
        }
    }
    #[must_use]
    pub fn diyanet_imsak_fajr_18_2026() -> Self {
        Self {
            id: "DIYANET_IMSAK_FAJR_MINUS_18",
            version: "Diyanet-current-methodology-2026-10-04",
            event: SolarEventKind::FajrThreshold,
            sun_altitude_deg: -18.0,
            evidence: EvidenceState::Modeled,
            quality: QualityClass::Reference,
            provenance: Provenance {
                source: "T.C. Diyanet İşleri Başkanlığı — imsak: astronomical dawn at Sun altitude -18°"
                    .into(),
                source_version: Some(
                    "https://kurul.diyanet.gov.tr/tr/video/imsak-nedir-ne-zaman-baslar/019d0027-3361-75de-b072-bc87fbe71859"
                        .into(),
                ),
                retrieved_at: Some("2026-10-04".into()),
            },
        }
    }

}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SolarEvent {
    pub jd_ut1: f64,
    pub altitude_deg: f64,
    pub kind: SolarEventKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RootDirection {
    Rising,
    Setting,
    Either,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolarEventError {
    NonFinite,
    NotBracketed,
    WrongDirection,
    IterationLimit,
}

/// Find a solar-altitude crossing without prescribing an ephemeris.
/// The caller supplies an altitude function backed by an authoritative or
/// explicitly-labelled astronomical provider. The two UT1 endpoints must
/// bracket a crossing.
pub fn solve_altitude_crossing<F>(
    altitude_deg: F,
    a_jd_ut1: f64,
    b_jd_ut1: f64,
    target_altitude_deg: f64,
    direction: RootDirection,
    tolerance_seconds: f64,
    kind: SolarEventKind,
) -> Result<SolarEvent, SolarEventError>
where
    F: Fn(f64) -> f64,
{
    if ![
        a_jd_ut1,
        b_jd_ut1,
        target_altitude_deg,
        tolerance_seconds,
    ]
    .iter()
    .all(|x| x.is_finite())
        || a_jd_ut1 >= b_jd_ut1
        || tolerance_seconds <= 0.0
    {
        return Err(SolarEventError::NonFinite);
    }

    let mut lo = a_jd_ut1;
    let mut hi = b_jd_ut1;
    let mut flo = altitude_deg(lo) - target_altitude_deg;
    let fhi = altitude_deg(hi) - target_altitude_deg;
    if !flo.is_finite() || !fhi.is_finite() {
        return Err(SolarEventError::NonFinite);
    }
    if flo == 0.0 {
        return Ok(SolarEvent {
            jd_ut1: lo,
            altitude_deg: target_altitude_deg,
            kind,
        });
    }
    if flo.signum() == fhi.signum() {
        return Err(SolarEventError::NotBracketed);
    }

    let rising = fhi > flo;
    if (direction == RootDirection::Rising && !rising)
        || (direction == RootDirection::Setting && rising)
    {
        return Err(SolarEventError::WrongDirection);
    }

    let tolerance_days = tolerance_seconds / 86_400.0;
    for _ in 0..100 {
        let mid = (lo + hi) / 2.0;
        let fm = altitude_deg(mid) - target_altitude_deg;
        if !fm.is_finite() {
            return Err(SolarEventError::NonFinite);
        }
        if (hi - lo).abs() <= tolerance_days || fm.abs() <= 1e-12 {
            return Ok(SolarEvent {
                jd_ut1: mid,
                altitude_deg: target_altitude_deg,
                kind,
            });
        }
        if fm.signum() == flo.signum() {
            lo = mid;
            flo = fm;
        } else {
            hi = mid;
        }
    }
    Err(SolarEventError::IterationLimit)
}


#[must_use]
pub fn conjunction_before_fajr_ut1(
    conjunction_jd_ut1: f64,
    fajr: SolarEvent,
) -> Result<bool, SolarEventError> {
    if !conjunction_jd_ut1.is_finite() || !fajr.jd_ut1.is_finite() {
        return Err(SolarEventError::NonFinite);
    }
    if fajr.kind != SolarEventKind::FajrThreshold {
        return Err(SolarEventError::WrongDirection);
    }
    Ok(conjunction_jd_ut1 < fajr.jd_ut1)
}

#[derive(Debug, Clone, PartialEq)]
pub struct FastingWindow {
    pub profile_id: String,
    pub start_fajr: SolarEvent,
    pub end_sunset: SolarEvent,
    pub provenance: Provenance,
}

impl FastingWindow {
    pub fn new(
        profile: &SolarThresholdProfile,
        start_fajr: SolarEvent,
        end_sunset: SolarEvent,
    ) -> Result<Self, SolarEventError> {
        if start_fajr.kind != SolarEventKind::FajrThreshold
            || end_sunset.kind != SolarEventKind::Sunset
            || start_fajr.jd_ut1 >= end_sunset.jd_ut1
        {
            return Err(SolarEventError::WrongDirection);
        }
        Ok(Self {
            profile_id: profile.id.into(),
            start_fajr,
            end_sunset,
            provenance: profile.provenance.clone(),
        })
    }

    #[must_use]
    pub fn duration_hours(&self) -> f64 {
        (self.end_sunset.jd_ut1 - self.start_fajr.jd_ut1) * 24.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_is_versioned_not_universalized() {
        let p = SolarThresholdProfile::indonesia_kemenag_fajr_20();
        assert_eq!(p.sun_altitude_deg, -20.0);
        assert_eq!(p.id, "ID_KEMENAG_FAJR_MINUS_20");

        let d = SolarThresholdProfile::diyanet_imsak_fajr_18_2026();
        assert_eq!(d.sun_altitude_deg, -18.0);
        assert_eq!(d.id, "DIYANET_IMSAK_FAJR_MINUS_18");
    }

    #[test]
    fn bisection_finds_rising_fajr_threshold() {
        let base = 2_460_000.0;
        let f = |jd: f64| -25.0 + 10.0 * (jd - base) / 0.1;
        let e = solve_altitude_crossing(
            f,
            base,
            base + 0.1,
            -20.0,
            RootDirection::Rising,
            0.01,
            SolarEventKind::FajrThreshold,
        )
        .unwrap();
        assert!((e.jd_ut1 - (base + 0.05)).abs() < 1e-7);
    }

    #[test]
    fn conjunction_must_be_strictly_before_fajr() {
        let fajr = SolarEvent {
            jd_ut1: 2_460_000.25,
            altitude_deg: -18.0,
            kind: SolarEventKind::FajrThreshold,
        };
        assert_eq!(conjunction_before_fajr_ut1(2_460_000.20, fajr), Ok(true));
        assert_eq!(conjunction_before_fajr_ut1(2_460_000.25, fajr), Ok(false));
        assert_eq!(conjunction_before_fajr_ut1(2_460_000.30, fajr), Ok(false));
    }

    #[test]
    fn conjunction_comparison_rejects_non_fajr_event() {
        let sunset = SolarEvent {
            jd_ut1: 2_460_000.25,
            altitude_deg: 0.0,
            kind: SolarEventKind::Sunset,
        };
        assert_eq!(
            conjunction_before_fajr_ut1(2_460_000.20, sunset),
            Err(SolarEventError::WrongDirection)
        );
    }

    #[test]
    fn fasting_window_requires_fajr_before_sunset() {
        let p = SolarThresholdProfile::indonesia_kemenag_fajr_20();
        let fajr = SolarEvent {
            jd_ut1: 2_460_000.7,
            altitude_deg: -20.0,
            kind: SolarEventKind::FajrThreshold,
        };
        let sunset = SolarEvent {
            jd_ut1: 2_460_001.0,
            altitude_deg: 0.0,
            kind: SolarEventKind::Sunset,
        };
        let w = FastingWindow::new(&p, fajr, sunset).unwrap();
        assert!((w.duration_hours() - 7.2).abs() < 1e-8);
    }
}
