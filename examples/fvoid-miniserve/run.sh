#!/bin/bash

miniserve=~/misc/apps/miniserve
ffmpeg=~/misc/apps/ffmpeg
fvoid="cargo run --"
dist=$(dirname $(realpath $0))/dist
if [ ! -d $dist ]; then
  mkdir $dist
  pushd $dist
  $fvoid pdf void.pdf
  $fvoid bmp void.bmp
  $fvoid svg void.svg
  $ffmpeg -f lavfi -i nullsrc=s=4x4:r=1:d=3600 void.mp4
  $ffmpeg -f lavfi -i nullsrc=s=4x4:r=1:d=3600 void.flv
  $ffmpeg -f lavfi -i nullsrc=s=4x4:r=1:d=3600 -hls_time 7200 void.m3u8
  # $ffmpeg -f lavfi -i nullsrc=size=4x4:rate=1:duration=3600 void.flv
  # $ffmpeg -f lavfi -i anullsrc=r=64:cl=mono:duration=3600 -b:a 8k void.mp3
  popd
fi
$miniserve --header access-control-allow-origin:* --header cache-control:max-age=3 -p 9902 -v $dist
