use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::lua_const::*
    },
    crate::{
        common_funcs::*,
        vars::*
    },
    super::common_param::*
};

#[inline(always)]
pub unsafe fn global_command_inputs(fighter: &mut L2CFighterCommon) {
    if WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_COMMAND_INPUT_TIMER) <= command_input_life {
        WorkModule::inc_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_COMMAND_INPUT_TIMER);
    }
    else {
        reset_i32(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_236_STEP);
        reset_i32(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_214_STEP);
        reset_i32(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_623_STEP);
    }

    if WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_SUPER_COMMAND_INPUT_TIMER) <= super_command_input_life {
        WorkModule::inc_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_SUPER_COMMAND_INPUT_TIMER);
    }
    else {
        reset_i32(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_236236_STEP);
    }

    dqcf(fighter);
    qcf(fighter);
    qcb(fighter);
    srk(fighter);
}

#[inline(always)]
pub unsafe fn dqcf(fighter: &mut L2CFighterCommon) {
    let dir = get_command_stick_direction(fighter.module_accessor, true);
    let flag = FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_236236_STEP;
    let timer_flag = FIGHTER_INSTANCE_WORK_ID_INT_SUPER_COMMAND_INPUT_TIMER;
    let step = WorkModule::get_int(fighter.module_accessor, flag);

    if step == 0 {
        if dir == 2 {
            inc_command(fighter.module_accessor, flag, timer_flag);
        }
    }
    else if step == 1 {
        if dir == 3 {
            inc_command(fighter.module_accessor, flag, timer_flag);
        }
        else if dir != 6
        && dir != 3
        && dir != 2 {
            reset_i32(fighter.module_accessor, flag);
        }
    }
    else if step == 2 {
        if dir == 6 {
            inc_command(fighter.module_accessor, flag, timer_flag);
        }
        else if dir != 6
        && dir != 3
        && dir != 9 {
            reset_i32(fighter.module_accessor, flag);
        }
    }
    else if step == 3 {
        if dir == 2 {
            inc_command(fighter.module_accessor, flag, timer_flag);
        }
        else if dir != 3
        && dir != 6
        && dir != 1 {
            reset_i32(fighter.module_accessor, flag);
        }
    }
    else if step == 4 {
        if dir == 3 {
            inc_command(fighter.module_accessor, flag, timer_flag);
        }
        else if dir != 6
        && dir != 3
        && dir != 2 {
            reset_i32(fighter.module_accessor, flag);
        }
    }
    else if step == 5 {
        if dir == 6 {
            inc_command(fighter.module_accessor, flag, timer_flag);
        }
        else if dir != 6
        && dir != 3
        && dir != 9 {
            reset_i32(fighter.module_accessor, flag);
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
            && dir != 5 {
                reset_i32(fighter.module_accessor, flag);
            }
        }
    }
    else if step > 6 {
        WorkModule::dec_int(fighter.module_accessor, flag);
        if step == 6 {
            reset_i32(fighter.module_accessor, flag);
        }
        else {
            if dir != 6
            && dir != 9
            && dir != 5 {
                reset_i32(fighter.module_accessor, flag);
            }
        }
    }
}

#[inline(always)]
pub unsafe fn qcf(fighter: &mut L2CFighterCommon) {
    let dir = get_command_stick_direction(fighter.module_accessor, true);
    let flag = FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_236_STEP;
    let timer_flag = FIGHTER_INSTANCE_WORK_ID_INT_COMMAND_INPUT_TIMER;
    let step = WorkModule::get_int(fighter.module_accessor, flag);

    if step == 0 {
        if dir == 2 {
            inc_command(fighter.module_accessor, flag, timer_flag);
        }
    }
    else if step == 1 {
        if dir == 3 {
            inc_command(fighter.module_accessor, flag, timer_flag);
        }
        else if dir != 6
        && dir != 3
        && dir != 2 {
            reset_i32(fighter.module_accessor, flag);
        }
    }
    else if step == 2 {
        if dir == 6 {
            inc_command(fighter.module_accessor, flag, timer_flag);
        }
        else if dir != 6
        && dir != 3
        && dir != 9 {
            reset_i32(fighter.module_accessor, flag);
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
                reset_i32(fighter.module_accessor, flag);
            }
        }
    }
    else if step > 3 {
        WorkModule::dec_int(fighter.module_accessor, flag);
        if step == 3 {
            reset_i32(fighter.module_accessor, flag);
        }
        else {
            if dir != 6
            && dir != 9
            && dir != 3
            && dir != 5 {
                reset_i32(fighter.module_accessor, flag);
            }
        }
    }
}

#[inline(always)]
pub unsafe fn qcb(fighter: &mut L2CFighterCommon) {
    let dir = get_command_stick_direction(fighter.module_accessor, true);
    let flag = FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_214_STEP;
    let timer_flag = FIGHTER_INSTANCE_WORK_ID_INT_COMMAND_INPUT_TIMER;
    let step = WorkModule::get_int(fighter.module_accessor, flag);

    if step == 0 {
        if dir == 2 {
            inc_command(fighter.module_accessor, flag, timer_flag);
        }
    }
    else if step == 1 {
        if dir == 1 {
            inc_command(fighter.module_accessor, flag, timer_flag);
        }
        else if dir != 4
        && dir != 1
        && dir != 2 {
            reset_i32(fighter.module_accessor, flag);
        }
    }
    else if step == 2 {
        if dir == 4 {
            inc_command(fighter.module_accessor, flag, timer_flag);
        }
        else if dir != 4
        && dir != 1
        && dir != 7 {
            reset_i32(fighter.module_accessor, flag);
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
                reset_i32(fighter.module_accessor, flag);
            }
        }
    }
    else if step > 3 {
        WorkModule::dec_int(fighter.module_accessor, flag);
        if step == 3 {
            reset_i32(fighter.module_accessor, flag);
        }
        else {
            if dir != 4
            && dir != 7
            && dir != 1
            && dir != 5 {
                reset_i32(fighter.module_accessor, flag);
            }
        }
    }
}

#[inline(always)]
pub unsafe fn srk(fighter: &mut L2CFighterCommon) {
    let dir = get_command_stick_direction(fighter.module_accessor, true);
    let flag = FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_623_STEP;
    let timer_flag = FIGHTER_INSTANCE_WORK_ID_INT_COMMAND_INPUT_TIMER;
    let step = WorkModule::get_int(fighter.module_accessor, flag);

    if step == 0 {
        if dir == 6
        || dir == 9
        || dir == 3 {
            inc_command(fighter.module_accessor, flag, timer_flag);
        }
    }
    else if step == 1 {
        if dir == 2
        || dir == 1 {
            inc_command(fighter.module_accessor, flag, timer_flag);
        }
        else if dir != 6
        && dir != 3
        && dir != 2 {
            reset_i32(fighter.module_accessor, flag);
        }
    }
    else if step == 2 {
        if dir == 3
        || dir == 6 {
            inc_command(fighter.module_accessor, flag, timer_flag);
        }
        else if dir != 3
        && dir != 2
        && dir != 6 {
            reset_i32(fighter.module_accessor, flag);
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
                reset_i32(fighter.module_accessor, flag);
            }
        }
    }
    else if step > 3 {
        WorkModule::dec_int(fighter.module_accessor, flag);
        if step == 3 {
            reset_i32(fighter.module_accessor, flag);
        }
        else {
            if dir != 6
            && dir != 9
            && dir != 5 {
                reset_i32(fighter.module_accessor, flag);
            }
        }
    }
}
