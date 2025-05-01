#!/bin/sh

flatpak remote-add --if-not-exists --user flathub https://dl.flathub.org/repo/flathub.flatpakrepo &&
flatpak-builder build --user --force-clean --install-deps-from=flathub .flatpak-manifest.json &&
flatpak build-export export build &&
flatpak build-bundle export auracite.flatpak zone.xiv.auracite --runtime-repo=https://flathub.org/repo/flathub.flatpakrepo
