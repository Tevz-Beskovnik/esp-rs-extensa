#!/bin/bash

MODE=$1
MONITOR=$2

if [[ -z "$MODE" ]]; then
    BUILD_MODE=""
elif [[ "$MODE" == "release" ]]; then
    BUILD_MODE="--release"
else
    echo "Mode can only be "release", if unset debug is used"
    exit
fi

if [[ $MONITOR != "monitor" ]] && [[ ! -z "$MONITOR" ]]; then
    echo "Second option is only monitor or nothing"
    exit
elif [[ $MONITOR = "monitor" ]]; then
    MONITOR="--monitor"
fi

espflash --version >> /dev/null

if [[ $? -ne 0 ]]; then
    echo "Please install espflash tool."
    exit
fi

cargo --version >> /dev/null

if [[ $? -ne 0 ]]; then
    echo "Please install rust."
    exit
fi

echo "Building in $MODE mode."
echo "Building app..."

cargo build $BUILD_MODE

echo "Flashing app to chip..."

espflash flash "target/xtensa-esp32-espidf/$MODE/esp-rs-extensa" $MONITOR

echo "Done!"