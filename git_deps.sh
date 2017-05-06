#!/bin/sh

# This scripts will clone the gtk-rs repos supplied as arguments into ./git
# and set up a path override for cargo to find them. You can do that in your
# crate directory or any of its parent directories.

set -e
set -u

if [ "${1-}" == "" ]; then
	echo "Usage: $0 <repo_name> [<repo_name> ...]"
	echo Availiable repos: cairo glib gdk gtk pango
	exit
fi

ask_yn() {
	ANS=
	read -p "$1 " ANS
	case "$ANS" in
		[yY]* ) return 0
			;;
		* )	return 1
			;;
	esac
}

GIT_URL="https://github.com/gtk-rs"
OVERRIDE=""

mkdir .cargo git 2> /dev/null || true

for CRATE in $@; do
	OVERRIDE="$OVERRIDE\"git/$CRATE\", "
	if [ -e "git/$CRATE" ]; then
		if ask_yn "git/$CRATE already exists. Overwrite? [y/N]"; then
			rm -rf "git/$CRATE"
		else
			continue
		fi
	fi
	git clone -q --depth 50 "$GIT_URL/$CRATE" git/$CRATE
done

if [ -e .cargo/config ]; then
	echo "paths = [$OVERRIDE]"
	if ! ask_yn ".cargo/config already exists. Overwrite [y/N]"; then
		exit
	fi
fi
echo "paths = [$OVERRIDE]" > .cargo/config
