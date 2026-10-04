#!/usr/bin/env python3
from pathlib import Path
bad=[]
for p in Path("crates").glob("*/Cargo.toml"):
    name=p.parent.name
    text=p.read_text()
    if name!="mtime-revelation" and "mtime-revelation" in text:
        bad.append(str(p))
if bad:
    raise SystemExit("Scientific/application crates must not depend on revelation layer: "+", ".join(bad))
print("Layering invariant PASS: revelation ontology has no dependency path into scientific crates")


# Antikythera core must remain independent of modern oracle/policy/semantic layers.
antikythera = Path("crates/mtime-antikythera/Cargo.toml").read_text()
for forbidden in [
    "mtime-jpl",
    "mtime-spk",
    "mtime-eop",
    "mtime-hijri",
    "mtime-worship",
    "mtime-observation",
    "mtime-authority",
    "mtime-revelation",
    "mtime-cosmology",
]:
    if forbidden in antikythera:
        raise SystemExit(f"Antikythera core must not depend on {forbidden}")

# Operational clock crates must not depend on cosmological inference.
for crate in ["mtime-timescales", "mtime-antikythera", "mtime-temporal", "mtime-clock"]:
    p = Path("crates") / crate / "Cargo.toml"
    if "mtime-cosmology" in p.read_text():
        raise SystemExit(f"{crate} must not depend on mtime-cosmology")

print("Layering invariant PASS: Antikythera core and operational M-Time remain isolated from cosmology/policy/revelation")
