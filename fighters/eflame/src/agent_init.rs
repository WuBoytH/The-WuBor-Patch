use super::*;

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[CHECK_AIR_ITEM_THROW_UNIQ].assign(&L2CValue::Bool(false));
    fighter.global_table[FALL_BRAKE_UNIQ].assign(&L2CValue::Bool(false));
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);
}
