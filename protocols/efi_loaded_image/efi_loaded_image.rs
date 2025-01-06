use core::ffi::c_void;

use r_efi::{
    efi::{Handle, MemoryType, Status, SystemTable},
    protocols::{device_path, loaded_image::{Protocol, PROTOCOL_GUID}}
};


pub struct LoadedImage {
    protocol: *mut Protocol,
}
impl LoadedImage {
    pub fn new(st: *mut SystemTable) -> Result<LoadedImage, Status> {
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
            Ok(LoadedImage {
                protocol,
            })
        } else {
            Err(status)
        }
    }
    ///Defines the revision of the EFI_LOADED_IMAGE_PROTOCOL structure. All future revisions will be backward compatible to the current revision.
    pub fn revision(&self) -> u32 {
        unsafe {(*self.protocol).revision}
    }
    ///Parent image’s image handle. NULL if the image is loaded directly from the firmware’s boot manager. Type EFI_HANDLE is defined in Services — Boot Services.
    pub fn parent_handle(&self) -> Handle {
        unsafe {(*self.protocol).parent_handle}
    }
    ///The image’s EFI system table pointer. Type EFI_SYSTEM_TABLE defined in EFI System Table.
    pub fn system_table(&self) -> *mut SystemTable {
        unsafe {(*self.protocol).system_table}
    }

    //source location of the image
    ///The device handle that the EFI Image was loaded from. Type EFI_HANDLE is defined in Services — Boot Services.
    pub fn device_handle(&self) -> Handle {
        unsafe {(*self.protocol).device_handle}
    }
    ///A pointer to the file path portion specific to DeviceHandle that the EFI Image was loaded from. EFI_DEVICE_PATH_PROTOCOL is defined in EFI Device Path Protocol .
    pub fn file_path(&self) -> *mut device_path::Protocol {
        unsafe {(*self.protocol).file_path}
    }
    ///Reserved. DO NOT USE.
    pub fn reserved(&self) -> *mut c_void {
        unsafe {(*self.protocol).reserved}
    }

    //image's load option
    ///The size in bytes of LoadOptions.
    pub fn load_options_size(&self) -> u32 {
        unsafe {(*self.protocol).load_options_size}
    }
    ///A pointer to the image’s binary load options. See the OptionalData parameter in the Load Options section of the Boot Manager chapter for information on the source of the LoadOptions data.
    pub fn load_options(&self) -> *mut c_void {
        unsafe {(*self.protocol).load_options}
    }

    //location where image was loaded
    ///The base address at which the image was loaded.
    pub fn image_base(&self) -> *mut c_void {
        unsafe {(*self.protocol).image_base}
    }
    ///The size in bytes of the loaded image.
    pub fn image_size(&self) -> u64 {
        unsafe {(*self.protocol).image_size}
    }
    ///The memory type that the code sections were loaded as. Type EFI_MEMORY_TYPE is defined in Services — Boot Services.
    pub fn image_code_type(&self) -> MemoryType {
        unsafe {(*self.protocol).image_code_type}
    }
    ///The memory type that the data sections were loaded as. Type EFI_MEMORY_TYPE is defined in Services — Boot Services.
    pub fn image_data_type(&self) -> MemoryType {
        unsafe {(*self.protocol).image_data_type}
    }
    ///The Unload() function is a callback that a driver registers to do cleanup when the UnloadImage boot service function is called.
    ///If the Unload() function pointer in an EFI_LOADED_IMAGE_PROTOCOL instance is NULL, the image does not support unload.
    pub fn unload(&self, image_handle: Handle) -> Status {
        if let Some(unload_fn) = unsafe{(*self.protocol).unload} {
            return unload_fn(image_handle)
        }

        Status::NOT_FOUND
    }
    
}
