use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::*,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    wubor_utils::table_const::*,
    super::helper::*
};

pub unsafe fn belmont_special_lw_main_inner(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mot_g;
    let mot_a;
    if ItemModule::is_have_item(fighter.module_accessor, 0) // Usually this only checks for if you hold Simon or Richter's Holy Water
    || ArticleModule::is_generatable(fighter.module_accessor, *FIGHTER_SIMON_GENERATE_ARTICLE_HOLYWATER) {
        mot_g = hash40("special_lw");
        mot_a = hash40("special_air_lw");
    }
    else {
        mot_g = hash40("special_lw_blank");
        mot_a = hash40("special_air_lw_blank");
    }
    WorkModule::set_int64(fighter.module_accessor, mot_g as i64, *FIGHTER_SIMON_STATUS_SPECIAL_LW_INT_MOTION);
    WorkModule::set_int64(fighter.module_accessor, mot_a as i64, *FIGHTER_SIMON_STATUS_SPECIAL_LW_INT_MOTION_AIR);
    let sum = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if sum < 0.0 {
        KineticModule::mul_speed(
            fighter.module_accessor,
            &Vector3f{x: 1.0, y: 0.0, z: 1.0},
            *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN
        );
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(belmont_special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn belmont_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
            let angle_param;
            let speed_param;
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                angle_param = hash40("throw_angle_ground");
                speed_param = hash40("throw_speed_ground");
            }
            else {
                angle_param = hash40("throw_angle_air");
                speed_param = hash40("throw_speed_air");
            }
            let throw_angle = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), angle_param);
            let throw_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), speed_param);
            if is_holywater {
                ItemModule::set_have_item_action(fighter.module_accessor, *ITEM_HOLYWATER_ACTION_SPECIAL_THROW, 0.0, item_part);
            }
            ItemModule::throw_item(
                fighter.module_accessor,
                throw_angle,
                throw_speed,
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
    }
    let mot_g = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_LW_INT_MOTION);
    let mot_a = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_LW_INT_MOTION_AIR);
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
    0.into()
}

pub unsafe fn belmont_special_lw_end_inner(fighter: &mut L2CFighterCommon) -> L2CValue {
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
