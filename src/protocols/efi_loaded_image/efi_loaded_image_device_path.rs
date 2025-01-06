use core::ffi::c_void;

use r_efi::{
    efi::{Guid, Handle, MemoryType, Status, SystemTable},
    protocols::{device_path, loaded_image_device_path::PROTOCOL_GUID}
};

///The Loaded Image Device Path Protocol uses the same protocol interface structure as the EFI Device Path Protocol defined in Chapter 10. The only difference between the Device Path Protocol and the Loaded Image Device Path Protocol is the protocol GUID value.

///The Loaded Image Device Path Protocol must be installed onto the image handle of a PE/COFF image loaded through the EFI Boot Service LoadImage(). A copy of the device path specified by the DevicePath parameter to the EFI Boot Service LoadImage() is made before it is installed onto the image handle. It is legal to call LoadImage() for a buffer in memory with a NULL DevicePath parameter. In this case, the Loaded Image Device Path Protocol is installed with a NULL interface pointer.
pub struct LoadedImageDevicePath {
    // protocol: *mut Protocol,
    protocol_guid: Guid
}
impl LoadedImageDevicePath {
    pub fn new() -> LoadedImageDevicePath {
        let protocol_guid = PROTOCOL_GUID;
        LoadedImageDevicePath {
            protocol_guid,
        }
    }
    
}
