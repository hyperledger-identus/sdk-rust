#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0
set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
output="${1:-}"

if [[ -z "$output" || "$output" != /* ]]; then
  echo "usage: build-docs-site.sh /absolute/output/path" >&2
  exit 2
fi
if [[ -e "$output" ]]; then
  echo "docs site output already exists: $output" >&2
  exit 2
fi

workspace="$(mktemp -d)"
cleanup() {
  if [[ -n "${workspace:-}" && -d "$workspace" ]]; then
    rm -rf -- "$workspace"
  fi
}
trap cleanup EXIT

cp -R "$repository_root/site/." "$workspace/"
mkdir -p "$workspace/src/diagrams"

while IFS= read -r -d '' diagram; do
  name="$(basename "$diagram" .dot)"
  dot -Tsvg "$diagram" -o "$workspace/src/diagrams/$name.svg"
done < <(find "$workspace/diagrams" -type f -name '*.dot' -print0 | sort -z)

mdbook build "$workspace" --dest-dir "$output"

if find "$output" -type l -print -quit | grep -q .; then
  echo "docs site contains a symbolic link" >&2
  exit 1
fi

# The generated 404 page intentionally links to the project root (`site-url`),
# which only exists once GitHub Pages mounts the artifact at `/sdk-rust/`.
find "$output" -type f -name '*.html' ! -name '404.html' -print0 \
  | xargs -0 lychee --offline --no-progress --root-dir "$output"
