use core::ffi::c_void;

use lib_efi::{efi::{Status, SystemTable}, protocols::simple_text_input_ex::{KeyData, KeyNotifyFunction, Protocol, PROTOCOL_GUID}};

pub struct TextInputEx {
    protocol: *mut Protocol,
}

pub enum ToggleState {
    ToggleStateValid = 0x80,
    KeyStateExposed  = 0x40,
    ScrollLockActive = 0x01,
    NumLockActive    = 0x02,
    CapsLockActive   = 0x04,
}
impl TextInputEx {
    pub fn new(st: *mut SystemTable) -> Result<TextInputEx,Status> {
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
            Ok(TextInputEx {
                protocol,
            })
        } else {
            Err(status)
        }
    }
    ///The Reset() function resets the input device hardware.
    ///The implementation of Reset is required to clear the contents of any input queues resident in memory used for buffering keystroke data and put the input stream in a known empty state.
    ///As part of initialization process, the firmware/device will make a quick but reasonable attempt to verify that the device is functioning. If the ExtendedVerification flag is TRUE the firmware may take an extended amount of time to verify the device is operating on reset. Otherwise the reset operation is to occur as quickly as possible.
    /// The hardware verification process is not defined by this specification and is left up to the platform firmware or driver to implement.
    pub fn reset(&self, extended: bool) -> Status {
        unsafe { ((*self.protocol).reset)(self.protocol, extended.into()) }
    }
    ///Reads the next keystroke from the input device.
    pub fn read_key_stroke_ex(&self) -> Result<KeyData, Status> {
        let mut key_data: KeyData = Default::default();
        let status = unsafe { ((*self.protocol).read_key_stroke_ex)(self.protocol, &mut key_data) };
        if status == Status::SUCCESS {
            Ok(key_data)
        } else {
            Err(status)
        }
    }

    pub fn wait_for_key_ex(&self) -> *mut c_void {
        unsafe { (*self.protocol).wait_for_key_ex }
    }
    ///Set certain state for the input device.
    pub fn set_state(&self, key_toggle_state: ToggleState) -> Status {
        unsafe { ((*self.protocol).set_state)(self.protocol, &mut (key_toggle_state as u8)) }
    }
    ///The RegisterKeystrokeNotify() function registers a function which will be called when a specified keystroke will occur. The keystroke being specified can be for any combination of KeyData.Key or KeyData.KeyState information.
    pub fn register_key_notify(&self, key_data: KeyData, key_notification_function: KeyNotifyFunction) -> Result<*mut c_void, Status> {
        let mut notify_handle: *mut c_void= core::ptr::null_mut();
        let status = unsafe { 
            ((*self.protocol).register_key_notify)(
                self.protocol,
                &key_data as *const _ as *mut KeyData,
                key_notification_function,
                &mut notify_handle
            )
            };
        if status == Status::SUCCESS {
            Ok(notify_handle)
        } else {
            Err(status)
        }
    }
    ///The UnregisterKeystrokeNotify() function removes the notification which was previously registered.
    pub fn unregister_key_notify(&self, notify_handle: *mut c_void) -> Status {
        unsafe { ((*self.protocol).unregister_key_notify)(self.protocol, notify_handle) }
    }
}
