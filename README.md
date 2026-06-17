# jelly-lib

`jelly-lib` is a Rust support library for the Jelly FPGA platform.

It builds on `jelly-mem_access` for low-level memory-mapped I/O access and provides helpers for:

- I2C access abstractions
- IMX219 sensor control
- video DMA PAC bindings
- video format regularizer PAC bindings

## Features

- `std` is enabled by default
- `no_std` builds are supported for the core modules
- `linux_i2c` is available only with the `std` feature

## Modules

- `i2c_hal`
- `imx219_sensor_driver`
- `video_dma_pac`
- `video_format_regularizer_pac`
- `linux_i2c` (std only)

## License

MIT