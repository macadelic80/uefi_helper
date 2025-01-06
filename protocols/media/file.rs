use core::{alloc, ffi::c_void, ptr::null_mut};

use r_efi::{
    efi::{Guid, Status, SystemTable},
    protocols::{
        device_path,
        file::{
            self, Info, IoToken, Protocol, SystemInfo, SystemVolumeLabel, INFO_ID, SYSTEM_INFO_ID,
            SYSTEM_VOLUME_LABEL_ID,
        },
        shell::FileInfo,
    },
};

use crate::protocols::logger::str_to_utf16;

#[repr(u64)]
pub enum OpenMode {
    Read = 0x0000000000000001,
    Write = 0x0000000000000002,
    Create = 0x8000000000000000,
}

#[repr(u64)]
pub enum Attribute {
    ReadOnly = 0x0000000000000001,
    Hidden = 0x0000000000000002,
    System = 0x0000000000000004,
    Reserved = 0x0000000000000008,
    Directory = 0x0000000000000010,
    Archive = 0x0000000000000020,
    ValidAttr = 0x0000000000000037,
}

pub trait InfoType {
    const GUID: Guid;
}

impl<const N: usize> InfoType for Info<N> {
    const GUID: Guid = INFO_ID;
}

impl<const N: usize> InfoType for SystemInfo<N> {
    const GUID: Guid = SYSTEM_INFO_ID;
}

impl<const N: usize> InfoType for SystemVolumeLabel<N> {
    const GUID: Guid = SYSTEM_VOLUME_LABEL_ID;
}

pub struct File {
    pub root: *mut Protocol,
}
impl File {
    pub fn new(root: *mut *mut Protocol) -> File {
        File {
            root: unsafe { *root },
        }
    }

    ///The version of the EFI_FILE_PROTOCOL interface. The version specified by this specification is EFI_FILE_PROTOCOL_LATEST_REVISION. Future versions are required to be backward compatible to version 1.0.
    pub fn revision(&self) -> u64 {
        unsafe { (*self.root).revision }
    }

    ///Opens or creates a new file.
    pub fn open(
        &self,
        str_file_name: &str,
        open_mode: OpenMode,
        attributes: Attribute,
    ) -> Result<File, Status> {
        let mut new_handle: *mut file::Protocol = null_mut();
        let mut file_name = str_to_utf16(str_file_name);
        let status = unsafe {
            ((*self.root).open)(
                self.root,
                &mut new_handle,
                file_name.as_mut_ptr(),
                open_mode as u64,
                attributes as u64,
            )
        };
        match status {
            Status::SUCCESS => Ok(File { root: new_handle }),
            _ => Err(status),
        }
        /*
        EFI_NOT_FOUND

        The specified file could not be found on the device.

        EFI_NO_MEDIA

        The device has no medium.

        EFI_MEDIA_CHANGED

        The device has a different medium in it or the medium is no longer supported.

        EFI_DEVICE_ERROR

        The device reported an error.

        EFI_VOLUME_CORRUPTED

        The file system structures are corrupted.

        EFI_WRITE_PROTECTED

        An attempt was made to create a file, or open a file for write when the media is write-protected.

        EFI_ACCESS_DENIED

        The service denied access to the file.

        EFI_OUT_OF_RESOURCES

        Not enough resources were available to open the file.

        EFI_VOLUME_FULL

        The volume is full.

        EFI_INVALID_PARAMETER

        This refers to a regular file, not a directory.
         */
    }
    ///The Close() function closes a specified file handle. All “dirty” cached file data is flushed to the device, and the file is closed. In all cases the handle is closed. The operation will wait for all pending asynchronous I/O requests to complete before completing.
    pub fn close(&self) -> Status {
        unsafe { ((*self.root).close)(self.root) }
    }
    ///The Delete() function closes and deletes a file. In all cases the file handle is closed. If the file cannot be deleted, the warning code EFI_WARN_DELETE_FAILURE is returned, but the handle is still closed.
    pub fn delete(&self) -> Status {
        unsafe { ((*self.root).delete)(self.root) }
        /*
        EFI_SUCCESS

        The file was closed and deleted, and the handle was closed.

        EFI_WARN_DELETE_FAILURE

        The handle was closed, but the file was not deleted.
        */
    }
    ///The Read() function reads data from a file.

