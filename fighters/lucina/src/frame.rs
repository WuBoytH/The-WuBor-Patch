use {
    crate::imports::*,
    crate::fighter::common::frame::common_fighter_frame,
    super::{vl, helper::*}
};

unsafe extern "C" fn lucina_reset_vars(fighter: &mut L2CFighterCommon) {
    if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH {
        VarModule::off_flag(fighter.module_accessor, vars::yu::instance::flag::DISABLE_SPECIAL_N_S);
        VarModule::set_int(fighter.module_accessor, vars::yu::instance::int::SP_GLOW_TIMER, 0);
        VarModule::set_int(fighter.module_accessor, vars::yu::instance::int::SP_EFFECT_TIMER, 0);
        VarModule::set_float(fighter.module_accessor, vars::yu::instance::float::SP_GAUGE_MAX, 100.0);
        VarModule::set_float(fighter.module_accessor, vars::yu::instance::float::SP_GAUGE, 0.0);
        VarModule::off_flag(fighter.module_accessor, vars::yu::instance::flag::AWAKENING);
        VarModule::off_flag(fighter.module_accessor, vars::yu::instance::flag::SHADOW_FRENZY);
    }
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
    || StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_CLIFF {
        VarModule::off_flag(fighter.module_accessor, vars::yu::instance::flag::DISABLE_SPECIAL_N_S);
    }
}

unsafe extern "C" fn lucina_meter_controller(fighter: &mut L2CFighterCommon) {
    if VarModule::get_float(fighter.module_accessor, vars::yu::instance::float::SP_GAUGE) >= 25.0
    || VarModule::is_flag(fighter.module_accessor, vars::yu::instance::flag::SHADOW_FRENZY) {
        if VarModule::get_int(fighter.module_accessor, vars::yu::instance::int::SP_GLOW_TIMER) == 0 {
            sp_glow_handler(fighter.module_accessor);
            VarModule::set_int(fighter.module_accessor, vars::yu::instance::int::SP_GLOW_TIMER, 4);
        }
        VarModule::dec_int(fighter.module_accessor, vars::yu::instance::int::SP_GLOW_TIMER);
    }
}

unsafe extern "C" fn lucina_training_tools(fighter: &mut L2CFighterCommon) {
    if smashball::is_training_mode()
    && [
        *FIGHTER_STATUS_KIND_GUARD_ON,
        *FIGHTER_STATUS_KIND_GUARD
    ].contains(&StatusModule::status_kind(fighter.module_accessor)) {
        let meter_max = VarModule::get_float(fighter.module_accessor, vars::yu::instance::float::SP_GAUGE_MAX);
        let meter_const = vars::yu::instance::float::SP_GAUGE;
        if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
            FGCModule::update_meter(fighter.module_accessor, -25.0, meter_max, meter_const);
        }
        if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
            FGCModule::update_meter(fighter.module_accessor, 25.0, meter_max, meter_const);
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

unsafe extern "C" fn lucina_meter_display(fighter: &mut L2CFighterCommon) {
    let level = VarModule::get_int(fighter.module_accessor, vars::yu::instance::int::SP_LEVEL);
    let sp = VarModule::get_float(fighter.module_accessor, vars::yu::instance::float::SP_GAUGE);
    let new_level = sp / vl::param_private::sp_single;
    if level != new_level as i32 {
        sp_diff_checker(fighter.module_accessor);
    }

    if VarModule::get_int(fighter.module_accessor, vars::yu::instance::int::SP_EFFECT_TIMER) > 0 {
        VarModule::dec_int(fighter.module_accessor, vars::yu::instance::int::SP_EFFECT_TIMER);
        if VarModule::get_int(fighter.module_accessor, vars::yu::instance::int::SP_EFFECT_TIMER) == 0 {
            sp_gauge_handler(fighter.module_accessor, true);
        }
    }
}

unsafe extern "C" fn lucina_normal_shadow_effects(fighter: &mut L2CFighterCommon) {
    if shadow_id(fighter.module_accessor) {
        let eff = VarModule::get_int(fighter.module_accessor, vars::yu::instance::int::SHADOW_EFF_ID) as u32;
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
            VarModule::set_int(fighter.module_accessor, vars::yu::instance::int::SHADOW_EFF_ID, eff as i32);
        }
        DamageModule::set_damage_mul(fighter.module_accessor, vl::param_private::shadow_type_damage_mul);
        if VarModule::is_flag(fighter.module_accessor, vars::yu::instance::flag::SHADOW_FRENZY) {
            let amount = vl::param_private::shadow_frenzy_sp_drain;
            VarModule::sub_float(fighter.module_accessor, vars::yu::instance::float::SP_GAUGE, amount);
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
            if !VarModule::is_flag(fighter.module_accessor, vars::yu::instance::flag::AWAKENING)
            && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
            && !MiscModule::is_damage_check(fighter.module_accessor, false)
            && sv_information::is_ready_go() {
                VarModule::set_float(fighter.module_accessor, vars::yu::instance::float::SP_GAUGE_MAX, vl::param_private::awakening_sp_max);
                FGCModule::update_meter(fighter.module_accessor, vl::param_private::awakening_sp_gain, vl::param_private::awakening_sp_max, vars::yu::instance::float::SP_GAUGE);
                VarModule::on_flag(fighter.module_accessor, vars::yu::instance::flag::AWAKENING);
                macros::FT_START_CUTIN(fighter);
            }
        }
        if VarModule::is_flag(fighter.module_accessor, vars::yu::instance::flag::AWAKENING) {
            DamageModule::set_damage_mul(fighter.module_accessor, vl::param_private::awakening_damage_mul);
        }
    }
}

