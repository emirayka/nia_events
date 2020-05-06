#[derive(Clone, Debug)]
pub enum XorgWorkerCommand {
    MouseMoveBy(i16, i16),
    MouseMoveTo(i16, i16),
    TextType(String),
}
