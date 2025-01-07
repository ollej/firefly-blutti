#!/usr/bin/env sh

set -euo pipefail

GAME_NAME="olle.blutti"
LEVELS=5
OUTPUT_DIR="resources/screenshots"

mkdir -p "$OUTPUT_DIR"
source .venv/bin/activate
echo "Starting emulator..."
firefly-emulator --id olle.blutti &
EMULATOR_PID=$!
sleep 1
echo "Screenshotting title screen..."
screenshot -f "$OUTPUT_DIR/screenshot-title.png" firefly-emulator
for ((level = 1 ; level <= $LEVELS ; level++ ))
do
    echo "Screenshotting level $level..."
    firefly_cli cheat 1 $level
    if [ "$level" -eq "1" ] || [ "$level" -eq "3" ]
    then
        echo "Waiting for particles..."
        sleep 6
    fi
    screenshot -f "$OUTPUT_DIR/screenshot-level${level}.png" firefly-emulator
done
echo "Killing emulator..."
kill $EMULATOR_PID

