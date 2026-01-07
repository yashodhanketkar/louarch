#!/bin/bash

# Required versions
REQUIRED_GO="1.25.5"
REQUIRED_WOFI="1.5.1"
REQUIRED_WALLUST="3.4.0"

echo "Running requirements check..."

# Check if version is greater than or equal to
version_ge() {
  [ "$(printf '%s\n' "$@" | sort -V | head -n1)" != "$1" ]
}

# Check each requirement
check_version() {
  tool=$1
  required=$2
  current=$3

  if version_ge "$required" "$current"; then
    echo "❌ $tool $current (requires $required+)"
    return 1
  else
    echo "✅ $tool $current"
    return 0
  fi
}

fail=0

# Check for go
if command -v go >/dev/null 2>&1; then
  sleep 0.1
  go_version=$(go version | awk '{print $3}' | sed 's/go//')
  check_version "GO" "$REQUIRED_GO" "$go_version" || fail=1
else
  echo "Go is not installed."
  go_version="0"
fi

# Check for wofi
if command -v wofi >/dev/null 2>&1; then
  sleep 0.1
  wofi_version=$(wofi --version | awk '{print $1}' | tr -dc '0-9.')
  check_version "Wofi" "$REQUIRED_WOFI" "$wofi_version" || fail=1
else
  echo "Wofi is not installed."
  wofi_version="0"
fi

# Check for wallust
if command -v wallust >/dev/null 2>&1; then
  sleep 0.1
  wallust_version=$(wallust --version | awk '{print $2}')
  check_version "Wallust" "$REQUIRED_WALLUST" "$wallust_version" || fail=1
else
  echo "Wallust is not installed."
  wallust_version="0"
fi

if [ "$fail" -eq 1 ]; then
  echo -e "One or more dependencies are missing or outdated."
  exit 1
else
  echo -e "All dependencies met. You're good to go!"
  exit 0
fi
