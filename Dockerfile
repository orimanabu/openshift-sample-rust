FROM quay.io/centos/centos:stream9 as builder
RUN dnf install -y rust cargo git
RUN git clone https://github.com/orimanabu/openshift-sample-rust
RUN cd openshift-sample-rust && cargo build --release

FROM quay.io/centos/centos:stream9
COPY --from=builder openshift-sample-rust/target/release/openshift-sample-rust /
ENTRYPOINT /openshift-sample-rust
