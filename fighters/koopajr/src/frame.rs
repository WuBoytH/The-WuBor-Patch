use super::*;

extern "C" {
    #[link_name = "common_fighter_frame"]
    pub fn common_fighter_frame(fighter: &mut L2CFighterCommon);
}

unsafe extern "C" fn down_throw_mash(fighter: &mut L2CFighterCommon) {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("throw_lw")
    && AttackModule::is_attack(fighter.module_accessor, 0, false)
    && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        VarModule::inc_int(fighter.module_accessor, vars::koopajr::status::int::THROW_LW_MASH_COUNT);
        let mash_count = VarModule::get_int(fighter.module_accessor, vars::koopajr::status::int::THROW_LW_MASH_COUNT);
        if mash_count <= 6 {
            MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_GAME, Hash40::new("game_throwlwmash"), -1);
        }
    }
}

unsafe extern "C" fn on_main(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    down_throw_mash(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, on_main);
}