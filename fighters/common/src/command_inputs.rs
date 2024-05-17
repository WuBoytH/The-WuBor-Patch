use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::lua_const::*
    },
    super::param::*,
    wubor_utils::{
        wua_bind::*,
        vars::*
    }
};

#[inline(always)]
pub unsafe extern "C" fn global_command_inputs(fighter: &mut L2CFighterCommon) {
    if WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_236_STEP) <= 3 {
        FGCModule::check_command_inc(
            fighter,
            FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_236_STEP,
            FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_236_TIMER,
            command_input_window
        );   
    }
    if WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_214_STEP) <= 3 {
        FGCModule::check_command_inc(
            fighter,
            FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_214_STEP,
            FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_214_TIMER,
            command_input_window
        );
    }
    if WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_623_STEP) <= 3 {
        FGCModule::check_command_inc(
            fighter,
            FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_623_STEP,
            FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_623_TIMER,
            command_input_window
        );
    }
    if WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_236236_STEP) <= 6 {
        FGCModule::check_command_inc(
            fighter,
            FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_236236_STEP,
            FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_236236_TIMER,
            super_command_input_window
        );
    }

    dqcf(fighter);
    qcf(fighter);
    qcb(fighter);
    srk(fighter);
}

#[inline(always)]
pub unsafe extern "C" fn dqcf(fighter: &mut L2CFighterCommon) {
    let dir = FGCModule::get_command_stick_direction(fighter, true);
    let flag = FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_236236_STEP;
    let timer_flag = FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_236236_TIMER;
    let step = WorkModule::get_int(fighter.module_accessor, flag);
    if step == 0 {
        if dir == 2
        || dir == 1 {
            WorkModule::inc_int(fighter.module_accessor, flag);
        }
    }
    else if step == 1 {
        if dir == 3 {
            WorkModule::inc_int(fighter.module_accessor, flag);
            WorkModule::set_int(fighter.module_accessor, 0, timer_flag);
        }
        else if dir == 6
        || dir == 9 {
            WorkModule::inc_int(fighter.module_accessor, flag);
            WorkModule::inc_int(fighter.module_accessor, flag);
            WorkModule::set_int(fighter.module_accessor, 0, timer_flag);
        }
        else if dir != 1
        && dir != 3
        && dir != 2 {
            WarkModule::reset_i32(fighter.module_accessor, flag);
        }
    }
    else if step == 2 {
        if dir == 6
        || dir == 9 {
            WorkModule::inc_int(fighter.module_accessor, flag);
        }
        else if dir != 6
        && dir != 3
        && dir != 2 {
            WarkModule::reset_i32(fighter.module_accessor, flag);
        }
    }
    else if step == 3 {
        if dir == 2
        || dir == 1 {
            WorkModule::inc_int(fighter.module_accessor, flag);
        }
        else if dir == 3 {
            WorkModule::inc_int(fighter.module_accessor, flag);
            WorkModule::inc_int(fighter.module_accessor, flag);
        }
        else if dir != 3
        && dir != 6
        && dir != 9 {
            WarkModule::reset_i32(fighter.module_accessor, flag);
        }
    }
    else if step == 4 {
        if dir == 3 {
            FGCModule::inc_command(fighter, flag, timer_flag);
        }
        else if dir == 3
        || dir == 6
        || dir == 9 {
            FGCModule::inc_command(fighter, flag, timer_flag);
        }
        else if dir != 1
        && dir != 3
        && dir != 2 {
            WarkModule::reset_i32(fighter.module_accessor, flag);
        }
    }
    else if step == 5 {
        if dir == 6
        || dir == 9 {
            FGCModule::inc_command(fighter, flag, timer_flag);
        }
        else if dir != 6
        && dir != 3
        && dir != 2 {
            WarkModule::reset_i32(fighter.module_accessor, flag);
        }
    }
    if step == 6 {
        if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
        || ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        || ControlModule::check_button_on_release(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
        || ControlModule::check_button_on_release(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
                WorkModule::set_int(fighter.module_accessor, 16, flag);
            }
        }
        else {
            if dir != 6
            && dir != 9
            && dir != 5
            && dir != 3 {
                WarkModule::reset_i32(fighter.module_accessor, flag);
            }
        }
    }
    else if step > 6 {
        WorkModule::dec_int(fighter.module_accessor, flag);
        if step == 6 {
            WarkModule::reset_i32(fighter.module_accessor, flag);
        }
        else {
            if dir != 6
            && dir != 9
            && dir != 5 {
                WarkModule::reset_i32(fighter.module_accessor, flag);
            }
        }
    }
}

