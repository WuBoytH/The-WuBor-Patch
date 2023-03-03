use crate::imports::status_imports::*;

#[status_script(agent = "kirby", status = FIGHTER_KIRBY_STATUS_KIND_SIMON_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn kirby_simon_specialn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    belmont_special_n_main_inner(fighter)
}

#[status_script(agent = "kirby", status = FIGHTER_KIRBY_STATUS_KIND_RICHTER_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn kirby_richter_specialn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    belmont_special_n_main_inner(fighter)
}

pub unsafe fn belmont_special_n_main_inner(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mot_g;
    let mot_a;
    let mut log =
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N |
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
        *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
    ;
    let axe_id = VarModule::get_int(fighter.battle_object, simon::instance::int::AXE_ID) as u32;
    let is_axe = sv_battle_object::category(axe_id) == *BATTLE_OBJECT_CATEGORY_WEAPON
    && [
        *WEAPON_KIND_SIMON_AXE,
        *WEAPON_KIND_RICHTER_AXE
    ].contains(&sv_battle_object::kind(axe_id));
    if ItemModule::is_have_item(fighter.module_accessor, 0) // Usually this only checks for if you hold Simon or Richter's Holy Water
    || !sv_battle_object::is_active(axe_id)
    || !is_axe {
        mot_g = hash40("special_n");
        mot_a = hash40("special_air_n");
        log |= *FIGHTER_LOG_MASK_FLAG_SHOOT;
        if !ItemModule::is_have_item(fighter.module_accessor, 0) {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE, false, 0);
        }
    }
    else {
        mot_g = hash40("special_n_blank");
        mot_a = hash40("special_air_n_blank");
    }
    WorkModule::set_int64(fighter.module_accessor, mot_g as i64, *FIGHTER_SIMON_STATUS_SPECIAL_N_INT_MOTION);
    WorkModule::set_int64(fighter.module_accessor, mot_a as i64, *FIGHTER_SIMON_STATUS_SPECIAL_N_INT_MOTION_AIR);
    FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, log as u64);
    fighter.sub_shift_status_main(L2CValue::Ptr(belmont_special_n_main_loop as *const () as _))
}

unsafe extern "C" fn belmont_special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, simon::status::flag::SPECIAL_N_SHOOT) {
        if ItemModule::is_have_item(fighter.module_accessor, 0) {
            let angle = if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_KIRBY_STATUS_KIND_SIMON_SPECIAL_N {
                45.0
            }
            else {
                69.420
            };
            ItemModule::throw_item(
                fighter.module_accessor,
                angle,
                2.4,
                1.0,
                0,
                true,
                1.0
            );
        }
        else {
            ArticleModule::shoot(fighter.module_accessor, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
        }
        VarModule::off_flag(fighter.battle_object, simon::status::flag::SPECIAL_N_SHOOT);
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
        }
    }
    if changing
    || StatusModule::is_situation_changed(fighter.module_accessor) {
        let mot;
        let kinetic;
        let correct;
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.set_situation(SITUATION_KIND_GROUND.into());
            mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_N_INT_MOTION);
            kinetic = *FIGHTER_KINETIC_TYPE_GROUND_STOP;
            correct = *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK;
        }
        else {
            fighter.set_situation(SITUATION_KIND_AIR.into());
            mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_N_INT_MOTION_AIR);
            kinetic = *FIGHTER_KINETIC_TYPE_AIR_STOP;
            correct = *GROUND_CORRECT_KIND_AIR;
        }
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));
        if kinetic != FIGHTER_KINETIC_TYPE_NONE {
            KineticModule::change_kinetic(fighter.module_accessor, kinetic);
            if changing {
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
    }
    if ItemModule::is_have_item(fighter.module_accessor, 0) {
        ItemModule::set_have_item_constraint_joint(
            fighter.module_accessor,
            Hash40::new("haver"),
            0
        );
    }
    0.into()
}

#[status_script(agent = "kirby", status = FIGHTER_KIRBY_STATUS_KIND_SIMON_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn kirby_simon_specialn_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    belmont_special_n_end_inner(fighter)
}

#[status_script(agent = "kirby", status = FIGHTER_KIRBY_STATUS_KIND_RICHTER_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn kirby_richter_specialn_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    belmont_special_n_end_inner(fighter)
}

pub unsafe fn belmont_special_n_end_inner(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_N_FLAG_HAVE_AXE) {
        ArticleModule::remove(fighter.module_accessor, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST));
    }
    ItemModule::set_have_item_constraint_joint(
        fighter.module_accessor,
        Hash40::new("havel"),
        0
    );
    0.into()
}

pub fn install() {
    install_status_scripts!(
        kirby_simon_specialn_main,
        kirby_richter_specialn_main,
        kirby_simon_specialn_end,
        kirby_richter_specialn_end
    );
}