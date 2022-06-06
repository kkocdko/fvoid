#!/bin/bash

miniserve=/home/kkocdko/misc/tools/miniserve
ffmpeg=/home/kkocdko/misc/tools/ffmpeg
fvoid=$(dirname $0)/../../target/debug/fvoid
dist=$(dirname $0)/dist
if [ ! -d $dist ]; then
  sh -c "cd $(dirname $0)/../.. && cargo build"
  mkdir $dist
  $fvoid pdf $dist/void.pdf
  $fvoid bmp $dist/void.bmp
  $fvoid svg $dist/void.svg
  $ffmpeg -f lavfi -i nullsrc=size=4x4:rate=1:duration=3600 -hls_time 7200 $dist/void.m3u8
  $ffmpeg -f lavfi -i nullsrc=size=4x4:rate=1:duration=3600 $dist/void.mp4
  $ffmpeg -f lavfi -i nullsrc=size=4x4:rate=1:duration=3600 $dist/void.flv
  # $ffmpeg -f lavfi -i anullsrc=r=64:cl=mono:duration=3600 -b:a 8k $dist/void.mp3
fi
$miniserve --header access-control-allow-origin:* --header cache-control:max-age=3 -p 9902 $dist
