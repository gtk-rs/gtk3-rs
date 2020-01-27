#!/bin/sh

set -x
set -e

if [ "$GTK" = latest -o "$GTK" = "3.24" ]; then
	BUNDLE="gtk-3.24.0-1"
	if [ "$TRAVIS_RUST_VERSION" = "nightly" ]; then
		FEATURES=gtk_3_24,gio/v2_44
	else
		FEATURES=gtk_3_24,gio/v2_44
	fi
elif [ "$GTK" = "3.22.30" ]; then
	BUNDLE="gtk-3.22.30-1"
	if [ "$TRAVIS_RUST_VERSION" = "nightly" ]; then
		FEATURES=gtk_3_22_30,gio/v2_44
	else
		FEATURES=gtk_3_22_30,gio/v2_44
	fi
elif [ "$GTK" = "3.18" ]; then
	BUNDLE="gtk-3.18.1-2"
	if [ "$TRAVIS_RUST_VERSION" = "nightly" ]; then
		FEATURES=gtk_3_18,gio/v2_44
	else
		FEATURES=gtk_3_18,gio/v2_44
	fi
fi

if [ -n "$BUNDLE" ] && [ "$TRAVIS_OS_NAME" != "osx" ]; then
	WD="$PWD"
	cd "$HOME"
	curl -LO "https://github.com/EPashkin/gtk-bootstrap/releases/download/$BUNDLE/deps.txz"
	tar xf deps.txz
	cd "$WD"
	export PKG_CONFIG_PATH="$HOME/local/lib/pkgconfig"
fi

if [ -n "$OTHER_TARGET" ]; then
  PKG_CONFIG_ALLOW_CROSS=1 cargo check $OTHER_TARGET --features "$FEATURES" --jobs 1 "$@"
else
  RUSTFLAGS="-C link-dead-code" travis_wait cargo build --features "$FEATURES" --jobs 1 "$@"
fi
