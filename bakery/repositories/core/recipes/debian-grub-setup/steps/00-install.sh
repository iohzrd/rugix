#!/bin/bash

set -euo pipefail

apt-get update -y

BOOT_DIR="${RUGIX_LAYER_DIR}/roots/boot"

if [ "${RECIPE_PARAM_WITH_SQUASHFS}" == "true" ]; then
    apt-get install -y initramfs-tools
    echo "squashfs" > "/usr/share/initramfs-tools/modules.d/rugix"
fi

mkdir -p "${BOOT_DIR}"

echo "Installing kernel..."
case "${RUGIX_ARCH}" in
    "amd64")
        apt-get install -y \
            linux-image-amd64 \
            linux-headers-amd64
        ;;
    "arm64")
        apt-get install -y \
            linux-image-arm64 \
            linux-headers-arm64
        ;;
    *)
        echo "Unsupported architecture '${RUGIX_ARCH}'."
        exit 1
esac

if [ "${RECIPE_PARAM_WITH_FIRMWARE}" == "true" ]; then
    echo "Installing free firmware..."
    apt-get install -y firmware-linux-free
fi

if [ "${RECIPE_PARAM_WITH_NONFREE}" == "true" ]; then
    # Make sure that the non-free sources are available.
    sed -i '/main/!b; /non-free/b; s/$/ non-free/' /etc/apt/sources.list
    sed -i '/main/!b; /non-free-firmware/b; s/$/ non-free-firmware/' /etc/apt/sources.list

    apt-get update -y
    
    if [ "${RECIPE_PARAM_WITH_FIRMWARE}" == "true" ]; then
        echo "Installing nonfree firmware..."
        apt-get install -y firmware-linux
    fi
fi

echo "Copying kernel and initrd..."
cp -L /vmlinuz "${BOOT_DIR}"
cp -L /initrd.img "${BOOT_DIR}"

echo "Installing second stage boot script..."
cp "${RECIPE_DIR}/files/grub.cfg" "${BOOT_DIR}"
