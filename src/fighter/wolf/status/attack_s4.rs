use crate::imports::status_imports::*;

unsafe extern "C" fn wolf_attack_s4_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let kinetic = if status == *FIGHTER_STATUS_KIND_DASH {
        *FIGHTER_KINETIC_TYPE_MOTION
    }
    else {
        *FIGHTER_KINETIC_TYPE_MOTION_RUN_STOP
    };
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        kinetic,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_FLOAT,
        *FS_SUCCEEDS_KEEP_HIT | *FS_SUCCEEDS_KEEP_VISIBILITY | *FS_SUCCEEDS_KEEP_NO_REACTION
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ATTACK_S4 |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_HAJIKI
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_4 as u32,
        0
    );
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Pre, *FIGHTER_STATUS_KIND_ATTACK_S4, wolf_attack_s4_pre);
}