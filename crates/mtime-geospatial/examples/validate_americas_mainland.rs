use mtime_geospatial::{AmericasMainlandGeoProvider, RegisteredSite};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: validate_americas_mainland <ne_110m_land.geojson>");
        std::process::exit(2);
    }

    let payload = std::fs::read(&args[1]).expect("read Natural Earth GeoJSON");
    let sites = vec![
        RegisteredSite::new("NEW_YORK", -74.0060, 40.7128).unwrap(),
        RegisteredSite::new("SANTIAGO", -70.6693, -33.4489).unwrap(),
        RegisteredSite::new("PANAMA", -79.5167, 8.9833).unwrap(),
        RegisteredSite::new("MEXICO_CITY", -99.1332, 19.4326).unwrap(),
        RegisteredSite::new("ANCHORAGE", -149.9003, 61.2181).unwrap(),
        RegisteredSite::new("HAVANA", -82.3666, 23.1136).unwrap(),
        RegisteredSite::new("HONOLULU", -157.8583, 21.3069).unwrap(),
        RegisteredSite::new("GREENLAND", -42.6043, 71.7069).unwrap(),
        RegisteredSite::new("WELLINGTON", 174.7772, -41.2889).unwrap(),
        RegisteredSite::new("JAKARTA", 106.8272, -6.1754).unwrap(),
        RegisteredSite::new("USHUAIA", -68.3030, -54.8019).unwrap(),
    ];
    let provider =
        AmericasMainlandGeoProvider::from_natural_earth_110m_land(&payload, sites).unwrap();

    let expected = [
        ("NEW_YORK", true),
        ("SANTIAGO", true),
        ("PANAMA", true),
        ("MEXICO_CITY", true),
        ("ANCHORAGE", true),
        ("HAVANA", false),
        ("HONOLULU", false),
        ("GREENLAND", false),
        ("WELLINGTON", false),
        ("JAKARTA", false),
        ("USHUAIA", false),
    ];

    println!("dataset_name={}", provider.dataset.name);
    println!("dataset_version={}", provider.dataset.version);
    println!("source_sha256={}", provider.dataset.source_sha256_hex);
    println!("source_git_blob_sha1={}", provider.dataset.source_git_blob_sha1);

    for (site_id, want) in expected {
        let got = provider.classify_site_id(site_id).expect("registered site");
        println!("site={site_id} americas_mainland={got}");
        assert_eq!(got, want, "classification mismatch for {site_id}");
    }
    assert_eq!(provider.classify_site_id("UNREGISTERED"), None);
}
