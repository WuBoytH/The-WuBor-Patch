use super::*;

unsafe extern "C" fn shinku_weapon_on_attack_mask(vtable: u64, weapon: &mut smash::app::Weapon, collision_kind_mask: u32) -> u64 {
    if !VarModule::is_flag(weapon.battle_object.module_accessor, vars::ryu_shinkuhadoken::status::instance::HIT) {
        if collision_kind_mask & (*COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) as u32 != 0 {
            VarModule::on_flag(weapon.battle_object.module_accessor, vars::ryu_shinkuhadoken::status::instance::HIT);
            // println!("hi");
            // KineticModule::unable_energy(weapon.battle_object.module_accessor, 0);
            WorkModule::set_int(weapon.battle_object.module_accessor, 35, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        }
    }
    else {
        if collision_kind_mask & *COLLISION_KIND_MASK_REFLECTOR as u32 != 0 {
            VarModule::off_flag(weapon.battle_object.module_accessor, vars::ryu_shinkuhadoken::status::instance::HIT);
            // KineticModule::enable_energy(weapon.battle_object.module_accessor, 0);
        }
    }
    // else {
    //     KineticModule::resume_energy(weapon.battle_object.module_accessor, 0);
    // }
    *(weapon as *mut smash::app::Weapon as *mut bool).add(0x90) = true;
    MiscModule::normal_weapon_hit_handler(vtable, weapon, collision_kind_mask)
}

pub fn install() {
    replace_weapon_vtable(
        *WEAPON_KIND_RYU_SHINKUHADOKEN,
        WeaponVtableSlot::WeaponOnAttackMask,
        shinku_weapon_on_attack_mask as *const () as _
    );
}