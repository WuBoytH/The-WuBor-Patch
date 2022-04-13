use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    wubor_utils::{
        wua_bind::*,
        vars::*,
        table_const::*
    },
    super::{
        vl,
        helper::*,
        vars::*
    },
};

unsafe fn lucina_reset_vars(fighter: &mut L2CFighterCommon) {
    if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_N_S);
        WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_GLOW_TIMER);
        WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_EFFECT_TIMER);
        WorkModule::set_float(fighter.module_accessor, 100.0, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE_MAX);
        WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_AWAKENING);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_SHADOW_FRENZY);
    }
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
    || StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_CLIFF {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_N_S);
    }
}

unsafe fn lucina_meter_controller(fighter: &mut L2CFighterCommon) {
    let meter_max = WorkModule::get_float(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE_MAX);
    let meter_const = FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE;
    
    if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    && !WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_SHADOW_FRENZY) {
        if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_STATUS_FLAG_IS_EX) {
            let mut amount = AttackModule::get_power(fighter.module_accessor, 0, false, 1.0, false);
            if shadow_id(fighter.module_accessor) == false {
                amount *= 0.75;
            }
            if WorkModule::get_float(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAIN_PENALTY) > 0.0 {
                amount *= 0.1;
            }
            FGCModule::update_meter(fighter.battle_object, amount, meter_max, meter_const);
        }
    }

    if WorkModule::get_float(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE) >= 25.0
    || WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_SHADOW_FRENZY) {
        if WorkModule::get_int(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_GLOW_TIMER) == 0 {
            sp_glow_handler(fighter.module_accessor);
            WorkModule::set_int(fighter.module_accessor, 4, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_GLOW_TIMER);
        }
        WorkModule::dec_int(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_GLOW_TIMER);
    }
}

unsafe fn lucina_training_tools(fighter: &mut L2CFighterCommon) {
    if smashball::is_training_mode()
    && [
        *FIGHTER_STATUS_KIND_GUARD_ON,
        *FIGHTER_STATUS_KIND_GUARD
    ].contains(&StatusModule::status_kind(fighter.module_accessor)) {
        let meter_max = WorkModule::get_float(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE_MAX);
        let meter_const = FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE;
        if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
            FGCModule::update_meter(fighter.battle_object, -25.0, meter_max, meter_const);
        }
        if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
            FGCModule::update_meter(fighter.battle_object, 25.0, meter_max, meter_const);
        }
        // if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
        //     if TRAINING_TOOLS[entry_id(fighter.module_accessor)] {
        //         TRAINING_TOOLS[entry_id(fighter.module_accessor)] = false;
        //         let pos = Vector3f{x: 0.0, y: 13.0, z: 0.0};
        //         let rot = Vector3f{x: 0.0, y: 90.0, z: 0.0};
        //         let onemoreeff: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_counter_flash"), Hash40::new("top"), &pos, &rot, 1.0, false, 0, 0, 0, 0, 0, false, false) as u32;
        //         EffectModule::set_rgb(fighter.module_accessor, onemoreeff, 0.0, 0.0, 5.0);
        //     }
        //     else {
        //         TRAINING_TOOLS[entry_id(fighter.module_accessor)] = true;
        //         let pos = Vector3f{x: 0.0, y: 13.0, z: 0.0};
        //         let rot = Vector3f{x: 0.0, y: 90.0, z: 0.0};
        //         let onemoreeff: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_counter_flash"), Hash40::new("top"), &pos, &rot, 1.0, false, 0, 0, 0, 0, 0, false, false) as u32;
        //         EffectModule::set_rgb(fighter.module_accessor, onemoreeff, 5.0, 0.0, 0.0);
        //     }
        // }
    }
}

