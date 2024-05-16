use crate::imports::*;

unsafe extern "C" fn gekkouga_special_hi_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
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
    let prev_rot_x = WorkModule::get_float(fighter.module_accessor, *FIGHTER_GEKKOUGA_STATUS_WORK_ID_FLOAT_QUICK_ATTACK_PREV_ROT_X);
    let prev_rot_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_GEKKOUGA_STATUS_WORK_ID_FLOAT_QUICK_ATTACK_PREV_ROT_Y);
    let prev_rot_z = WorkModule::get_float(fighter.module_accessor, *FIGHTER_GEKKOUGA_STATUS_WORK_ID_FLOAT_QUICK_ATTACK_PREV_ROT_Z);
    ModelModule::set_joint_rotate(
        fighter.module_accessor,
        Hash40::new("rot"),
        &Vector3f{x: prev_rot_x, y: prev_rot_y, z: prev_rot_z},
        MotionNodeRotateCompose{_address: 0},
        MotionNodeRotateOrder{_address: 0}
    );

    let next_x = -prev_rot_x / 10.0;
    let next_y = -prev_rot_y / 10.0;
    let next_z = -prev_rot_z / 10.0;

    WorkModule::set_float(fighter.module_accessor, next_x, *FIGHTER_GEKKOUGA_STATUS_WORK_ID_FLOAT_QUICK_ATTACK_ADD_ROT_X);
    WorkModule::set_float(fighter.module_accessor, next_y, *FIGHTER_GEKKOUGA_STATUS_WORK_ID_FLOAT_QUICK_ATTACK_ADD_ROT_Y);
    WorkModule::set_float(fighter.module_accessor, next_z, *FIGHTER_GEKKOUGA_STATUS_WORK_ID_FLOAT_QUICK_ATTACK_ADD_ROT_Z);
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_HI_END, gekkouga_special_hi_end_pre);
}