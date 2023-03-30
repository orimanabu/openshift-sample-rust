# openshift-sample-rust

## Run with cargo

```
cargo build
cargo run
```

## Run with Podman

```
podman build -t localhost/openshift-sample-rust .
podman run -p 3000:3000 localhost/openshift-sample-rust
```

## Run on OpenShift

```
oc new-app https://github.com/orimanabu/openshift-sample-rust
```
