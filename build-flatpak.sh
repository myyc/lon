#!/bin/bash
set -e

flatpak-builder --user --force-clean build-dir dev.myyc.lon.json
flatpak-builder --user --install build-dir dev.myyc.lon.json
flatpak run dev.myyc.lon
