# Diesel Enum Derive Macro

## Overview

A Rust procedural macro library that provides custom derive macros `FromSqlDerive` and `ToSqlDerive` for seamless PostgreSQL enum serialization and deserialization with Diesel ORM.

## Features

- Automatically implement `FromSql` and `ToSql` traits for PostgreSQL enum types
- Simple attribute-based configuration
- Works with unit variants of enums
- Supports custom SQL type specification

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
diesel-enum-sqltype = "0.1.6"
diesel = { version = "x.x.x", features = ["postgres"] }
```

## Usage

```rust
#[derive(FromSqlDerive, ToSqlDerive)]
#[diesel(sql_type = "PaymentTypeT")] //imported from schema.rs
enum PaymentType {
    Card,
    Bank,
    Wire,
}
```

## Limitations

- Only supports enums with unit variants
- Requires explicit `sql_type` attribute

## License

[MIT](LICENSE)

## Contributing

Contributions welcome! Please open an issue or submit a pull request.
