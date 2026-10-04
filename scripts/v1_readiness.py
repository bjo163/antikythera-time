#!/usr/bin/env python3
import json
from pathlib import Path

from external_review_readiness import evaluate_external_reviews

path=Path("data/v1-gates.json")
data=json.loads(path.read_text())
assert data["schema"]=="mtime-v1-readiness-1"
gates=data["gates"]
ids=[g["id"] for g in gates]
assert len(ids)==len(set(ids)), "duplicate v1 gate id"

# External review gates cannot be promoted by editing this file alone.
# A PASS must be backed by a SATISFIED, hash-verified review package for the
# exact pinned candidate.
external=evaluate_external_reviews()
for gate in gates:
    gate_id=gate["id"]
    if gate_id in external["gates"] and gate["status"]=="PASS":
        evidence_state=external["gates"][gate_id]
        assert evidence_state=="SATISFIED", (
            f"{gate_id} cannot be PASS without SATISFIED external evidence; "
            f"current evidence state={evidence_state}"
        )

blocked=[g for g in gates if g["status"].startswith("BLOCKED")]
needs_external=[g for g in gates if "NEEDS_EXTERNAL" in g["status"]]
computed_ready=not blocked and not needs_external and all(g["status"]=="PASS" for g in gates)
assert data["ready"]==computed_ready, (
    f"ready field mismatch: declared={data['ready']} computed={computed_ready}"
)
print(f"M-Time v1 readiness: {'READY' if computed_ready else 'BLOCKED'}")
for g in blocked+needs_external:
    print(f"- {g['id']}: {g['status']}")
