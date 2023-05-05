///////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////
/// TODO: NEED TO MAKE MATCHING ON MODS BETTER.
/// WHEN CHECKING FOR SHORTCUT ALL OTHER MODS SHOULD BE FALSE, NOT JUST THE ONE BEING CHECKED.
/// THE CAPSLOCK CAN BE IGNORED THOUGH.

#[derive(Default, Debug)]
pub struct KeyMods {
    pub l_shift: bool,
    pub r_shift: bool,
    pub l_control: bool,
    pub r_control: bool,
    pub l_alt: bool, // Option on macOS
    pub r_alt: bool,
    pub l_system: bool, // Command on macOS, Windows on Windows
    pub r_system: bool,

    pub caps_lock: bool,
}

impl From<u16> for KeyMods {
    fn from(value: u16) -> Self {
        Self {
            l_shift: value & 0x0001 != 0,
            r_shift: value & 0x0002 != 0,
            l_control: value & 0x0040 != 0,
            r_control: value & 0x0080 != 0,
            l_alt: value & 0x0100 != 0,
            r_alt: value & 0x0200 != 0,
            l_system: value & 0x0400 != 0,
            r_system: value & 0x0800 != 0,
            caps_lock: value & 0x2000 != 0,
        }
    }
}
