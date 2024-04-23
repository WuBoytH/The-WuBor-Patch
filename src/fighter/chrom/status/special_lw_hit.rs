use crate::imports::*;

unsafe extern "C" fn chrom_speciallw_hit_init(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn chrom_speciallw_hit_exec(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_HIT, chrom_speciallw_hit_init);
    agent.status(Exec, *FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_HIT, chrom_speciallw_hit_exec);
}