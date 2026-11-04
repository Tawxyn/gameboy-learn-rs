#!/bin/sh
set -eu

cd "$(dirname "$0")"
EMULATOR="$PWD/scripts/sameboy.sh" exec cargo gb run
