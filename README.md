# ascii-telnet-rust

[![Build](https://github.com/gabe565/ascii-telnet-rust/actions/workflows/docker.yml/badge.svg)](https://github.com/gabe565/ascii-telnet-rust/actions/workflows/docker.yml)
[![Artifact Hub](https://img.shields.io/endpoint?url=https://artifacthub.io/badge/repository/gabe565)](https://artifacthub.io/packages/helm/gabe565/ascii-telnet)

The original Star Wars telnet server is currently down, so why not implement it in Rust? This server will open a TCP server on `0.0.0.0:23` which streams the original Star Wars ASCII movie over telnet.

See it in action by running `telnet gabecook.com` or `nc gabecook.com 23`.

<p align="center">
  <a href="https://asciinema.org/a/431278"><img src="https://asciinema.org/a/431278.svg"/></a>
</p>

## Running

The app supports building locally or in a Docker container at `ghcr.io/gabe565/ascii-telnet-rust`.

### Local
```shell
$ # To build and run in one step
$ cargo run 
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/ascii-telnet`
[2021-08-05T23:14:37Z INFO  ascii_telnet] Listening on 0.0.0.0:23
$ # You can now run `telnet localhost` to see the movie.
$
$ # To get a release binary:
$ cargo build --release
$ # The binary will be available in ./target/release.
```

### Docker
```shell
$ # An image is available at `ghcr.io/gabe565/ascii-telnet-rust`
$ docker run --rm -it -p '23:23' ghcr.io/gabe565/ascii-telnet-rust
```

### Kubernetes

A Helm chart is available for Kubernetes deployment.
For more information, go to
[Artifact Hub](https://artifacthub.io/packages/helm/gabe565/ascii-telnet) or
[gabe565/charts](https://github.com/gabe565/charts/tree/main/charts/ascii-telnet).
