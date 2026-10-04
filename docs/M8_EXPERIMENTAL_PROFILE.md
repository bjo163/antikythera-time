# M8 Experimental Digital Profile

M8 produced one strong out-of-sample candidate:

```text
basis: SIN_2_DRACONIC
coefficient: -0.113859414732°
training: 1900-1999 monthly
validation: 2000-2100 monthly
```

Validation result:

```text
P95  0.306383668° → 0.253203167°
MAX  0.417154165° → 0.307967979°
```

This candidate is exposed only through:

`MTIME_DIGITAL_ANTIKYTHERA_V2_EXPERIMENTAL`

The default remains:

`MTIME_DIGITAL_ANTIKYTHERA_V1`

The experimental profile changes the lunar path only. It is not considered production/default until M16 long-horizon falsification and independent M15 implementation reproduce the improvement.

The coefficient is explicit, versioned, removable, and carries its train/validation provenance. No JPL state table is embedded in the Antikythera core.
