use super::*;

unsafe extern "C" fn breath_on_hit(vtable: u64, weapon: &mut smash::app::Weapon, something: u32) -> u64 {
    if something & 6 != 0 {
        StatusModule::change_status_request(weapon.battle_object.module_accessor, vars::koopa_breath::status::HIT, false);
        return 1;
    }
    MiscModule::normal_weapon_hit_handler(vtable, weapon, something)
}

pub fn install() {
    replace_weapon_vtable(
        *WEAPON_KIND_KOOPA_BREATH,
        0x1d,
        breath_on_hit as *const () as _
    );
}