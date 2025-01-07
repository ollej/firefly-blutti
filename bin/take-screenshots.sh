#!/usr/bin/env sh

GAME_NAME="olle.blutti"
LEVELS=5
OUTPUT_DIR="resources/screenshots"

# Install screenshot command in virtualenv if necessary
if [ ! -d ".venv" ]
then
    echo "Virtual env not found, creating a new one..."
    python3 -m venv .venv
fi
source .venv/bin/activate
pip3 show package_name 1>/dev/null
if [ $? == 0 ]; then
    echo "Screenshot package already installed."
else
    echo "Installing screenshot package..."
    python3 -m pip --require-virtualenv install screenshot
fi

set -euo pipefail

mkdir -p "$OUTPUT_DIR"
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

