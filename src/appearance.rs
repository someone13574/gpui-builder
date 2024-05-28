pub mod colors {
    use gpui::{rgb, Rgba};
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref BG: Rgba = rgb(0x181818);
        pub static ref BORDER: Rgba = rgb(0x505050);
        pub static ref SIDEBAR_BG: Rgba = rgb(0x181818);
    }
}

pub mod sizes {
    use gpui::{px, AbsoluteLength};
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref SIDEBAR_WIDTH: AbsoluteLength = px(200.0).into();
        pub static ref BORDER_WIDTH: AbsoluteLength = px(1.0).into();
    }
}
