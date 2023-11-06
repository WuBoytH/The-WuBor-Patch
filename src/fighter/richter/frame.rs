use {
    smash::{
        lua2cpp::*,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smashline::*,
    custom_var::*,
    wubor_utils::{wua_bind::*, vars::*}
};

#[fighter_frame( agent = FIGHTER_KIND_RICHTER, main )]
fn richter_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if VarModule::is_flag(fighter.module_accessor, fighter::instance::flag::DISABLE_SPECIAL_HI)
        && (StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH
        || StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
        || MiscModule::is_damage_check(fighter.module_accessor, false)) {
            VarModule::off_flag(fighter.module_accessor, fighter::instance::flag::DISABLE_SPECIAL_HI);
        }
    }
}

pub fn install(agent : &mut smashline::Agent) {
    agent.on_line(smashline::Main, richter_frame);
}