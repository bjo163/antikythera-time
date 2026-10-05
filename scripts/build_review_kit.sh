#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
cd "$ROOT"

OUT_DIR="${1:-/tmp/mtime-m20-review-kit}"
ARCHIVE="${OUT_DIR}.tar.gz"
ARCHIVE_SHA="${ARCHIVE}.sha256"
WORK="/tmp/mtime-m20-candidate-$$"

cleanup() {
  git worktree remove --force "$WORK" >/dev/null 2>&1 || true
}
trap cleanup EXIT

CANDIDATE_RELEASE="$(python3 - <<'PY'
import json
d=json.load(open("reviews/external/requirements.json", encoding="utf-8"))
print(d["candidate"]["release"])
PY
)"
CANDIDATE_SHA="$(python3 - <<'PY'
import json
d=json.load(open("reviews/external/requirements.json", encoding="utf-8"))
print(d["candidate"]["git_sha"])
PY
)"

if ! [[ "$CANDIDATE_SHA" =~ ^[0-9a-f]{40}$ ]]; then
  echo "invalid candidate SHA: $CANDIDATE_SHA" >&2
  exit 2
fi

git cat-file -e "$CANDIDATE_SHA^{commit}"

rm -rf "$OUT_DIR" "$ARCHIVE" "$ARCHIVE_SHA" "$WORK"
git worktree add --detach "$WORK" "$CANDIDATE_SHA" >/dev/null

mkdir -p   "$OUT_DIR/spec"   "$OUT_DIR/docs"   "$OUT_DIR/vectors"   "$OUT_DIR/review-protocol/packets"

cat > "$OUT_DIR/CANDIDATE.txt" <<EOF
M-Time external review candidate
release=$CANDIDATE_RELEASE
git_sha=$CANDIDATE_SHA

Review this exact candidate. A later main/dev commit is not the same review target.
EOF

for file in   spec/ANTIKYTHERA-DIGITAL-V1.md   spec/MTIME-2.0.md   spec/RFC-0001-MTIME-1.0.md   spec/MTBUNDLE-1.md; do
  cp "$WORK/$file" "$OUT_DIR/spec/"
done

for file in   docs/ANTIKYTHERA_CORE.md   docs/ANTIKYTHERA_CALIBRATION.md   docs/M6_ACCURACY_PROGRAM.md   docs/M10_UNCERTAINTY.md   docs/M16_LONGSPAN_FALSIFICATION.md   docs/M16_VALIDATED_INTERVAL.md   docs/M17_REVELATION_ONTOLOGY.md   docs/REVELATION_ONTOLOGY.md   docs/M19_THREAT_REVIEW.md   docs/HISTORICAL_FALSIFICATION.md   docs/FFI_SAFETY.md; do
  cp "$WORK/$file" "$OUT_DIR/docs/"
done

(
  cd "$WORK"
  cargo run --quiet -p mtime-temporal --example mts2_conformance
) > "$OUT_DIR/vectors/mts2-conformance.txt"

cat > "$OUT_DIR/vectors/malformed-mts2.txt" <<'EOF'
# hex payload | required behavior
<empty> | reject
4d5453310000 | reject
4d54533200 | reject
4d5453320005616263 | reject
EOF

cp reviews/external/review-template.json "$OUT_DIR/review-protocol/"
cp reviews/external/review.schema.json "$OUT_DIR/review-protocol/"
cp reviews/external/requirements.json "$OUT_DIR/review-protocol/"
cp reviews/external/REVIEWER_QUICKSTART.md "$OUT_DIR/review-protocol/"
cp reviews/external/CANDIDATE.md "$OUT_DIR/review-protocol/"
cp reviews/external/CANDIDATE-3.md "$OUT_DIR/review-protocol/"
cp reviews/external/packets/*.md "$OUT_DIR/review-protocol/packets/"
cp reviews/external/packets/index.json "$OUT_DIR/review-protocol/packets/"

cat > "$OUT_DIR/README.txt" <<'EOF'
M-Time M20 external review handoff kit.

Start with:
  review-protocol/REVIEWER_QUICKSTART.md

For independent reproduction, do not copy project implementation code.
Implement from the public specifications in a separate repository and compare
against the frozen vectors.

Matching vectors alone is not scientific validation. Use the category packet
for the required scope.
EOF

cat > "$OUT_DIR/verify-kit.py" <<'PY'
#!/usr/bin/env python3
from __future__ import annotations
import hashlib
from pathlib import Path
import sys

root = Path(__file__).resolve().parent
manifest = root / "SHA256SUMS"
if not manifest.exists():
    raise SystemExit("SHA256SUMS missing")

checked = 0
for raw in manifest.read_text(encoding="utf-8").splitlines():
    if not raw.strip():
        continue
    digest, rel = raw.split("  ", 1)
    path = root / rel.removeprefix("./")
    if not path.is_file():
        raise SystemExit(f"missing file: {rel}")
    h = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            h.update(chunk)
    actual = h.hexdigest()
    if actual != digest:
        raise SystemExit(f"SHA-256 mismatch: {rel}: expected {digest}, got {actual}")
    checked += 1

if checked == 0:
    raise SystemExit("no checksum entries")
print(f"review kit integrity: PASS ({checked} files)")
PY
chmod +x "$OUT_DIR/verify-kit.py"

test "$(grep -c '^V|' "$OUT_DIR/vectors/mts2-conformance.txt")" -eq 135
test "$(grep -c '^E|' "$OUT_DIR/vectors/mts2-conformance.txt")" -eq 135

(
  cd "$OUT_DIR"
  find . -type f ! -name SHA256SUMS -print0     | sort -z     | xargs -0 sha256sum > SHA256SUMS
  python3 verify-kit.py
)

tar -C "$(dirname "$OUT_DIR")" -czf "$ARCHIVE" "$(basename "$OUT_DIR")"
sha256sum "$ARCHIVE" > "$ARCHIVE_SHA"

echo "review_kit_release=$CANDIDATE_RELEASE"
echo "review_kit_candidate_sha=$CANDIDATE_SHA"
echo "valid_v1_vectors=135"
echo "experimental_v2_vectors=135"
echo "implementation_source_in_kit=false"
echo "review_kit_dir=$OUT_DIR"
echo "review_kit_archive=$ARCHIVE"
echo "review_kit_archive_sha256=$ARCHIVE_SHA"
