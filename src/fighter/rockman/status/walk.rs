use crate::imports::*;
use super::rockbuster::helper::*;

unsafe extern "C" fn rockman_walk_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_walk_uniq_process_main_common(
        hash40("walk_fast").into(),
        hash40("walk_middle").into(),
        hash40("walk_slow").into(),
        L2CValue::Ptr(rockman_rockbuster_walk_mot_helper as *const () as _)
    );
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Exec, *FIGHTER_STATUS_KIND_WALK, rockman_walk_exec);
}