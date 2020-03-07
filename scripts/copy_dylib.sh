#!/bin/sh

cd `dirname $0`

cp ../Core/build/DEBUG/libAltseed_Core.dylib ../target/debug/.
cp ../Core/build/RELEASE/libAltseed_Core.dylib ../target/release/.