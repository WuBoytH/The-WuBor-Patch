use super::*;

unsafe extern "C" fn main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    HitModule::set_hit_stop_mul(weapon.module_accessor, 0.0, HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_ALL as u8}, 0.0);
    let status = weapon.global_table[STATUS_KIND].get_i32();
    original_status(Main, weapon, status)(weapon)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_PICKEL_TABLE_STATUS_KIND_AIR, main);
    agent.status(Main, *WEAPON_PICKEL_TABLE_STATUS_KIND_GROUND, main);
}