# Versioned Americas-Mainland Geospatial Provider — v0.8.0

M-Time now has an executable geospatial provider for the represented Diyanet condition requiring qualifying visibility on the North or South American mainland.

## Dataset identity

```text
Natural Earth: ne_110m_land
version: 5.1.2
release tag: v5.1.2
Git blob SHA-1:
04811d72fff2701ec67587e30ad8942675b511e3

SHA-256:
9e0729ee253ca7d7a5c4ae9395fb1902264c5377c52e224d13dd85010e2835d9
```

The workflow downloads the version-pinned GeoJSON, verifies the SHA-256, and creates an M-Time source-ingestion record before classification.

## Geometry semantics

The provider does not classify "America" using a rectangular bounding box.

It:

1. parses Polygon/MultiPolygon GeoJSON;
2. identifies the connected land polygon containing an anchor at 100°W, 40°N inside continental North America;
3. applies point-in-polygon classification with interior-hole exclusion;
4. maps registered site IDs to explicit longitude/latitude coordinates.

The connected polygon includes the North/Central/South American mainland and excludes disconnected islands.

## Live classification gate

Actions run **37194423011**:

```text
NEW_YORK       true
SANTIAGO       true
PANAMA         true
MEXICO_CITY    true
ANCHORAGE      true

HAVANA         false
HONOLULU       false
GREENLAND      false
WELLINGTON     false
JAKARTA        false
USHUAIA        false
```

Ushuaia is intentionally false because the test coordinate is on Tierra del Fuego rather than the connected South American mainland.

## Diyanet integration gate

With the same real dataset:

```text
NEW_YORK:
5°/8° threshold = pass
Americas mainland = true
conjunction before Wellington fajr = true
complete represented rule = true

HAVANA:
5°/8° threshold = pass
Americas mainland = false
complete represented rule = false

WELLINGTON:
5°/8° threshold = pass
Americas mainland = false
complete represented rule = false

UNKNOWN_SITE:
geospatial evidence = unknown
complete represented rule = unknown
```

## Resolution boundary

Natural Earth 110m is a small-scale global cartographic dataset. It is suitable for mainland-vs-island policy classification at ordinary city/site scale, but it is not a high-resolution cadastral/coastline product.

Near-coast, tiny-isthmus, offshore-platform, and sub-kilometer boundary cases require a higher-resolution provider and explicit validation.
