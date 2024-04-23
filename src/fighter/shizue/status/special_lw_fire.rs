use crate::imports::*;

unsafe extern "C" fn shizue_special_lw_fire_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    let original = original_status(Main, fighter, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_LW_FIRE);
    original(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_LW_FIRE, shizue_special_lw_fire_main);
}