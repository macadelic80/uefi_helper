use lib_efi::{
    efi::{Event, Status, SystemTable},
    protocols::absolute_pointer::{Mode, Protocol, State, PROTOCOL_GUID}
};


pub struct AbsolutePointer {
    protocol: *mut Protocol,
}
impl AbsolutePointer {
    pub fn new(st: *mut SystemTable) -> Result<AbsolutePointer, Status> {
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
            Ok(AbsolutePointer {
                protocol,
            })
        } else {
            Err(status)
        }
    }

    ///Resets the pointer device hardware.
    pub fn reset(&self, extended: bool) -> Status {
        unsafe { ((*self.protocol).reset)(self.protocol, extended.into()) }
    }
    ///Retrieves the current state of a pointer device.
    pub fn get_state(&self) -> Result<State, Status> {
        let mut state: State = Default::default();
        let status = unsafe { ((*self.protocol).get_state)(self.protocol, &mut state) };
        if status == Status::SUCCESS {
            Ok(state)
        } else {
            Err(status)
        }
    }

    pub fn mode(&self) -> *mut Mode {
        unsafe { (*self.protocol).mode }
    }

    pub fn wait_for_input(&self) -> Event {
        unsafe { (*self.protocol).wait_for_input }
    }
    
}
