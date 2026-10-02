use mtime_core::{EarthObserver,TemporalError};

#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum Body{Sun,Moon}
#[derive(Debug,Clone,Copy,PartialEq)]pub struct EquatorialState{pub right_ascension_deg:f64,pub declination_deg:f64,pub distance_au:f64}
#[derive(Debug,Clone,Copy,PartialEq)]pub struct EclipticState{pub longitude_deg:f64,pub latitude_deg:f64,pub distance_au:f64}
pub trait EphemerisProvider{fn id(&self)->&'static str;fn equatorial(&self,body:Body,jd_tt:f64)->Result<EquatorialState,TemporalError>;fn ecliptic(&self,body:Body,jd_tt:f64)->Result<EclipticState,TemporalError>;}
#[derive(Debug,Clone,Copy,PartialEq)]pub struct ApparentHorizonState{pub altitude_deg:f64,pub azimuth_deg:f64,pub refraction_applied:bool}
#[must_use]pub fn wrap_degrees(x:f64)->f64{x.rem_euclid(360.0)}
#[must_use]pub fn angular_separation_deg(a:EquatorialState,b:EquatorialState)->f64{let ra1=a.right_ascension_deg.to_radians();let ra2=b.right_ascension_deg.to_radians();let d1=a.declination_deg.to_radians();let d2=b.declination_deg.to_radians();(d1.sin()*d2.sin()+d1.cos()*d2.cos()*(ra1-ra2).cos()).clamp(-1.0,1.0).acos().to_degrees()}
#[must_use]pub fn gmst_deg(jd_ut1:f64)->f64{let t=(jd_ut1-2_451_545.0)/36_525.0;wrap_degrees(280.460_618_37+360.985_647_366_29*(jd_ut1-2_451_545.0)+0.000_387_933*t*t-t*t*t/38_710_000.0)}
pub fn topocentric_horizon(state:EquatorialState,observer:EarthObserver,jd_ut1:f64)->Result<ApparentHorizonState,TemporalError>{if !jd_ut1.is_finite(){return Err(TemporalError::NonFinite);}let lst=wrap_degrees(gmst_deg(jd_ut1)+observer.longitude_deg);let h=wrap_degrees(lst-state.right_ascension_deg);let hs=if h>180.0{h-360.0}else{h};let lat=observer.latitude_deg.to_radians();let dec=state.declination_deg.to_radians();let hr=hs.to_radians();let alt=(lat.sin()*dec.sin()+lat.cos()*dec.cos()*hr.cos()).clamp(-1.0,1.0).asin();let y=-hr.sin()*dec.cos();let x=dec.sin()*lat.cos()-dec.cos()*lat.sin()*hr.cos();Ok(ApparentHorizonState{altitude_deg:alt.to_degrees(),azimuth_deg:wrap_degrees(y.atan2(x).to_degrees()),refraction_applied:false})}
pub fn geocentric_elongation_deg<P:EphemerisProvider>(provider:&P,jd_tt:f64)->Result<f64,TemporalError>{Ok(angular_separation_deg(provider.equatorial(Body::Sun,jd_tt)?,provider.equatorial(Body::Moon,jd_tt)?))}
#[derive(Debug,Clone,Copy)]pub struct FixedEphemeris{pub sun_eq:EquatorialState,pub moon_eq:EquatorialState,pub sun_ecl:EclipticState,pub moon_ecl:EclipticState}
impl EphemerisProvider for FixedEphemeris{fn id(&self)->&'static str{"FIXED_TEST_EPHEMERIS"}fn equatorial(&self,body:Body,_:f64)->Result<EquatorialState,TemporalError>{Ok(match body{Body::Sun=>self.sun_eq,Body::Moon=>self.moon_eq})}fn ecliptic(&self,body:Body,_:f64)->Result<EclipticState,TemporalError>{Ok(match body{Body::Sun=>self.sun_ecl,Body::Moon=>self.moon_ecl})}}
#[cfg(test)]mod tests{use super::*;#[test]fn identical_states_have_zero_separation(){let a=EquatorialState{right_ascension_deg:10.0,declination_deg:-5.0,distance_au:1.0};assert!(angular_separation_deg(a,a)<1e-7);}#[test]fn horizon_is_finite(){let s=EquatorialState{right_ascension_deg:120.0,declination_deg:5.0,distance_au:1.0};let o=EarthObserver::new(106.8,-6.3,50.0).unwrap();let h=topocentric_horizon(s,o,2_460_000.5).unwrap();assert!(h.altitude_deg.is_finite()&&h.azimuth_deg.is_finite());}}


pub const WGS84_A_KM: f64 = 6378.137;
pub const WGS84_F: f64 = 1.0 / 298.257_223_563;
pub const WGS84_E2: f64 = WGS84_F * (2.0 - WGS84_F);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Atmosphere {
    pub pressure_hpa: f64,
    pub temperature_c: f64,
}

impl Atmosphere {
    #[must_use]
    pub const fn standard() -> Self {
        Self {
            pressure_hpa: 1010.0,
            temperature_c: 10.0,
        }
    }
}

#[must_use]
pub fn observer_ecef_km(observer: EarthObserver) -> [f64; 3] {
    let lon = observer.longitude_deg.to_radians();
    let lat = observer.latitude_deg.to_radians();
    let h = observer.height_m / 1000.0;
    let sin_lat = lat.sin();
    let cos_lat = lat.cos();
    let n = WGS84_A_KM / (1.0 - WGS84_E2 * sin_lat * sin_lat).sqrt();
    [
        (n + h) * cos_lat * lon.cos(),
        (n + h) * cos_lat * lon.sin(),
        (n * (1.0 - WGS84_E2) + h) * sin_lat,
    ]
}

#[must_use]
pub fn ecef_to_inertial_km(ecef: [f64; 3], jd_ut1: f64) -> [f64; 3] {
    let theta = gmst_deg(jd_ut1).to_radians();
    let c = theta.cos();
    let s = theta.sin();
    [
        c * ecef[0] - s * ecef[1],
        s * ecef[0] + c * ecef[1],
        ecef[2],
    ]
}

#[must_use]
pub fn topocentric_vector_km(
    geocentric_object_km: [f64; 3],
    observer: EarthObserver,
    jd_ut1: f64,
) -> [f64; 3] {
    let o = ecef_to_inertial_km(observer_ecef_km(observer), jd_ut1);
    [
        geocentric_object_km[0] - o[0],
        geocentric_object_km[1] - o[1],
        geocentric_object_km[2] - o[2],
    ]
}

pub fn topocentric_geometric_horizon_from_vector(
    geocentric_object_km: [f64; 3],
    observer: EarthObserver,
    jd_ut1: f64,
) -> Result<ApparentHorizonState, TemporalError> {
    if !jd_ut1.is_finite() || !geocentric_object_km.iter().all(|x| x.is_finite()) {
        return Err(TemporalError::NonFinite);
    }
    let v = topocentric_vector_km(geocentric_object_km, observer, jd_ut1);
    let theta = gmst_deg(jd_ut1).to_radians();
    let c = theta.cos();
    let s = theta.sin();
    // inertial -> Earth-fixed
    let x = c * v[0] + s * v[1];
    let y = -s * v[0] + c * v[1];
    let z = v[2];

    let lon = observer.longitude_deg.to_radians();
    let lat = observer.latitude_deg.to_radians();
    let east = -lon.sin() * x + lon.cos() * y;
    let north =
        -lat.sin() * lon.cos() * x - lat.sin() * lon.sin() * y + lat.cos() * z;
    let up =
        lat.cos() * lon.cos() * x + lat.cos() * lon.sin() * y + lat.sin() * z;
    let r = (east * east + north * north + up * up).sqrt();
    if !r.is_finite() || r == 0.0 {
        return Err(TemporalError::NonFinite);
    }
    let altitude = (up / r).clamp(-1.0, 1.0).asin().to_degrees();
    let azimuth = wrap_degrees(east.atan2(north).to_degrees());
    Ok(ApparentHorizonState {
        altitude_deg: altitude,
        azimuth_deg: azimuth,
        refraction_applied: false,
    })
}

/// Saemundsson/Bennett-class true-to-apparent refraction approximation.
/// This is an explicit optional atmospheric model, not part of the geometric
/// MABIMS altitude unless a profile/source explicitly requests refraction.
pub fn apply_refraction(
    geometric: ApparentHorizonState,
    atmosphere: Atmosphere,
) -> Result<ApparentHorizonState, TemporalError> {
    let h = geometric.altitude_deg;
    if !h.is_finite()
        || !atmosphere.pressure_hpa.is_finite()
        || !atmosphere.temperature_c.is_finite()
        || atmosphere.pressure_hpa <= 0.0
        || atmosphere.temperature_c <= -273.0
    {
        return Err(TemporalError::NonFinite);
    }
    if !(-1.0..=90.0).contains(&h) {
        return Err(TemporalError::InvalidInput(
            "refraction approximation supported for true altitude -1..=90 deg",
        ));
    }
    let angle_deg = h + 10.3 / (h + 5.11);
    let standard_arcmin = 1.02 / angle_deg.to_radians().tan();
    let scale =
        (atmosphere.pressure_hpa / 1010.0) * (283.0 / (273.0 + atmosphere.temperature_c));
    Ok(ApparentHorizonState {
        altitude_deg: h + standard_arcmin * scale / 60.0,
        azimuth_deg: geometric.azimuth_deg,
        refraction_applied: true,
    })
}

#[cfg(test)]
mod topocentric_tests {
    use super::*;

