use super::*;

unsafe extern "C" fn breath_weapon_on_attack_mask(vtable: u64, weapon: &mut smash::app::Weapon, something: u32) -> u64 {
    if something & 6 != 0 {
        StatusModule::change_status_request(weapon.battle_object.module_accessor, vars::koopa_breath::status::HIT, false);
        return 1;
    }
    MiscModule::normal_weapon_hit_handler(vtable, weapon, something)
}

pub fn install() {
    replace_weapon_vtable(
        *WEAPON_KIND_KOOPA_BREATH,
        WeaponVtableSlot::WeaponOnAttackMask,
        breath_weapon_on_attack_mask as *const () as _
    );
}