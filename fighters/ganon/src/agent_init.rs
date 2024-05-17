use super::*;

extern "C" {
    #[link_name = "specialn_pre_generic"]
    pub fn specialn_pre_generic(fighter: &mut L2CFighterCommon) -> L2CValue;
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[CHECK_SPECIAL_N_UNIQ].assign(&L2CValue::Ptr(specialn_pre_generic as *const () as _));
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_start(on_start);
}
