use crate::imports::*;
use crate::offsets;

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
    unsafe {
        let _ = skyline::patching::Patch::in_text(offsets::weapon::ryu_shinkuhadoken::vtable::ON_HIT).data(shinku_on_hit as *const () as u64);
    }
}
