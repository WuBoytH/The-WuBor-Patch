use crate::imports::status_imports::*;

unsafe extern "C" fn chrom_attack_dash_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3);
    1.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Pre, *FIGHTER_STATUS_KIND_ATTACK_DASH, chrom_attack_dash_pre);
}