FROM registry.access.redhat.com/ubi9/ubi:latest
COPY target/release/openshift-sample-rust /
ENTRYPOINT /openshift-sample-rust
