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
foo: 'oo'
hsa: 'sbaa'
bar: `bar`
baz: "baz"
```

```console
$ smartconf --format json -o main.json main.smc
```

main.json
```
{
    "foo": "oo",
    "hsa": "sbaa",
    "bar": "bar",
    "baz": "baz",
}
```

> [!Warning]
> It currently can not even do this thing shown above
