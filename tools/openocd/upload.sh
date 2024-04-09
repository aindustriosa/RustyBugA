#!/bin/bash

echo "Uploading $1 with openocd and stlink probe"

openocd -f ${TOOLS_DIR}/openocd/stlink-stm32f1.cfg \
    -c init -c halt \
    -c "flash write_image erase $1" \
    -c reset -c shutdown
