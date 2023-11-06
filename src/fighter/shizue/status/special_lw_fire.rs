use crate::imports::status_imports::*;

#[status_script(agent = "shizue", status = FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_LW_FIRE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe extern "C" fn shizue_special_lw_fire_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    let original = smashline::original_status(smashline::Main, fighter, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_LW_FIRE);
    original(fighter)
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_LW_FIRE, shizue_special_lw_fire_main);
}