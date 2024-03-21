# syntax=docker/dockerfile:1
ARG PROJECT_NAME=axum
ARG APP_BIN_NAME=axum-example
ARG PORT=3000
ARG CONFIG_FILE=
ARG CONFIG_FIL_PATH=

ARG USER_NAME=${PROJECT_NAME}
ARG USER_HOME=/home/${USER_NAME}
ARG APP_BIN_PATH="/usr/local/bin/${APP_BIN_NAME}"

FROM --platform=$BUILDPLATFORM rust:slim AS builder

ARG BUILDPLATFORM
ARG TARGETPLATFORM

RUN apt update -qqqqy \
  && BUILD_OPTIONS=""; TARGET="" \
  && if [ "${TARGETPLATFORM}" != "${BUILDPLATFORM}" ]; then \
        case "${TARGETPLATFORM}" in \
            'linux/arm64') \
                apt install -qqqqy --no-install-recommends g++-aarch64-linux-gnu libc6-dev-arm64-cross \
                TARGET="aarch64-unknown-linux-gnu" \
                BUILD_OPTIONS="--target=${TARGET}" \
                ;; \
            'linux/amd64') \
                apt install -qqqqy --no-install-recommends g++-x86_64-linux-gnu libc6-dev-amd64-cross \
                TARGET="x86_64-unknown-linux-gnu" \
                BUILD_OPTIONS="--target=${TARGET}" \
                ;; \
            *) \
                ;; \
        esac \
 &&   rustup target add ${TARGET} \
 ;  fi \
 ; export BUILD_OPTIONS

FROM builder AS build-debug
COPY . /work
WORKDIR /work
RUN cargo build ${BUILD_OPTIONS}

FROM builder AS build-prod
COPY . /work
WORKDIR /work
RUN cargo build --release ${BUILD_OPTIONS}


FROM --platform=$TARGETPLATFORM buildpack-deps:curl AS base

ARG USER_NAME
ARG USER_HOME
ARG APP_BIN_NAME
ARG PORT
ARG APP_BIN_PATH
ARG CONFIG_FILE
ARG CONFIG_FIL_PATH

COPY run.sh /usr/local/bin/run.sh
COPY ${CONFIG_FILE} ${CONFIG_FIL_PATH}

RUN groupadd -g 1000 "${USER_NAME}" \
 && useradd -g 1000 -l -m -s /usr/bin/zsh -u 1000 "${USER_NAME}" \
 && chmod +x /usr/local/bin/run.sh

ENV PATH="${PATH}:/usr/local/bin"
ENV APP_BIN_PATH="${APP_BIN_PATH}"
ENV CONFIG_FIL_PATH="${CONFIG_FIL_PATH}"

USER ${USER_NAME}

WORKDIR ${USER_HOME}

HEALTHCHECK --interval=60s --retries=5 CMD curl --fail http://localhost/health:${PORT} || exit 1

EXPOSE ${PORT}

CMD [ "/usr/local/bin/run.sh" ]



FROM --platform=$TARGETPLATFORM base AS debug

ARG APP_BIN_PATH

COPY --from=build-debug /work/target/debug/${APP_BIN_NAME} ${APP_BIN_PATH}



FROM --platform=$TARGETPLATFORM base AS prod

ARG APP_BIN_PATH

COPY --from=build-prod /work/target/release/${APP_BIN_NAME} ${APP_BIN_PATH}

FROM --platform=$TARGETPLATFORM prod