unsafe extern "C" fn lucina_on_empty_sp_meter(fighter: &mut L2CFighterCommon) {
    if VarModule::get_float(fighter.module_accessor, vars::yu::instance::float::SP_GAUGE) <= 0.0 {
        VarModule::set_float(fighter.module_accessor, vars::yu::instance::float::SP_GAUGE, 0.0);
        VarModule::off_flag(fighter.module_accessor, vars::yu::instance::flag::SHADOW_FRENZY);
        sp_diff_checker(fighter.module_accessor);
    }
}

unsafe extern "C" fn lucina_one_more_check(fighter: &mut L2CFighterCommon) {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    // if status == *FIGHTER_STATUS_KIND_THROW {
    //     if !CatchModule::is_catch(fighter.module_accessor)
    //     && !VarModule::is_flag(fighter.module_accessor, vars::yu::status::flag::CAN_ONE_MORE) {
    //         VarModule::on_flag(fighter.module_accessor, vars::yu::status::flag::CAN_ONE_MORE);
    //     }
    // }
    // else if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
    // && !CatchModule::is_catch(fighter.module_accessor)
    // && status != *FIGHTER_STATUS_KIND_SPECIAL_HI {
    //     VarModule::on_flag(fighter.module_accessor, vars::yu::status::flag::CAN_ONE_MORE);
    // }
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() != *FIGHTER_STATUS_KIND_SPECIAL_LW {
        if !WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW)
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
            VarModule::on_flag(fighter.module_accessor, vars::yu::instance::flag::ROMAN_ON_HIT);
        }
        else {
            VarModule::off_flag(fighter.module_accessor, vars::yu::instance::flag::ROMAN_ON_HIT);
        }
    }
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
    if status == *FIGHTER_STATUS_KIND_THROW
    && !CatchModule::is_catch(fighter.module_accessor) {
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0
        && spent_meter(fighter.module_accessor, true) {
            VarModule::on_flag(fighter.module_accessor, vars::yu::instance::flag::ROMAN_ON_HIT);
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), true.into());
        }
        else if shadow_id(fighter.module_accessor)
        && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0
        && VarModule::get_float(fighter.module_accessor, vars::yu::instance::float::SP_GAUGE) >= 25.0
        && !VarModule::is_flag(fighter.module_accessor, vars::yu::instance::flag::SHADOW_FRENZY) {
            fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT.into(), true.into());
        }
    }
    if shadow_id(fighter.module_accessor)
    && [hash40("appeal_hi_l"), hash40("appeal_hi_r")].contains(&MotionModule::motion_kind(fighter.module_accessor))
    && !VarModule::is_flag(fighter.module_accessor, vars::yu::instance::flag::SHADOW_FRENZY)
    && VarModule::get_float(fighter.module_accessor, vars::yu::instance::float::SP_GAUGE) >= 25.0
    && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0 {
        fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT.into(), true.into());
    }
}

unsafe extern "C" fn lucina_sb_flash(fighter: &mut L2CFighterCommon) {
    if VarModule::get_int(fighter.module_accessor, vars::yu::instance::int::SP_FLASH_TIMER) > 0 {
        if VarModule::get_int(fighter.module_accessor, vars::yu::instance::int::SP_FLASH_TIMER) % 10 == 0 {
            if VarModule::is_flag(fighter.module_accessor, vars::yu::instance::flag::SHADOW_FRENZY) {
                macros::FLASH(fighter, 0.4, 0.0, 1.0, 1.0);
            }
            else {
                macros::FLASH(fighter, 1.0, 1.0, 0.0, 0.75);
            }
        }
        else if VarModule::get_int(fighter.module_accessor, vars::yu::instance::int::SP_FLASH_TIMER) % 5 == 0 {
            macros::COL_NORMAL(fighter);
        }
        VarModule::dec_int(fighter.module_accessor, vars::yu::instance::int::SP_FLASH_TIMER);
    }
}

unsafe extern "C" fn on_main(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    lucina_reset_vars(fighter);
    lucina_meter_controller(fighter);
    lucina_training_tools(fighter);
    lucina_normal_shadow_effects(fighter);
    lucina_on_empty_sp_meter(fighter);
    lucina_meter_display(fighter);
    lucina_one_more_check(fighter);
    lucina_sb_flash(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, on_main);
}