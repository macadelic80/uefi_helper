use core::{ffi::c_void, ptr::null_mut};

use r_efi::{efi::{Event, Status, SystemTable}, protocols::serial_io::{Mode, Protocol, PROTOCOL_GUID}};


pub struct SerialIO {
    protocol: *mut Protocol,
}
impl SerialIO {
    pub fn new(st: *mut SystemTable) -> Result<SerialIO, Status> {
        let mut protocol: *mut Protocol = core::ptr::null_mut();
        let mut guid = PROTOCOL_GUID;
        let boot_services = unsafe{&mut *st}.boot_services;
        let status = unsafe {
            ((*boot_services).locate_protocol)(
                &mut guid,
                core::ptr::null_mut(),
                &mut protocol as *mut *mut Protocol as *mut _,
            )
        };
    
        if status == Status::SUCCESS {
            Ok(SerialIO {
                protocol,
            })
        } else {
            Err(status)
        }
    }

    ///Resets the serial device.
    pub fn reset(&self) -> Status {
        unsafe { ((*self.protocol).reset)(self.protocol) }
    }
    ///Sets the baud rate, receive FIFO depth, transmit/receive time out, parity, data bits, and stop bits on a serial device.
    pub fn set_attribute(
        &self,
        baud_rate: u64,
        receive_fifo_depth: u32,
        time_out: u32,
        parity: u32,
        data_bits: u32,
        stop_bits: u32
    ) -> Status {
        unsafe { ((*self.protocol).set_attribute)(self.protocol, baud_rate, receive_fifo_depth, time_out, parity, data_bits, stop_bits) }
    }

    ///Retrieves the status of the control bits on a serial device.
    pub fn get_control(&self) -> Result<u32, Status> {
        let mut control: u32 = Default::default();
        let status = unsafe { ((*self.protocol).get_control)(self.protocol, &mut control) };
        if status == Status::SUCCESS {
            Ok(control)
        } else {
            Err(status)
        }
    }

    ///Sets the control bits on a serial device.
    pub fn set_control(&self, control: u32) -> Status {
        unsafe { ((*self.protocol).set_control)(self.protocol, control) }
    }

    ///Writes data to a serial device.
    pub fn write(&self, buffer_size: *mut usize, buffer: *mut c_void) -> Status {
        unsafe { ((*self.protocol).write)(self.protocol, buffer_size, buffer) }
    }

    ///Read data from a serial device.
    pub fn read(&self, buffer_size: *mut usize) -> Result<*mut c_void, Status> {
        let buffer: *mut c_void = null_mut();
        let status = unsafe { ((*self.protocol).read)(self.protocol, buffer_size, buffer) };
        if status == Status::SUCCESS {
            Ok(buffer)
        } else {
            Err(status)
        }
    }

    pub fn mode(&self) -> *mut Mode {
        unsafe { (*self.protocol).mode }
    }

    pub fn revision(&self) -> u32 {
        unsafe { (*self.protocol).revision }
    }
    
}
