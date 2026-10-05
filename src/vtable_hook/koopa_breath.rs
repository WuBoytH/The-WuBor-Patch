use crate::imports::*;
use crate::offsets;

#[skyline::hook(offset = offsets::weapon::koopa_breath::ON_HIT)]
unsafe extern "C" fn breath_on_hit(vtable: u64, weapon: *mut app::Weapon, something: u32) -> u64 {
    if something & 6 != 0 {
        StatusModule::change_status_request((*weapon).battle_object.module_accessor, koopa_breath::status::HIT, false);
        return 1;
    }
    MiscModule::normal_weapon_hit_handler(vtable, weapon, something)
}

pub fn install() {
    skyline::install_hooks!(
        breath_on_hit
    );
}
