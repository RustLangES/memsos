#!/usr/bin/env bash

for tga in kernel/static/*; do
    echo $tga
    magick $tga -alpha off -depth 8 $tga
done
