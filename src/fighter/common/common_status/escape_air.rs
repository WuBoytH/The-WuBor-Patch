use {
    smash::{
        lua2cpp::{L2CFighterCommon, *},
        hash40,
        phx::*,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    custom_var::*,
    wubor_utils::{
        vars::*,
        table_const::*
    }
};

#[skyline::hook(replace = L2CFighterCommon_setup_escape_air_slide_common)]
pub unsafe fn setup_escape_air_slide_common(fighter: &mut L2CFighterCommon, param_1: L2CValue, param_2: L2CValue) {
    let mut stickx = param_1.get_f32();
    let mut sticky = param_2.get_f32();
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
        if VarModule::is_flag(fighter.battle_object, commons::instance::flag::FORCE_ESCAPE_AIR_SLIDE) {
            let length = sv_math::vec2_length(stickx, sticky);
            let escape_air_slide_stick = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("escape_air_slide_stick"));
            if length < escape_air_slide_stick {
                stickx = 1.0 * PostureModule::lr(fighter.module_accessor);
                sticky = 0.0;
            }
            VarModule::off_flag(fighter.battle_object, commons::instance::flag::FORCE_ESCAPE_AIR_SLIDE);
        }
        StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
        let normalize = sv_math::vec2_normalize(stickx, sticky);
        let mut dirx = normalize.x;
        let mut diry = normalize.y;
        WorkModule::set_float(fighter.module_accessor, dirx, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_X);
        WorkModule::set_float(fighter.module_accessor, diry, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y);
        let speed_x = 
            KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) +
            KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_DAMAGE)
        ;
        let speed_y = 
            KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) +
            KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_DAMAGE)
        ;
        let escape_air_slide_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_speed"));
        let slide_speed_x = escape_air_slide_speed * speed_x;
        let slide_speed_y = escape_air_slide_speed * speed_y;
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            ENERGY_STOP_RESET_TYPE_FREE,
            slide_speed_x,
            slide_speed_y,
            0.0,
            0.0,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            -1.0,
            -1.0
        );
        let escape_air_slide_stiff_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_stiff_frame")); // new
        let escape_air_slide_u_stiff_frame = escape_air_slide_stiff_frame;
        let escape_air_slide_d_stiff_frame = escape_air_slide_stiff_frame;
        dirx = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_X);
        diry = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y);
        let arctangent = diry.atan2(dirx.abs());
        let stiff_lerp = if 0.0 > arctangent.to_degrees() {
            fighter.lerp(
                escape_air_slide_stiff_frame.into(),
                escape_air_slide_d_stiff_frame.into(),
                (arctangent.to_degrees() / 90.0).into()
            )
        }
        else {
            fighter.lerp(
                escape_air_slide_stiff_frame.into(),
                escape_air_slide_u_stiff_frame.into(),
                (arctangent.to_degrees() / 90.0).into()
            )
        };
        let escape_air_slide_stiff_start_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_stiff_start_frame"));
        WorkModule::set_float(fighter.module_accessor, escape_air_slide_stiff_start_frame, *FIGHTER_STATUS_ESCAPE_AIR_STIFF_START_FRAME);
        let escape_air_slide_back_end_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_back_end_frame"));
        let add_xlu = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_ADD_XLU_START_FRAME);
        WorkModule::set_int(fighter.module_accessor, escape_air_slide_back_end_frame + add_xlu, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_BACK_END_FRAME);
        WorkModule::set_float(fighter.module_accessor, stiff_lerp.get_f32(), *FIGHTER_STATUS_ESCAPE_AIR_STIFF_FRAME);
        EffectModule::req_on_joint(
            fighter.module_accessor,
            Hash40::new("sys_smash_flash_s"),
            Hash40::new("hip"),
            &Vector3f{x: 0.0, y: 4.0, z: 8.0},
            &ZERO_VECTOR,
            1.1,
            &Vector3f{x: 18.0, y: 12.0, z: 0.0},
            &ZERO_VECTOR,
            false,
            0,
            0,
            0
        );
    }
}

