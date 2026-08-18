FROM ghcr.io/gtk-rs/gtk-rs-core/core:latest

RUN dnf update --assumeyes && \
    dnf --assumeyes install \
        atk-devel \
        gawk \
        gtk3-devel \
        procps-ng \
        python3-requests \
        wayland-devel \
        xorg-x11-server-Xvfb && \
    dnf clean all --assumeyes
