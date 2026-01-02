#!/usr/bin/env bash

GAME_NAME="olle.blutti"
LEVELS=5

set -euo pipefail

echo "Starting emulator..."
firefly-emulator --id $GAME_NAME &
EMULATOR_PID=$!
sleep 1
for ((level = 0 ; level <= $LEVELS ; level++ ))
do
    echo "Loading level $level..."
    firefly_cli runtime cheat set-level $level
    sleep 0.5
    if ps -p $EMULATOR_PID > /dev/null
    then
        echo "Level $level loaded successfully."
    else
        echo
        echo "*** Failed loading level $level ***" 1>&2
        exit 1
    fi
done
echo "All levels loaded successfully"
echo "Killing emulator..."
kill $EMULATOR_PID

