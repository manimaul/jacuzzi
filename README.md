# Jacuzzi (SPA Web Server)

Jacuzzi is a simple (http & http2) web server for serving SPA (Single Page Applications).
Jacuzzi is built with [Rust](https://www.rust-lang.org/) on top of [Warp](https://crates.io/crates/warp).

![Image of a jacuzzi](sample/images/tub.jpg)

### Why is Jacuzzi good for Serving SPA (Single Page Applications)
Jacuzzi is configured to return the `index.html` file for any GET request that would normally return a 404 (not found). 
You can point Jacuzzi at a directory with `index.html` in it's root and it will serve all content `inside` that directory 
normally. Any unknown GET request path that would normally 404 will instead 200 with the content of `index.html`. 
This allows the router to detect the route and serve the correct content.

* [yew-router](https://github.com/yewstack/yew_router#how-it-works)
* [react-router](https://github.com/ReactTraining/react-router/blob/v3/docs/guides/Histories.md#configuring-your-server)

------

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
docker run -p 8443:8443 manimaul/jacuzzi:0.2.1 /var/www/sample 
```

Build with Docker
```bash
./build_container_image.sh
```
