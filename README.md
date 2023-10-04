# nova
`nova_web` - a web server framework written in Rust

[![crates.io](https://img.shields.io/crates/v/nova_web?label=latest)](https://crates.io/crates/nova_web)
![downloads](https://img.shields.io/crates/d/nova_web.svg)
[![Documentation](https://docs.rs/nova_web/badge.svg?version=latest)](https://docs.rs/nova_web/latest)
![MIT or Apache 2.0 licensed](https://img.shields.io/crates/l/nova_web.svg) \
[![CI](https://github.com/pw-order-of-devs/nova/actions/workflows/default.yml/badge.svg)](https://github.com/pw-order-of-devs/nova/actions/workflows/default.yml)
[![Dependency Status](https://deps.rs/crate/nova_web/latest/status.svg)](https://deps.rs/crate/nova_web)

More information about this crate can be found in the [crate documentation][docs]. \
Examples of how to use the crate can be found in [examples][examples].

### Crate features

- Macro-free routing handling
- serde integration for parsing request body
- [todo] Handlers registered by macros
- [todo] OpenApi support
- custom Middleware integration
- [todo] custom Dependency Injection integration

[todo] Custom middleware include pre-implemented commonly used middlewares like [middleware..].\
Custom Dependency Injection provides a flexibility to control the dependencies like database connection or other external integrations.

### Usage example

in Cargo.toml:
```toml
[dependencies]
nova_web = "0.0.0"
tokio = "1.32"
serde = { version = "1.0", features = ["derive"] }
```

```rust
use nova_web::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
struct RequestBody {
    id: String,
    name: String,
}

fn hello_world(_: HttpRequest, res: HttpResponse) -> ServerResponse {
    Ok(res
        .status(HttpStatus::OK)
        .body("Hello, world!"))
}

fn hello_json(req: HttpRequest, res: HttpResponse) -> ServerResponse {
    let body: RequestBody = req.json()?;
    Ok(res
        .status(HttpStatus::OK)
        .body(&format!("Hello, {}!", body.name)))
}

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    Server::create("0.0.0.0", 3000)
        .service("/hello", vec![
            get("/", hello_world),
            post("/json", hello_json),
        ].into())
        .bind()
        .await
}
```

### Performance

[todo] prepare benchmarks

### Safety

This crate uses restrictive linter settings on top of `#![forbid(unsafe_code)]` to ensure safety and clearness of source code.

### Minimum supported Rust version

`MSRV` for `nova` is 1.63

### Examples

Usage examples can be found [here][examples]. The [docs] also include code snippets.\
[todo] More examples, snippets and information can be found on [wiki].

### License

The project is licensed under the [Apache license][license.apache] or [MIT license][license.mit]

### Contribution
Unless explicitly stated otherwise, any contribution intentionally submitted for inclusion in `nova`, shall be licensed as Apache or MIT, without any additional terms or conditions.

[examples]: https://github.com/pw-order-of-devs/nova/tree/main/examples
[docs]: https://docs.rs/nova_web
[wiki]: https://github.com/pw-order-of-devs/nova/wiki
[discussion]: https://github.com/pw-order-of-devs/nova/discussions/new?category=q-a
[license.apache]: https://github.com/pw-order-of-devs/nova/blob/main/nova/LICENSE-APACHE
[license.mit]: https://github.com/pw-order-of-devs/nova/blob/main/nova/LICENSE-MIT
