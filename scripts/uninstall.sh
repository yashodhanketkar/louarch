#!/bin/bash

echo "Uninstalling louarch..."

echo "Removing config files..."
rm -rf $HOME/.local/share/louarch/

echo "Removing binary..."
sudo rm -rf /usr/bin/louarch
