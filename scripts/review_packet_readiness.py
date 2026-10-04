#!/usr/bin/env python3
"""Validate that M20 reviewer packets exactly track external gate requirements."""

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
REQ = ROOT / "reviews" / "external" / "requirements.json"
INDEX = ROOT / "reviews" / "external" / "packets" / "index.json"

req = json.loads(REQ.read_text(encoding="utf-8"))
idx = json.loads(INDEX.read_text(encoding="utf-8"))

assert idx["schema"] == "mtime-external-review-packet-index-1"
assert idx["candidate"] == req["candidate"], "packet index candidate must equal requirements candidate"

requirements = {x["category"]: x for x in req["requirements"]}
packets = {x["category"]: x for x in idx["packets"]}

assert set(packets) == set(requirements), "packet categories must match external review requirements"
assert len(idx["packets"]) == len(packets), "duplicate packet category"
issues = [p["issue"] for p in idx["packets"]]
assert len(issues) == len(set(issues)), "duplicate packet issue mapping"

release = req["candidate"]["release"]
sha = req["candidate"]["git_sha"]

for category, requirement in requirements.items():
    packet = packets[category]
    assert set(packet["required_topics"]) == set(requirement["required_topics"]), (
        f"{category}: packet topics differ from requirements"
    )
    path = ROOT / packet["file"]
    assert path.exists() and path.is_file(), f"{category}: missing packet file {packet['file']}"
    text = path.read_text(encoding="utf-8")
    assert category in text, f"{category}: packet must state category token"
    assert release in text, f"{category}: packet must state frozen release"
    assert sha in text, f"{category}: packet must state frozen git sha"
    for topic in requirement["required_topics"]:
        assert topic in text, f"{category}: missing required topic token {topic}"

print(f"M20 reviewer packets: PASS ({len(packets)} categories)")
print(f"candidate: {release} @ {sha}")
