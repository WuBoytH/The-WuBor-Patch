#![allow(dead_code)]

use super::*;
use super::escape_air_slide::*;

#[skyline::hook(replace = L2CFighterCommon_status_EscapeAir)]
unsafe extern "C" fn status_escapeair(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_escape_air_common();
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("escape_air"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    let rate_penalty = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_FLOAT_MOTION_RATE_PENALTY);
    let add_xlu = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_ADD_XLU_START_FRAME);
    if 0 < add_xlu {
        let xlu = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_INT_HIT_XLU_FRAME);
        let some = xlu - add_xlu;
        let ratio = xlu as f32 / some as f32;
        let inverse = 1.0 / ratio;
        let rate = inverse * rate_penalty;
        MotionModule::set_rate(fighter.module_accessor, rate);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_EscapeAir_Main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_sub_escape_air_common)]
unsafe extern "C" fn sub_escape_air_common(fighter: &mut L2CFighterCommon) {
    // ControlModule::reset_trigger(fighter.module_accessor);

    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_ESCAPE_WORK_INT_FRAME);
    WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_AIR_LASSO);
    }
    else {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_AIR_LASSO);
    }
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP_BUTTON);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_CEIL);

    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_NO_WATER_INOUT_FRAME);

    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_escape_air_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_escape_air_uniq as *const () as _));
}

