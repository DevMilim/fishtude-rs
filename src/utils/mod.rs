use sdl2::rect::Rect;

#[macro_export]
macro_rules! render_entityes {
    ($canvas:expr, $texture_map:expr,$($entity:expr),*) => {
        $(
            let _ = $entity.update();
            let _ = $entity.render(&mut $canvas, &$texture_map);
        )*
    };
}

pub struct Position {
    pub x: i32,
    pub y: i32,
}

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
