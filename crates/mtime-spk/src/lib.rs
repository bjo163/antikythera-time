use mtime_astro::{Body, EclipticState, EphemerisProvider, EquatorialState};
use mtime_core::TemporalError;
use oxiephemeris_de::spk::SpkFile;

pub const AU_KM: f64 = 149_597_870.7;
pub const NAIF_SSB: i32 = 0;
pub const NAIF_EMB: i32 = 3;
pub const NAIF_SUN: i32 = 10;
pub const NAIF_MOON: i32 = 301;
pub const NAIF_EARTH: i32 = 399;
const J2000_OBLIQUITY_DEG: f64 = 23.439_291_111;

#[derive(Debug)]
pub struct SpkEphemeris<'a> {
    spk: SpkFile<'a>,
}

impl<'a> SpkEphemeris<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Result<Self, TemporalError> {
        let spk = SpkFile::parse(bytes).map_err(|_| TemporalError::MissingReferenceData)?;
        Ok(Self { spk })
    }

    pub fn geocentric_vector_km(
        &self,
        body: Body,
        jd_tdb: (f64, f64),
    ) -> Result<[f64; 3], TemporalError> {
        match body {
            Body::Moon => {
                let moon_emb = self
                    .spk
                    .state(NAIF_MOON, NAIF_EMB, jd_tdb)
                    .map_err(|_| TemporalError::MissingReferenceData)?;
                let earth_emb = self
                    .spk
                    .state(NAIF_EARTH, NAIF_EMB, jd_tdb)
                    .map_err(|_| TemporalError::MissingReferenceData)?;
                Ok(sub(moon_emb.pos_km, earth_emb.pos_km))
            }
            Body::Sun => {
                let sun_ssb = self
                    .spk
                    .state(NAIF_SUN, NAIF_SSB, jd_tdb)
                    .map_err(|_| TemporalError::MissingReferenceData)?;
                let emb_ssb = self
                    .spk
                    .state(NAIF_EMB, NAIF_SSB, jd_tdb)
                    .map_err(|_| TemporalError::MissingReferenceData)?;
                let earth_emb = self
                    .spk
                    .state(NAIF_EARTH, NAIF_EMB, jd_tdb)
                    .map_err(|_| TemporalError::MissingReferenceData)?;
                let earth_ssb = add(emb_ssb.pos_km, earth_emb.pos_km);
                Ok(sub(sun_ssb.pos_km, earth_ssb))
            }
        }
    }

    /// Approximate apparent geocentric Sun vector for observer-style
    /// topocentric work. This keeps Earth at reception time, iterates solar
    /// light-time, and applies first-order annual aberration from Earth's
    /// barycentric velocity. It intentionally remains separate from the
    /// geometric Sun vector used by center-to-center elongation.
    pub fn apparent_geocentric_sun_vector_km(
        &self,
        jd_tdb: (f64, f64),
    ) -> Result<[f64; 3], TemporalError> {
        const C_KM_S: f64 = 299_792.458;
        const SECONDS_PER_DAY: f64 = 86_400.0;
        const VELOCITY_STEP_SECONDS: f64 = 60.0;

        let earth_now = self.earth_ssb_position_km(jd_tdb)?;
        let mut emit = jd_tdb;
        let mut natural = sub(self.sun_ssb_position_km(emit)?, earth_now);

        for _ in 0..3 {
            let light_seconds = norm(natural) / C_KM_S;
            emit = shift_jd(jd_tdb, -light_seconds / SECONDS_PER_DAY);
            natural = sub(self.sun_ssb_position_km(emit)?, earth_now);
        }

        let dt_days = VELOCITY_STEP_SECONDS / SECONDS_PER_DAY;
        let earth_minus = self.earth_ssb_position_km(shift_jd(jd_tdb, -dt_days))?;
        let earth_plus = self.earth_ssb_position_km(shift_jd(jd_tdb, dt_days))?;
        let earth_velocity_km_s = [
            (earth_plus[0] - earth_minus[0]) / (2.0 * VELOCITY_STEP_SECONDS),
            (earth_plus[1] - earth_minus[1]) / (2.0 * VELOCITY_STEP_SECONDS),
            (earth_plus[2] - earth_minus[2]) / (2.0 * VELOCITY_STEP_SECONDS),
        ];

        aberrate_vector_first_order(natural, earth_velocity_km_s, C_KM_S)
    }

    fn sun_ssb_position_km(&self, jd_tdb: (f64, f64)) -> Result<[f64; 3], TemporalError> {
        self.spk
            .state(NAIF_SUN, NAIF_SSB, jd_tdb)
            .map(|state| state.pos_km)
            .map_err(|_| TemporalError::MissingReferenceData)
    }

    fn earth_ssb_position_km(&self, jd_tdb: (f64, f64)) -> Result<[f64; 3], TemporalError> {
        let emb_ssb = self
            .spk
            .state(NAIF_EMB, NAIF_SSB, jd_tdb)
            .map_err(|_| TemporalError::MissingReferenceData)?;
        let earth_emb = self
            .spk
            .state(NAIF_EARTH, NAIF_EMB, jd_tdb)
            .map_err(|_| TemporalError::MissingReferenceData)?;
        Ok(add(emb_ssb.pos_km, earth_emb.pos_km))
    }

    pub fn geometric_elongation_deg(
        &self,
        jd_tdb: (f64, f64),
    ) -> Result<f64, TemporalError> {
        let sun = self.geocentric_vector_km(Body::Sun, jd_tdb)?;
        let moon = self.geocentric_vector_km(Body::Moon, jd_tdb)?;
        vector_angle_deg(sun, moon)
    }

    fn equatorial_from_tdb(
        &self,
        body: Body,
        jd_tdb: (f64, f64),
    ) -> Result<EquatorialState, TemporalError> {
        vector_to_equatorial(self.geocentric_vector_km(body, jd_tdb)?)
    }
}

