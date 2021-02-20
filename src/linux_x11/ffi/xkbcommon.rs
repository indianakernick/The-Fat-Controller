#[allow(non_camel_case_types)]
pub type xkb_keysym_t = u32;

#[link(name = "xkbcommon")]
extern {
    pub fn xkb_keysym_to_utf32(keysym: xkb_keysym_t) -> u32;
}
