use crate::imports::status_imports::*;

#[common_status_script( status = FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE )]
unsafe fn escape_air_slide_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MOTION_FALL,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        false,
        false,
        0,
        0,
        0,
        0
    );
    0.into()
}

#[common_status_script( status = FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS )]
unsafe fn escape_air_slide_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status = fighter.global_table[PREV_STATUS_KIND].get_i32();
    if [
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_SAVING_DAMAGE_FLY
    ].contains(&prev_status) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_PREV_STATUS_PASSIVE_GROUND);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_PREV_STATUS_PASSIVE_AIR);
        sv_kinetic_energy!(
            controller_set_accel_x_mul,
            fighter,
            0.0
        );
        sv_kinetic_energy!(
            controller_set_accel_x_add,
            fighter,
            0.0
        );
    }
    if [
        *FIGHTER_STATUS_KIND_DAMAGE_FALL,
        *FIGHTER_STATUS_KIND_TREAD_FALL
    ].contains(&prev_status) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_PREV_STATUS_PASSIVE_GROUND);
    }
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
    FighterWorkModuleImpl::calc_escape_air_slide_param(fighter.module_accessor, 0.0);
    fighter.setup_escape_air_slide();
    0.into()
}

#[common_status_script( status = FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN )]
unsafe fn escape_air_slide_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_escape_air_common();
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("escape_air_slide"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_EscapeAir_Main as *const () as _))
}

#[common_status_script( status = FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END )]
unsafe fn escape_air_slide_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    escape_air_slide_end_inner(fighter)
}

pub unsafe fn escape_air_slide_end_inner(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status == *FIGHTER_STATUS_KIND_FALL
    || status == *FIGHTER_STATUS_KIND_LANDING {
        let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("landing_frame_escape_air_slide"));
        WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        let escape_air_slide_landing_speed_max = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("escape_air_slide_landing_speed_max")) * 0.75;
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
        let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
        let landing_speed_mul_escape_air_slide = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("landing_speed_mul_escape_air_slide"));
        let mut landing_speed = speed_x * landing_speed_mul_escape_air_slide;
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
        let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
        if escape_air_slide_landing_speed_max < landing_speed.abs() {
            if landing_speed < 0.0 {
                landing_speed = -escape_air_slide_landing_speed_max;
            }
            else {
                landing_speed = escape_air_slide_landing_speed_max;
            }
        }
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            landing_speed,
            speed_y
        );
        if status == *FIGHTER_STATUS_KIND_LANDING {
            if !MotionModule::is_end(fighter.module_accessor) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_TURN);
            }
            // WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_LANDING_CLIFF_STOP);
        }
    }
    else {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
        let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
        let mut air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
        if speed_x.abs() > air_speed_x_stable {
            air_speed_x_stable *= speed_x.signum();
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
            let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                air_speed_x_stable,
                speed_y
            );
        }
    }
    0.into()
}

#[common_status_script( status = FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, condition = LUA_SCRIPT_STATUS_FUNC_CALC_PARAM )]
unsafe fn escape_air_slide_calc_param(fighter: &mut L2CFighterCommon) -> L2CValue {
    FighterWorkModuleImpl::calc_escape_air_slide_param(fighter.module_accessor, 0.0);
    0.into()
}

pub fn install() {
    install_status_scripts!(
        escape_air_slide_pre,
        escape_air_slide_init,
        escape_air_slide_main,
        escape_air_slide_end,
        escape_air_slide_calc_param
    );
}