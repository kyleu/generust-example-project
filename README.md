# generust-example-project

[![Build Status](https://travis-ci.org/generust-example-project/generust-example-project.svg?branch=master)](https://travis-ci.org/generust-example-project/generust-example-project)
[![Crate](https://meritbadge.herokuapp.com/generust-example-project)](https://crates.io/crates/generust-example-project)
[![Docs](https://docs.rs/generust-example-project/badge.svg)](https://docs.rs/generust-example-project)
[![Dependencies](https://deps.rs/repo/github/generust-example-project/generust-example-project/status.svg)](https://deps.rs/repo/github/generust-example-project/generust-example-project)

generust-example-project is an example application built using [generust](https://github.com/kyleu/generust)

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