unsafe fn get_airdash_mul(fighter: &mut L2CFighterCommon) -> f32 {
    // don't do this
    let fighter_kind = fighter.global_table[KIND].get_i32();
    if [
        *FIGHTER_KIND_MEWTWO,
        *FIGHTER_KIND_RIDLEY
    ].contains(&fighter_kind) {
        return 1.0;
    }
    if [
        *FIGHTER_KIND_DONKEY,
        *FIGHTER_KIND_KIRBY,
        *FIGHTER_KIND_CAPTAIN,
        *FIGHTER_KIND_DAISY,
        *FIGHTER_KIND_SHEIK,
        *FIGHTER_KIND_MARIOD,
        *FIGHTER_KIND_PICHU,
        *FIGHTER_KIND_FALCO,
        *FIGHTER_KIND_CHROM,
        *FIGHTER_KIND_METAKNIGHT,
        *FIGHTER_KIND_PIT,
        *FIGHTER_KIND_LUCARIO,
        *FIGHTER_KIND_TOONLINK,
        *FIGHTER_KIND_MIISWORDSMAN,
        *FIGHTER_KIND_REFLET,
        *FIGHTER_KIND_DUCKHUNT,
        *FIGHTER_KIND_BAYONETTA,
        *FIGHTER_KIND_BUDDY,
        *FIGHTER_KIND_TANTAN
        ].contains(&fighter_kind) {
        return 0.94;
    }
    if [
        *FIGHTER_KIND_SAMUS,
        *FIGHTER_KIND_SAMUSD,
        *FIGHTER_KIND_MARTH,
        *FIGHTER_KIND_YOUNGLINK,
        *FIGHTER_KIND_ROY,
        *FIGHTER_KIND_GAMEWATCH,
        *FIGHTER_KIND_WARIO,
        *FIGHTER_KIND_PFUSHIGISOU,
        *FIGHTER_KIND_SNAKE,
        *FIGHTER_KIND_IKE,
        *FIGHTER_KIND_DIDDY,
        *FIGHTER_KIND_LUCAS,
        *FIGHTER_KIND_SONIC,
        *FIGHTER_KIND_PIKMIN,
        *FIGHTER_KIND_DEDEDE,
        *FIGHTER_KIND_ROBOT,
        *FIGHTER_KIND_WOLF,
        *FIGHTER_KIND_ROCKMAN,
        *FIGHTER_KIND_LITTLEMAC,
        *FIGHTER_KIND_PACMAN,
        *FIGHTER_KIND_KEN,
        *FIGHTER_KIND_KAMUI,
        *FIGHTER_KIND_KROOL,
        *FIGHTER_KIND_SHIZUE,
        *FIGHTER_KIND_GAOGAEN,
        *FIGHTER_KIND_PACKUN,
        *FIGHTER_KIND_BRAVE,
        *FIGHTER_KIND_MASTER,
        *FIGHTER_KIND_PICKEL,
        *FIGHTER_KIND_EFLAME,
        *FIGHTER_KIND_DEMON,
        *FIGHTER_KIND_TRAIL
    ].contains(&fighter_kind) {
        return 0.82
    }
    0.88
}

#[skyline::hook(replace = L2CFighterCommon_sub_escape_air_common_main)]
unsafe fn sub_escape_air_common_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return true.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return true.into();
        }
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
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE)
    && !CancelModule::is_enable_cancel(fighter.module_accessor) {
        let airdash_params = get_airdash_params(fighter);
        if fighter.global_table[MOTION_FRAME].get_f32() >= airdash_params.attack_frame {
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LASSO);
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ITEM_THROW);
            if fighter.sub_transition_group_check_air_item_throw().get_bool()
            || fighter.sub_transition_group_check_air_lasso().get_bool()
            || fighter.sub_transition_group_check_air_special().get_bool()
            || fighter.sub_transition_group_check_air_attack().get_bool() {
                return true.into();
            }
        }
        if fighter.global_table[MOTION_FRAME].get_f32() >= airdash_params.cancel_frame {
            if [*FIGHTER_KIND_MEWTWO].contains(&fighter.global_table[KIND].get_i32()) {
                let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
                sv_kinetic_energy!(
                    set_accel,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                    -air_accel_y
                );
            }
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into();
    }
    0.into()
}

