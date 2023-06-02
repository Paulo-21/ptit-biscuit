#!/bin/sh
depth=$1
fen=$2
move=$3

./target/release/ptit-biscuit $depth "$fen"