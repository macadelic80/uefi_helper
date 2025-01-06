use core::{ptr::null_mut};

use lib_efi::{
    efi::{Status, SystemTable},
    protocols::{device_path, file, simple_file_system::{Protocol, PROTOCOL_GUID}}
};

use super::file::File;


pub struct FileSystem {
    protocol: *mut Protocol,
}
impl FileSystem {
    pub fn new(st: *mut SystemTable) -> Result<FileSystem, Status> {
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
            Ok(FileSystem {
                protocol,
            })
        } else {
            Err(status)
        }
    }

    ///The version of the EFI_FILE_PROTOCOL. The version specified by this specification is 0x00010000. All future revisions must be backwards compatible. If a future version is not backwards compatible, it is not the same GUID.
    pub fn revision(&self) -> u64 {
        unsafe {(*self.protocol).revision}
    }

    ///The OpenVolume() function opens a volume, and returns a file handle to the volume’s root directory. This handle is used to perform all other file I/O operations. The volume remains open until all the file handles to it are closed.
    /// If the medium is changed while there are open file handles to the volume, all file handles to the volume will return EFI_MEDIA_CHANGED. To access the files on the new medium, the volume must be reopened with OpenVolume(). If the new medium is a different file system than the one supplied in the EFI_HANDLE’s DevicePath for the EFI_SIMPLE_SYSTEM_PROTOCOL, OpenVolume() will return EFI_UNSUPPORTED.
    pub fn open_volume(&self) -> Result<File, Status> {
        let mut root: *mut file::Protocol = null_mut();
        let status = unsafe { ((*self.protocol).open_volume)(self.protocol, &mut root)};
        
        if status == Status::SUCCESS {
            Ok(File{root: root})
        } else {
            Err(status)
        }
    }
}
