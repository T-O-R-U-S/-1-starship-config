#!/bin/zsh
# Using a build.sh instead of Makefile directly
# due to weirdness when using for loops... :^(

cd starship_bins

# dead_code is allowed because the shared lib uses it
# but Rust always throws a warning when it's not used.
# This was simply to de-clutter :)
for f in ../*.rs; echo Building: $f && rustc -A dead_code $f;