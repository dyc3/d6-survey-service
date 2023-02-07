#!/bin/bash

# A small script to use in CI to ensure that all generated files are up to date.

set -e -x -o pipefail

cd "$(dirname "$0")/.."

yarn run generate
yarn run format

git diff --quiet || exit 1