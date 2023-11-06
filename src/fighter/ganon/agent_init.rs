use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::*,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    wubor_utils::table_const::*,
    crate::fighter::common::agent_inits::*
};

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_GANON {
            return;
        }
        fighter.global_table[CHECK_SPECIAL_N_UNIQ].assign(&L2CValue::Ptr(specialn_pre_generic as *const () as _));
    }
}

pub fn install(agent : &mut smashline::Agent) {
    agent.on_start(on_start);
}
