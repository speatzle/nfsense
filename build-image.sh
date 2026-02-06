#!/bin/bash
set -euo pipefail
IMAGE_VERSION="git$(date --utc +%Y%m%d%H%M)-$(git rev-parse --short=10 HEAD)"
#IMAGE_VERSION="v0.0.1"

rm -f mkosi/mkosi.output/nfsense* initrd*

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


COMPRESSED_FILE=$(find mkosi/mkosi.output -name "nfsense_${IMAGE_VERSION}.raw.zst" -print -quit)

if [ -z "$COMPRESSED_FILE" ]; then
    echo "Error: Could not find image for version ${IMAGE_VERSION}"
    exit 1
fi

RAW_FILE="${COMPRESSED_FILE%.zst}"
echo "Decompressing"
zstd -d -f "$COMPRESSED_FILE" -o "$RAW_FILE"

QCOW2_FILE="${RAW_FILE%.raw}.qcow2"


echo "Converting to qcow2"
qemu-img convert -p -f raw -O qcow2 -c "$RAW_FILE" "$QCOW2_FILE"
qemu-img resize "$QCOW2_FILE" 32G

echo "Calculate hash for qcow2"
sha256sum "$QCOW2_FILE" >> "nfsense_${IMAGE_VERSION}.SHA256SUM"

echo "Done"
