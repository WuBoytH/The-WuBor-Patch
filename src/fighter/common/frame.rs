use {
    smash::{
        lua2cpp::*,
        hash40,
        phx::*,
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    smashline::*,
    custom_var::*,
    super::fgc::*,
    wubor_utils::{vars::*, table_const::*}
};

unsafe fn training_fgc_setup(fighter: &mut L2CFighterCommon) {
    if smashball::is_training_mode() {
        if fighter.global_table[KIND].get_i32() != *FIGHTER_KIND_NANA {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
            && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                FGC_TRAINING = !FGC_TRAINING;
                if !FGC_TRAINING {
                    let pos = Vector3f{x: 0.0, y: 13.0, z: 0.0};
                    let rot = Vector3f{x: 0.0, y: 90.0, z: 0.0};
                    let onemoreeff: u32 = EffectModule::req_follow(fighter.module_accessor, smash::phx::Hash40{hash: hash40("sys_flame")}, smash::phx::Hash40{hash: hash40("top")}, &pos, &rot, 1.0, false, 0, 0, 0, 0, 0, false, false) as u32;
                    EffectModule::set_rgb(fighter.module_accessor, onemoreeff, 0.0, 0.0, 5.0);
                }
                else {
                    let pos = Vector3f{x: 0.0, y: 13.0, z: 0.0};
                    let rot = Vector3f{x: 0.0, y: 90.0, z: 0.0};
                    let onemoreeff: u32 = EffectModule::req_follow(fighter.module_accessor, smash::phx::Hash40{hash: hash40("sys_flame")}, smash::phx::Hash40{hash: hash40("top")}, &pos, &rot, 1.0, false, 0, 0, 0, 0, 0, false, false) as u32;
                    EffectModule::set_rgb(fighter.module_accessor, onemoreeff, 5.0, 0.0, 0.0);
                }
            }
        }
        VarModule::set_flag(fighter.battle_object, fighter::instance::flag::IS_FGC, FGC_TRAINING);
    }
}

unsafe fn hit_cancel_frame_set(fighter: &mut L2CFighterCommon) {
    let frame = fighter.global_table[STATUS_FRAME].get_f32();
    let hit_frame = VarModule::get_float(fighter.battle_object, fighter::status::float::HIT_FRAME);

    if frame < hit_frame {
        VarModule::set_float(fighter.battle_object, fighter::status::float::HIT_FRAME, 0.0);
    }

    if hit_frame == 0.0 {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
            VarModule::set_float(fighter.battle_object, fighter::status::float::HIT_FRAME, frame);
        }
    }
    
    if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    || AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD)
    && frame != hit_frame {
        VarModule::set_float(fighter.battle_object, fighter::status::float::HIT_FRAME, frame);
    }
}

unsafe fn special_jump_stick_flick(fighter: &mut L2CFighterCommon) {
    if VarModule::get_float(fighter.battle_object, fighter::instance::float::FLICK_DOWN) > 0.0
    && !fighter.global_table[IS_STOP].get_bool() {
        VarModule::sub_float(fighter.battle_object, fighter::instance::float::FLICK_DOWN, 1.0);
    }

    if fighter.global_table[STICK_Y].get_f32() < -0.8
    && fighter.global_table[FLICK_Y].get_i32() < 4
    && fighter.global_table[FLICK_Y_DIR].get_f32() < 0.0 {
        VarModule::set_float(fighter.battle_object, fighter::instance::float::FLICK_DOWN, 5.0);
    }
}

// Use this for general per-frame fighter-level hooks
#[fighter_frame_callback( main )]
fn common_fighter_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if utility::get_category(&mut *fighter.module_accessor) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
            training_fgc_setup(fighter);
            fgc_frame(fighter);
            hit_cancel_frame_set(fighter);
            special_jump_stick_flick(fighter);
        }
    }
}

pub fn install() {
    install_agent_frame_callbacks!(
        common_fighter_frame
    );
}