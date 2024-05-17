use {
    smash::{
        lua2cpp::L2CFighterCommon,
        lib::L2CValue
    },
    wubor_utils::table_const::*,
    crate::fighter::common::agent_inits::*
};

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[CHECK_SPECIAL_N_UNIQ].assign(&L2CValue::Ptr(specialn_pre_generic as *const () as _));
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_start(on_start);
}
