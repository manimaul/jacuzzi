# Jacuzzi (SPA Web Server)

Jacuzzi is a simple (http & http2) web server for serving SPA (Single Page Applications).
Jacuzzi is built with [Rust](https://www.rust-lang.org/) on top of [Actix Web](https://actix.rs/).

![Image of a jacuzzi](sample/images/tub.jpg)

Run with [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
```bash
cargo run -- sample/
```

Run with Docker
```bash
docker run -p 8088:8088 manimaul/jacuzzi /var/www/sample 
```

Build with Docker
```bash
docker build -t manimaul/jacuzzi .
```
