#!/bin/sh
if [ ! -f "./bin/wasmcloud_host" ]; then
wget -c https://github.com/wasmCloud/wasmcloud-otp/releases/download/v0.57.4/x86_64-linux.tar.gz -O - | tar -xz
ls
fi
