#![no_std]

//! USB Human Interface Device (HID) support for microcontrollers based on `usb-device`.
//!
//! This crate provides `Hid` structure for use with `usb-device` crate
//! as described by [https://www.usb.org/hid].
//!
//! See [usbd-hid-device-example](https://github.com/agalakhov/usbd-hid-device-example) for usage example.

mod hidclass;

pub use hidclass::{Hid, USB_CLASS_HID};

/// Trait for types that can be used as HID reports.
///
/// You have to implement this trait for your HID report structure.
/// It is wise to use macros to generate the `DESCRIPTOR` constant,
/// but it is still possible to write it manually.
///
/// # Example
/// ```
/// use usbd_hid_device::HidReport;
///
/// /// A mouse with three buttons and a wheel.
/// pub struct MouseReport {
///     bytes: [u8; 4],
/// }
/// # impl AsRef<[u8]> for MouseReport {
/// #    fn as_ref(&self) -> &[u8] {
/// #        &self.bytes
/// #    }
/// # }
///
/// impl HidReport for MouseReport {
///     const DESCRIPTOR: &'static [u8] = &[
///         0x05, 0x01,     // USAGE_PAGE Generic Desktop
///         0x09, 0x02,     // USAGE Mouse
///         0xa1, 0x01,     // COLLECTION Application
///             0x09, 0x01,     // USAGE Pointer
///             0xa1, 0x00,     // COLLECTION Physical
///
///                 0x05, 0x09,     // USAGE_PAGE Button
///                 0x19, 0x01,     // USAGE_MINIMUM Button 1
///                 0x29, 0x03,     // USAGE_MAXIMUM Button 3
///                 0x15, 0x00,     // LOGICAL_MINIMUM 0
///                 0x25, 0x01,     // LOGICAL_MAXIMUM 1
///                 0x95, 0x03,     // REPORT_COUNT 3
///                 0x75, 0x01,     // REPORT_SIZE 1
///                 0x81, 0x02,     // INPUT Data,Var,Abs
///                 0x95, 0x01,     // REPORT_COUNT 1
///                 0x75, 0x05,     // REPORT_SIZE 5
///                 0x81, 0x01,     // INPUT Cnst,Ary,Abs
///
///                 0x05, 0x01,     // USAGE_PAGE Generic Desktop
///                 0x09, 0x30,     // USAGE X
///                 0x09, 0x31,     // USAGE Y
///                 0x09, 0x38,     // USAGE Wheel
///                 0x15, 0x81,     // LOGICAL_MINIMUM -127
///                 0x25, 0x7f,     // LOGICAL_MAXIMUM 127
///                 0x75, 0x08,     // REPORT_SIZE 8
///                 0x95, 0x03,     // REPORT_COUNT 3
///                 0x81, 0x06,     // INPUT Data,Var,Rel
///
///             0xc0,           // END COLLECTION
///         0xc0,           // END COLLECTION
///     ];
/// }  
/// ```
pub trait HidReport: AsRef<[u8]> {
    /// HID report descriptor as byte array.
    ///
    /// The complete manual for HID report descriptors can be found at
    /// [https://www.usb.org/document-library/hid-usage-tables-112]
    const DESCRIPTOR: &'static [u8];
}
