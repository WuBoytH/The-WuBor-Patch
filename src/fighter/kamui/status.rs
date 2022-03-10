use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    wubor_utils::table_const::*
};

#[status_script(agent = "kamui", status = FIGHTER_KAMUI_STATUS_KIND_SPECIAL_LW_HIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn kamui_speciallw_hit_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        *FS_SUCCEEDS_KEEP_VISIBILITY
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

#[status_script(agent = "kamui", status = FIGHTER_KAMUI_STATUS_KIND_SPECIAL_LW_HIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn kamui_speciallw_hit_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
    kamui_speciallw_hit_mot(fighter);
    kamui_speciallw_hit_dragon_mot(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(kamui_speciallw_hit_main_loop as *const () as _))
}

unsafe extern "C" fn kamui_speciallw_hit_mot(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("special_air_lw_hit"),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame(
            fighter.module_accessor,
            Hash40::new("special_air_lw_hit"),
            -1.0,
            1.0,
            0.0,
            false,
            false);
        }
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("special_lw_hit"),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame(
                fighter.module_accessor,
                Hash40::new("special_lw_hit"),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
    }
    return
}

unsafe extern "C" fn kamui_speciallw_hit_dragon_mot(fighter: &mut L2CFighterCommon) {
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
                ArticleModule::change_motion(
                    fighter.module_accessor,
                    *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,
                    Hash40::new("special_air_lw_hit"),
                    false,
                    -1.0
                );
            }
            else {
                ArticleModule::change_motion(
                    fighter.module_accessor,
                    *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,
                    Hash40::new("special_air_lw_hit"),
                    true,
                    -1.0
                );
            }
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
                ArticleModule::change_motion(
                    fighter.module_accessor,
                    *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,
                    Hash40::new("special_lw_hit"),
                    false,
                    -1.0);
            }
            else {
                ArticleModule::change_motion(
                    fighter.module_accessor,
                    *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,
                    Hash40::new("special_lw_hit"),
                    true,
                    -1.0
                );
            }
        }
    }
    return
}

unsafe extern "C" fn kamui_speciallw_hit_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool()
    && fighter.sub_air_check_fall_common().get_bool() == true {
        return 1.into();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        kamui_speciallw_hit_mot(fighter);
        kamui_speciallw_hit_dragon_mot(fighter);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    0.into()
}

#[status_script(agent = "kamui", status = FIGHTER_KAMUI_STATUS_KIND_SPECIAL_LW_HIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn kamui_speciallw_hit_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_FINAL_VISUAL_ATTACK_OTHER
    && ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON) {
        ArticleModule::remove_exist(
            fighter.module_accessor,
            *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,
            ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)
        );
        VisibilityModule::set_whole(fighter.module_accessor, true);
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        kamui_speciallw_hit_pre,
        kamui_speciallw_hit_main,
        kamui_speciallw_hit_end
    );
}