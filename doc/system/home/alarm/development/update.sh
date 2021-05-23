#!/usr/bin/env bash

ABSPATH=$(cd "$(dirname "$0")"; pwd)
BASE_DIR="$ABSPATH/makair-control-ui/"

pushd "$BASE_DIR" > /dev/null
  echo "-- This script will update the MakAir Control UI --"

  # Give some time to cancel
  sleep 1

  echo "--"
  echo "Step 1: Updating source from Git..."

  git pull || exit 1

  echo "--"
  echo "Step 2: Re-building binary... (development mode; not optimized!)"

  RUSTFLAGS='-C link-arg=-s' cargo build || exit 1

  echo "--"
  echo "Step 3: Overwriting old binary..."

  cp -f ./target/debug/makair-control ../../ || exit 1

  echo "--"
  echo "Step 4: Re-starting the current UI runtime..."

  ../../makair-restart.sh || exit 1

  echo "--"
  echo "Done."
popd > /dev/null

exit 0
