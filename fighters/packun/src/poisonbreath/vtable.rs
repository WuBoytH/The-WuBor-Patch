use super::*;

unsafe extern "C" fn poisonbreath_weapon_on_attack_mask(vtable: u64, weapon: &mut smash::app::Weapon, collision_mask: u32) -> u64 {
    let module_accessor = weapon.battle_object.module_accessor;
    let status = StatusModule::status_kind(module_accessor);
    if status == *WEAPON_PACKUN_POISONBREATH_STATUS_KIND_START
    && collision_mask & 6 != 0 {
        StatusModule::change_status_request(module_accessor, vars::packun_poisonbreath::status::BURST, false);
        return 1;
    }
    MiscModule::normal_weapon_hit_handler(vtable, weapon, collision_mask)
}

pub fn install() {
    replace_weapon_vtable(
        *WEAPON_KIND_PACKUN_POISONBREATH,
        WeaponVtableSlot::WeaponOnAttackMask,
        poisonbreath_weapon_on_attack_mask as *const () as _
    );
}
