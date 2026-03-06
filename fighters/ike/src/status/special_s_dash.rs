use super::*;
use super::helper;

unsafe extern "C" fn ike_special_s_dash_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.module_accessor, vars::fighter::instance::flag::DISABLE_SPECIAL_S);
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false, // Disables Jostle
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
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn ike_special_s_dash_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);

    // let offset_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_hit_offset_x"));
    // let offset_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_hit_offset_y"));
    // let size = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_hit_size"));
    // search!(
    //     fighter,
    //     MA_MSC_CMD_SEARCH_SEARCH_FP,
    //     0,
    //     0,
    //     Hash40::new("rot"),
    //     size,
    //     0.0,
    //     offset_y,
    //     offset_x,
    //     COLLISION_KIND_MASK_HIT,
    //     (*COLLISION_CATEGORY_MASK_ITEM | *COLLISION_CATEGORY_MASK_FIGHTER | *COLLISION_CATEGORY_MASK_ENEMY),
    //     COLLISION_SITUATION_MASK_ALL,
    //     0,
    //     COLLISION_PART_MASK_ALL,
    //     HIT_STATUS_MASK_ALL,
    //     false,
    //     0,
    //     false,
    //     COLLISION_SHAPE_TYPE_SPHERE,
    //     false
    // );

    let count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_S_WORK_INT_CHARGE_COUNT);
    let spd_up_max = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_charge_dash_spd_up_max"));
    let count = count.min(spd_up_max);

    let mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("dash_speed_x_mul"));

    let params = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        (
            "special_s_air_dash_spd_min",
            "special_s_air_dash_spd_mul",
            "special_s_air_dash_brake_x",
            ENERGY_STOP_RESET_TYPE_AIR,
            *SITUATION_KIND_AIR
        )
    }
    else {
        (
            "special_s_ground_dash_spd_min",
            "special_s_ground_dash_spd_mul",
            "special_s_ground_dash_brake_x",
            ENERGY_STOP_RESET_TYPE_GROUND,
            *SITUATION_KIND_GROUND
        )
    };

    let spd_min = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40(params.0));
    let spd_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40(params.1));

    let spd_base = (spd_min + (spd_mul * count as f32)) * lr;

    let spd_brake = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40(params.2));

    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        params.3,
        spd_base * mul,
        0.0,
        0.0,
        0.0,
        0.0
    );

    sv_kinetic_energy!(
        set_brake,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        spd_brake,
        0.0
    );

    WorkModule::set_int(fighter.module_accessor, params.4, *FIGHTER_IKE_STATUS_SPECIAL_S_WORK_INT_SITUATION_PREV);

    sv_kinetic_energy!(
        set_limit_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        -1.0,
        0.0
    );

    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, fighter.module_accessor);
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);

    0.into()
}

unsafe extern "C" fn ike_special_s_dash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    GroundModule::select_cliff_hangdata(fighter.module_accessor, 1);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT);
    let special_s_dash_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_dash_frame"));
    WorkModule::set_int(fighter.module_accessor, special_s_dash_frame, *FIGHTER_IKE_STATUS_SPECIAL_S_WORK_INT_DASH_COUNT);
    if !StopModule::is_stop(fighter.module_accessor) {
        ike_special_s_dash_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(ike_special_s_dash_substatus as *const () as _));
    ike_special_s_dash_mot_helper(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(ike_special_s_dash_main_loop as *const () as _))
}

unsafe extern "C" fn ike_special_s_dash_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_S_WORK_INT_DASH_COUNT);
    }
    0.into()
}

unsafe extern "C" fn ike_special_s_dash_mot_helper(fighter: &mut L2CFighterCommon) {
    let mot = if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
        Hash40::new("special_air_s_dash")
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_NONE.into());
        Hash40::new("special_s_dash")
    };
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT) {
        MotionModule::change_motion(
            fighter.module_accessor,
            mot,
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT);
    }
    else {
        MotionModule::change_motion_inherit_frame(
            fighter.module_accessor,
            mot,
            -1.0,
            1.0,
            0.0,
            false,
            false
        );
    }
}

unsafe extern "C" fn ike_special_s_dash_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }
    if fighter.sub_ground_check_stop_wall().get_bool() {
        return 0.into();
    }
    let dash_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_S_WORK_INT_DASH_COUNT);
    if dash_count <= 0 {
        fighter.change_status(FIGHTER_IKE_STATUS_KIND_SPECIAL_S_END.into(), false.into());
        return 0.into();
    }
    // <WuBor>
    let pad_flag = fighter.global_table[PAD_FLAG].get_i32();
    // Press Attack or Special to attack
    if pad_flag & (*FIGHTER_PAD_FLAG_ATTACK_TRIGGER | *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER) != 0 {
        fighter.change_status(FIGHTER_IKE_STATUS_KIND_SPECIAL_S_ATTACK.into(), false.into());
        return 0.into();
    }
    // Press Shield to end the dash early
    if pad_flag & *FIGHTER_PAD_FLAG_GUARD_TRIGGER != 0 {
        fighter.change_status(FIGHTER_IKE_STATUS_KIND_SPECIAL_S_END.into(), false.into());
        return 0.into();
    }
    // </WuBor>
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        ike_special_s_dash_mot_helper(fighter);
    }
    helper::special_s::ike_special_s_main_loop_helper(fighter);
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_DASH, ike_special_s_dash_pre);
    agent.status(Init, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_DASH, ike_special_s_dash_init);
    agent.status(Main, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_DASH, ike_special_s_dash_main);
}