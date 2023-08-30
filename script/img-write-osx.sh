#!/usr/bin/env bash

if [[ -f "disk1.vhd" ]]; then
  mv disk1.vhd disk1.img
fi


if [[ -f "disk2.vhd" ]]; then
  mv disk2.vhd disk2.img
fi

export DISK1_NAME=disk1.dmg

dd if=boot.bin of=$DISK1_NAME bs=512 conv=notrunc count=1

dd if=loader.bin of=$DISK1_NAME bs=512 conv=notrunc seek=1

dd if=kernel.elf of=$DISK1_NAME bs=512 conv=notrunc seek=100

export DISK2_NAME=disk2.dmg
export TARGET_PATH=mp
if [[ -d $TARGET_PATH ]]; then
  rm $TARGET_PATH
fi
hdiutil attach $DISK2_NAME -mountpoint $TARGET_PATH
cp -v *.elf $TARGET_PATH
hdiutil umont $TARGET_PATH -verbose