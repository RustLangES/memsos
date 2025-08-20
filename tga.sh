#!/usr/bin/env bash

for tga in crates/images/static/*; do
    echo $tga
    magick $tga -alpha off -depth 8 $tga
done
