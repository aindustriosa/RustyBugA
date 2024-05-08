#!/bin/bash

echo "Uploading $1 with blackmagic probe"

# Default port is /dev/ttyACM0 or the second argument
BMP_PORT=${2:-/dev/ttyACM0}

# Check if gdb-multiarch is installed
GDB=$(command -v gdb-multiarch)
if [ -z "$GDB" ]; then
    echo "gdb-multiarch not found. Please install it."
    exit 1
fi

gdb-multiarch -n -batch \
    -x ${TOOLS_DIR}/blackmagic/gdb-functions \
    -ex "upload-remote $1 $BMP_PORT"
