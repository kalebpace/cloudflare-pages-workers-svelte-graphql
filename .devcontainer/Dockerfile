FROM node:15-buster-slim as base

FROM base as development
RUN apt-get update && apt-get install -y \
    sudo \
    git \
    curl \
    python3 \
    make \
    g++ \
    && apt-get clean

ARG USERNAME=node
ARG USER_UID=1000
ARG USER_GID=$USER_UID
RUN groupmod --gid $USER_GID $USERNAME \
    && usermod --uid $USER_UID --gid $USER_GID $USERNAME \
    && chown -R $USER_UID:$USER_GID /home/$USERNAME
USER $USERNAME

# install devtools
ENV NPM_CONFIG_PREFIX=~/.npm-global
RUN echo "export PATH=~/.npm-global/bin:$PATH" >> ~/.profile
RUN npm i -g @vue/cli @cloudflare/wrangler
