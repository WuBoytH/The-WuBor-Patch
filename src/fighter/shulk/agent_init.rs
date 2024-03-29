use {
    crate::imports::status_imports::*,
    crate::fighter::common::agent_inits::*
};

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_SHULK_MONAD_TYPE_NONE, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_TYPE);
    fighter.global_table[CHECK_SPECIAL_LW_UNIQ].assign(&L2CValue::Ptr(speciallw_pre_generic as *const () as _));
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_start(on_start);
}