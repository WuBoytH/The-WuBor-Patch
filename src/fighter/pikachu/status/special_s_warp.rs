use crate::imports::*;

// Quick Attack is now on side b lol

unsafe extern "C" fn pikachu_special_s_warp_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
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
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK
        ) as u64,
        *FIGHTER_STATUS_ATTR_IGNORE_SPEED_LIMIT as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn pikachu_special_s_warp_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_INT_QUICK_ATTACK_COUNT);
    if count == 0 {
        let speed_x = WorkModule::get_float(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLOAT_QUICK_ATTACK_WARP_SPEED_X);
        let speed_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLOAT_QUICK_ATTACK_WARP_SPEED_Y);
        PostureModule::set_lr(fighter.module_accessor, speed_x.signum());
        PostureModule::update_rot_y_lr(fighter.module_accessor);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            speed_x,
            speed_y
        );
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            speed_x,
            speed_y
        );
        let lr = PostureModule::lr(fighter.module_accessor);
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            speed_x * lr,
            speed_y
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        // WorkModule::set_float(fighter.module_accessor, lr, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLOAT_QUICK_ATTACK_PREV_STICK_X);
        // WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLOAT_QUICK_ATTACK_PREV_STICK_Y);
        0.into()
    }
    else {
        let original = original_status(Init, fighter, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP);
        original(fighter)
    }
}

unsafe extern "C" fn pikachu_special_s_warp_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    let count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_INT_QUICK_ATTACK_COUNT);
    if count != 0 {
        let original = original_status(Main, fighter, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP);
        return original(fighter);
    }
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_SPECIALUPDUMMY, false, -1);
    ArticleModule::change_motion(
        fighter.module_accessor,
        *FIGHTER_PIKACHU_GENERATE_ARTICLE_SPECIALUPDUMMY,
        Hash40::new("special_air_hi_1"),
        false,
        -1.0
    );
    ArticleModule::set_frame(fighter.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_SPECIALUPDUMMY, 0.0);
    let special_hi_move_time_ = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_move_time_"));
    WorkModule::set_int(fighter.module_accessor, special_hi_move_time_, *FIGHTER_PIKACHU_STATUS_WORK_ID_INT_QUICK_ATTACK_MOVE_TIME_COUNTER);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_air_hi1"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(pikachu_special_s_warp_1_main_loop as *const () as _))
}

unsafe extern "C" fn pikachu_special_s_warp_1_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if !fighter.global_table[IS_STOP].get_bool() {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_INT_QUICK_ATTACK_MOVE_TIME_COUNTER);
    }
    let counter = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_INT_QUICK_ATTACK_MOVE_TIME_COUNTER);
    if counter >= 0 {
        if !fighter.global_table[IS_STOP].get_bool() {
            if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32)
            || GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32) {
                fighter.change_status(FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END.into(), false.into());
                return 0.into();
            }
        }
    }
    else {
        fighter.change_status(FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END.into(), false.into());
        return 1.into();
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP, pikachu_special_s_warp_pre);
    agent.status(Init, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP, pikachu_special_s_warp_init);
    agent.status(Main, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP, pikachu_special_s_warp_main);
}