pub struct AirDashParams {
    attack_frame: f32,
    cancel_frame: f32
}

unsafe fn get_airdash_params(fighter: &mut L2CFighterCommon) -> AirDashParams {
    let attack_frame: f32;
    let cancel_frame: f32;
    let fighter_kind = fighter.global_table[KIND].get_i32();
    if [
        *FIGHTER_KIND_MEWTWO
    ].contains(&fighter_kind) {
        attack_frame = 24.0;
        cancel_frame = 34.0;
    }
    else {
        attack_frame = 14.0;
        cancel_frame = 28.0;
    }
    AirDashParams{attack_frame, cancel_frame}
}

#[skyline::hook(replace = L2CFighterCommon_sub_escape_air_common_strans_main)]
pub unsafe fn sub_escape_air_common_strans_main(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    && {
        let air_lasso_type = WorkModule::get_param_int(fighter.module_accessor, hash40("air_lasso_type"), 0);
        air_lasso_type != *FIGHTER_AIR_LASSO_TYPE_NONE
    }
    && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
    && !LinkModule::is_link(fighter.module_accessor, *FIGHTER_LINK_NO_CONSTRAINT) {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
        return 1.into();
    }

    // // early return if airdashing

    // if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
    //     return 0.into();
    // }

    let air_escape_passive_trigger_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("air_escape_passive_trigger_frame"));
    let passive_trigger_frame_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("passive_trigger_frame_mul"), 0);
    let passive_frame = air_escape_passive_trigger_frame as f32 * passive_trigger_frame_mul;

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_PREV_STATUS_PASSIVE_GROUND) {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB)
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
        && FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32)
        && {
            let stick_x = fighter.global_table[STICK_X].get_f32();
            let passive_fb_cont_value = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("passive_fb_cont_value"));
            passive_fb_cont_value <= stick_x.abs()
        }
        && (escape_frame as f32) < passive_frame {
            fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE_FB.into(), true.into());
            return 1.into();
        }
        
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE)
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
        && FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32)
        && (escape_frame as f32) < passive_frame {
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
        }
        && (escape_frame as f32) < passive_frame {
            fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE_WALL_JUMP.into(), true.into());
            return 1.into();
        }

        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP)
        && FighterUtil::is_touch_passive_ground(fighter.module_accessor, (*GROUND_TOUCH_FLAG_LEFT | *GROUND_TOUCH_FLAG_RIGHT) as u32)
        && {
            let stick_y = fighter.global_table[STICK_Y].get_f32();
            let jump_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_stick_y"));
            jump_stick_y <= stick_y
        }
        && (escape_frame as f32) < passive_frame {
            fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE_WALL_JUMP.into(), true.into());
            return 1.into();
        }

        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL)
        && FighterUtil::is_touch_passive_ground(fighter.module_accessor, (*GROUND_TOUCH_FLAG_LEFT | *GROUND_TOUCH_FLAG_RIGHT) as u32)
        && (escape_frame as f32) < passive_frame {
            fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE_WALL.into(), false.into());
            return 1.into();
        }

        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_CEIL)
        && FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_UP as u32)
        && (escape_frame as f32) < passive_frame {
            fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE_CEIL.into(), false.into());
            return 1.into();
        }
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_escape_air_uniq)]
pub unsafe fn sub_escape_air_uniq(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        let slide = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE);
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
        if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
            if slide {
                fighter.exec_escape_air_slide();
            }
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
                    if 0.0 < stiff_start {
                        let frame = MotionModule::frame(fighter.module_accessor);
                        if stiff_start < frame {
                            let end_frame = MotionModule::end_frame(fighter.module_accessor);
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
                            let stiff_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_STIFF_FRAME);
                            if stiff_frame < frame {
                                WorkModule::set_float(fighter.module_accessor, end_frame, *FIGHTER_STATUS_ESCAPE_AIR_STIFF_FRAME);
                            }
                            let diff = cancel - frame;
                            let stiff_diff = stiff_frame - frame;
                            let stiff_rate = diff / stiff_diff;
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
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
                fighter.sub_fighter_cliff_check(L2CValue::Void());
            }
        }
        fighter.sub_fall_common_uniq(param_1);
    }
    else {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_INT_FRAME);
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_FRAME);
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_AIR_LASSO) {
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
pub unsafe fn exec_escape_air_slide(fighter: &mut L2CFighterCommon) {
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
            let airdash_mul = get_airdash_mul(fighter);
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                escape_air_slide_speed * dir_x * airdash_mul,
                escape_air_slide_speed * dir_y * airdash_mul * 0.65
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
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_GRAVITY) {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
            let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
            let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
            let speed_x_mul =
            if [*FIGHTER_KIND_MEWTWO].contains(&fighter.global_table[KIND].get_i32()) {
                0.2
            }
            else {
                0.65
            };
            let speed_y_mul = 
            if [*FIGHTER_KIND_MEWTWO].contains(&fighter.global_table[KIND].get_i32()) {
                0.2
            }
            else {
                if speed_y < 0.0 {
                    1.0
                }
                else {
                    0.5
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
            if [*FIGHTER_KIND_MEWTWO].contains(&fighter.global_table[KIND].get_i32()) {
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

#[skyline::hook(replace = L2CFighterCommon_status_end_EscapeAir)]
unsafe fn status_end_escapeair(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status == *FIGHTER_STATUS_KIND_FALL
    || status == *FIGHTER_STATUS_KIND_LANDING {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
            let landing_frame_escape_air_slide = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("landing_frame_escape_air_slide"));
            let landing_frame_escape_air_slide_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("landing_frame_escape_air_slide_max"));
            let frame = MotionModule::frame(fighter.module_accessor);
            let end_frame = MotionModule::end_frame(fighter.module_accessor);
            let frame_ratio = frame / end_frame;
            let landing_frame = fighter.lerp(landing_frame_escape_air_slide.into(), landing_frame_escape_air_slide_max.into(), frame_ratio.into()).get_f32();
            WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            let escape_air_slide_landing_speed_max = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("escape_air_slide_landing_speed_max")) * 0.75;
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
            let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
            let landing_speed_mul_escape_air_slide = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("landing_speed_mul_escape_air_slide"));
            let mut landing_speed = speed_x * landing_speed_mul_escape_air_slide;
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
            let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
            if escape_air_slide_landing_speed_max < landing_speed.abs() {
                if landing_speed < 0.0 {
                    landing_speed = -escape_air_slide_landing_speed_max;
                }
                else {
                    landing_speed = escape_air_slide_landing_speed_max;
                }
            }
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                landing_speed,
                speed_y
            );
        }
        else {
            let landing_frame_escape_air = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("landing_frame_escape_air"));
            WorkModule::set_float(fighter.module_accessor, landing_frame_escape_air as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        }
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
    VarModule::off_flag(fighter.battle_object, commons::instance::flag::FORCE_ESCAPE_AIR_SLIDE);
    fighter.status_end_Jump();
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            setup_escape_air_slide_common,
            sub_escape_air_common_main,
            sub_escape_air_common_strans_main,
            sub_escape_air_uniq,
            exec_escape_air_slide,
            status_end_escapeair
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}