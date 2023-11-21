FROM mcr.microsoft.com/devcontainers/rust:1

RUN export DEBIAN_FRONTEND=noninteractive && \
    apt-get update -q && \
    apt-get install -y --no-install-recommends \
        curl \
        jq \
        git \
        sudo \
        zsh \
        build-essential \
        openssl

USER vscode