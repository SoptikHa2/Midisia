#!/bin/bash

OFFSET_TOP_DROP=5
PIXELS_TOP_GET=1
EXTRACT_FRAMES_PER_SECOND=15
BACKGROUND_COLOR="0:0:0"
LOOK_FOR_COLORS="255:0:0"


set -euo pipefail

if [[ "$#" -eq 0 ]]; then
    echo "Usage: $0 <URL>" >&2
    exit 1
fi

tmpdir=""

trap "rm -rf \"\$tmpdir\"" SIGINT SIGTERM EXIT
tmpdir="$(mktemp -d)"

# Download the video
youtube-dl -f "worstvideo" -o "$tmpdir/VIDEO-%(title)s.%(ext)s" "$1"

# Split it into individual frames (remove -r option to dump all the frames)
ffmpeg -i "$tmpdir/VIDEO-*" -r 15/1 "$tmpdir/PHOTO-%08d.png"

# Get one pixel line from the frames
for photo in "$tmpdir"/PHOTO-*.png; do
    convert -crop "0x$PIXELS_TOP_GET+0+$OFFSET_TOP_DROP" -define png:color-type=2 "$photo" "$photo.cropped" &
done

# Get pressed notes
echo Parsing images, this may take a while
for photo in "$tmpdir"/PHOTO-*.png.cropped; do
    ./read-pixels "$photo" "$BACKGROUND_COLOR,$LOOK_FOR_COLORS" >> "$tmpdir/pixels.dat"
done


