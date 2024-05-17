use super::*;

extern "C" {
    #[link_name = "common_fighter_frame"]
    pub fn common_fighter_frame(fighter: &mut L2CFighterCommon);
}

unsafe extern "C" fn check_trick(fighter: &mut L2CFighterCommon) {
    if VarModule::is_flag(fighter.module_accessor, vars::sonic::status::flag::ENABLE_TRICK)
    && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE != 0 {
        VarModule::off_flag(fighter.module_accessor, vars::sonic::status::flag::ENABLE_TRICK);
        fighter.change_status(vars::sonic::status::TRICK.into(), true.into());
    }
}

unsafe extern "C" fn on_main(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    check_trick(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, on_main);
}