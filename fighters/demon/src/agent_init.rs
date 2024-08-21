use super::*;

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Bool(false));
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);
}