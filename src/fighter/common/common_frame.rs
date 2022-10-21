use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::Vector3f,
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    custom_var::*,
    super::{
        // command_inputs::*,
        common_fgc::*
    },
    wubor_utils::{
        vars::*,
        table_const::*
    }
};

unsafe fn fgc_setup(fighter: &mut L2CFighterCommon) {
    if smashball::is_training_mode() {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
        && ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
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
    if FighterUtil::is_hp_mode(fighter.module_accessor)
    && !smashball::is_training_mode() {
        VarModule::on_flag(fighter.battle_object, commons::instance::flag::IS_FGC);
        FGC_TRAINING = false;
    }
    else if FGC_TRAINING {
        VarModule::on_flag(fighter.battle_object, commons::instance::flag::IS_FGC);
    }
    else {
        VarModule::off_flag(fighter.battle_object, commons::instance::flag::IS_FGC);
    }
}

unsafe fn hit_cancel_frame_set(fighter: &mut L2CFighterCommon) {
    let frame = fighter.global_table[MOTION_FRAME].get_f32();
    let hit_frame = VarModule::get_float(fighter.battle_object, commons::status::float::HIT_FRAME);

    if frame < hit_frame {
        VarModule::set_float(fighter.battle_object, commons::status::float::HIT_FRAME, 0.0);
    }

    if hit_frame == 0.0 {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
            VarModule::set_float(fighter.battle_object, commons::status::float::HIT_FRAME, frame);
        }
    }
    
    if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    || AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD)
    && frame != hit_frame {
        VarModule::set_float(fighter.battle_object, commons::status::float::HIT_FRAME, frame);
    }
}

unsafe fn special_jump_stick_flick(fighter: &mut L2CFighterCommon) {
    if VarModule::get_float(fighter.battle_object, commons::instance::float::FLICK_DOWN) > 0.0
    && !fighter.global_table[IS_STOP].get_bool() {
        VarModule::sub_float(fighter.battle_object, commons::instance::float::FLICK_DOWN, 1.0);
    }

    if fighter.global_table[STICK_Y].get_f32() < -0.8
    && fighter.global_table[FLICK_Y].get_i32() < 4
    && fighter.global_table[FLICK_Y_DIR].get_f32() < 0.0 {
        VarModule::set_float(fighter.battle_object, commons::instance::float::FLICK_DOWN, 5.0);
    }
}

unsafe fn super_jump_gravity(fighter: &mut L2CFighterCommon) {
    if VarModule::is_flag(fighter.battle_object, commons::instance::flag::SUPER_JUMP) {
        let super_jump_frame = VarModule::get_float(fighter.battle_object, commons::instance::float::SUPER_JUMP_FRAME);
        if fighter.global_table[MOTION_FRAME].get_f32() >= 9.0 - super_jump_frame {
            let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                -air_accel_y
            );
            VarModule::off_flag(fighter.battle_object, commons::instance::flag::SUPER_JUMP);
            VarModule::set_float(fighter.battle_object, commons::instance::float::SUPER_JUMP_FRAME, 0.0);
        }
    }
}

// Use this for general per-frame fighter-level hooks
#[fighter_frame_callback]
fn common_fighter_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        fgc_setup(fighter);
        fgc_frame(fighter);
        hit_cancel_frame_set(fighter);
        special_jump_stick_flick(fighter);
        super_jump_gravity(fighter);
    }
}

pub fn install() {
    install_agent_frame_callbacks!(
        common_fighter_frame
    );
}