    #[test]
    fn wgs84_equator_radius_matches_semimajor_axis() {
        let o = EarthObserver::new(0.0, 0.0, 0.0).unwrap();
        let ecef = observer_ecef_km(o);
        assert!((ecef[0] - WGS84_A_KM).abs() < 1e-12);
        assert!(ecef[1].abs() < 1e-12 && ecef[2].abs() < 1e-12);
    }

    #[test]
    fn lunar_parallax_pushes_geocentric_horizon_below_local_horizon() {
        let o = EarthObserver::new(0.0, 0.0, 0.0).unwrap();
        let jd = 2_451_545.0;
        let theta = gmst_deg(jd).to_radians();
        // Construct a geocentric Moon direction exactly on the observer's
        // local east horizon in the inertial frame.
        let east_ecef = [0.0, 384_400.0, 0.0];
        let moon = [
            theta.cos() * east_ecef[0] - theta.sin() * east_ecef[1],
            theta.sin() * east_ecef[0] + theta.cos() * east_ecef[1],
            0.0,
        ];
        let h = topocentric_geometric_horizon_from_vector(moon, o, jd).unwrap();
        assert!(h.altitude_deg < -0.9 && h.altitude_deg > -1.1);
    }

    #[test]
    fn standard_refraction_at_true_horizon_is_about_half_degree() {
        let g = ApparentHorizonState {
            altitude_deg: 0.0,
            azimuth_deg: 270.0,
            refraction_applied: false,
        };
        let a = apply_refraction(g, Atmosphere::standard()).unwrap();
        assert!(a.altitude_deg > 0.45 && a.altitude_deg < 0.55);
        assert!(a.refraction_applied);
    }
}


const ARCSEC_TO_RAD: f64 = core::f64::consts::PI / (180.0 * 3600.0);

/// High-accuracy geometric topocentric horizon transform using the
/// IAU 2006/2000A celestial-to-terrestrial matrix (SOFA algorithm family),
/// IERS polar motion, and WGS84 observer coordinates.
///
/// Input object vector is geocentric ICRF/GCRS-oriented km.
/// TT and UT1 are supplied as two-part Julian Dates.
pub fn topocentric_horizon_iau2006(
    geocentric_object_icrf_km: [f64; 3],
    observer: EarthObserver,
    tt: (f64, f64),
    ut1: (f64, f64),
    xp_arcsec: f64,
    yp_arcsec: f64,
) -> Result<ApparentHorizonState, TemporalError> {
    if !geocentric_object_icrf_km.iter().all(|x| x.is_finite())
        || !tt.0.is_finite()
        || !tt.1.is_finite()
        || !ut1.0.is_finite()
        || !ut1.1.is_finite()
        || !xp_arcsec.is_finite()
        || !yp_arcsec.is_finite()
    {
        return Err(TemporalError::NonFinite);
    }

    let rc2t = sofars::pnp::c2t06a(
        tt.0,
        tt.1,
        ut1.0,
        ut1.1,
        xp_arcsec * ARCSEC_TO_RAD,
        yp_arcsec * ARCSEC_TO_RAD,
    );
    let object_itrs = mat_vec(rc2t, geocentric_object_icrf_km);
    let observer_itrs = observer_ecef_km(observer);
    let topo = [
        object_itrs[0] - observer_itrs[0],
        object_itrs[1] - observer_itrs[1],
        object_itrs[2] - observer_itrs[2],
    ];
    local_enu_horizon(topo, observer)
}

fn mat_vec(m: [[f64; 3]; 3], v: [f64; 3]) -> [f64; 3] {
    [
        m[0][0] * v[0] + m[0][1] * v[1] + m[0][2] * v[2],
        m[1][0] * v[0] + m[1][1] * v[1] + m[1][2] * v[2],
        m[2][0] * v[0] + m[2][1] * v[1] + m[2][2] * v[2],
    ]
}

fn local_enu_horizon(
    topo_itrs_km: [f64; 3],
    observer: EarthObserver,
) -> Result<ApparentHorizonState, TemporalError> {
    let [x, y, z] = topo_itrs_km;
    let lon = observer.longitude_deg.to_radians();
    let lat = observer.latitude_deg.to_radians();

    let east = -lon.sin() * x + lon.cos() * y;
    let north =
        -lat.sin() * lon.cos() * x - lat.sin() * lon.sin() * y + lat.cos() * z;
    let up =
        lat.cos() * lon.cos() * x + lat.cos() * lon.sin() * y + lat.sin() * z;

    let r = (east * east + north * north + up * up).sqrt();
    if !r.is_finite() || r == 0.0 {
        return Err(TemporalError::NonFinite);
    }
    Ok(ApparentHorizonState {
        altitude_deg: (up / r).clamp(-1.0, 1.0).asin().to_degrees(),
        azimuth_deg: wrap_degrees(east.atan2(north).to_degrees()),
        refraction_applied: false,
    })
}

#[cfg(test)]
mod iau_topocentric_tests {
    use super::*;

