#!/usr/bin/env bash

if ! [ -d ./isodir ]; then
  mkdir -p isodir/boot/limine
  mkdir -p isodir/EFI/BOOT
  cp -v prebuilt/limine.conf ./isodir/boot/limine/
  cp -v prebuilt/limine-bios.sys prebuilt/limine-bios-cd.bin prebuilt/limine-uefi-cd.bin isodir/boot/limine/
  cp -v prebuilt/BOOTX64.EFI prebuilt/BOOTIA32.EFI isodir/EFI/BOOT/
fi

cd ./helium
cargo build 
cd ..

cp -v helium/target/x86_64-limine/debug/helium isodir/boot/
xorriso -as mkisofs -b boot/limine/limine-bios-cd.bin \
-no-emul-boot -boot-load-size 4 -boot-info-table \
--efi-boot boot/limine/limine-uefi-cd.bin \
-efi-boot-part --efi-boot-image --protective-msdos-label \
isodir -o image.iso

limine bios-install ./image.iso
