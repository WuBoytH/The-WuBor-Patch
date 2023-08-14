use crate::imports::status_imports::*;

#[status("rockman", FIGHTER_STATUS_KIND_ATTACK_S3)]
unsafe fn rockman_attack_s3_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_AttackS3()
}

pub fn install() {
    rockman_attack_s3_pre::install();
}