#[skyline::hook(replace = L2CFighterCommon_sub_escape_air_common_main)]
unsafe extern "C" fn sub_escape_air_common_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return true.into();
    }
    if fighter.sub_escape_air_common_strans_main().get_bool() {
        return true.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING) {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
            return true.into();
        }
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return true.into();
        }
    }
    else if VarModule::is_flag(fighter.module_accessor, vars::escape_air::flag::SLIDE_ENABLE_ATTACK) {
        if fighter.sub_transition_group_check_air_special().get_bool()
        || fighter.sub_transition_group_check_air_item_throw().get_bool()
        || fighter.sub_transition_group_check_air_lasso().get_bool()
        || fighter.sub_transition_group_check_air_attack().get_bool() {
            return true.into();
        }
    }
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE {
        let airdash_params = get_airdash_params(fighter);
        if fighter.global_table[STATUS_FRAME].get_f32() >= airdash_params.attack_frame
        && !VarModule::is_flag(fighter.module_accessor, vars::escape_air::flag::SLIDE_ENABLE_ATTACK) {
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LASSO);
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ITEM_THROW);
            VarModule::on_flag(fighter.module_accessor, vars::escape_air::flag::SLIDE_ENABLE_ATTACK);
        }
        if fighter.global_table[STATUS_FRAME].get_f32() >= airdash_params.cancel_frame
        && !VarModule::is_flag(fighter.module_accessor, vars::escape_air::flag::SLIDE_ENABLE_CANCEL) {
            if get_airdash_tier(fighter) == AirDashTier::Teleport {
                let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
                sv_kinetic_energy!(
                    set_accel,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                    -air_accel_y
                );
            }
            CancelModule::enable_cancel(fighter.module_accessor);
            VarModule::on_flag(fighter.module_accessor, vars::escape_air::flag::SLIDE_ENABLE_CANCEL);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into();
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_escape_air_common_strans_main)]
pub unsafe extern "C" fn sub_escape_air_common_strans_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let escape_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_INT_FRAME);
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW)
    && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
    && ItemModule::is_have_item(fighter.module_accessor, 0)
    && {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW);
        sv_module_access::item(fighter.lua_state_agent);
        !fighter.pop_lua_stack(1).get_bool()
    }
    && {
        let escape_throw_item_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("escape_throw_item_frame"));
        escape_frame <= escape_throw_item_frame
    }
    && !fighter.can_entry_cliff_air_lasso().get_bool() {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_AIR_LASSO)
    && !VarModule::is_flag(fighter.module_accessor, vars::escape_air::flag::SLIDE_ENABLE_ATTACK)
    && {
        let air_lasso_type = WorkModule::get_param_int(fighter.module_accessor, hash40("air_lasso_type"), 0);
        air_lasso_type != *FIGHTER_AIR_LASSO_TYPE_NONE
    }
    && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
    && !LinkModule::is_link(fighter.module_accessor, *FIGHTER_LINK_NO_CONSTRAINT) {
        fighter.change_status(FIGHTER_STATUS_KIND_AIR_LASSO.into(), false.into());
        return 1.into();
    }

    if !fighter.is_enable_passive().get_bool() {
        return 0.into();
    }

    let air_escape_passive_trigger_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("air_escape_passive_trigger_frame"));
    let passive_trigger_frame_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("passive_trigger_frame_mul"), 0);
    let passive_frame = air_escape_passive_trigger_frame as f32 * passive_trigger_frame_mul;

    let passive_trigger_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("passive_trigger_frame")) as f32;
    let tech = fighter.sub_check_passive_button_for_damage((passive_trigger_frame * passive_trigger_frame_mul).into()).get_bool();

    if escape_frame as f32 >= passive_frame
    || !tech {
        return 0.into();
    }

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_PREV_STATUS_PASSIVE_GROUND) {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB)
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
        && FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32)
        && {
            let stick_x = fighter.global_table[STICK_X].get_f32();
            let passive_fb_cont_value = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("passive_fb_cont_value"));
            passive_fb_cont_value <= stick_x.abs()
        } {
            fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE_FB.into(), true.into());
            return 1.into();
        }
        
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE)
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
        && FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
            fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE.into(), true.into());
            return 1.into();
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_PREV_STATUS_PASSIVE_AIR) {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP_BUTTON)
        && FighterUtil::is_touch_passive_ground(fighter.module_accessor, (*GROUND_TOUCH_FLAG_LEFT | *GROUND_TOUCH_FLAG_RIGHT) as u32)
        && {
            let jump_trigger_count = ControlModule::get_trigger_count(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP as u8);
            ((jump_trigger_count & 0xff) as f32) < passive_frame
        } {
            fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE_WALL_JUMP.into(), true.into());
            return 1.into();
        }

        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP)
        && FighterUtil::is_touch_passive_ground(fighter.module_accessor, (*GROUND_TOUCH_FLAG_LEFT | *GROUND_TOUCH_FLAG_RIGHT) as u32)
        && {
            let stick_y = fighter.global_table[STICK_Y].get_f32();
            let jump_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_stick_y"));
            jump_stick_y <= stick_y
        } {
            fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE_WALL_JUMP.into(), true.into());
            return 1.into();
        }

        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL)
        && FighterUtil::is_touch_passive_ground(fighter.module_accessor, (*GROUND_TOUCH_FLAG_LEFT | *GROUND_TOUCH_FLAG_RIGHT) as u32) {
            fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE_WALL.into(), false.into());
            return 1.into();
        }

        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_CEIL)
        && FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_UP as u32) {
            fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE_CEIL.into(), false.into());
            return 1.into();
        }
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_escape_air_uniq)]
pub unsafe extern "C" fn sub_escape_air_uniq(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        let slide = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE;
        let escape_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_INT_FRAME);
        let item_air_catch_frame_escape = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("item_air_catch_frame_escape"));
        if escape_frame <= item_air_catch_frame_escape {
            fighter.sub_GetLightItemImm(L2CValue::Void());
        }
        if slide {
            if ItemModule::is_have_item(fighter.module_accessor, 0) {
                let escape_throw_item_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("escape_throw_item_frame"));
                if escape_frame < escape_throw_item_frame {
                    fighter.sub_AIRChkDropItemImm();
                }
            }
        }
        if slide {
            fighter.exec_escape_air_slide();
        }
        else {
            let xlu_start = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_ADD_XLU_START_FRAME);
            if 0 < xlu_start {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_FLAG_HIT_XLU) {
                    let rate_penalty = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_FLOAT_MOTION_RATE_PENALTY);
                    MotionModule::set_rate(fighter.module_accessor, rate_penalty);
                    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_ESCAPE_AIR_ADD_XLU_START_FRAME);
                }
            }
            if StatusModule::is_changing(fighter.module_accessor) {
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_END_STIFF) {
                    if CancelModule::is_enable_cancel(fighter.module_accessor) {
                        MotionModule::set_rate(fighter.module_accessor, 1.0);
                        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_END_STIFF);
                    }
                }
            }
            else {
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_STIFF)
                && !slide {
                    let stiff_start = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_STIFF_START_FRAME);
                    if 0.0 <= stiff_start {
                        let frame = MotionModule::frame(fighter.module_accessor);
                        if stiff_start <= frame {
                            let mot = MotionModule::motion_kind(fighter.module_accessor);
                            let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new_raw(mot), true);
                            let end_frame = if cancel_frame == 0.0 {
                                MotionModule::end_frame(fighter.module_accessor)
                            }
                            else {
                                cancel_frame
                            };
                            let which_cancel = if !slide {
                                hash40("escape_air_cancel_frame")
                            }
                            else {
                                hash40("escape_air_slide_cancel_frame")
                            };
                            let mut cancel = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), which_cancel);
                            if cancel < 0.0 {
                                cancel = end_frame;
                            }
                            let cancel_diff = cancel - frame;
                            let cancel_motion_diff = end_frame - frame;
                            let stiff_rate = cancel_diff / cancel_motion_diff;
                            WorkModule::set_float(fighter.module_accessor, cancel_motion_diff, *FIGHTER_STATUS_ESCAPE_AIR_STIFF_FRAME);
                            let rate = MotionModule::rate(fighter.module_accessor);
                            MotionModule::set_rate(fighter.module_accessor, rate * stiff_rate);
                            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_STIFF);
                        }
                    }
                }
            }
        }
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_FALL) {
            return 0.into();
        }
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_KINE_FALL) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_KINE_FALL);
            if !fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE {
                fighter.sub_fighter_cliff_check(L2CValue::Void());
            }
        }
        fighter.sub_fall_common_uniq(param_1);
    }
    else {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_INT_FRAME);
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_FRAME);
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_AIR_LASSO)
        && !VarModule::is_flag(fighter.module_accessor, vars::escape_air::flag::SLIDE_ENABLE_ATTACK) {
            let escape_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_INT_FRAME);
            let attack_air_lasso_enable_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("attack_air_lasso_enable_frame"));
            if attack_air_lasso_enable_frame < escape_frame {
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_AIR_LASSO);
            }
        }
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_exec_escape_air_slide)]
pub unsafe extern "C" fn exec_escape_air_slide(fighter: &mut L2CFighterCommon) {
    let mut slide_step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_STEP);
    let back_end_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_BACK_END_FRAME);
    if slide_step == 0 {
        let slide_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_FRAME);
        if back_end_frame <= slide_frame {
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_FRAME);
            slide_step = 1;
            WorkModule::set_int(fighter.module_accessor, slide_step, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_STEP);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_KINE_FALL) {
        return;
    }
    if slide_step == 0 {
        let slide_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_FRAME);
        let frame = back_end_frame as f32 - 1.0;
        let result = slide_frame as f32 / frame;
        let back_accel = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_BACK_ACCEL);
        let curve = sv_math::bezier_curve(0.5, 0.8, 0.9, 1.0, result);
        let accel = curve - back_accel;
        let escape_air_slide_back_distance = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_back_distance"));
        // let end = accel * -escape_air_slide_back_distance;
        let end = accel * escape_air_slide_back_distance; // new
        WorkModule::set_float(fighter.module_accessor, curve, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_BACK_ACCEL);
        let dir_x = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_X);
        let dir_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            dir_x * end,
            dir_y * end
        );
    }
    else if slide_step == 1 {
        let slide_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_FRAME);
        if slide_frame != 0 {
            if 1 <= slide_frame {
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_KEEP_AIR_TURNED_OFF) {
                    StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), false);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_KEEP_AIR_TURNED_OFF);
                }
                if !WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING) {
                    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
                }
            }
        }
        else {
            let boma = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
            KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, boma);
            KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, boma);
            sv_kinetic_energy!(
                clear_speed_ex,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_DAMAGE
            );
            let dir_x = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_X);
            let dir_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y);
            // sv_kinetic_energy!(
            //     reset_energy,
            //     fighter,
            //     FIGHTER_KINETIC_ENERGY_ID_STOP,
            //     ENERGY_STOP_RESET_TYPE_ESCAPE_AIR_SLIDE,
            //     dir_x,
            //     dir_y,
            //     0.0,
            //     0.0,
            //     0.0
            // );
            let escape_air_slide_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_speed"));
            let airdash_mul = match get_airdash_tier(fighter) {
                AirDashTier::Bad => 0.79,
                AirDashTier::Good => 0.93,
                AirDashTier::Great => 1.0,
                AirDashTier::Teleport => 1.0,
                _ => 0.86
            };
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                escape_air_slide_speed * dir_x * airdash_mul,
                escape_air_slide_speed * dir_y * airdash_mul * 0.75
            );
            let brake_x = dir_x.abs() * 0.15;
            let brake_y: f32 = if dir_y < 0.0 {
                0.0
            }
            else {
                0.12
            };
            sv_kinetic_energy!(
                set_brake,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                brake_x,
                brake_y
            );

            // Effects

            let angle = dir_y.atan2(dir_x).to_degrees();
            let lr = PostureModule::lr(fighter.module_accessor);
            let hip = &mut Vector3f{x: 0.0, y: 0.0, z: 0.0};
            ModelModule::joint_global_offset_from_top(fighter.module_accessor, Hash40::new("hip"), hip);
            let pos = Vector3f{
                x: 10.0 * dir_x * lr,
                y: hip.y,
                z: 0.0
            };
            let rot = if lr > 0.0 {
                Vector3f{x: 0.0, y: 0.0, z: 180.0 + angle}
            }
            else {
                Vector3f{x: 0.0, y: 0.0, z: angle}
            };
            let line = EffectModule::req_on_joint(
                fighter.module_accessor,
                Hash40::new("sys_attack_speedline"),
                Hash40::new("top"),
                &pos,
                &rot,
                0.85,
                &Vector3f{x: 0.0, y: 0.0, z: 0.0},
                &Vector3f{x: 0.0, y: 0.0, z: 0.0},
                false,
                0,
                0,
                0
            ) as u32;
            EffectModule::set_rate(fighter.module_accessor, line, 0.5);

            SoundModule::play_se(fighter.module_accessor, Hash40::new("se_common_airdash"), true, false, false, false, enSEType(0));
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_GRAVITY) {
            let tier = get_airdash_tier(fighter);
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
            let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
            let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
            let speed_x_mul = match tier {
                AirDashTier::Teleport => 0.2,
                AirDashTier::Bad => 0.60,
                AirDashTier::Good => 0.70,
                AirDashTier::Great => 0.75,
                _ => 0.65
            };
            let speed_y_mul = match tier {
                AirDashTier::Teleport => 0.2,
                _ => {
                    if speed_y < 0.0 {
                        1.0
                    }
                    else {
                        0.5
                    }
                }
            };
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
            sv_kinetic_energy!(
                reset_energy,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST_NO_CAP,
                speed_x * speed_x_mul,
                0.0,
                0.0,
                0.0,
                0.0
            );
            let speed_x_limit = match tier {
                AirDashTier::Teleport | AirDashTier::Bad => 0.8,
                AirDashTier::Good | AirDashTier::Great => 1.0,
                _ => 0.9
            };
            sv_kinetic_energy!(
                set_stable_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                speed_x_limit
            );
            sv_kinetic_energy!(
                set_limit_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                speed_x_limit
            );
            let speed_x_accel = 0.02;
            sv_kinetic_energy!(
                controller_set_accel_x_add,
                fighter,
                speed_x_accel
            );
            sv_kinetic_energy!(
                controller_set_accel_x_mul,
                fighter,
                speed_x_accel
            );
            sv_kinetic_energy!(enable, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            sv_kinetic_energy!(
                reset_energy,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
                0.0,
                speed_y * speed_y_mul,
                0.0,
                0.0,
                0.0
            );
            if get_airdash_tier(fighter) == AirDashTier::Teleport {
                let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
                sv_kinetic_energy!(
                    set_accel,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                    -air_accel_y * 0.1
                );
            }
            sv_kinetic_energy!(enable, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_STOP, fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_KINE_FALL);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_GRAVITY);
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_FRAME);
            WorkModule::set_int(fighter.module_accessor, 2, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_STEP);
        }
    }
}

