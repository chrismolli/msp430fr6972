# msp430fr6972 ![Crates.io](https://img.shields.io/crates/v/msp430fr6972)
Peripheral Access Crate (PAC) for TI MSP430FR6972 created using [msp430_svd](https://github.com/pftbest/msp430_svd) and [svd2rust](https://github.com/rust-embedded/svd2rust).

## Memory Layout
Use the following definitions in your project's `memory.x`.

```
MEMORY{
  RAM : ORIGIN = 0x1C00, LENGTH = 0x0800
  ROM : ORIGIN = 0x4400, LENGTH = 0xBB80
  VECTORS : ORIGIN = 0xFF90, LENGTH = 0x70
}
```
