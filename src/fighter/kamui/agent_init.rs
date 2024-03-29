use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    wubor_utils::table_const::*
};

unsafe extern "C" fn kamui_escapeair_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_s_jump") {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
    }
    0.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[CHECK_AIR_ESCAPE_UNIQ].assign(&L2CValue::Ptr(kamui_escapeair_pre as *const () as _));
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_start(on_start);
}
