FROM debian
RUN DEBIAN_FRONTEND=noninteractive apt-get update && apt-get install -y \
    build-essential \
    sudo \
    curl \
    git

ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME

USER $USERNAME
WORKDIR /home/$USERNAME

# install devtools
# install pnpm
RUN curl -fsSL https://get.pnpm.io/install.sh | sh -
ENV PNPM_HOME="/home/$USERNAME/.local/share/pnpm"
ENV PATH="$PATH:$PNPM_HOME"

# install node
RUN pnpm env use --global lts

# install wrangler
RUN pnpm add --global @cloudflare/wrangler

# install cargo
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="$PATH:/${USERNAME}/.cargo/bin"

