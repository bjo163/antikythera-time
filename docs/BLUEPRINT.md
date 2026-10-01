# M-Time Engineering Blueprint

## North Star
M-Time is a universal temporal interoperability protocol. The SI second remains the physical unit. Existing UTC/TAI/IAU/IERS conventions remain references. M-Time adds semantic layers so astronomy, calendar rules, observations, jurisdiction, authority, cosmology, historical reconstruction, and revelation-text concepts are not silently collapsed.

## Flagship problem
Hijri/worship-calendar disagreement is the first end-to-end proving ground.

physical/coordinate time -> celestial state -> versioned calendar profile -> criterion result -> observation/rukyat evidence -> jurisdiction -> authority decision -> calendar outcome -> ExplainDifference

## Architecture decision
Rust is the authoritative computational core. Web/UI layers SHOULD consume Rust via WASM or thin APIs. JavaScript/Python remain independent compatibility implementations/checkers.

## Antikythera role
Antikythera contributes a computational grammar: phenomenon -> cycle/ratio -> state -> prediction. It is not a metrological authority and does not directly determine cosmic age.

## Revelation role
Revelation-text mappings are TEXTUAL_REFERENCE / CONCEPTUAL and MUST NOT supply hidden numerical scientific priors.
