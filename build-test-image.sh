#!/usr/bin/env bash

if ! [ -d ./test_isodir ]; then
  mkdir -p test_isodir/boot/limine
  mkdir -p test_isodir/EFI/BOOT
  cp -v prebuilt/limine.conf ./test_isodir/boot/limine/
  cp -v prebuilt/limine-bios.sys prebuilt/limine-bios-cd.bin prebuilt/limine-uefi-cd.bin test_isodir/boot/limine/
  cp -v prebuilt/BOOTX64.EFI prebuilt/BOOTIA32.EFI test_isodir/EFI/BOOT/
fi

cd ./helium
cargo test 
cd ..

cp -v helium/target/x86_64-limine/debug/helium_test test_isodir/boot/helium
xorriso -as mkisofs -b boot/limine/limine-bios-cd.bin \
-no-emul-boot -boot-load-size 4 -boot-info-table \
--efi-boot boot/limine/limine-uefi-cd.bin \
-efi-boot-part --efi-boot-image --protective-msdos-label \
test_isodir -o test_image.iso

limine bios-install ./test_image.iso
