use crate::imports::status_imports::*;

unsafe extern "C" fn gekkouga_special_hi_init(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn gekkouga_special_hi_exec(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, gekkouga_special_hi_init);
    agent.status(smashline::Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, gekkouga_special_hi_exec);
}