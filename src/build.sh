#!/bin/bash

cargo build &> /dev/null
../target/debug/./color ~/Desktop/openstreetmap.png --descriptive
