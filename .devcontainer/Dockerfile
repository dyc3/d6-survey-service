FROM mcr.microsoft.com/devcontainers/base:bullseye

ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get -y update --no-install-recommends \
	&& apt-get -y install --no-install-recommends \
	build-essential \
	curl \
	ca-certificates \
	apt-utils \
	dialog \
	git \
	vim \
	libsqlite3-dev \
	libpq-dev \
	&& apt-get autoremove -y \
	&& apt-get clean -y

RUN curl -fsSL https://raw.githubusercontent.com/devcontainers/features/main/src/node/install.sh | bash -
RUN curl -fsSL https://raw.githubusercontent.com/devcontainers/features/main/src/rust/install.sh | bash -
RUN curl -fsSL https://raw.githubusercontent.com/devcontainers/features/main/src/github-cli/install.sh | bash -

ENV DEBIAN_FRONTEND=dialog

USER $USERNAME