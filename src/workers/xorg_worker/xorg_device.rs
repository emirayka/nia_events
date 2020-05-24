use crate::Error;
use xcb::Connection;

pub struct XorgDevice {
    connection: Connection,
    root: xcb::xproto::Window,
}

impl XorgDevice {
    pub fn new(connection: Connection, root: xcb::xproto::Window) -> XorgDevice {
        XorgDevice { connection, root }
    }

    pub fn type_text(&self, text: &str) -> Result<(), Error> {
        xcb::test::fake_input(
            &self.connection,
            xcb::ffi::xproto::XCB_KEY_PRESS,
            74,
            xcb::base::CURRENT_TIME,
            xcb::base::NONE,
            0,
            0,
            0,
        )
        .request_check()
        .map_err(|_| Error::xorg_error("Error while sending xorg request."))?;

        xcb::test::fake_input(
            &self.connection,
            xcb::ffi::xproto::XCB_KEY_RELEASE,
            74,
            xcb::base::CURRENT_TIME,
            xcb::base::NONE,
            0,
            0,
            0,
        )
        .request_check()
        .map_err(|_| Error::xorg_error("Error while sending xorg request."))?;

        Ok(())
    }

    pub fn mouse_move_by(&self, x: i16, y: i16) -> Result<(), Error> {
        xcb::xproto::warp_pointer(
            &self.connection,
            xcb::base::NONE,
            xcb::base::NONE,
            0,
            0,
            0,
            0,
            x,
            y,
        )
        .request_check()
        .map_err(|_| Error::xorg_error("Error while sending xorg request."))?;

        Ok(())
    }

    pub fn mouse_move_to(&self, x: i16, y: i16) -> Result<(), Error> {
        xcb::xproto::warp_pointer(
            &self.connection,
            xcb::base::NONE,
            self.root,
            0,
            0,
            0,
            0,
            x,
            y,
        )
        .request_check()
        .map_err(|_| Error::xorg_error("Error while sending xorg request."))?;

        Ok(())
    }
}
