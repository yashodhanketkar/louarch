#!/bin/bash

echo "Checking system requirements..."

# check if requirements are met
sleep 0.5
./scripts/check-requirements.sh
if [ $? -ne 0 ]; then
    echo "Installation aborted due to unmet dependencies."
    exit 1
fi

echo "Requirements met. Proceeding with installation..."

# setting up directory path variables
BASEPATHREPO=$(pwd)
BASEPATHINSTALL=$HOME/.local/share/louarch
BINPATH=/usr/bin/louarch

# checking for previous installation
# checks for existence of lib directory

# if previous installation detected, ask user whether to
# ? remove and proceed
# : abort installation
if [[ -d $BASEPATHINSTALL ]]; then
    echo "Previous installation detected."
    read -p "Do you want remove previous installation and proceed? (y/n) " yn
    if [[ $yn == "y" ]]; then
        echo "Removing previous installation..."
        rm -rf $BASEPATHINSTALL
        sudo rm /usr/bin/louarch
    else
        # abort installation
        echo "Installation aborted."
        exit 0
    fi
fi

# create config directory
install -dm 755 $BASEPATHINSTALL

# Installing config files
install -m 644 $BASEPATHREPO/data/config.json $BASEPATHINSTALL/config.json
install -m 644 $BASEPATHREPO/LICENSE $BASEPATHINSTALL/LICENSE
install -m 644 $BASEPATHREPO/README.md $BASEPATHINSTALL/README.md

# compress binary if upx is installed
if command -V upx >/dev/null 2>&1; then
    sleep 0.1
    upx --best --lzma $BASEPATHREPO/target/release/louarch
fi
# copy binary to users bin directory
sudo install -m 755 $BASEPATHREPO/target/release/louarch /usr/bin/louarch

echo "Installation successful."
