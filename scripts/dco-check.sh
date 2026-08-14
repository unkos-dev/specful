#!/usr/bin/env bash
# Checks that every commit in a range carries a Signed-off-by trailer
# matching the commit author, per the Developer Certificate of Origin.
# Usage: dco-check.sh <base> <head>
set -euo pipefail

base="$1"
head="$2"
missing=0

for sha in $(git rev-list "$base".."$head" --no-merges); do
  author_name=$(git log -1 --format=%an "$sha")
  author_email=$(git log -1 --format=%ae "$sha")
  expected="Signed-off-by: ${author_name} <${author_email}>"
  # Only structurally valid trailers count; a matching line in prose or a
  # malformed identity does not satisfy the DCO.
  if ! git log -1 --format=%B "$sha" \
      | git interpret-trailers --parse --only-trailers \
      | grep -Fxq "$expected"; then
    echo "::error::commit $sha lacks a Signed-off-by trailer matching its author (${author_name} <${author_email}>); use git commit -s"
    missing=1
  fi
done
exit "$missing"
