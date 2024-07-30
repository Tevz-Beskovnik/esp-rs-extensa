#!/bin/bash

PORT=$1
CHIP_TYPE=$2

if [[ $1 = "--help" ]] || [[ $1 = "-h" ]]; then
    echo "SPIFFS FLASH TOOL v0.0.1"
    echo "Usage:"
    echo ""
    echo "flash_spiffs_to_board.sh [PORT] [CHIP_TYPE]"
    echo ""
    echo "Values:"
    echo "PORT - port to witch the chip is connected to"
    echo "CHIP_TPYE - type of the chip that is connected, default is auto"
    exit
fi

espflash --version >> /dev/null

if [[ $? -ne 0 ]]; then
    echo "Please install espflash tool."
    exit
fi

python3 --version >> /dev/null

if [[ $? -ne 0 ]]; then
    echo "Please install python3."
    exit
fi

esptool.py --version >> /dev/null

if [[ $? -ne 0 ]]; then
    echo "Please install esptool.py."
    exit
fi

echo "Generating binary image of partition table..."

espflash partition-table ./util/partitions.csv

espflash partition-table --to-binary ./util/partitions.csv -o partitions.bin

echo "Creating spiffs.bin image..."

python3 "$HOME/esp/esp-idf/components/spiffs/spiffsgen.py" 1048576 ./spiffs spiffs.bin

if [[ -z "$CHIP_TYPE" ]]; then
    CHIP_TYPE=auto
fi

if [[ ! -z "$PORT" ]]; then
    PORT="--port $PORT"
fi


echo "Attempting to flash partitions to chip..."

esptool.py $PORT --chip "$CHIP_TYPE" write_flash -z 0x8000 partitions.bin

echo "Attempting to flash image to chip..."

esptool.py $PORT --chip "$CHIP_TYPE" write_flash -z 0x110000 spiffs.bin

echo "Cleaning up generated files..."

rm spiffs.bin
rm partitions.bin

echo "Done!"