use super::*;
use super::helper::*;

#[no_mangle]
pub unsafe extern "C" fn belmont_special_n_main_inner(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mot_g;
    let mot_a;
    let mut log =
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N |
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
        *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
    ;
    let axe_id = VarModule::get_int(fighter.module_accessor, vars::simon::instance::int::AXE_ID) as u32;
    let axe_kind = if fighter.global_table[KIND].get_i32() == *FIGHTER_KIND_SIMON {
        *WEAPON_KIND_SIMON_AXE
    }
    else {
        *WEAPON_KIND_RICHTER_AXE
    };
    let is_axe = sv_battle_object::category(axe_id) == *BATTLE_OBJECT_CATEGORY_WEAPON
    && sv_battle_object::kind(axe_id) == axe_kind;
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

#[no_mangle]
unsafe extern "C" fn belmont_special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, vars::simon::status::flag::SPECIAL_N_SHOOT) {
        if ItemModule::is_have_item(fighter.module_accessor, 0) {
            let angle = if fighter.global_table[KIND].get_i32() == *FIGHTER_KIND_SIMON {
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
        VarModule::off_flag(fighter.module_accessor, vars::simon::status::flag::SPECIAL_N_SHOOT);
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
    let mot_g = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_N_INT_MOTION);
    let mot_a = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_N_INT_MOTION_AIR);
    belmont_mot_kinetic_helper(
        fighter,
        changing.into(),
        mot_g.into(),
        mot_a.into(),
        FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),
        FIGHTER_KINETIC_TYPE_AIR_STOP.into(),
        GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK.into(),
        GROUND_CORRECT_KIND_AIR.into()
    );
    if ItemModule::is_have_item(fighter.module_accessor, 0) {
        ItemModule::set_have_item_constraint_joint(
            fighter.module_accessor,
            Hash40::new("haver"),
            0
        );
    }
    0.into()
}

#[no_mangle]
pub unsafe extern "C" fn belmont_special_n_end_inner(fighter: &mut L2CFighterCommon) -> L2CValue {
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

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, belmont_special_n_main_inner);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, belmont_special_n_end_inner);
}