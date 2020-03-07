#!/bin/sh
cd `dirname $0`
cargo test -- --nocapture --test-threads=1