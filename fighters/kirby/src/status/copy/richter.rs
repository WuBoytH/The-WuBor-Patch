use super::*;

unsafe extern "C" fn richter_special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
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
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON |
            *FIGHTER_LOG_MASK_FLAG_SHOOT
        ) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );
    0.into()
}

pub unsafe extern "C" fn richter_mot_kinetic_helper(
    fighter: &mut L2CFighterCommon,
    some_bool: L2CValue,
    mot_g: L2CValue,
    mot_a: L2CValue,
    kinetic_g: L2CValue,
    kinetic_a: L2CValue,
    correct_g: L2CValue,
    correct_a: L2CValue
) -> L2CValue {
    if !some_bool.get_bool()
    && !StatusModule::is_situation_changed(fighter.module_accessor) {
        return false.into();
    }
    let mot;
    let kinetic;
    let correct;
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        mot = mot_g.get_u64();
        kinetic = kinetic_g.get_i32();
        correct = correct_g.get_i32();
    }
    else {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        mot = mot_a.get_u64();
        kinetic = kinetic_a.get_i32();
        correct = correct_a.get_i32();
    }
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));
    if kinetic != FIGHTER_KINETIC_TYPE_NONE {
        KineticModule::change_kinetic(fighter.module_accessor, kinetic);
        if some_bool.get_bool() {
            FighterMotionModuleImpl::change_motion_kirby_copy(
                fighter.module_accessor,
                Hash40::new_raw(mot),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
        }
        else {
            FighterMotionModuleImpl::change_motion_inherit_frame_kirby_copy(
                fighter.module_accessor,
                Hash40::new_raw(mot),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
    }
    true.into()
}

unsafe extern "C" fn richter_special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mot_g;
    let mot_a;
    if ItemModule::is_have_item(fighter.module_accessor, 0) // Usually this only checks for if you hold Simon or Richter's Holy Water
    || ArticleModule::is_generatable(fighter.module_accessor, *FIGHTER_SIMON_GENERATE_ARTICLE_HOLYWATER) {
        mot_g = hash40("special_n");
        mot_a = hash40("special_air_n");
    }
    else {
        mot_g = hash40("special_n_blank");
        mot_a = hash40("special_air_n_blank");
    }
    WorkModule::set_int64(fighter.module_accessor, mot_g, *FIGHTER_SIMON_STATUS_SPECIAL_N_INT_MOTION);
    WorkModule::set_int64(fighter.module_accessor, mot_a, *FIGHTER_SIMON_STATUS_SPECIAL_N_INT_MOTION_AIR);

    fighter.sub_shift_status_main(L2CValue::Ptr(richter_special_n_main_loop as *const () as _))
}

unsafe extern "C" fn richter_special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_LW_FLAG_GENERATE_HOLYWATER) {
        if !ItemModule::is_have_item(fighter.module_accessor, 0) {
            if !ItemModule::is_have_item(fighter.module_accessor, *FIGHTER_HAVE_ITEM_WORK_EXTRA) {
                ArticleModule::generate_article_have_item(
                    fighter.module_accessor,
                    *FIGHTER_SIMON_GENERATE_ARTICLE_HOLYWATER,
                    *FIGHTER_HAVE_ITEM_WORK_EXTRA,
                    Hash40::new("invalid")
                );
                ItemModule::set_have_item_constraint_joint(
                    fighter.module_accessor,
                    Hash40::new("haver"),
                    *FIGHTER_HAVE_ITEM_WORK_EXTRA
                );
                let mot = MotionModule::motion_kind_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_HAVE_ITEM);
                if mot == 0x10ba1c049e {
                    MotionModule::remove_motion_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_HAVE_ITEM, false);
                }
            }
        }
        else {
            ItemModule::set_have_item_constraint_joint(
                fighter.module_accessor,
                Hash40::new("haver"),
                0
            );
        }
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_LW_FLAG_GENERATE_HOLYWATER);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_LW_FLAG_SHOOT_HOLYWATER) {
        let item_kind = ItemModule::get_have_item_kind(fighter.module_accessor, 0);
        let item_kind_extra = ItemModule::get_have_item_kind(fighter.module_accessor, *FIGHTER_HAVE_ITEM_WORK_EXTRA);
        let is_holywater;
        let item_part = if [
            *ITEM_KIND_SIMONHOLYWATER, *ITEM_KIND_RICHTERHOLYWATER
        ].contains(&item_kind_extra) {
            is_holywater = item_kind_extra == *ITEM_KIND_RICHTERHOLYWATER;
            ArticleModule::shoot_exist(
                fighter.module_accessor,
                *FIGHTER_SIMON_GENERATE_ARTICLE_HOLYWATER,
                ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL),
                false
            );
            *FIGHTER_HAVE_ITEM_WORK_EXTRA
        }
        else if ItemModule::is_have_item(fighter.module_accessor, 0) {
            is_holywater = item_kind == *ITEM_KIND_RICHTERHOLYWATER;
            0
        }
        else {
            is_holywater = false;
            -1
        };
        if item_part != -1 {
            if is_holywater {
                ItemModule::set_have_item_action(fighter.module_accessor, *ITEM_HOLYWATER_ACTION_SPECIAL_THROW, 0.0, item_part);
            }
            ItemModule::throw_item(
                fighter.module_accessor,
                -45.0,
                3.0,
                1.0,
                item_part,
                true,
                1.0
            );
        }
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_LW_FLAG_SHOOT_HOLYWATER);
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    let changing = StatusModule::is_changing(fighter.module_accessor);
    if !changing {
        if MotionModule::is_end(fighter.module_accessor) {
            let status = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                *FIGHTER_STATUS_KIND_WAIT
            }
            else {
                *FIGHTER_STATUS_KIND_FALL
            };
            fighter.change_status(status.into(), false.into());
            return 0.into();
        }

        if StatusModule::is_situation_changed(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
                return 0.into();
            }
        }
    }
    let mot_g = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_N_INT_MOTION);
    let mot_a = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_N_INT_MOTION_AIR);
    richter_mot_kinetic_helper(
        fighter,
        changing.into(),
        mot_g.into(),
        mot_a.into(),
        FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),
        FIGHTER_KINETIC_TYPE_AIR_STOP.into(),
        GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK.into(),
        GROUND_CORRECT_KIND_AIR.into()
    );
    0.into()
}

unsafe extern "C" fn richter_special_n_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if [
        *ITEM_KIND_SIMONHOLYWATER,
        *ITEM_KIND_RICHTERHOLYWATER
    ].contains(&ItemModule::get_have_item_kind(fighter.module_accessor, *FIGHTER_HAVE_ITEM_WORK_EXTRA)) {
        ArticleModule::remove(fighter.module_accessor, *FIGHTER_SIMON_GENERATE_ARTICLE_HOLYWATER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    ItemModule::set_have_item_constraint_joint(
        fighter.module_accessor,
        Hash40::new("havel"),
        0
    );
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_KIRBY_STATUS_KIND_RICHTER_SPECIAL_N, richter_special_n_pre);
    agent.status(Main, *FIGHTER_KIRBY_STATUS_KIND_RICHTER_SPECIAL_N, richter_special_n_main);
    agent.status(End, *FIGHTER_KIRBY_STATUS_KIND_RICHTER_SPECIAL_N, richter_special_n_end);
}
