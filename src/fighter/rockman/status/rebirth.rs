use crate::imports::status_imports::*;

#[status_script(agent = "rockman", status = FIGHTER_STATUS_KIND_REBIRTH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe extern "C" fn rockman_rebirth_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mot = MotionModule::motion_kind(fighter.module_accessor);
    if [
        hash40("appeal_lw_l"),
        hash40("appeal_lw_r")
    ].contains(&mot) {
        VisibilityModule::set_whole(fighter.module_accessor, true);
    }
    fighter.status_end_Rebirth();
    0.into()
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::End, *FIGHTER_STATUS_KIND_LADDER_ATTACK, rockman_rebirth_end);
}