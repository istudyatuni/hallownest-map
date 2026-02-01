#!/usr/bin/env sh

# download full from https://www.hallownest.net/downloads/
input_file='Hallownest Map (100%).png'
# avif, webp, jxl
format='webp'
out_dir="out-$format"
parts=10

mkdir -p "$out_dir"

# approx time: 10m
# "-limit memory" is needed to offload data to disk
magick "$input_file" \
    -debug All \
    -limit memory 256MiB \
    -crop "${parts}x${parts}@" \
    -set filename:tile \
    "%[fx:page.y/page.height*${parts}]_%[fx:page.x/page.width*${parts}]" \
    +repage +adjoin "$out_dir/part_%[filename:tile].$format"
