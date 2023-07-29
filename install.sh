#!/bin/bash

set -xe

cargo build --release

sudo cp target/release/silm /usr/local/bin/silm
