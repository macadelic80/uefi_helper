use r_efi::{
    efi::{self, Status},
    protocols::simple_text_output::Protocol,
};


pub enum BackgroundColor {
    Black       = 0x00,
    Blue        = 0x10,
    Green       = 0x20,
    Cyan        = 0x30,
    Red         = 0x40,
    Magenta     = 0x50,
    Brown       = 0x60,
    LightGray   = 0x70,
}

pub enum ForegroudColor {
    None         = -1,
    BLACK        = 0x00,
    BLUE         = 0x01,
    GREEN        = 0x02,
    CYAN         = 0x03,
    RED          = 0x04,
    MAGENTA      = 0x05,
    BROWN        = 0x06,
    LIGHTGRAY    = 0x07,
    // BRIGHT       = 0x08,
    DARKGRAY     = 0x08,
    LIGHTBLUE    = 0x09,
    LIGHTGREEN   = 0x0A,
    LIGHTCYAN    = 0x0B,
    LIGHTRED     = 0x0C,
    LIGHTMAGENTA = 0x0D,
    YELLOW       = 0x0E,
    WHITE        = 0x0F,
}

pub struct ScreenDimension {
    pub columns: usize,
    pub rows: usize,
}

pub struct TextOutput {
    protocol: *mut Protocol,
}
impl TextOutput {
    pub fn new(st: *mut efi::SystemTable) -> TextOutput {
        TextOutput {
            protocol: unsafe { (*st).con_out },
        }
    }

    ///Displays the string on the device at the current cursor location. See OutputString()
    pub fn output_string(&self, str: *mut u16) -> Status {
        unsafe { ((*self.protocol).output_string)(self.protocol, str) }
    }
    ///Tests to see if the ConsoleOut device supports this string.
    pub fn test_string(&self, str: *mut u16) -> Status {
        unsafe { ((*self.protocol).test_string)(self.protocol, str) }
    }
    ///Queries information concerning the output device’s supported text mode.
    pub fn query_mode(&self, mode_number: usize) -> Result<ScreenDimension, Status> {
        let mut columns = 0;
        let mut rows = 0;
        let r = (unsafe { (&mut *self.protocol) }.query_mode)(
            self.protocol,
            mode_number,
            &mut columns,
            &mut rows,
        );
        if r == Status::SUCCESS {
            Ok(ScreenDimension {
                columns: columns,
                rows: rows,
            })
        } else {
            Err(r)
        }
    }
    ///Sets the current mode of the output device.
    /// The SetMode() function sets the output device(s) to the requested mode. On success the device is in the geometry for the requested mode, and the device has been cleared to the current background color with the cursor at (0,0).
    pub fn set_mode(&self, mode_number: usize) -> Status {
        unsafe { ((*self.protocol).set_mode)(self.protocol, mode_number) }

    }
    ///The SetAttribute() function sets the background and foreground colors for the OutputString() and ClearScreen()
    /// The color mask can be set even when the device is in an invalid text mode.
    ///Devices supporting a different number of text colors are required to emulate the above colors to the best of the device’s capabilities.
    pub fn set_attribute(&self, foreground: ForegroudColor, background: BackgroundColor) -> Status {
        let attribute: usize = foreground as usize | ((background as usize) << 4);
        unsafe { ((*self.protocol).set_attribute)(self.protocol, attribute) }
    }
    ///Clears the output device(s) display to the currently selected background color.
    /// The ClearScreen() function clears the output device(s) display to the currently selected background color. The cursor position is set to (0, 0).
    pub fn clear_screen(&self) -> Status {
        unsafe { ((*self.protocol).clear_screen)(self.protocol) }

    }
    ///The SetCursorPosition() function sets the current coordinates of the cursor position. The upper left corner of the screen is defined as coordinate (0, 0).
    pub fn set_cursor_position(&self, dimension: ScreenDimension) -> Status {
        unsafe { ((*self.protocol).set_cursor_position)(self.protocol, dimension.columns, dimension.rows) }
    }
    ///Turns the visibility of the cursor on/off.
    pub fn enable_cursor(&self, visible: bool) -> Status {
        unsafe { ((*self.protocol).enable_cursor)(self.protocol, visible.into()) }
    }
}
