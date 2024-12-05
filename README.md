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

## Simple example

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
```json
{
    "foo": "oo",
    "hsa": "sbaa",
    "bar": "bar",
    "baz": "baz"
}
```

## Setting variable name

```console
$ smartconf --format vim -N foo main.smf > main.vim
```

main.vim
```vim
let g:foo = {
\   'foo': "oo",
\   'hsa': "sbaa",
\   'bar': "bar",
\   'baz': "baz",
\}
```

## Include

main.smf
```smartconf
include 'second.smf'
foo: 'foo'
```

second.smf
```smartconf
bar: 'baz'
```

```console
$ smartconf --format json main.smf > main.json
```

main.json
```json
{
    "foo": "foo",
    "bar": "baz"
}
```
