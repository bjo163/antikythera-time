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
