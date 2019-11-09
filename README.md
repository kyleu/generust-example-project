# Generust

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](License)

## A Rust project template for dynamic web applications

`Generust` is a `cargo-generate` template that provides a Rust web server and WASM client with some [interesting features](doc/features.md).

It uses [actix-web](https://actix.rs), [maud](https://maud.lambda.xyz), [UIKit](https://getuikit.com), and [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) to serve your app as a server or webview, via HTTP and WebSockets.

## Use `cargo generate` to clone this template and make it your own

```cargo install cargo-generate
cargo generate --git https://github.com/kyleu/generust.git --name my-project
cd my-project
./bootstrap
```

This will package the WASM client, compile the UIKit SCSS, and build the main application.
You can execute `cargo run` to start the server and open a system webview pointing to it, or `cargo run -- --help` to see the CLI options.
[Scripts](doc/scripts.md) are provided in `./bin` that will help you build and publish the app.

## Example Projects

[rustimate](https://github.com/kyleu/rustimate)

After you've created your project, the variables after this line will be replaced with your project's information.

# generust-example-project

[![Build Status](https://travis-ci.org/generust-example-project/generust-example-project.svg?branch=master)](https://travis-ci.org/generust-example-project/generust-example-project)
[![Crate](https://meritbadge.herokuapp.com/generust-example-project)](https://crates.io/crates/generust-example-project)
[![Docs](https://docs.rs/generust-example-project/badge.svg)](https://docs.rs/generust-example-project)
[![Dependencies](https://deps.rs/repo/github/generust-example-project/generust-example-project/status.svg)](https://deps.rs/repo/github/generust-example-project/generust-example-project)

generust-example-project is a work in progress...

Running as a client application or shared server, `generust-example-project` has a focus on performance, correctness, and developer comfort.

See [installing.md](doc/installing.md) for installation guidance.

See [scripts.md](doc/scripts.md) for available tools for building, running, and packaging the app.

## Crates

`generust-example-project` splits its code into several library crates:

- `generust-example-project-assets`: Contains embedded static files intended to be served from the web application
- `generust-example-project-client`: Run in the client's browser as a WebAssembly package, includes templates
- `generust-example-project-controllers`: Contains actix-web HTTP controllers, usually calling methods from `generust-example-project-service`
- `generust-example-project-core`: Contains definitions that are shared between server and client
- `generust-example-project-service`: Contains the primary logic for the application. It receives RequestMessages and emits ResponseMessages
- `generust-example-project-templates`: Contains Maud templates used by the server to render responses
- `generust-example-project`: Stored in the root of the project, this is the app's main library and binary

## Config Directory

By default, generust-example-project stores config files in your system's user configuration directory.

- macOS: ~/Library/Application Support/generust-example-project
- Linux: ~/.config/generust-example-project
- Windows: %APPDATA%/generust-example-project/generust-example-project
