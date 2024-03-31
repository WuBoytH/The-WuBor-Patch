use crate::imports::*;

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    VarModule::add_reset_statuses(
        fighter.battle_object_id,
        *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_LW_HIT,
        vec![
            *FIGHTER_STATUS_KIND_SPECIAL_LW
        ]
    );
    VarModule::add_reset_statuses(
        fighter.battle_object_id,
        *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_LW_TURN,
        vec![
            *FIGHTER_STATUS_KIND_SPECIAL_LW
        ]
    );
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_start(on_start);
}
