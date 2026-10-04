# Diyanet Policy Context Providers

M-Time keeps the 5°/8° astronomy criterion separate from the additional calendar-policy conditions described by Diyanet.

The provider-wired path removes manual boolean entry from the evaluation call:

- threshold-passing site IDs come from the astronomical criterion evaluator;
- an `AmericasMainlandProvider` classifies whether a passing site is on North/South American mainland;
- a `WellingtonFajrProvider` supplies a computed, explicitly versioned Wellington imsak/fajr event;
- conjunction and fajr are compared on the explicitly labelled UT1 Julian-date scale;
- missing geospatial or worship-time evidence remains `UNKNOWN`, rather than being silently converted to pass/fail.

This is an interface boundary, not a claim that M-Time already ships a production-grade global landmass polygon dataset or a universal fajr angle. Those remain external/versioned provider responsibilities until backed by an authoritative dataset/profile.

Official 1447/2026 Diyanet statements represented by the profile require all four conditions together: 8° elongation, 5° Moon altitude at sunset, conjunction before Wellington imsak, and qualifying visibility on North/South American mainland.