    #[test]
    fn iau_transform_returns_finite_horizon() {
        let observer = EarthObserver::new(106.8272, -6.1754, 8.0).unwrap();
        let h = topocentric_horizon_iau2006(
            [380_000.0, 40_000.0, 10_000.0],
            observer,
            (2_461_118.5, 0.417467407),
            (2_461_118.5, 0.416667333),
            0.1,
            0.3,
        )
        .unwrap();
        assert!(h.altitude_deg.is_finite() && h.azimuth_deg.is_finite());
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HorizonPoint {
    pub azimuth_deg: f64,
    pub obstruction_altitude_deg: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalHorizonProfile {
    points: Vec<HorizonPoint>,
    pub provenance: String,
}

impl LocalHorizonProfile {
    pub fn new(
        mut points: Vec<HorizonPoint>,
        provenance: impl Into<String>,
    ) -> Result<Self, TemporalError> {
        if points.len() < 2
            || points.iter().any(|p| {
                !p.azimuth_deg.is_finite()
                    || !p.obstruction_altitude_deg.is_finite()
                    || !(0.0..360.0).contains(&p.azimuth_deg)
                    || !(-10.0..=90.0).contains(&p.obstruction_altitude_deg)
            })
        {
            return Err(TemporalError::InvalidInput(
                "local horizon requires >=2 finite azimuth points in [0,360) and obstruction altitude in [-10,90]",
            ));
        }
        points.sort_by(|a, b| {
            a.azimuth_deg
                .partial_cmp(&b.azimuth_deg)
                .unwrap_or(core::cmp::Ordering::Equal)
        });
        if points
            .windows(2)
            .any(|w| (w[0].azimuth_deg - w[1].azimuth_deg).abs() < 1e-12)
        {
            return Err(TemporalError::InvalidInput(
                "local horizon azimuth points must be unique",
            ));
        }
        Ok(Self {
            points,
            provenance: provenance.into(),
        })
    }

    #[must_use]
    pub fn obstruction_altitude_deg(&self, azimuth_deg: f64) -> f64 {
        let az = wrap_degrees(azimuth_deg);
        for pair in self.points.windows(2) {
            if az >= pair[0].azimuth_deg && az <= pair[1].azimuth_deg {
                return linear_horizon(pair[0], pair[1], az);
            }
        }

        // Circular interpolation across north (360 -> 0).
        let last = *self.points.last().expect("validated non-empty horizon");
        let first = self.points[0];
        let first_wrapped = HorizonPoint {
            azimuth_deg: first.azimuth_deg + 360.0,
            obstruction_altitude_deg: first.obstruction_altitude_deg,
        };
        let az_wrapped = if az < first.azimuth_deg { az + 360.0 } else { az };
        linear_horizon(last, first_wrapped, az_wrapped)
    }
}

fn linear_horizon(a: HorizonPoint, b: HorizonPoint, azimuth_deg: f64) -> f64 {
    let width = b.azimuth_deg - a.azimuth_deg;
    if width.abs() < 1e-15 {
        return a.obstruction_altitude_deg;
    }
    let t = (azimuth_deg - a.azimuth_deg) / width;
    a.obstruction_altitude_deg
        + t * (b.obstruction_altitude_deg - a.obstruction_altitude_deg)
}

/// Geometric depression of the ideal sea horizon caused by observer height.
/// Returns a positive angle in degrees; the ideal horizon is lower by this amount.
pub fn geometric_horizon_dip_deg(height_m: f64) -> Result<f64, TemporalError> {
    if !height_m.is_finite() || height_m < 0.0 {
        return Err(TemporalError::InvalidInput(
            "observer height for horizon dip must be finite and non-negative",
        ));
    }
    let h_km = height_m / 1000.0;
    Ok((WGS84_A_KM / (WGS84_A_KM + h_km))
        .clamp(-1.0, 1.0)
        .acos()
        .to_degrees())
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LocalHorizonClearance {
    pub apparent_altitude_deg: f64,
    pub obstruction_altitude_deg: f64,
    pub clearance_deg: f64,
}

/// Compare an already-selected geometric/apparent altitude with a surveyed
/// local-horizon profile. No calendar rule is applied here.
pub fn local_horizon_clearance(
    horizon: ApparentHorizonState,
    profile: &LocalHorizonProfile,
) -> LocalHorizonClearance {
    let obstruction = profile.obstruction_altitude_deg(horizon.azimuth_deg);
    LocalHorizonClearance {
        apparent_altitude_deg: horizon.altitude_deg,
        obstruction_altitude_deg: obstruction,
        clearance_deg: horizon.altitude_deg - obstruction,
    }
}

#[cfg(test)]
mod local_horizon_tests {
    use super::*;

    #[test]
    fn horizon_profile_interpolates_across_north_wrap() {
        let h = LocalHorizonProfile::new(
            vec![
                HorizonPoint { azimuth_deg: 350.0, obstruction_altitude_deg: 2.0 },
                HorizonPoint { azimuth_deg: 10.0, obstruction_altitude_deg: 4.0 },
                HorizonPoint { azimuth_deg: 180.0, obstruction_altitude_deg: 1.0 },
            ],
            "survey fixture",
        )
        .unwrap();
        assert!((h.obstruction_altitude_deg(0.0) - 3.0).abs() < 1e-12);
    }

    #[test]
    fn duplicate_horizon_azimuth_is_rejected() {
        assert!(LocalHorizonProfile::new(
            vec![
                HorizonPoint { azimuth_deg: 10.0, obstruction_altitude_deg: 1.0 },
                HorizonPoint { azimuth_deg: 10.0, obstruction_altitude_deg: 2.0 },
            ],
            "bad fixture",
        )
        .is_err());
    }

    #[test]
    fn horizon_dip_is_zero_at_sea_level_and_positive_above_it() {
        assert!(geometric_horizon_dip_deg(0.0).unwrap().abs() < 1e-12);
        assert!(geometric_horizon_dip_deg(1000.0).unwrap() > 0.9);
    }

    #[test]
    fn clearance_preserves_obstruction_as_separate_input() {
        let p = LocalHorizonProfile::new(
            vec![
                HorizonPoint { azimuth_deg: 0.0, obstruction_altitude_deg: 2.0 },
                HorizonPoint { azimuth_deg: 180.0, obstruction_altitude_deg: 2.0 },
            ],
            "survey fixture",
        )
        .unwrap();
        let c = local_horizon_clearance(
            ApparentHorizonState {
                altitude_deg: 3.5,
                azimuth_deg: 90.0,
                refraction_applied: true,
            },
            &p,
        );
        assert!((c.clearance_deg - 1.5).abs() < 1e-12);
    }
}
