use crate::imports::*;

extern "C" {
    #[link_name = "shinku_on_hit_inner"]
    fn shinku_on_hit_inner(
        vtable: u64,
        weapon: &mut app::Weapon,
        something: u32
    ) -> u64;
}

unsafe extern "C" fn shinku_on_hit(vtable: u64, weapon: &mut app::Weapon, something: u32) -> u64 {
    shinku_on_hit_inner(vtable, weapon, something)
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x5214940).data(shinku_on_hit as u64);
}