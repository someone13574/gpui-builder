pub mod colors {
    use gpui::{rgb, Rgba};
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref BG: Rgba = rgb(0x181818);
        pub static ref BORDER: Rgba = rgb(0x505050);
        pub static ref SIDEBAR_BG: Rgba = rgb(0x181818);
        pub static ref TEXT: Rgba = rgb(0xb0b0b0);
    }
}
