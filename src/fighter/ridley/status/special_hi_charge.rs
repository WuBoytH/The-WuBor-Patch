use crate::imports::status_imports::*;

#[status_script(agent = "ridley", status = FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn ridley_special_hi_charge_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    ridley_special_hi_charge_pre_inner(fighter)
}

#[status_script(agent = "ridley", status = FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_F, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn ridley_special_hi_charge_f_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    ridley_special_hi_charge_pre_inner(fighter)
}

#[status_script(agent = "ridley", status = FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_B, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn ridley_special_hi_charge_b_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    ridley_special_hi_charge_pre_inner(fighter)
}

#[status_script(agent = "ridley", status = FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn ridley_special_hi_charge_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    ridley_special_hi_charge_pre_inner(fighter)
}

unsafe fn ridley_special_hi_charge_pre_inner(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), // Was ALWAYS_BOTH_SIDES
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}

pub fn install() {
    install_status_scripts!(
        ridley_special_hi_charge_hi_pre,
        ridley_special_hi_charge_f_pre,
        ridley_special_hi_charge_b_pre,
        ridley_special_hi_charge_lw_pre
    );
}