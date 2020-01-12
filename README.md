# USB HID device support for embedded devices

This crate provides minimalistic USB HID support for microcontrollers
containing USB peripheral. It works with any microcontroler with 
[usb-device](https://crates.io/crates/usb-device) support in HAL.

The support for HID provided by this crate is *type-safe*. Since USB HID
requires complex report descriptors for all reports, only sending of structures
that have associated HID report descriptor is permitted.

There is complete usage example in [usb-hid-device-example](https://github.com/agalakhov/usbd-hid-device-example).
