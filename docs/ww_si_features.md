---
level: advanced
---

# Features of ww_si

Everything is a cargo feature, so a firmware compiles only what it uses. `std` (the default) enables all of them.

| Feature        | What                                                                                              |
|----------------|---------------------------------------------------------------------------------------------------|
| `time`         | `Second`, `Hertz`, `HertzPerSecond`, `Baud`, [`Timed<T>`](timed.md)                          |
| `electrical`   | `Ampere`, `Volt`, `Ohm`, `Farad`, `Siemens`, `Henry`, `Coulomb`, `Watt`, `Weber`, `Tesla`         |
| `fields`       | `ElectricFieldStrength`, `MagneticFieldStrength`, `CurrentDensity`, `ChargeDensity`, `Permittivity`, `Permeability`, `Resistivity`, `Conductivity`, .. |
| `mechanical`   | `Meter`, `Gram`, `Newton`, `Pascal`, `Joule`, `Torque`, `Litre`, `VolumetricFlow`, `Speed` .. `Yank` |
| `angle`        | `Radian`, `Degree`, `Steradian`, `AngularVelocity`, `AngularAcceleration` (rad is a dimension)    |
| `thermal`      | `Kelvin` (°C is a point, `origins::Celsius`)                                                      |
| `chemical`     | `Mole`, `Katal`                                                                                   |
| `photometric`  | `Candela`, `Lumen`, `Lux`                                                                         |
| `radiometric`  | `Irradiance`, `RadiantIntensity`, `Radiance`                                                      |
| `radiation`    | `Becquerel`, `Gray`, `Sievert`, `ElectronVolt`                                                    |
| `logarithmic`  | `DeciBel`, `DeciBelmW` (plain structs without a prefix)                                           |
| `dimensionless` | `Ratio`: duty, probability, scale (a group of its own, as it is used next to any other)          |
| `information`  | `Bit`, `Nibble` (4 bit), `Byte` (8 bit), `Halfword` (2 byte, 16 bit), `Word` (4 byte, 32 bit), `Doubleword` (8 byte, 64 bit) (bit is a dimension) |
| `counting`     | `Item`, `Sample`, `Frame`, `Packet`, `Line`, `Row`, `Record`, `Entry`, `Cycle`, `Block`, `Sector`, `Chunk`, `Char`, `Code`: counted bases |
| `quantities`   | all of the groups above                                                                           |
| `customary`    | imperial and US customary units (inch, pound, psi, ..), opt-in: not in `quantities` or `std`      |
| `nuclear`      | `Dalton`, `Barn`, `Curie`, `Rad`, `Rem`, `Roentgen`, named years, opt-in like `customary`         |
| `host`         | host helpers: [`Unwrapper`](timestamps.md)                                               |
| `u16`, `f32`, .. `all` | number types of `NumericValue`, passed on to `ww_numeric` (see [ww_numeric](ww_numeric.md)) |

```toml
# firmware with voltages and timestamps, stored as u16, u32 and f32
ww_si = { version = "0.5", default-features = false, features = ["time", "electrical", "u16", "u32", "f32"] }
```

The bytes on the wire are the same for every feature set.
