use {
    crate::imports::*,
    crate::fighter::common::frame::common_fighter_frame
};

unsafe extern "C" fn ganon_reset_special_n(fighter: &mut L2CFighterCommon) {
    if VarModule::is_flag(fighter.module_accessor, fighter::instance::flag::DISABLE_SPECIAL_N)
    && (StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_CLIFF
    || StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND) {
        VarModule::off_flag(fighter.module_accessor, fighter::instance::flag::DISABLE_SPECIAL_N);
    }
}

unsafe extern "C" fn ganon_throw_cancel_teleport(fighter: &mut L2CFighterCommon) {
    if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_THROW
    && !CatchModule::is_catch(fighter.module_accessor)
    && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_N.into(), true.into());
    }
}

unsafe extern "C" fn on_main(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    ganon_reset_special_n(fighter);
    ganon_throw_cancel_teleport(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, on_main);
}