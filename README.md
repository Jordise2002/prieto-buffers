# Prieto-buffers

**Prieto-buffers** is a minimal serialization protocol designed for embedded systems. It is built with `no_std` support and focuses on simplicity, deterministic layout, and low binary overhead.

At the moment, the crate supports only fixed-size types and does not include support for dynamically sized data (such as `String` or `Vec<T>`).

---

## Features

- `no_std` compatible
- Minimal binary format
- Fixed-size type serialization
- Struct-based derivation via `#[derive(PrietoBuffersSerde)]`
- Optional field identifiers for flexible schemas

---

## Example

```rust
#[derive(PrietoBuffersSerde)]
struct TestInnerStruct {
    x: u8,
    y: i8,
}

#[derive(PrietoBuffersSerde)]
struct TestStruct {
    #[field_id(3)]
    a: u8,
    b: i8,
    c: bool,
    #[field_id(0)]
    f: TestInnerStruct,
    d: u16,
    e: i16,
}
```

## How it works

Prieto-buffers serializes each struct as a sequence of fields, each preceded by a compact header.

### Field encoding

Each field is serialized in little-endian format and is preceded by a field header, which encodes:

- Field ID (0–32)
- Field type (see `FieldType`)

This allows the deserializer to correctly interpret each field even if the order changes between versions.

### Struct encoding

When a struct is serialized, an additional prefix is written containing:

- The number of fields in the struct

This helps the deserializer iterate over the serialized data efficiently.

---

## Schema compatibility

Prieto-buffers is designed to be forward- and backward-compatible as long as:

- Field IDs match between versions
- Field types remain consistent for each field ID

This allows:
- Safe reordering of fields
- Ignoring unknown fields during deserialization

---

## Limitations

- No support for dynamically sized types (`String`, `Vec<T>`, etc.)
- Field ID range limited to 0–32
---

## Design goals

The goal of Prieto-buffers is to provide a small, predictable, embedded-friendly serialization format that avoids the complexity and overhead of larger frameworks while still allowing basic schema evolution. We target low payload communication protocols like CAN and CAN-FD.
