use {
    smash::{
        lua2cpp::L2CFighterCommon,
        // phx::*,
        app::*,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    custom_var::*,
    // custom_cancel::*,
    wubor_utils::{vars::*, table_const::*},
    crate::fighter::common::agent_inits::*
};

pub unsafe extern "C" fn ike_status_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR
    || status == *FIGHTER_STATUS_KIND_REBIRTH {
        VarModule::off_flag(fighter.module_accessor, fighter::instance::flag::DISABLE_SPECIAL_S);
    }
    0.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_IKE {
            return;
        }
        fighter.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(specials_pre_generic as *const () as _));
        fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(ike_status_end_control as *const () as _));
    }
}

pub fn install(agent : &mut smashline::Agent) {
    agent.on_start(on_start);
}
