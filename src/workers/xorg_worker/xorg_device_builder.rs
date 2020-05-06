use crate::Error;
use crate::XorgDevice;

pub struct XorgDeviceBuilder {
    display_name: Option<String>,
}

impl XorgDeviceBuilder {
    pub fn new() -> XorgDeviceBuilder {
        XorgDeviceBuilder {
            display_name: None,
        }
    }

    pub fn set_display_name(mut self, name: String) -> XorgDeviceBuilder {
        self.display_name = Some(name);

        self
    }

    pub fn build(self) -> Result<XorgDevice, Error> {
        let display_name = self.display_name;
        let display_name_str = match &display_name {
            Some(string) => Some(string.as_str()),
            None => None
        };

        let connection_result = xcb::Connection::connect(display_name_str)
            .map_err(|_| Error::xorg_error("Cannot connect to xorg."))?;

        let (connection, screen_num) = connection_result;

        let setup = connection.get_setup();
        let screen = match setup.roots().nth(screen_num as usize) {
            Some(result) => result,
            None => {
                return Err(Error::xorg_error("Cannot connect to xorg."));
            }
        };
        let root = screen.root();

        let xorg_device = XorgDevice::new(
            connection,
            root
        );

        Ok(xorg_device)
    }

    pub fn build_default() -> Result<XorgDevice, Error> {
        XorgDeviceBuilder::new()
            .build()
    }
}