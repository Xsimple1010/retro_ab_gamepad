use gilrs::Button;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KeyMap {
    pub native: Button,
    pub retro: u32,
    pub pressed: bool,
}
