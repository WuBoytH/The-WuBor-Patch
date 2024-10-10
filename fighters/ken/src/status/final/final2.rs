use super::*;

unsafe extern "C" fn ken_final2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.module_accessor, vars::ken::instance::flag::SKIP_FINAL_PROX_CHECK);

    let kinetic = if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        *FIGHTER_KINETIC_TYPE_MOTION
    }
    else {
        *FIGHTER_KINETIC_TYPE_MOTION_AIR
    };
    KineticModule::change_kinetic(fighter.module_accessor, kinetic);

    let final_shake_scale = WorkModule::get_param_float(fighter.module_accessor, hash40("param_private"), hash40("final_shake_scale"));
    AttackModule::set_damage_shake_scale(fighter.module_accessor, final_shake_scale);

    let color = FighterUtil::renderer_get_clear_color();
    let rate = FighterUtil::renderer_get_color_rate();
    WorkModule::set_float(fighter.module_accessor, color.x, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLOAT_ORIGINAL_BG_COLOR_R);
    WorkModule::set_float(fighter.module_accessor, color.y, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLOAT_ORIGINAL_BG_COLOR_G);
    WorkModule::set_float(fighter.module_accessor, color.z, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLOAT_ORIGINAL_BG_COLOR_B);
    WorkModule::set_float(fighter.module_accessor, color.w, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLOAT_ORIGINAL_BG_COLOR_A);
    WorkModule::set_float(fighter.module_accessor, rate, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLOAT_ORIGINAL_COLOR_RATE);

    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_INT_INVISIBLE_STAGE_TIME);
    WorkModule::set_flag(fighter.module_accessor, false, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_INVISIBLE_STAGE);

    ken_final_set_area(fighter, false.into());

    ModelModule::disable_gold_eye(fighter.module_accessor);

    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x201bc9217c));

    // HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);

    let mot = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        "final2"
    }
    else {
        "final2_air"
    };
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new(mot),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    ItemModule::set_change_status_event(fighter.module_accessor, false);

    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KEN_GENERATE_ARTICLE_SHINRYUKEN, false, -1);

    let main_loop = smashline::api::get_target_function("lua2cpp_ken.nrs", 0x3e630).unwrap();
    fighter.sub_shift_status_main(L2CValue::Ptr(main_loop as *const () as _))
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_RYU_STATUS_KIND_FINAL2, ken_final2_main);
}