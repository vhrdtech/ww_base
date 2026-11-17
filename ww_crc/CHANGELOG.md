# ww_crc changelog

## Unreleased

### ⚠️ Breaking

- Repository: ww_crc lives in [vhrdtech/ww_base](https://github.com/vhrdtech/ww_base) with its history; take it by git from there, not from wire_weaver.

### 🚀 Features

- New crate: the CRCs of `ww_framer`, `ww_link`, `ww_block`, `ww_settings` and `ww_stream` as statics (`CRC32_ISO_HDLC`, `CRC32C`, `CRC16_USB`, `CRC16_IBM_SDLC`, `CRC8_SMBUS`), so a firmware links one table per algorithm, its size picked once with `table-256` or `table-slice16` (bitwise without) (TR-31).
