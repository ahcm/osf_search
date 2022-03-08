# osf_search
OSF Search example frontend

# How to run

Install Rust.

Run:
```
$ cargo run --release
    Finished release [optimized] target(s) in 0.22s
     Running `target/release/osf_search`
🔧 Configured for production.
    => address: 0.0.0.0
    => port: 8000
    => log: critical
    => workers: 16
    => secret key: generated
    => limits: forms = 32KiB
    => keep-alive: 5s
    => read timeout: 5s
    => write timeout: 5s
    => tls: disabled
Warning: environment is 'production', but no `secret_key` is configured
🚀 Rocket has launched from http://0.0.0.0:8000
```
Then browse to http://127.0.0.1:8000

## Optional Rocket configuration
To configure the included rocket webserver create a file Rocket.toml.

See more on Rocket.toml and startup options:

https://rocket.rs/v0.4/guide/configuration/

E.g.

```toml
[production]
address = "127.0.0.1"
port = 7777
workers = 12
keep_alive = 5
log = "critical"
secret_key = "XXX"
```

Where you replace **XXX** in secret_key with e.g the output of:
```
$ openssl rand -base64 32
```

