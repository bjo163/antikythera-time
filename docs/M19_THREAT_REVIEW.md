# M19 API / Protocol Threat Review

Threats considered:

- malformed/non-finite numeric input;
- i128 precision loss in JSON/JavaScript;
- profile confusion;
- stale reference bundles;
- hash/signature substitution;
- revoked/expired keys;
- replayed observation/authority packets;
- duplicate observation IDs with conflicting evidence;
- hidden UTC/leap/EOP assumptions;
- cosmology/revelation dependency injection.

Current mitigations include decimal-string i128, MTS binary magic/length checks, explicit profile IDs, source integrity registry, duplicate-conflict detection, and dependency-layer CI.

Open before v1.0: external security review, decoder/FFI fuzzing, hardware secure-update design, and operational key-rotation review.
