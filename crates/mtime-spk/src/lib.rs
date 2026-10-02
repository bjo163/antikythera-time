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
}