unsafe fn lucina_meter_display(fighter: &mut L2CFighterCommon) {
    let level = WorkModule::get_int(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_LEVEL);
    let sp = WorkModule::get_float(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE);
    let new_level = sp / vl::param_private::sp_single;
    if level != new_level as i32 {
        sp_diff_checker(fighter.module_accessor);
    }

    if WorkModule::get_float(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAIN_PENALTY) > 0.0 {
        WarkModule::count_down(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAIN_PENALTY, 1.0);
    }

    if WorkModule::get_int(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_EFFECT_TIMER) > 0 {
        WorkModule::dec_int(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_EFFECT_TIMER);
        if WorkModule::get_int(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_EFFECT_TIMER) == 0 {
            sp_gauge_handler(fighter.module_accessor, true);
        }
    }
}

unsafe fn lucina_normal_shadow_effects(fighter: &mut L2CFighterCommon) {
    if shadow_id(fighter.module_accessor) {
        let eff = WorkModule::get_int(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_INT_SHADOW_EFF_ID ) as u32;
        if !EffectModule::is_exist_effect(fighter.module_accessor, eff) {
            EffectModule::req_follow(
                fighter.module_accessor,
                Hash40::new("sys_aura_light"),
                Hash40::new("hip"),
                &ZERO_VECTOR,
                &ZERO_VECTOR,
                4.0,
                false,
                0,
                0,
                0,
                0,
                0,
                false,
                false
            );
            let eff = EffectModule::get_last_handle(fighter.module_accessor) as u32;
            EffectModule::set_rgb(fighter.module_accessor, eff, 0.0, 0.0, 1.0);
            EffectModule::set_rate(fighter.module_accessor, eff, 0.3);
            WorkModule::set_int(fighter.module_accessor, eff as i32, FIGHTER_YU_INSTANCE_WORK_ID_INT_SHADOW_EFF_ID);
        }
        DamageModule::set_damage_mul(fighter.module_accessor, vl::param_private::shadow_type_damage_mul);
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_SHADOW_FRENZY) {
            let amount = vl::param_private::shadow_frenzy_sp_drain;
            WarkModule::count_down(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE, amount);
        }
        if SoundModule::is_playing(fighter.module_accessor, Hash40::new("vc_lucina_missfoot01")) {
            SoundModule::stop_se(fighter.module_accessor, Hash40::new("vc_lucina_missfoot01"), 0);
            macros::PLAY_SE(fighter, Hash40::new("vc_shadow_missfoot01"));
        }
        if SoundModule::is_playing(fighter.module_accessor, Hash40::new("vc_lucina_missfoot02")) {
            SoundModule::stop_se(fighter.module_accessor, Hash40::new("vc_lucina_missfoot02"), 0);
            macros::PLAY_SE(fighter, Hash40::new("vc_shadow_missfoot02"));
        }
        if SoundModule::is_playing(fighter.module_accessor, Hash40::new("vc_lucina_damage_twinkle")) {
            SoundModule::stop_se(fighter.module_accessor, Hash40::new("vc_lucina_damage_twinkle"), 0);
            macros::PLAY_SE(fighter, Hash40::new("vc_shadow_damage_twinkle"));
        }
        if SoundModule::is_playing(fighter.module_accessor, Hash40::new("vc_lucina_knockout")) {
            SoundModule::stop_se(fighter.module_accessor, Hash40::new("vc_lucina_knockout"), 0);
            macros::PLAY_SE(fighter, Hash40::new("vc_shadow_knockout"));
        }
    }
    else {
        if DamageModule::damage(fighter.module_accessor, 0) >= vl::param_private::awakening_activation_damage {
            if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_AWAKENING)
            && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
            && !MiscModule::is_damage_check(fighter.module_accessor, false)
            && sv_information::is_ready_go() {
                WorkModule::set_float(fighter.module_accessor, vl::param_private::awakening_sp_max, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE_MAX);
                FGCModule::update_meter(fighter.battle_object, vl::param_private::awakening_sp_gain, vl::param_private::awakening_sp_max, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE);
                WorkModule::on_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_AWAKENING);
                macros::FT_START_CUTIN(fighter);
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_AWAKENING) {
            DamageModule::set_damage_mul(fighter.module_accessor, vl::param_private::awakening_damage_mul);
        }
    }
}

