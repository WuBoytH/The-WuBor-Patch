use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    smashline::*,
    wubor_utils::table_const::*
};

#[status_script(agent = "jack", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn jack_specials_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    jack_special_mot_helper(fighter, true.into(), hash40("special_s1").into(), hash40("special_air_s1").into());
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_09 - 1);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_09 - 1);
    jack_special_s_main_helper(fighter)
}

unsafe extern "C" fn jack_special_mot_helper(fighter: &mut L2CFighterCommon, da_bool: L2CValue, ground_mot: L2CValue, air_mot: L2CValue) {
    let mot;
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        mot = air_mot.get_u64();
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_S_FLAG_FALL_NORMAL) == false {
            let speed_max_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("speed_max_y"));
            let accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("accel_y"));
            fighter.clear_lua_stack();
            lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -accel_y);
            sv_kinetic_energy::set_accel(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_max_y);
            sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_max_y);
            sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_S_FLAG_CONTROL_ENERGY) {
            let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            fighter.clear_lua_stack();
            lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, *ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, sum_speed_x, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        }
        if !(da_bool.get_bool() == false) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot), 0.0, 1.0, false, 0.0, false, false);
            return;
        }
    }
    else {
        mot = ground_mot.get_u64();
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        if !(da_bool.get_bool() == false) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot), 0.0, 1.0, false, 0.0, false, false);
            return;
        }
    }
    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot), -1.0, 1.0, 0.0, false, false);
    return
}

unsafe extern "C" fn jack_special_s_main_helper(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
        let air_start_speed_mul_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("air_start_speed_mul_x"));
        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x * air_start_speed_mul_x, 0.0);
        sv_kinetic_energy::set_speed(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
        let air_start_speed_mul_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("air_start_speed_mul_y"));
        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y * air_start_speed_mul_y);
        sv_kinetic_energy::set_speed(fighter.lua_state_agent);
    }
    WorkModule::set_float(fighter.module_accessor, 20.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    fighter.sub_shift_status_main(L2CValue::Ptr(jack_special_s_main_loop as *const () as _))
}

unsafe extern "C" fn jack_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }
    if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RESET);
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if !MotionModule::is_end(fighter.module_accessor) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_S_FLAG_SET_FALL_NORMAL) {
            let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            fighter.clear_lua_stack();
            lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, sum_speed_y, 0.0, 0.0, 0.0);
            sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_S_FLAG_SET_FALL_NORMAL);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_S_FLAG_FALL_NORMAL);
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_S_FLAG_ENABLE_CONTROL_ENERGY) {
            let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            fighter.clear_lua_stack();
            lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, *ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, sum_speed_x, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_S_FLAG_ENABLE_CONTROL_ENERGY);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_S_FLAG_CONTROL_ENERGY);
        }
        if StatusModule::is_changing(fighter.module_accessor) {
            return 0.into();
        }
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
            && MotionModule::frame(fighter.module_accessor) >= 34.0 {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
                return 0.into();
            }
            else {
                jack_special_mot_helper(fighter, false.into(), hash40("special_s1").into(), hash40("special_air_s1").into());
            }
        }
    }
    else {
        if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

#[status_script(agent = "jack", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn jack_specials_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
    ItemModule::set_attach_item_visibility(fighter.module_accessor, true, 0);
    VisibilityModule::set_whole(fighter.module_accessor, true);
    macros::AFTER_IMAGE_OFF(fighter, 0);
    if [
        *FIGHTER_STATUS_KIND_FALL_SPECIAL,
        *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL
    ].contains(&fighter.global_table[STATUS_KIND].get_i32()) {
        WorkModule::set_float(fighter.module_accessor,  20.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        jack_specials_end
    );
}