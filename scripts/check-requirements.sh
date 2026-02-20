#!/bin/bash

# Required versions
REQUIRED_CARGO="1.90.0"
REQUIRED_ROFI="2.0.0"
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

# Check for cargo
if command -v cargo >/dev/null 2>&1; then
  sleep 0.1
  cargo_version=$(cargo --version | awk '{print $2}')
  check_version "cargo" "$REQUIRED_CARGO" "$cargo_version" || fail=1
else
  echo "Cargo is not installed."
  cargo_version="0"
fi

# Check for wofi
if command -v rofi >/dev/null 2>&1; then
  sleep 0.1
  rofi_version=$(rofi -v | awk -F'[ -]' '{ print $2 }')
  check_version "Rofi" "$REQUIRED_ROFI" "$rofi_version" || fail=1
else
  echo "Rofi is not installed."
  rofi_version="0"
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
