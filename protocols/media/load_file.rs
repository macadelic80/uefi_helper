use core::ffi::c_void;

use r_efi::{
    efi::{Status, SystemTable},
    protocols::{device_path, load_file::{Protocol, PROTOCOL_GUID}}
};


pub struct LoadFile {
    protocol: *mut Protocol,
}
impl LoadFile {
    pub fn new(st: *mut SystemTable) -> Result<LoadFile, Status> {
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
            Ok(LoadFile {
                protocol,
            })
        } else {
            Err(status)
        }
    }
    ///Causes the driver to load a specified file.
    pub fn load_file(&self, file_path: *mut device_path::Protocol, boot_policy: bool, buffer_size: *mut usize, buffer: *mut c_void) -> Status {
        unsafe {((*self.protocol).load_file)(self.protocol, file_path, boot_policy.into(), buffer_size, buffer)}
    }    
}
