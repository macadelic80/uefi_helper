use r_efi::{efi::{Event, Status, SystemTable}, protocols::simple_pointer::{Mode, Protocol, State}};


pub struct Pointer {
    protocol: *mut Protocol,
}
impl Pointer {
    pub fn new(st: *mut SystemTable) -> Result<Pointer, Status> {
        let mut protocol: *mut Protocol = core::ptr::null_mut();
        let mut guid = r_efi::protocols::simple_pointer::PROTOCOL_GUID;
        let boot_services = unsafe{&mut *st}.boot_services;
        let status = unsafe {
            ((*boot_services).locate_protocol)(
                &mut guid,
                core::ptr::null_mut(),
                &mut protocol as *mut *mut Protocol as *mut _,
            )
        };
    
        if status == Status::SUCCESS {
            Ok(Pointer {
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
    ///Pointer to EFI_SIMPLE_POINTER_MODE data. The type EFI_SIMPLE_POINTER_MODE is defined in “Related Definitions” below.
    pub fn mode(&self) -> Mode {
        unsafe { *(*self.protocol).mode }
    }
    ///Event to use with EFI_BOOT_SERVICES.WaitForEvent() to wait for input from the pointer device.
    pub fn wait_for_input(&self) -> Event {
        unsafe { (*self.protocol).wait_for_input }
    }
    
}