    ///If This is not a directory, the function reads the requested number of bytes from the file at the file’s current position and returns them in Buffer. If the read goes beyond the end of the file, the read length is truncated to the end of the file. The file’s current position is increased by the number of bytes returned.

    ///If This is a directory, the function reads the directory entry at the file’s current position and returns the entry in Buffer. If the Buffer is not large enough to hold the current directory entry, then EFI_BUFFER_TOO_SMALL is returned and the current file position is not updated. BufferSize is set to be the size of the buffer needed to read the entry. On success, the current position is updated to the next directory entry. If there are no more directory entries, the read returns a zero-length buffer. EFI_FILE_INFO is the structure returned as the directory entry.
    pub fn read(&self, size: usize) -> Result<(*mut c_void, usize), Status> {
        let buffer: *mut c_void = null_mut();
        let mut buffer_size: usize = size;
        let status = unsafe { ((*self.root).read)(self.root, &mut buffer_size, buffer) };

        match status {
            Status::SUCCESS => Ok((buffer, buffer_size)),
            _ => Err(status),
        }

        /*
        EFI_NO_MEDIA

        The device has no medium.

        EFI_DEVICE_ERROR

        The device reported an error.

        EFI_DEVICE_ERROR

        An attempt was made to read from a deleted file.

        EFI_DEVICE_ERROR

        On entry, the current file position is beyond the end of the file.

        EFI_VOLUME_CORRUPTED

        The file system structures are corrupted.

        EFI_BUFFER_TOO_SMALL

        The BufferSize is too small to read the current directory entry. BufferSize has been updated with the size needed to complete the request.
            */
    }
    ///The Write() function writes the specified number of bytes to the file at the current file position. The current file position is advanced the actual number of bytes written, which is returned in BufferSize. Partial writes only occur when there has been a data error during the write attempt (such as “file space full”). The file is automatically grown to hold the data if required. Direct writes to opened directories are not supported.
    pub fn write(&self, buffer: &mut [u8]) -> Result<usize, Status> {
        let mut buffer_size: usize = buffer.len();
        let status = unsafe {
            ((*self.root).write)(
                self.root,
                &mut buffer_size,
                buffer.as_mut_ptr() as *mut c_void,
            )
        };

        match status {
            Status::SUCCESS => Ok(buffer_size),
            _ => Err(status),
        }
        /*
        EFI_UNSUPPORT

        Writes to open directory files are not supported.

        EFI_NO_MEDIA

        The device has no medium.

        EFI_DEVICE_ERROR

        The device reported an error.

        EFI_DEVICE_ERROR

        An attempt was made to write to a deleted file.

        EFI_VOLUME_CORRUPTED

        The file system structures are corrupted.

        EFI_WRITE_PROTECTED

        The file or medium is write-protected.

        EFI_ACCESS_DENIED

        The file was opened read only.

        EFI_VOLUME_FULL

        The volume is full.
         */
    }
    ///Returns the current file position.
    pub fn get_position(&self) -> Result<u64, Status> {
        let mut size: u64 = 0;
        let status = unsafe { ((*self.root).get_position)(self.root, &mut size) };
        match status {
            Status::SUCCESS => Ok(size),
            _ => Err(status),
        }
    }
    ///Sets the current file position.
    pub fn set_position(&self, position: u64) -> Status {
        unsafe { ((*self.root).set_position)(self.root, position) }
    }
    ///Gets the requested file or volume information.
    pub fn _get_info(
        &self,
        information_type: *mut Guid,
        buffer: *mut c_void,
    ) -> Result<(*mut c_void, usize), Status> {
        let mut buffer_size: usize = 0;
        let status = unsafe {
            ((*self.root).get_info)(self.root, information_type, &mut buffer_size, buffer)
        };
        match status {
            Status::SUCCESS => Ok((buffer, buffer_size)),
            _ => Err(status),
        }
    }
    pub fn get_info<T: InfoType>(&self) -> Result<*mut T, Status> {
        let mut buffer_size: usize = 0;
        let mut information_type = T::GUID;

        match self._get_info(&mut information_type, null_mut()) {
            Ok((_, size)) => {
                buffer_size = size;
            }
            Err(Status::BUFFER_TOO_SMALL) => {
            }
            Err(status) => return Err(status), // Propagation d'autres erreurs
        }

        const MAX_BUFFER_SIZE: usize = 1024;
        if buffer_size > MAX_BUFFER_SIZE {
            return Err(Status::BUFFER_TOO_SMALL); 
        }

        let mut buffer = [0u8; MAX_BUFFER_SIZE];

        match self._get_info(&mut information_type, buffer.as_mut_ptr() as *mut c_void) {
            Ok((_, _)) => {
                Ok(buffer.as_mut_ptr() as *mut T)
            }
            Err(status) => Err(status),
        }
    }
    ///Sets the requested file information.
    pub fn _set_info(
        &self,
        information_type: *mut Guid,
        buffer_size: *mut usize,
        buffer: *mut c_void,
    ) -> Status {
        unsafe { ((*self.root).get_info)(self.root, information_type, buffer_size, buffer) }
    }
    pub fn set_info<T: InfoType>(&self, value: *mut T) -> Status {
        let mut information_type = T::GUID;
        let buffer_size = core::mem::size_of::<T>();
        
        // Vérification de la taille maximale
        const MAX_BUFFER_SIZE: usize = 512; // Adaptez cette valeur selon vos besoins
        if buffer_size > MAX_BUFFER_SIZE {
            return Status::BUFFER_TOO_SMALL;
        }
    
        // Étape 1 : Convertir la valeur en pointeur vers les données brutes
        let buffer_ptr = value as *const T as *const c_void;
    
        // Étape 2 : Appeler `_set_info` pour appliquer les changements
        self._set_info(&mut information_type, &mut (buffer_size as usize), buffer_ptr as *mut c_void)
    }
    ///Flushes all modified data associated with the file to the device.
    pub fn flush(&self) -> Status {
        unsafe { ((*self.root).flush)(self.root) }
    }
    ///Opens a new file relative to the source directory's location.
    pub fn open_ex(
        &self,
        str_file_name: &str,
        open_mode: OpenMode,
        attributes: Attribute,
        token: *mut IoToken,
    ) -> Result<(File, *mut IoToken), Status> {
        let mut new_handle: *mut file::Protocol = null_mut();
        let mut file_name = str_to_utf16(str_file_name);
        let status = unsafe {
            ((*self.root).open_ex)(
                self.root,
                &mut new_handle,
                file_name.as_mut_ptr(),
                open_mode as u64,
                attributes as u64,
                token,
            )
        };
        match status {
            Status::SUCCESS => Ok((File { root: new_handle }, token)),
            _ => Err(status),
        }
    }
    ///Reads data from a file.
    pub fn read_ex(&self, token: *mut IoToken) -> Result<*mut IoToken, Status> {
        let status = unsafe { ((*self.root).read_ex)(self.root, token) };

        match status {
            Status::SUCCESS => Ok(token),
            _ => Err(status),
        }
    }
    ///Writes data to a file.
    pub fn write_ex(&self, token: *mut IoToken) -> Result<*mut IoToken, Status> {
        let status = unsafe { ((*self.root).write_ex)(self.root, token) };

        match status {
            Status::SUCCESS => Ok(token),
            _ => Err(status),
        }
    }
    ///Flushes all modified data associated with a file to a device.
    pub fn flush_ex(&self, token: *mut IoToken) -> Result<*mut IoToken, Status> {
        let status = unsafe { ((*self.root).flush_ex)(self.root, token) };

        match status {
            Status::SUCCESS => Ok(token),
            _ => Err(status),
        }
    }
}