impl EphemerisProvider for SpkEphemeris<'_> {
    fn id(&self) -> &'static str {
        "JPL_DE_SPK_GEOMETRIC_J2000"
    }

    fn equatorial(
        &self,
        body: Body,
        jd_tt: f64,
    ) -> Result<EquatorialState, TemporalError> {
        // This trait predates explicit TDB input. For precise work callers should use
        // geocentric_vector_km/equatorial_from_tdb with a TT->TDB transform.
        self.equatorial_from_tdb(body, (jd_tt, 0.0))
    }

    fn ecliptic(
        &self,
        body: Body,
        jd_tt: f64,
    ) -> Result<EclipticState, TemporalError> {
        let v = self.geocentric_vector_km(body, (jd_tt, 0.0))?;
        vector_to_j2000_ecliptic(v)
    }
}

#[must_use]
pub fn add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

#[must_use]
pub fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

pub fn vector_angle_deg(a: [f64; 3], b: [f64; 3]) -> Result<f64, TemporalError> {
    let na = norm(a);
    let nb = norm(b);
    if !na.is_finite() || !nb.is_finite() || na == 0.0 || nb == 0.0 {
        return Err(TemporalError::NonFinite);
    }
    let dot = a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    Ok((dot / (na * nb)).clamp(-1.0, 1.0).acos().to_degrees())
}

pub fn vector_to_equatorial(v: [f64; 3]) -> Result<EquatorialState, TemporalError> {
    let r = norm(v);
    if !r.is_finite() || r == 0.0 {
        return Err(TemporalError::NonFinite);
    }
    let ra = v[1].atan2(v[0]).to_degrees().rem_euclid(360.0);
    let dec = (v[2] / r).clamp(-1.0, 1.0).asin().to_degrees();
    Ok(EquatorialState {
        right_ascension_deg: ra,
        declination_deg: dec,
        distance_au: r / AU_KM,
    })
}

pub fn vector_to_j2000_ecliptic(v: [f64; 3]) -> Result<EclipticState, TemporalError> {
    let eps = J2000_OBLIQUITY_DEG.to_radians();
    let rotated = [
        v[0],
        v[1] * eps.cos() + v[2] * eps.sin(),
        -v[1] * eps.sin() + v[2] * eps.cos(),
    ];
    let r = norm(rotated);
    if !r.is_finite() || r == 0.0 {
        return Err(TemporalError::NonFinite);
    }
    Ok(EclipticState {
        longitude_deg: rotated[1].atan2(rotated[0]).to_degrees().rem_euclid(360.0),
        latitude_deg: (rotated[2] / r).clamp(-1.0, 1.0).asin().to_degrees(),
        distance_au: r / AU_KM,
    })
}

fn norm(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

fn shift_jd(jd: (f64, f64), delta_days: f64) -> (f64, f64) {
    (jd.0, jd.1 + delta_days)
}

pub fn aberrate_vector_first_order(
    natural_vector_km: [f64; 3],
    observer_velocity_km_s: [f64; 3],
    c_km_s: f64,
) -> Result<[f64; 3], TemporalError> {
    let r = norm(natural_vector_km);
    if !r.is_finite()
        || r == 0.0
        || !c_km_s.is_finite()
        || c_km_s <= 0.0
        || !observer_velocity_km_s.iter().all(|x| x.is_finite())
    {
        return Err(TemporalError::NonFinite);
    }

    let n = [
        natural_vector_km[0] / r,
        natural_vector_km[1] / r,
        natural_vector_km[2] / r,
    ];
    let beta = [
        observer_velocity_km_s[0] / c_km_s,
        observer_velocity_km_s[1] / c_km_s,
        observer_velocity_km_s[2] / c_km_s,
    ];
    let ndotb = n[0] * beta[0] + n[1] * beta[1] + n[2] * beta[2];
    let shifted = [
        n[0] + beta[0] - n[0] * ndotb,
        n[1] + beta[1] - n[1] * ndotb,
        n[2] + beta[2] - n[2] * ndotb,
    ];
    let s = norm(shifted);
    if !s.is_finite() || s == 0.0 {
        return Err(TemporalError::NonFinite);
    }
    Ok([
        shifted[0] / s * r,
        shifted[1] / s * r,
        shifted[2] / s * r,
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_angle_is_geometric_center_to_center() {
        assert!((vector_angle_deg([1.0, 0.0, 0.0], [0.0, 1.0, 0.0]).unwrap() - 90.0).abs() < 1e-12);
    }

    #[test]
    fn equatorial_vector_conversion() {
        let e = vector_to_equatorial([AU_KM, 0.0, 0.0]).unwrap();
        assert!(e.right_ascension_deg.abs() < 1e-12);
        assert!(e.declination_deg.abs() < 1e-12);
        assert!((e.distance_au - 1.0).abs() < 1e-12);
    }

    #[test]
    fn zero_vector_is_rejected() {
        assert!(vector_to_equatorial([0.0, 0.0, 0.0]).is_err());
    }

    #[test]
    fn first_order_aberration_shifts_toward_observer_velocity() {
        let natural = [0.0, AU_KM, 0.0];
        let apparent = aberrate_vector_first_order(
            natural,
            [29.78, 0.0, 0.0],
            299_792.458,
        )
        .unwrap();
        let shift_arcsec =
            vector_angle_deg(natural, apparent).unwrap() * 3600.0;
        assert!(apparent[0] > 0.0);
        assert!(shift_arcsec > 20.0 && shift_arcsec < 21.0);
    }
}
