FROM node:15-buster-slim as base


FROM base as development
RUN apt-get update && apt-get install -y \
    sudo \
    build-essential \
    nodejs \
    npm \
    curl \
    git \
    python3 \
    make \
    g++ \
    && apt-get clean

ARG USERNAME=node
ARG USER_UID=1000
ARG USER_GID=$USER_UID
# RUN groupadd --gid $USER_GID $USERNAME \
#     && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
#     && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
#     && chmod 0440 /etc/sudoers.d/$USERNAME
run groupmod --gid $USER_GID $USERNAME \
    && usermod --uid $USER_UID --gid $USER_GID $USERNAME \
    && chown -R $USER_UID:$USER_GID /home/$USERNAME
USER $USERNAME

# install devtools
ENV NPM_CONFIG_PREFIX=~/.npm-global
RUN echo "export PATH=~/.npm-global/bin:$PATH" >> ~/.profile
RUN npm i -g @vue/cli @cloudflare/wrangler

# install cargo
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

