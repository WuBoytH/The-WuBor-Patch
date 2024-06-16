use super::*;

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    VarModule::set_int(fighter.module_accessor, vars::samusd::instance::int::CSHOT_ID, *BATTLE_OBJECT_ID_INVALID);
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);
}
