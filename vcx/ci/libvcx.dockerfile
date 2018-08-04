FROM libindy
#
# cargo deb for debian packaging of libvcx
RUN cargo install cargo-deb --color=never
COPY . /sdk

ENV PATH=${PATH}:/sdk/vcx/ci/scripts


WORKDIR /sdk/vcx/libvcx

RUN cargo update-version && \
    cargo test --color=never --no-default-features --features "ci sovtoken" -- --test-threads=1 && \
    cargo update-so && \
    cargo deb --no-build && \ 
    find -type f -name "libvcx*.deb" -exec dpkg -i {} \;

WORKDIR /sdk/vcx/wrappers/node/

RUN npm ci

WORKDIR /sdk/vcx/libvcx
