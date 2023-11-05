use crate::imports::status_imports::*;

#[status_script(agent = "chrom", status = FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_HIT, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe extern "C" fn chrom_speciallw_hit_init(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

#[status_script(agent = "chrom", status = FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_HIT, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe extern "C" fn chrom_speciallw_hit_exec(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Init, *FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_HIT, chrom_speciallw_hit_init);
    agent.status(smashline::Exec, *FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_HIT, chrom_speciallw_hit_exec);
}