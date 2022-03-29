use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::{Hash40, Vector3f},
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    smashline::*,
    super::{
        // command_inputs::*,
        common_fgc::*
    },
    wubor_utils::{
        wua_bind::*,
        vars::*,
        table_const::*
    }
};

unsafe fn fgc_setup(fighter : &mut L2CFighterCommon) {
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
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC);
        FGC_TRAINING = false;
    }
    else if FGC_TRAINING {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC);
    }
    else {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC);
    }
}

unsafe fn taunt_holds(fighter : &mut L2CFighterCommon) {
    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_APPEAL
    && WorkModule::is_flag(fighter.module_accessor, FIGHTER_STATUS_APPEAL_WORK_FLAG_APPEAL_HOLD) {
        let button = WorkModule::get_int(fighter.module_accessor, FIGHTER_STATUS_APPEAL_WORK_INT_APPEAL_HELD_BUTTON);
        if ControlModule::check_button_off(fighter.module_accessor, button) {
            let lr = PostureModule::lr(fighter.module_accessor);
            let mot = if lr < 0.0 {
                WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_APPEAL_WORK_INT_MOTION_KIND_L)
            }
            else {
                WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_APPEAL_WORK_INT_MOTION_KIND_R)
            };
            let restart_frame = WorkModule::get_int(fighter.module_accessor, FIGHTER_STATUS_APPEAL_WORK_INT_APPEAL_RESTART_FRAME) as f32;
            MotionModule::change_motion_force_inherit_frame(
                fighter.module_accessor,
                Hash40::new_raw(mot),
                restart_frame,
                1.0,
                0.0
            );
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_STATUS_APPEAL_WORK_FLAG_APPEAL_HOLD);
        }
    }
}

// Use this for general per-frame fighter-level hooks
#[fighter_frame_callback]
fn common_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {

        fgc_setup(fighter);
        common_fgc(fighter);
        taunt_holds(fighter);

        // Checks what frame you hit the opponent.

        if WorkModule::get_float(fighter.module_accessor, FIGHTER_STATUS_WORK_ID_FLOAT_CANCEL_TIMER) > 0.0
        && !fighter.global_table[IN_HITLAG].get_bool() {
            WarkModule::count_down(fighter.module_accessor, FIGHTER_STATUS_WORK_ID_FLOAT_CANCEL_TIMER, 1.0);
        }

        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        || AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
            WorkModule::set_float(fighter.module_accessor, 10.0, FIGHTER_STATUS_WORK_ID_FLOAT_CANCEL_TIMER);
        }

        // if WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_USED_GROUND_NORMALS) != 0b0000000
        // && CancelModule::is_enable_cancel(fighter.module_accessor) {
        //     WorkModule::set_int(fighter.module_accessor, 0b0000000, FIGHTER_INSTANCE_WORK_ID_INT_USED_GROUND_NORMALS);
        // }

        // global_command_inputs(fighter);
    }
}

pub fn install() {
    install_agent_frame_callbacks!(
        common_fighter_frame
    );
}