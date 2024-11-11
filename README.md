# smartconf

Configurable configuration reader in Rust

> [!Warning]
> It is currently in alpha state

# Requirements

- `rustc`
- `cargo` (not necessarily)

# Installation

```console
$ cargo install --path=.
```

# Usage

main.smc
```smartconf
foo: 1
bar: 2
baz: 3
```

```console
$ smartconf --format json -o main.json main.smc
```

main.json
```
{
    "foo": 1,
    "bar": 2,
    "baz": 3,
}
```

> [!Warning]
> It currently can not even do this thing shown above
