FROM ghcr.io/gtk-rs/gtk-rs-core/core:latest

RUN dnf update -y && \
    dnf install xorg-x11-server-Xvfb procps-ng \
    dbus-devel libxkbcommon-devel wayland-devel wayland-protocols-devel mesa-libEGL-devel \
    libXi-devel libXrandr-devel libXcursor-devel libXdamage-devel libXinerama-devel libXtst-devel -y && \
    dnf clean all -y

RUN git clone https://gitlab.gnome.org/GNOME/at-spi2-atk.git --depth=1 && \
    (cd /at-spi2-atk && \
        meson setup builddir --prefix=/usr --buildtype release -Dtests=false -Datk:introspection=false -Dat-spi2-core:introspection=no && \
        meson install -C builddir) && \
    git clone https://gitlab.gnome.org/GNOME/gtk.git --depth=1 -b gtk-3-24 && \
    (cd /gtk && \
        meson setup builddir --prefix=/usr --buildtype release -Dintrospection=false -Dexamples=false -Dtests=false -Ddemos=false -Dlibepoxy:tests=false && \
        meson install -C builddir) && \
    rm -rf /at-spi2-atk /gtk
