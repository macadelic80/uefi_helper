use r_efi::{
    efi::{Status, SystemTable},
    protocols::graphics_output::{BltOperation, BltPixel, Mode, ModeInformation, Protocol, PROTOCOL_GUID},
};

#[repr(C)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

pub struct GraphicOutput {
    protocol: *mut Protocol,
}
impl GraphicOutput {
    pub fn new(st: *mut SystemTable) -> Result<GraphicOutput, Status> {
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
            Ok(GraphicOutput {
                protocol,
            })
        } else {
            Err(status)
        }
    }
    ///Returns information for an available graphics mode that the graphics device and the set of active video output devices supports.
    pub fn query_mode(&self, mode_number: u32) -> Result<ModeInformation, Status> {
        let mut size_of_info: usize = 0;
        let mut info: *mut ModeInformation = core::ptr::null_mut();
        let r = (unsafe { &mut *self.protocol }.query_mode)(
            self.protocol,
            mode_number,
            &mut size_of_info,
            &mut info,
        );
        if r == Status::SUCCESS {
            if !info.is_null() {
                unsafe {
                    Ok(*info)
                }
            } else {
                Err(Status::NOT_FOUND)
            }
        } else {
            Err(r)
        }
    }
    /// Set the video device into the specified mode and clears the visible portions of the output display to black.
    pub fn set_mode(&self, mode_number: u32) -> Status {
        unsafe { ((*self.protocol).set_mode)(self.protocol, mode_number) }
    }
    ///Blt a rectangle of pixels on the graphics screen. Blt stands for BLock Transfer.
    pub fn blt(&self,
        blt_buffer: *mut BltPixel,
        blt_operation: BltOperation,
        // source_x: usize,
        // source_y: usize,
        source: *const Coordinate,
        // destination_x: usize,
        // destination_y: usize,
        destination: *const Coordinate,
        // width: usize,
        // height: usize,
        dimension: *const Coordinate,
        delta: usize,
    ) -> Status {
        unsafe { ((*self.protocol).blt)(
            self.protocol,
            blt_buffer,
            blt_operation,
            (*source).x,
            (*source).y,
            (*destination).x,
            (*destination).y,
            (*dimension).x,
            (*dimension).y,
            delta,
            ) }
    }

    ///Pointer to EFI_SIMPLE_POINTER_MODE data. The type EFI_SIMPLE_POINTER_MODE is defined in “Related Definitions” below.
    pub fn mode(&self) -> Mode {
        unsafe { *(*self.protocol).mode }
    }
}
