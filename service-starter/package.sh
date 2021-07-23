#!/bin/bash

DIST="service-starter"
SRC=$DIST
VER=$1

rm -rf $DIST $DIST.tar.bz2 $DIST-dbg.tar.bz2

mkdir -p $DIST $DIST/bin $DIST/config $DIST/dbscript $DIST/log $DIST/tests

echo "build..."
cargo build --release
#cargo build

echo "copy bin"
cp target/release/$SRC $DIST/bin/$DIST
#cp target/debug/$SRC $DIST/bin/$DIST
cp service-ctl.sh $DIST/bin/

echo "copy config"
cp -rf config/*.yml $DIST/config/

echo "copy dbscript"
cp -rf dbscript/*.sql $DIST/dbscript
cp -rf dbscript/*.sh $DIST/dbscript

echo "copy tests"
cp -rf tests/*.sh $DIST/tests/
cp -rf tests/*.json $DIST/tests/
cp -rf tests/*.py $DIST/tests/

cp -rf $DIST $DIST-dbg
strip $DIST/bin/$DIST

echo "package..."
tar -jcvf $DIST-$VER.tar.bz2 $DIST
tar -jcvf $DIST-dbg-$VER.tar.bz2 $DIST-dbg

echo "clean"
rm -rf $DIST
rm -rf $DIST-dbg

echo "done"
