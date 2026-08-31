# Prieto-buffers

**Prieto-buffers** is a minimal serialization protocol designed for embedded systems. It is built with `no_std` support and focuses on simplicity, deterministic layout, and low binary overhead.

In addition to fixed-size types, the crate supports dynamically sized data such as fixed-size arrays (`[T; N]`), `Vec<T>` and `String`, including zero-terminated ("C string" style) encoding.

---

## Features

- `no_std` compatible
- Minimal binary format
- Fixed-size type serialization
- Support for `[T; N]`, `Vec<T>` and `String`
- Optional zero-ended (null-terminated) encoding for byte arrays/vectors/strings via `#[zero_ended]`
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

### Arrays, vectors and strings

Fixed-size arrays (`[T; N]`) and dynamically sized `Vec<T>` and `String` are both serialized as an `Array` field, prefixed with the number of elements and their type.

By default, arrays/vectors are serialized in full (all `N` elements for arrays, or the full length for vectors). Marking a byte array/vector/string field with `#[zero_ended]` instead encodes it like a C string: only the bytes up to (and including) the first `0` byte are written, which can save space when the buffer is mostly padding.

```rust
#[derive(PrietoBuffersSerde)]
struct TestStruct {
    #[zero_ended]
    name: [u8; 64],
    payload: Vec<u8>,
}
```

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

- Field ID range limited to 0–32
---

## Design goals

The goal of Prieto-buffers is to provide a small, predictable, embedded-friendly serialization format that avoids the complexity and overhead of larger frameworks while still allowing basic schema evolution. We target low payload communication protocols like CAN and CAN-FD.
