#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Quit,
    ToggleColors,
    ToggleStyles,
    ToggleInfo,
    AddSpeed,
    ReduceSpeed,
    PowSix,
    PowSeven,
}
