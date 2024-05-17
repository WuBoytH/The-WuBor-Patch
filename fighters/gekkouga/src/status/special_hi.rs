use super::*;

unsafe extern "C" fn gekkouga_special_hi_init(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn gekkouga_special_hi_exec(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, gekkouga_special_hi_init);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, gekkouga_special_hi_exec);
}