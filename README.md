# ascii-telnet-rust

The original Star Wars telnet server is currently down, so why not implement it in Rust? This server will open a TCP server on `0.0.0.0:23` which streams the original Star Wars ASCII movie over telnet.

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
$ # The will be available in ./target/release.
```

### Docker
```shell
$ # An image is available at `ghcr.io/gabe565/ascii-telnet-rust`
$ docker run --rm -it -p '23:23' ghcr.io/gabe565/ascii-telnet-rust
```
