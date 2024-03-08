#[derive(PartialEq, Eq, Hash, Clone)]
pub enum TexturesMap {
    Background,
    Clouds,
    Coin,
    Fish,
    FishingMechanic,
    FishingPointer,
    Font,
    Number,
    Player,
    UiX,
}

#[derive(PartialEq, Clone)]
pub enum Screens {
    Game,
    Menu,
    SkillTree,
}

#[derive(PartialEq, Clone)]
pub struct GameContext {
    gold: u16,
    screens: Screens,
}
