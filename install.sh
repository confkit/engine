#!/bin/sh
set -e
OS=$(uname | tr '[:upper:]' '[:lower:]')
if [ "$OS" = "darwin" ]; then
  URL="https://github.com/confkit-io/confkit/releases/latest/download/confkit-macos"
else
  URL="https://github.com/confkit-io/confkit/releases/latest/download/confkit-linux"
fi
curl -L "$URL" -o /usr/local/bin/confkit
chmod +x /usr/local/bin/confkit
echo "✓ confkit installed to /usr/local/bin/confkit"
confkit --help 