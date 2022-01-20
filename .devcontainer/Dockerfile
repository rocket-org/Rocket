# Merged with devcontainer from Microsoft VSCode
# https://github.com/microsoft/vscode-dev-containers/tree/v0.212.0/containers/alpine/.devcontainer

ARG VARIANT="3.15"
FROM alpine:${VARIANT}

ARG USERNAME=rocket
ARG USER_UID=10000
ARG USER_GID=$USER_UID
ARG INSTALL_ZSH="false"

ARG ROCKET_MSRV="1.53.0"
ARG ROCKET_REPO="https://github.com/rocket-org/Rocket.git"
ARG ROCKET_VOLUME="/home/$USERNAME/Rocket"

COPY library-scripts/*.sh library-scripts/*.env /tmp/library-scripts/

RUN echo "Updating packages..." \
    && apk update --no-cache \
    && echo "Executing common-alpine.sh..." \
    && ash /tmp/library-scripts/common-alpine.sh "${INSTALL_ZSH}" "${USERNAME}" "${USER_UID}" "${USER_GID}" \
    && rm -rf /tmp/library-scripts \
    && usermod -p $(echo "$RANDOM-x-$RANDOM-x-$RANDOM-x-$RANDOM-x-$RANDOM" | hexdump -e '"%02x"') "${USERNAME}" \
    && echo "" > /etc/motd \
    && echo "Installing required packages..." \
    && apk add -U --no-cache bash build-base cmake curl-dev lldb mariadb-dev \
            musl-dev openssh-server openssh-keygen \
            openssl-dev perl pkgconf-dev postgresql-dev python3 \
    && sed -i -e 's/^[#]Port .*$/Port 2222/' \
            -e 's/^[#]PermitRootLogin .*$/PermitRootLogin no/' \
            -e 's/^[#]PasswordAuthentication .*$/PasswordAuthentication no/' \
            /etc/ssh/sshd_config \
    && mkdir "${ROCKET_VOLUME}" \
    && chown $USER_UID:$USER_GID "${ROCKET_VOLUME}" \
    && chmod 0755 "${ROCKET_VOLUME}" \
    && umask 077 && for ciph in rsa ecdsa ed25519 ; do \
        ssh-keygen -f /etc/ssh/ssh_host_${ciph}_key -t "$ciph" -N "" || exit 1 ; \
        done \
    && exit 0

EXPOSE 2222
CMD ["sudo", "/usr/sbin/sshd", "-D"]

USER rocket
WORKDIR /home/rocket

RUN echo "Installing rustup..." \
    && curl -sSf -o /tmp/rustup-init https://sh.rustup.rs \
    && sh /tmp/rustup-init -y -c rust-docs,rust-src,rust-analysis,clippy \
    && rm /tmp/rustup-init \
    && source $HOME/.cargo/env \
    && echo "Installing rust beta..." \
    && rustup toolchain install beta \
    && echo "Installing rust nightly..." \
    && rustup toolchain install --force nightly \
    && echo "Installing rust MSRV ${ROCKET_MSRV}..." \
    && rustup toolchain install "${ROCKET_MSRV}" \
    && echo "Checking rust installation..." \
    && rustc --version && cargo --version \
    && rustc +beta --version && cargo +beta --version \
    && rustc +nightly --version && cargo +nightly --version \
    && rustc +${ROCKET_MSRV} --version && cargo +${ROCKET_MSRV} --version \
    && rustup default ${ROCKET_MSRV} \
    && echo "This needs now some time here... please be patient..." \
    && exit 0

RUN echo "Cloning Rocket Github repository..." ; \
    [ -d "${ROCKET_VOLUME}/.git" ] \
    && echo "Directory \"${ROCKET_VOLUME}\" exists... skipping clone." ; \
    [ ! -d "${ROCKET_VOLUME}/.git" ] \
    && { git clone "${ROCKET_REPO}" || exit 1 ; } ; \
    exit 0

VOLUME /home/rocket/Rocket

RUN echo "Installing tools with cargo..." \
    && source $HOME/.cargo/env \
    || exit 1 ; \
    echo "Cargo config crt-static = false" \
    && echo -e "\n[target.x86_64-unknown-linux-musl]\nrustflags = [\"-C\", \"target-feature=-crt-static\"]\n" >> $HOME/.cargo/config.toml \
    || exit 1 ; \
    no_sccache_cleanup=0 ; \
    [ -d "$HOME/.cache/sccache" ] \
    && no_sccache_cleanup=1 ; \
    cargo +stable install sccache \
    && echo "Config sccache..." \
    && echo -e "\n[build]\nrustc-wrapper = \"$HOME/.cargo/bin/sccache\"\n" >> $HOME/.cargo/config.toml \
    && echo "Cargo installing..." \
    && cargo +stable install cargo-audit cargo-binutils cargo-cache cargo-crev cargo-duplicates \
            cargo-expand cargo-license cargo-msrv cargo-outdated cargo-trim cargo-update \
            drill fd-find flamegraph sd \
    && echo "Cleanup..." \
    && cargo +stable cache -a \
    || exit 1 ; \
    [ "$no_sccache_cleanup" -ne 1 ] \
    && rm -r $HOME/.cache/sccache/* \
    || exit 1 ; \
    exit 0

VOLUME /home/rocket/.cache/sccache

RUN ssh-keygen -t ed25519 -f ~/.ssh/id_ed25519 \
    && cat ~/.ssh/id_ed25519.pub >> ~/.ssh/authorized_keys \
    && echo "" \
    && echo "============================================================================" \
    && echo "============== Login with user $USERNAME and this openssh key ==============" \
    && cat ~/.ssh/id_ed25519 \
    && echo "============================================================================" \
    && exit 0
