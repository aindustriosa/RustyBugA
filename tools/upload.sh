#!/bin/bash

# This script is used to upload the firmware to the board.
# It is used from `cargo run` when used the apropiate option in .cargo/config (parameter: runner)

# Set the upload programming probe to use
UPLOAD_TOOL=${UPLOAD_TOOL:-'openocd'}

# set this script directory as the reference directory for relative paths
export TOOLS_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

case ${UPLOAD_TOOL} in

    'openocd')
        ${TOOLS_DIR}/openocd/upload.sh $1
        ;;

    'blackmagic')
        ${TOOLS_DIR}/blackmagic/upload.sh $1
        ;;

    *)
        echo "Unknown upload tool: ${UPLOAD_TOOL}"
        exit 1
        ;;
esac

