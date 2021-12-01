use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::Vector3f,
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    smashline::*,
    crate::{
        common_funcs::*,
        vars::*
    },
    super::common_param::*
};

#[inline(always)]
pub unsafe fn global_fgc(fighter: &mut L2CFighterCommon) {
    let status = StatusModule::status_kind(fighter.module_accessor);
    if !is_damage_check(fighter.module_accessor, false) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ESCAPE_XLU_START_1F);
        FGC_HITSTUN_MUL[entry_id(fighter.module_accessor)] = 1.2;
    }
    else  {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ESCAPE_XLU_START_1F);
    }
    if status == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
            let frame : f32;
            if !is_damage_check(fighter.module_accessor, true) {
                frame = 7.0;
            }
            else {
                frame = 30.0;
            }
            if MotionModule::frame(fighter.module_accessor) >= frame {
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        }
    }
}

// Use this for general per-frame fighter-level hooks
#[fighter_frame_callback]
fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status = StatusModule::status_kind(fighter.module_accessor);

        if utility::get_kind(&mut *fighter.module_accessor) == *FIGHTER_KIND_DONKEY {
            IS_DK[entry_id(fighter.module_accessor)] = true;
        }

        // The code to set up Funny Mode.

        if ItemModule::is_attach_item(fighter.module_accessor, ItemKind(*ITEM_KIND_USAGIHAT))
        && !WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FUNNY) {
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FUNNY);
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RABBIT_CAP) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RABBIT_CAP);
        }
        if status == *FIGHTER_STATUS_KIND_DEAD
        && WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FUNNY) {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FUNNY);
        }

        // The code to set up FGC Mode.

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
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC)
        }

        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC) {
            global_fgc(fighter);
        }

        // Checks what frame you hit the opponent.

        if WorkModule::get_float(fighter.module_accessor, FIGHTER_STATUS_WORK_ID_FLOAT_CANCEL_TIMER) > 0.0
        && MotionModule::frame(fighter.module_accessor) > HIT_FRAME[entry_id(fighter.module_accessor)] + 1.0 {
            count_down(fighter.module_accessor, FIGHTER_STATUS_WORK_ID_FLOAT_CANCEL_TIMER, 1.0);
        }

        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        || AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
            HIT_FRAME[entry_id(fighter.module_accessor)] = MotionModule::frame(fighter.module_accessor);
            WorkModule::set_float(fighter.module_accessor, 10.0, FIGHTER_STATUS_WORK_ID_FLOAT_CANCEL_TIMER);
        }
        
        // Command Inputs

        let dir = get_command_stick_direction(fighter.module_accessor, true);
        if WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_COMMAND_INPUT_TIMER) <= command_input_life {
            WorkModule::inc_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_COMMAND_INPUT_TIMER);
        }
        else {
            reset_i32(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_236_STEP);
            reset_i32(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_214_STEP);
            reset_i32(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_623_STEP);
        }

        let qcb = WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_214_STEP);
        let qcf = WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_236_STEP);
        let srk = WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_623_STEP);

        // Quarter Circle Back

        if qcb == 0 {
            if dir == 2 {
                inc_command(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_214_STEP);
            }
        }
        else if qcb == 1 {
            if dir == 1 {
                inc_command(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_214_STEP);
            }
            else if dir != 4
            && dir != 1
            && dir != 2 {
                inc_command(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_214_STEP);
            }
        }
        else if qcb == 2 {
            if dir == 4 {
                inc_command(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_214_STEP);
            }
            else if dir != 4
            && dir != 1
            && dir != 7 {
                reset_i32(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_214_STEP);
            }
        }
        else if qcb == 3 {
            if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
            || ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
            || ControlModule::check_button_on_release(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
            || ControlModule::check_button_on_release(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                WorkModule::set_int(fighter.module_accessor, 13, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_214_STEP);
            }
            else {
                if dir != 4
                && dir != 7
                && dir != 5 {
                    reset_i32(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_214_STEP);
                }
            }
        }
        else if qcb > 3 {
            WorkModule::dec_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_214_STEP);
            if qcb == 3 {
                reset_i32(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_214_STEP);
            }
            else {
                if dir != 4
                && dir != 7
                && dir != 5 {
                    reset_i32(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_214_STEP);
                }
            }
        }

        // Quarter Circle Forward

        if qcf == 0 {
            if dir == 2 {
                inc_command(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_236_STEP);
            }
        }
        else if qcf == 1 {
            if dir == 3 {
                inc_command(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_236_STEP);
            }
            else if dir != 6
            && dir != 3
            && dir != 2 {
                reset_i32(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_236_STEP);
            }
        }
        else if qcf == 2 {
            if dir == 6 {
                inc_command(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_236_STEP);
            }
            else if dir != 6
            && dir != 3
            && dir != 9 {
                reset_i32(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_236_STEP);
            }
        }
        else if qcf == 3 {
            if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
            || ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
            || ControlModule::check_button_on_release(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
            || ControlModule::check_button_on_release(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                WorkModule::set_int(fighter.module_accessor, 13, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_236_STEP);
            }
            else {
                if dir != 6
                && dir != 9
                && dir != 5 {
                    reset_i32(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_236_STEP);
                }
            }
        }
        else if qcf > 3 {
            WorkModule::dec_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_236_STEP);
            if qcf == 3 {
                reset_i32(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_236_STEP);
            }
            else {
                if dir != 6
                && dir != 9
                && dir != 5 {
                    reset_i32(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_236_STEP);
                }
            }
        }

        // Shoryuken

        if srk == 0 {
            if dir == 6
            || dir == 9
            || dir == 3 {
                inc_command(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_623_STEP);
            }
        }
        else if srk == 1 {
            if dir == 2
            || dir == 1 {
                inc_command(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_623_STEP);
            }
            else if dir != 6
            && dir != 3
            && dir != 2
            && dir != 5 {
                reset_i32(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_623_STEP);
            }
        }
        else if srk == 2 {
            if dir == 3
            || dir == 6 {
                inc_command(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_623_STEP);
            }
            else if dir != 3
            && dir != 2
            && dir != 1 {
                reset_i32(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_623_STEP);
            }
        }
        else if srk == 3 {
            if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
            || ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
            || ControlModule::check_button_on_release(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
            || ControlModule::check_button_on_release(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                WorkModule::set_int(fighter.module_accessor, 13, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_623_STEP);
            }
            else {
                if dir != 6
                && dir != 2
                && dir != 3
                && dir != 5
                && dir != 9 {
                    reset_i32(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_623_STEP);
                }
            }
        }
        else if srk > 3 {
            WorkModule::dec_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_623_STEP);
            if srk == 3 {
                reset_i32(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_623_STEP);
            }
            else {
                if dir != 6
                && dir != 2
                && dir != 3
                && dir != 5 {
                    reset_i32(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_623_STEP);
                }
            }
        }

        // The Counter-Hit Code (only applicable to Jabs, Tilts, and Smash Attacks)

        if [
            *FIGHTER_STATUS_KIND_ATTACK,
            *FIGHTER_STATUS_KIND_ATTACK_S3,
            *FIGHTER_STATUS_KIND_ATTACK_HI3,
            *FIGHTER_STATUS_KIND_ATTACK_LW3,
            *FIGHTER_STATUS_KIND_ATTACK_S4_START,
            *FIGHTER_STATUS_KIND_ATTACK_HI4_START,
            *FIGHTER_STATUS_KIND_ATTACK_LW4_START,
            *FIGHTER_STATUS_KIND_ATTACK_S4,
            *FIGHTER_STATUS_KIND_ATTACK_HI4,
            *FIGHTER_STATUS_KIND_ATTACK_LW4,
            *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD,
            *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD,
            *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD,
            *FIGHTER_STATUS_KIND_ATTACK_DASH,
            *FIGHTER_STATUS_KIND_ATTACK_AIR,
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            *FIGHTER_STATUS_KIND_SPECIAL_HI,
            *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F,
            *FIGHTER_RYU_STATUS_KIND_ATTACK_NEAR,
            *FIGHTER_SIMON_STATUS_KIND_ATTACK_HOLD_START,
            *FIGHTER_SIMON_STATUS_KIND_ATTACK_HOLD,
            *FIGHTER_SIMON_STATUS_KIND_ATTACK_LW32,
            *FIGHTER_PICKEL_STATUS_KIND_ATTACK_FALL,
            *FIGHTER_PICKEL_STATUS_KIND_ATTACK_FALL_AERIAL,
            *FIGHTER_PICKEL_STATUS_KIND_ATTACK_JUMP,
            *FIGHTER_PICKEL_STATUS_KIND_ATTACK_WAIT,
            *FIGHTER_PICKEL_STATUS_KIND_ATTACK_WALK,
            *FIGHTER_PICKEL_STATUS_KIND_ATTACK_LANDING,
            *FIGHTER_PICKEL_STATUS_KIND_ATTACK_WALK_BACK,
            *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND1,
            *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND2,
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_COMBO,
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_WAIT,
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_WALK,
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT,
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT_RV,
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_LANDING,
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_LADDER,
            *FIGHTER_METAKNIGHT_STATUS_KIND_ATTACK_S3,
            *FIGHTER_METAKNIGHT_STATUS_KIND_ATTACK_LW3,
        ].contains(&status) {
            COUNTER_HIT_STATE[entry_id(fighter.module_accessor)] = 1;
        }
        else {
            COUNTER_HIT_STATE[entry_id(fighter.module_accessor)] = 0;
        }
    }
}

pub fn install() {
    install_agent_frame_callbacks!(
        global_fighter_frame,
        // once_per_weapon_frame
    );
}