#!/bin/sh

set -e
set -u

GIT_URL="https://github.com/rust-gnome"
OVERRIDE=""

mkdir .cargo git
for CRATE in $@; do
	OVERRIDE="$OVERRIDE\"git/$CRATE\", "
	git clone -q --depth 50 "$GIT_URL/$CRATE" git/$CRATE
done
echo "paths = [$OVERRIDE]" > .cargo/config
