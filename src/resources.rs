#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FrameInput {
    pub controller: Controller,
    pub dt: f32
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Controller {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,

    pub btn_a: bool
}
