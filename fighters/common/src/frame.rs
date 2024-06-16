use super::*;

unsafe extern "C" fn hit_cancel_frame_set(fighter: &mut L2CFighterCommon) {
    let frame = fighter.global_table[STATUS_FRAME].get_f32();
    let hit_frame = VarModule::get_float(fighter.module_accessor, vars::fighter::status::float::HIT_FRAME);

    if frame < hit_frame {
        VarModule::set_float(fighter.module_accessor, vars::fighter::status::float::HIT_FRAME, 0.0);
    }

    if hit_frame == 0.0 {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
            VarModule::set_float(fighter.module_accessor, vars::fighter::status::float::HIT_FRAME, frame);
        }
    }
    
    if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    || AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD)
    && frame != hit_frame {
        VarModule::set_float(fighter.module_accessor, vars::fighter::status::float::HIT_FRAME, frame);
    }
}

unsafe extern "C" fn special_jump_stick_flick(fighter: &mut L2CFighterCommon) {
    if VarModule::get_float(fighter.module_accessor, vars::fighter::instance::float::FLICK_DOWN) > 0.0
    && !fighter.global_table[IS_STOP].get_bool() {
        VarModule::sub_float(fighter.module_accessor, vars::fighter::instance::float::FLICK_DOWN, 1.0);
    }

    if fighter.global_table[STICK_Y].get_f32() < -0.8
    && fighter.global_table[FLICK_Y].get_i32() < 4
    && fighter.global_table[FLICK_Y_DIR].get_f32() < 0.0 {
        VarModule::set_float(fighter.module_accessor, vars::fighter::instance::float::FLICK_DOWN, 5.0);
    }
}

unsafe fn purged_handler(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_STATUS_KIND_REBIRTH {
        VarModule::off_flag(fighter.module_accessor, vars::fighter::instance::flag::PURGED);
    }
    if VarModule::is_flag(fighter.module_accessor, vars::fighter::instance::flag::PURGED) {
        let eff = VarModule::get_int(fighter.module_accessor, vars::fighter::instance::int::PURGED_EFF_HANDLE) as u32;
        if !EffectModule::is_exist_effect(fighter.module_accessor, eff) {
            EffectModule::req_follow(
                fighter.module_accessor,
                Hash40::new("sys_aura_light"),
                Hash40::new("hip"),
                &vars::ZERO_VECTOR,
                &vars::ZERO_VECTOR,
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
            EffectModule::set_rate(fighter.module_accessor, eff, 0.8);
            EffectModule::set_rgb(fighter.module_accessor, eff, 0.0, 1.0, 0.0);
            VarModule::set_int(fighter.module_accessor, vars::fighter::instance::int::PURGED_EFF_HANDLE, eff as i32);
        }
        VarModule::dec_int(fighter.module_accessor, vars::fighter::instance::int::PURGED_TIMER);
        if VarModule::get_int(fighter.module_accessor, vars::fighter::instance::int::PURGED_TIMER) <= 0 {
            VarModule::off_flag(fighter.module_accessor, vars::fighter::instance::flag::PURGED);
        }
    }
    else {
        let eff = VarModule::get_int(fighter.module_accessor, vars::fighter::instance::int::PURGED_EFF_HANDLE) as u32;
        if EffectModule::is_exist_effect(fighter.module_accessor, eff) {
            EffectModule::kill(fighter.module_accessor, eff, false, false);
        }
    }
}

unsafe extern "C" fn acd(fighter: &mut L2CFighterCommon) {
    // Jabs and Tilts cancel into roll on hit or block
    // (Potentially) half distance when canceled into
    // Normal intangibility

    // Status system
    // Every specific action has a unique status kind (most of the time)
    // ex richter bounce on hit is still considered an aerial
    // but greninja's just goes straight into double jump
    // ??????????

    // Check what status we're in
    // either through StatusModule::status_kind(fighter.module_accessor);
    // OR
    // fighter.global_table[0xB].get_i32()

    let status = fighter.global_table[0xB].get_i32();

    // check status
    if [
        *FIGHTER_STATUS_KIND_ATTACK, // Jab
    ].contains(&status) {
        // Now we check if we hit something
        // and check if we're not in hitlag
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
        && !fighter.global_table[IS_STOP].get_bool() {
            let escape = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
            let escape_f = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);

            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
            fighter.sub_transition_group_check_ground_escape();

            if !escape {
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
            }
            if !escape_f {
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn common_fighter_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        hit_cancel_frame_set(fighter);
        special_jump_stick_flick(fighter);
        purged_handler(fighter);
        acd(fighter);
    }
}