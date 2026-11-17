## Unreleased

### ⚠️ Breaking

- Repository: ww_keyed lives in [vhrdtech/ww_base](https://github.com/vhrdtech/ww_base) with its history; take it by git from there, not from wire_weaver.

### 🚀 Features

* New crate (API-67): `KeyEvent<I, K>` (`Added { index, key }`, `Removed { index }`), the item of the `keys`
  stream of a `keyed!` array. An API crate with `keyed!`
  depends on it.
