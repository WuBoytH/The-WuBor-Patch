use super::*;

unsafe extern "C" fn propeller_mash(weapon: &mut L2CWeaponCommon) {
    if AttackModule::is_attack(weapon.module_accessor, 0, false)
    && WorkModule::is_flag(weapon.module_accessor, 0x1000000C) {
        WorkModule::off_flag(weapon.module_accessor, 0x1000000C);
        VarModule::inc_int(weapon.module_accessor, vars::krool_backpack::instance::int::SPECIAL_HI_MASHED_COUNT);
        let mashed_count = VarModule::get_int(weapon.module_accessor, vars::krool_backpack::instance::int::SPECIAL_HI_MASHED_COUNT);
        if mashed_count < 8 {
            MotionAnimcmdModule::call_script_single(weapon.module_accessor, *FIGHTER_ANIMCMD_GAME, Hash40::new("game_flymash"), -1);
        }
    }
}

unsafe extern "C" fn on_main(weapon: &mut L2CWeaponCommon) {
    propeller_mash(weapon);
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, on_main);
}