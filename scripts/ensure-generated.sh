#!/bin/bash

# A small script to use in CI to ensure that all generated files are up to date.

set -e -x -o pipefail

cd "$(dirname "$0")/.."

yarn run generate
yarn run format

if ! git diff --quiet; then
  echo "Generated files are out of date. Please run 'yarn generate' and 'yarn format' and commit the changes."
  git diff
  exit 1
fi