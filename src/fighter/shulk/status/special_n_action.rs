use crate::imports::*;

unsafe extern "C" fn shulk_special_n_action_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let start_attr = if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_SPECIAL_N {
        0
    }
    else {
        *FIGHTER_STATUS_ATTR_START_TURN
    };
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N as u64,
        start_attr as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_N_ACTION, shulk_special_n_action_pre);
}