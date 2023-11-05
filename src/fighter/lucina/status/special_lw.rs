use crate::imports::status_imports::*;

#[status_script(agent = "lucina", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe extern "C" fn lucina_speciallw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.module_accessor, yu::status::flag::SPECIAL_LW_DECIDE_ROMAN_DIREC);
    VarModule::off_flag(fighter.module_accessor, yu::status::flag::SPECIAL_LW_ROMAN_MOVE);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_lw"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    else {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_lw"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        lucina_speciallw_substatus(fighter);
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(lucina_speciallw_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_speciallw_main_loop as *const () as _))
}

unsafe extern "C" fn lucina_speciallw_substatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, yu::status::flag::SPECIAL_LW_DECIDE_ROMAN_DIREC) {
        let dir = FGCModule::get_command_stick_direction(fighter, false);
        let movement = match dir {
            4 | 7 | 1 => -2.0,
            6 | 9 | 3 => 2.0,
            _ => 0.0
        };
        VarModule::set_float(fighter.module_accessor, yu::status::float::SPECIAL_LW_ROMAN_MOVE, movement);
        VarModule::off_flag(fighter.module_accessor, yu::status::flag::SPECIAL_LW_DECIDE_ROMAN_DIREC);
    }
    0.into()
}

unsafe extern "C" fn lucina_speciallw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if VarModule::is_flag(fighter.module_accessor, yu::status::flag::SPECIAL_LW_ROMAN_MOVE) {
        let move_x = VarModule::get_float(fighter.module_accessor, yu::status::float::SPECIAL_LW_ROMAN_MOVE);
        PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{
            x: move_x,
            y: 0.0
        });
        VarModule::set_float(fighter.module_accessor, yu::status::float::SPECIAL_LW_ROMAN_MOVE, move_x * 0.9);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, lucina_speciallw_main);
}