unsafe fn lucina_on_empty_sp_meter(fighter: &mut L2CFighterCommon) {
    if WorkModule::get_float(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE) <= 0.0 {
        WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_SHADOW_FRENZY);
        sp_diff_checker(fighter.module_accessor);
    }
}

unsafe fn lucina_one_more_check(fighter: &mut L2CFighterCommon) {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status == *FIGHTER_STATUS_KIND_THROW {
        if CatchModule::is_catch(fighter.module_accessor) == false
        && !WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_STATUS_FLAG_CAN_ONE_MORE) {
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_YU_STATUS_FLAG_CAN_ONE_MORE);
        }
    }
    else if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
    && CatchModule::is_catch(fighter.module_accessor) == false
    && status != *FIGHTER_STATUS_KIND_SPECIAL_HI {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_YU_STATUS_FLAG_CAN_ONE_MORE);
    }
    if ![*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT].contains(&status)
    && !fighter.global_table[IN_HITLAG].get_bool()
    && !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
        let cat1 = fighter.global_table[CMD_CAT1].get_i32();
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_STATUS_FLAG_CAN_ONE_MORE) {
            if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0
            && spent_meter(fighter.module_accessor, true) {
                WorkModule::on_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_ROMAN_ON_HIT);
                fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), true.into());
            }
            else if fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0
            && WorkModule::get_float(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE) >= 25.0
            && !WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_SHADOW_FRENZY)
            && shadow_id(fighter.module_accessor) {
                fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT.into(), true.into());
            }
        }
        else if !MiscModule::is_damage_check(fighter.module_accessor, false)
        && status != *FIGHTER_STATUS_KIND_SPECIAL_HI
        && CatchModule::is_catch(fighter.module_accessor) == false
        && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0
        && spent_meter(fighter.module_accessor, true) {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_ROMAN_ON_HIT);
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), true.into());
        }
    }
    if shadow_id(fighter.module_accessor)
    && [hash40("appeal_hi_l"), hash40("appeal_hi_r")].contains(&MotionModule::motion_kind(fighter.module_accessor))
    && WorkModule::get_float(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE) >= 25.0
    && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0 {
        fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT.into(), true.into());
    }
}

unsafe fn lucina_sb_flash(fighter: &mut L2CFighterCommon) {
    if WorkModule::get_int(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_FLASH_TIMER) > 0 {
        if WorkModule::get_int(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_FLASH_TIMER) % 10 == 0 {
            if WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_SHADOW_FRENZY) {
                macros::FLASH(fighter, 0.4, 0.0, 1.0, 1.0);
            }
            else {
                macros::FLASH(fighter, 1.0, 1.0, 0.0, 0.75);
            }
        }
        else if WorkModule::get_int(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_FLASH_TIMER) % 5 == 0 {
            macros::COL_NORMAL(fighter);
        }
        WorkModule::dec_int(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_FLASH_TIMER);
    }
}

unsafe fn lucina_heroic_bravery_grab(fighter: &mut L2CFighterCommon) {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_HEROIC_GRAB) {
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("catch_wait")
        || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_CATCH_ATTACK {
            fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), true.into());
        }
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("throw_hi")
        && MotionModule::frame(fighter.module_accessor) > 22.0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
        }
    }
    if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CATCH_WAIT
    && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CATCH_DASH
    && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CATCH_TURN
    && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CATCH_PULL
    && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CATCH_JUMP
    && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CATCH_CUT
    && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CATCH
    && CatchModule::is_catch(fighter.module_accessor) == false
    && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_THROW {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_HEROIC_GRAB);
    }
}

#[fighter_frame( agent = FIGHTER_KIND_LUCINA )]
fn lucina_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        lucina_reset_vars(fighter);
        lucina_meter_controller(fighter);
        lucina_training_tools(fighter);
        lucina_normal_shadow_effects(fighter);
        lucina_on_empty_sp_meter(fighter);
        lucina_meter_display(fighter);
        lucina_one_more_check(fighter);
        lucina_sb_flash(fighter);
        lucina_heroic_bravery_grab(fighter);
    }
}

pub fn install() {
    install_agent_frames!(
        lucina_frame
    );
}