#[inline(always)]
pub unsafe extern "C" fn qcf(fighter: &mut L2CFighterCommon) {
    let dir = FGCModule::get_command_stick_direction(fighter, true);
    let flag = FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_236_STEP;
    let timer_flag = FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_236_TIMER;
    let step = WorkModule::get_int(fighter.module_accessor, flag);

    if step == 0 {
        if dir == 2
        || dir == 1 {
            WorkModule::inc_int(fighter.module_accessor, flag);
        }
    }
    else if step == 1 {
        if dir == 3 {
            WorkModule::inc_int(fighter.module_accessor, flag);
            WorkModule::set_int(fighter.module_accessor, 0, timer_flag);
        }
        else if dir == 6
        || dir == 9 {
            WorkModule::inc_int(fighter.module_accessor, flag);
            WorkModule::inc_int(fighter.module_accessor, flag);
            WorkModule::set_int(fighter.module_accessor, 0, timer_flag);
        }
        else if dir != 6
        && dir != 3
        && dir != 2 {
            WarkModule::reset_i32(fighter.module_accessor, flag);
        }
    }
    else if step == 2 {
        if dir == 6
        || dir == 9 {
            WorkModule::inc_int(fighter.module_accessor, flag);
        }
        else if dir != 6
        && dir != 3
        && dir != 2 {
            WarkModule::reset_i32(fighter.module_accessor, flag);
        }
    }
    if step == 3 {
        if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
        || ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        || ControlModule::check_button_on_release(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
        || ControlModule::check_button_on_release(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
                WorkModule::set_int(fighter.module_accessor, 13, flag);
            }
        }
        else {
            if dir != 6
            && dir != 9
            && dir != 3
            && dir != 5 {
                WarkModule::reset_i32(fighter.module_accessor, flag);
            }
        }
    }
    else if step > 3 {
        WorkModule::dec_int(fighter.module_accessor, flag);
        if step == 3 {
            WarkModule::reset_i32(fighter.module_accessor, flag);
        }
        else {
            if dir != 6
            && dir != 9
            && dir != 3
            && dir != 5 {
                WarkModule::reset_i32(fighter.module_accessor, flag);
            }
        }
    }
}

#[inline(always)]
pub unsafe extern "C" fn qcb(fighter: &mut L2CFighterCommon) {
    let dir = FGCModule::get_command_stick_direction(fighter, true);
    let flag = FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_214_STEP;
    let timer_flag = FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_214_TIMER;
    let step = WorkModule::get_int(fighter.module_accessor, flag);

    if step == 0 {
        if dir == 2
        || dir == 3 {
            WorkModule::inc_int(fighter.module_accessor, flag);
        }
    }
    else if step == 1 {
        if dir == 1 {
            WorkModule::inc_int(fighter.module_accessor, flag);
            WorkModule::set_int(fighter.module_accessor, 0, timer_flag);
        }
        else if dir == 4
        || dir == 7 {
            WorkModule::inc_int(fighter.module_accessor, flag);
            WorkModule::inc_int(fighter.module_accessor, flag);
            WorkModule::set_int(fighter.module_accessor, 0, timer_flag);
        }
        else if dir != 4
        && dir != 1
        && dir != 2 {
            WarkModule::reset_i32(fighter.module_accessor, flag);
        }
    }
    else if step == 2 {
        if dir == 4
        || dir == 7 {
            WorkModule::inc_int(fighter.module_accessor, flag);
        }
        else if dir != 4
        && dir != 1
        && dir != 7 {
            WarkModule::reset_i32(fighter.module_accessor, flag);
        }
    }
    if step == 3 {
        if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
        || ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        || ControlModule::check_button_on_release(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
        || ControlModule::check_button_on_release(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
                WorkModule::set_int(fighter.module_accessor, 13, flag);
            }
        }
        else {
            if dir != 4
            && dir != 7
            && dir != 1
            && dir != 5 {
                WarkModule::reset_i32(fighter.module_accessor, flag);
            }
        }
    }
    else if step > 3 {
        WorkModule::dec_int(fighter.module_accessor, flag);
        if step == 3 {
            WarkModule::reset_i32(fighter.module_accessor, flag);
        }
        else {
            if dir != 4
            && dir != 7
            && dir != 1
            && dir != 5 {
                WarkModule::reset_i32(fighter.module_accessor, flag);
            }
        }
    }
}

#[inline(always)]
pub unsafe extern "C" fn srk(fighter: &mut L2CFighterCommon) {
    let dir = FGCModule::get_command_stick_direction(fighter, true);
    let flag = FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_623_STEP;
    let timer_flag = FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_623_TIMER;
    let step = WorkModule::get_int(fighter.module_accessor, flag);

    if step == 0 {
        if dir == 6
        || dir == 9
        || dir == 3 {
            WorkModule::inc_int(fighter.module_accessor, flag);
        }
    }
    else if step == 1 {
        if dir == 2
        || dir == 1 {
            WorkModule::inc_int(fighter.module_accessor, flag);
            WorkModule::set_int(fighter.module_accessor, 0, timer_flag);
        }
        else if dir != 6
        && dir != 3
        && dir != 2 {
            WarkModule::reset_i32(fighter.module_accessor, flag);
        }
    }
    else if step == 2 {
        if dir == 3
        || dir == 6 {
            WorkModule::inc_int(fighter.module_accessor, flag);
        }
        else if dir != 3
        && dir != 2
        && dir != 6 {
            WarkModule::reset_i32(fighter.module_accessor, flag);
        }
    }
    if step == 3 {
        if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
        || ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        || ControlModule::check_button_on_release(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
        || ControlModule::check_button_on_release(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
                WorkModule::set_int(fighter.module_accessor, 13, flag);
            }
        }
        else {
            if dir != 6
            && dir != 2
            && dir != 3
            && dir != 5
            && dir != 9 {
                WarkModule::reset_i32(fighter.module_accessor, flag);
            }
        }
    }
    else if step > 3 {
        WorkModule::dec_int(fighter.module_accessor, flag);
        if step == 3 {
            WarkModule::reset_i32(fighter.module_accessor, flag);
        }
        else {
            if dir != 6
            && dir != 9
            && dir != 5 {
                WarkModule::reset_i32(fighter.module_accessor, flag);
            }
        }
    }
}
