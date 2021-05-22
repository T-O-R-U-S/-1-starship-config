#!/bin/zsh
# Using a build.sh instead of Makefile
# due to weirdness when using for loops... :^(

cd starship_bins

for f in ../*.rs; echo $f && rustc $f;