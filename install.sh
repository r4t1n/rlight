#!/bin/bash

target="target/release/rlight"
target_path="/usr/local/bin"

udev_rules="90-rlight.rules"
udev_path="/lib/udev/rules.d"

if [ "$EUID" -ne 0 ]
  then echo "Error: root privileges needed"
  exit
fi

cp $target $target_path
cp $udev_rules $udev_path