#[skyline::hook(replace = L2CFighterCommon_FighterStatusUniqProcessEscapeAir_calc_param)]
pub unsafe extern "C" fn fighterstatusuniqprocessescapeair_calc_param(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_EscapeAir)]
unsafe extern "C" fn status_end_escapeair(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status == *FIGHTER_STATUS_KIND_FALL
    || status == *FIGHTER_STATUS_KIND_LANDING {
        let landing_frame_escape_air = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("landing_frame_escape_air"));
        WorkModule::set_float(fighter.module_accessor, landing_frame_escape_air as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        if status == *FIGHTER_STATUS_KIND_LANDING {
            if !MotionModule::is_end(fighter.module_accessor) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_TURN);
            }
            // WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_LANDING_CLIFF_STOP);
        }
    }
    else {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
        let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
        let mut air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
        if speed_x.abs() > air_speed_x_stable {
            air_speed_x_stable *= speed_x.signum();
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
            let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                air_speed_x_stable,
                speed_y
            );
        }
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_escapeair,
            sub_escape_air_common,
            sub_escape_air_common_main,
            sub_escape_air_common_strans_main,
            sub_escape_air_uniq,
            exec_escape_air_slide,
            fighterstatusuniqprocessescapeair_calc_param,
            status_end_escapeair
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}