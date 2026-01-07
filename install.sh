#!/bin/bash

echo "Checking system requirements..."

# check if requirements are met
sleep 0.5
./check-requirements.sh
if [ $? -ne 0 ]; then
  echo "Installation aborted due to unmet dependencies."
  exit 1
fi

echo "Requirements met. Proceeding with installation..."

# setting up directory path variables
BASEPATHREPO=$(pwd)
BASEPATHINSTALL=$HOME/.local/share/louarch

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
  else
    # abort installation
    echo "Installation aborted."
    exit 0
  fi
fi

# create installation directory
mkdir -p $HOME/.local/share/louarch

# copy config files
if [[ ! -f $BASEPATHINSTALL/config.json ]]; then
  cp -r $BASEPATHREPO/data/config.json $BASEPATHINSTALL/config.json
fi

# copy LICENSE and README.md
cp -r $BASEPATHREPO/LICENSE $BASEPATHINSTALL/LICENSE
cp -r $BASEPATHREPO/README.md $BASEPATHINSTALL/README.md

# copy binary to users bin directory
sudo cp -r $BASEPATHREPO/build/louarch /usr/bin/louarch

echo "Installation successful."
