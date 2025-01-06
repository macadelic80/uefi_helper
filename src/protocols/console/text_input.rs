use core::ffi::c_void;

use r_efi::{efi::{Event, Status, SystemTable}, protocols::simple_text_input::{InputKey, Protocol}};

// The Simple Text Input protocol defines the minimum input required to support the ConsoleIn device.
/// The Simple Text Input protocol defines the minimum input required to support the ConsoleIn device.

pub struct TextInput {
    protocol: *mut Protocol,
}

impl TextInput {
    pub fn new(st: *mut SystemTable) -> TextInput {
        TextInput {
            protocol: unsafe { (*st).con_in },
        }
    }
    ///Resets the input device hardware.
    pub fn reset(&self, extended: bool) -> Status {
        unsafe { ((*self.protocol).reset)(self.protocol, extended.into()) }
    }
    ///Reads the next keystroke from the input device.
    pub fn read_key_stroke(&self) -> Result<InputKey, Status> {
        let mut key: InputKey = Default::default();
        let status = unsafe { ((*self.protocol).read_key_stroke)(self.protocol, &mut key) };
        if status == Status::SUCCESS {
            Ok(key)
        } else {
            Err(status)
        }
    }

    pub fn wait_for_key(&self) -> Event {
        unsafe { (*self.protocol).wait_for_key }
    }
}
