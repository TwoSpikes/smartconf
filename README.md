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

main.smf
```smartconf
foo: 'oo'
hsa: 'sbaa'
# this is a comment
bar: `bar`
baz: "baz"
```

```console
$ smartconf --format json main.smf > main.json
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
