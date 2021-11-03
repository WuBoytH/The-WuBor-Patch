use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    crate::table_const::*
};

#[status_script(agent = "luigi", status = FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_CHARGE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn luigi_specialscharge(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_DISCHARGE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_FLASHING);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_WORK_FLOAT_CHARGE);
    if !StopModule::is_stop(fighter.module_accessor) {
        luigi_specialschargestop(fighter);
    }
    fighter.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(luigi_specialschargestop as *const () as _));
    luigi_specialscharge2(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(luigi_specialschargemain as *const () as _))
}

unsafe extern "C" fn luigi_specialschargestop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let charge_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("charge_speed_mul"));
    WorkModule::add_float(fighter.module_accessor, charge_speed, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_WORK_FLOAT_CHARGE);
    0.into()
}

unsafe extern "C" fn luigi_specialscharge2(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_FIRST) == false {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_hold"), 1.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_FIRST);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s_hold"), -1.0, 1.0, 0.0, false, false);
        }
        WorkModule::set_int(fighter.module_accessor, fighter.sub_end_added_lines().get_i32(), *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_INT_MTRANS);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_FIRST) == false {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_hold"), 1.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_FIRST);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s_hold"), -1.0, 1.0, 0.0, false, false);
        }
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_INT_MTRANS);
    }
    return
}

unsafe extern "C" fn luigi_specialschargemain(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) == false {
        let charge = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_WORK_FLOAT_CHARGE);
        let charge_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("charge_frame"));
        if charge_frame <= charge {
            fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END.into(), false.into());
        }
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            luigi_specialscharge2(fighter);
        }
    }
    else {
        fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END.into(), false.into());
    }
    L2CValue::I32(0)
}

pub fn install() {
    install_status_scripts!(
        luigi_specialscharge
    );
}