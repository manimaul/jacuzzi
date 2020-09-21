# Jacuzzi (SPA Web Server)

Jacuzzi is a simple (http & http2) web server for serving SPA (Single Page Applications).
Jacuzzi is built with [Rust](https://www.rust-lang.org/) on top of [Actix Web](https://actix.rs/).

![Image of a jacuzzi](sample/images/tub.jpg)

Run with [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
```bash
TLS_DIR="$(pwd)/self_signed_cert" cargo run -- sample/
```

Install with Cargo
```bash
cargo install --path . 
```

Run (After installing 😀)
```
TLS_DIR="$(pwd)/self_signed_cert" jacuzzi sample/
```

Run with Docker
```bash
docker run -p 8443:8443 manimaul/jacuzzi:0.1.1 /var/www/sample 
```

Build with Docker
```bash
./build_container_image.sh
```
