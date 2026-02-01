#!/bin/bash
set -euo pipefail
IMAGE_VERSION="$(git rev-parse --short=10 HEAD)-$(date --utc +%Y%m%d%H%M)"

rm -f mkosi/mkosi.output/nfSense*

if [[ $# -ne 1 ]]; then
    echo "Prod Mode"
    mkosi --directory mkosi --profile=prod  --image-version="${IMAGE_VERSION}" build
else
    if [ "$1" ==  "dev" ]; then
        echo "Dev Mode"
        mkosi --directory mkosi --profile=dev --image-version="${IMAGE_VERSION}" build
        echo "Done"
    else
        echo 'Too many/few arguments, expecting [dev]' >&2
        exit 1
    fi
fi

RAW_FILE=$(find mkosi/mkosi.output -name "*${IMAGE_VERSION}.raw" -print -quit)

if [ -z "$RAW_FILE" ]; then
    echo "Error: Could not find raw image for version ${IMAGE_VERSION}"
    exit 1
fi
QCOW2_FILE="${RAW_FILE%.raw}.qcow2"


echo "Converting to qcow2"
qemu-img convert -p -f raw -O qcow2 -c "$RAW_FILE" "$QCOW2_FILE"
echo "Done"
