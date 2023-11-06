use crate::imports::status_imports::*;

#[status_script(agent = "rockman", status = FIGHTER_STATUS_KIND_ATTACK_S3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe extern "C" fn rockman_attack_s3_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_AttackS3()
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Pre, *FIGHTER_STATUS_KIND_ATTACK_S3, rockman_attack_s3_pre);
}