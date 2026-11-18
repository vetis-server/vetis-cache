# vetis-cache

[![Crates.io downloads](https://img.shields.io/crates/d/vetis-cache)](https://crates.io/crates/vetis-cache) [![crates.io](https://img.shields.io/crates/v/vetis-cache?style=flat-square)](https://crates.io/crates/vetis-cache) [![Build Status](https://github.com/vetis-server/vetis-cache/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/vetis-server/vetis-cache/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/vetis-cache) [![Documentation](https://docs.rs/vetis-cache/badge.svg)](https://docs.rs/vetis-cache/latest/vetis-cache) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/vetis-server/vetis-cache/blob/main/LICENSE.md)  [![codecov](https://codecov.io/gh/vetis-server/vetis-cache/graph/badge.svg?token=T0HSBAPVSI)](https://codecov.io/gh/vetis-server/vetis-cache)

Cache support for the vetis HTTP server framework. This module provides efficient caching.

## Features

- Cache content


## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
vetis-cache = "0.1.0"
```

### Runtime Features

Choose the appropriate runtime feature for your project:

- `runtime-tokio` (default): Use with Tokio runtime
- `runtime-smol`: Use with Smol runtime

## Configuration

### Using API

Configure cache hook using the builder pattern:

```rust,ignore
use vetis_cache::CacheHookConfig;

let config = CacheHookConfig::builder()
    .build()?;
```

### vetis.yaml

```yaml
...
name: cache
```

### Configuration Options

#### StaticPathConfig

- **matcher**: If set, this hook will be only called if request meet the criteria defined by him, otherwise, all requests will call this hook (optional)

## Runtime-Specific Modules

Based on your selected runtime feature, import the appropriate module:

```rust
// For Tokio runtime
#[cfg(feature = "runtime-tokio")]
use vetis_cache::tokio;

// For Smol runtime
#[cfg(feature = "runtime-smol")]
use vetis_cache::smol;
```

## License

Licensed under either of

- Apache License, Version 2.0
  (LICENSE-APACHE or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  (LICENSE-MIT or <https://opensource.org/licenses/MIT>)

at your option.

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
