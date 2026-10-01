# Hijri / Worship Resolution Architecture

M-Time separates five questions:

1. **What is the sky state?** — astronomy.
2. **Does a named criterion pass?** — versioned calendar profile.
3. **What was actually observed?** — rukyat evidence.
4. **Who has jurisdiction/authority?** — normative scope.
5. **What date became official?** — authority decision.

The engine MUST never use one field to answer all five questions.

## MABIMS Indonesia 2026

The bootstrap profile evaluates:
- topocentric lunar altitude >= 3 degrees;
- geocentric Sun–Moon elongation >= 6.4 degrees.

The official Indonesian workflow also uses hisab + rukyatulhilal and Sidang Isbat for Ramadan, Syawal and Zulhijjah. Exact legal/decision semantics are represented in the authority layer, not in the astronomical core.

## Flagship output

Every comparison should return:
- shared sky facts;
- different observers/sites;
- different profile clauses;
- different observation evidence;
- different jurisdiction;
- different authority decision;
- unknown/missing causes.
