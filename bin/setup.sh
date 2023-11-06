#!/bin/sh

lib="../lib"
url="https://github.com/vincentmader/mpl-styles"

mkdir -p "$lib"
cd "$lib" && git clone "$url"

# Create directory for save files.
mkdir -p "../saves"
