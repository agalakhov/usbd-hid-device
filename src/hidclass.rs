//! USB HID class definitions.

use super::HidReport;
use core::marker::PhantomData;
use usb_device::{class_prelude::*, Result};

/// This should be used as `device_class` when building the `UsbDevice`.
pub const USB_CLASS_HID: u8 = 0x03;

/// The bInterfaceSubClass member declares whether a device supports a boot
/// interface, otherwise it is 0.
const USB_SUBCLASS_HID: u8 = 0x00;
/// The bInterfaceProtocolmember of an Interface descriptor only has meaning
/// if the bInterfaceSubClassmember declares that the device supports a boot
/// interface, otherwise it is 0.
const USB_PROTOCOL_HID: u8 = 0x00;

const HID_VER: [u8; 2] = [0x10, 0x01];
const HID_COUNTRY_NONE: u8 = 0x00;

const DT_HID: u8 = 0x21;
const DT_REPORT: u8 = 0x22;

/// USB-HID device.
///
/// This is the type-safe driver for human interace devices. To use it, you have to
/// provide some structure for HID report with its corresponding HID report descriptor.
///
/// # Example
/// ```
/// use stm32f3xx_hal::usb::UsbBus;
/// use usbd_hid_device::Hid;
/// # use stm32f3xx_hal::usb::Peripheral;
/// # use usbd_hid_device::HidReport;
///
/// # return; // don't run this test on a x86
/// # let usb: Peripheral = unimplemented!();
/// let usb_alloc = UsbBus::new(usb);
/// let hid = Hid::<MouseReport, _>::new(&usb_alloc, 10);
///
/// # struct MouseReport;
/// # impl AsRef<[u8]> for MouseReport {
/// #    fn as_ref(&self) -> &[u8] {
/// #        unimplemented!()
/// #    }
/// # }
/// # impl HidReport for MouseReport {
/// #    const DESCRIPTOR: &'static [u8] = &[];
/// # }
/// ```
pub struct Hid<'a, R: HidReport, B: UsbBus> {
    data_if: InterfaceNumber,
    write_ep: EndpointIn<'a, B>,
    _report: PhantomData<&'a R>,
}

impl<'a, R: HidReport, B: UsbBus> Hid<'a, R, B> {
    /// Create a new `Hid` for the given USB allocator.
    ///
    /// `poll_ms` is the period of host poloing for the reports, milliseconds.
    /// Lower values mean lower latency but require more bandwidth of USB bus
    /// which may conflict with other devices on the same hub.
    /// Values around 10 are reasonable for most devices.
    pub fn new(alloc: &'a UsbBusAllocator<B>, poll_ms: u8) -> Self {
        Self {
            data_if: alloc.interface(),
            write_ep: alloc.interrupt(64, poll_ms),
            _report: PhantomData,
        }
    }

    /// Send HID report.
    ///
    /// This function sends HID report to the host as soon as possible.
    /// It converts report to bytes using `AsRef<[u8]>`. Result of the
    /// conversion MUST match the format described in `R::DESCRIPTOR`.
    pub fn send_report(&mut self, report: &R) -> Result<usize> {
        self.write_ep.write(report.as_ref())
    }
}

impl<R: HidReport, B: UsbBus> Hid<'_, R, B> {
    fn get_descriptor(&self, xfer: ControlIn<B>) {
        let (ty, idx) = xfer.request().descriptor_type_index();
        if ty == DT_REPORT && idx == 0 {
            xfer.accept_with_static(R::DESCRIPTOR).ok();
        }
    }
}

impl<R: HidReport, B: UsbBus> UsbClass<B> for Hid<'_, R, B> {
    fn get_configuration_descriptors(&self, writer: &mut DescriptorWriter) -> Result<()> {
        writer.interface(
            self.data_if,
            USB_CLASS_HID,
            USB_SUBCLASS_HID,
            USB_PROTOCOL_HID,
        )?;

        writer.write(
            DT_HID,
            &[
                HID_VER[0], HID_VER[1], // bcdHID
                HID_COUNTRY_NONE,       // bCountryCode
                0x01,                   // bNumDescriptors
                DT_REPORT,              // bDescriptorType
                R::DESCRIPTOR.len() as u8,
                (R::DESCRIPTOR.len() >> 8) as u8, // wDescriptorLength
            ],
        )?;

        writer.endpoint(&self.write_ep)?;

        Ok(())
    }

    fn control_in(&mut self, xfer: ControlIn<B>) {
        let req = xfer.request();

        if !(req.recipient == control::Recipient::Interface
            && req.index == u8::from(self.data_if) as u16)
        {
            return;
        }

        match (req.request_type, req.request) {
            (control::RequestType::Standard, control::Request::GET_DESCRIPTOR) => {
                self.get_descriptor(xfer)
            }
            _ => (),
        }
    }
}
