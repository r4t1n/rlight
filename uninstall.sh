#!/bin/bash

target="/usr/local/bin/rlight"
udev_target="/lib/udev/rules.d/90-rlight.rules"

if [ "$EUID" -ne 0 ]
  then echo "Error: root privileges needed"
  exit
fi

rm $target $udev_target
