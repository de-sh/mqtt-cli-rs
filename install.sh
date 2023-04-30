#!/bin/sh

ARCH=$(uname -m)
OS=$(uname -o)
BIN_DIR=/usr/bin

case "${ARCH}, ${OS}" in
    "x86_64, GNU/Linux") TARGET="x86_64-unknown-linux-musl"
        ;;
    "armv7, GNU/Linux") TARGET="armv7-unknown-linux-gnueabihf"
        ;;
    "armv7l, GNU/Linux") TARGET="armv7-unknown-linux-gnueabihf"
        ;;
    "aarch64, GNU/Linux") TARGET="aarch64-unknown-linux-gnu"
        ;;
    "arm64, Darwin") TARGET="aarch64-apple-darwin"
        ;;
    "x86_64, Darwin") TARGET="x86_64-apple-darwin"
        ;;
    *) echo "Unknown target, no uplink binary available. Open an issue to add support for your platform."
       echo "https://github.com/de-sh/mqtt-cli-rs/issues/new?labels=new-target&title=Add+support+for+${OS}+on+${ARCH}"; exit
        ;;
esac

echo "Creating directory to store executable: ${BIN_DIR}"
mkdir -p ${BIN_DIR}

# Find link to download latest release of mqtt-cli-rs
BIN_URL=$(curl -H "Accept: application/vnd.github.v3+json" -s https://api.github.com/repos/de-sh/mqtt-cli-rs/releases/latest | grep ${TARGET} | grep "download_url" | cut -d : -f 2,3 | tr -d \" )

echo ""
echo "Downloading mqtt-cli-rs for the target ${TARGET}"
echo "url: ${BIN_URL}"
curl -SfL -o "${BIN_DIR}/mqtt-cli-rs" ${BIN_URL}
chmod +x "${BIN_DIR}/mqtt-cli-rs"

echo "Add ${BIN_DIR} to PATH"