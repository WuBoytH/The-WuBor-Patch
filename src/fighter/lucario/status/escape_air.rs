use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    smashline::*,
    custom_var::*,
    wubor_utils::{vars::*, table_const::*}
};

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_ESCAPE_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn lucario_escape_air_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status == *FIGHTER_STATUS_KIND_FALL
    || status == *FIGHTER_STATUS_KIND_LANDING {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
            if !VarModule::is_flag(fighter.battle_object, lucario::instance::flag::FORCE_LANDING_FALL_SPECIAL) {
                let landing_frame_escape_air_slide = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("landing_frame_escape_air_slide"));
                let landing_frame_escape_air_slide_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("landing_frame_escape_air_slide_max"));
                let frame = MotionModule::frame(fighter.module_accessor);
                let end_frame = MotionModule::end_frame(fighter.module_accessor);
                let frame_ratio = frame / end_frame;
                let landing_frame = fighter.lerp(landing_frame_escape_air_slide.into(), landing_frame_escape_air_slide_max.into(), frame_ratio.into()).get_f32();
                WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            }
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
        }
        else {
            if !VarModule::is_flag(fighter.battle_object, lucario::instance::flag::FORCE_LANDING_FALL_SPECIAL) {
                let landing_frame_escape_air = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("landing_frame_escape_air"));
                WorkModule::set_float(fighter.module_accessor, landing_frame_escape_air as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            }
        }
        if status == *FIGHTER_STATUS_KIND_LANDING {
            if !MotionModule::is_end(fighter.module_accessor) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_TURN);
            }
            // WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_LANDING_CLIFF_STOP);
        }
    }
    VarModule::off_flag(fighter.battle_object, commons::instance::flag::FORCE_ESCAPE_AIR_SLIDE);
    fighter.status_end_Jump();
    0.into()
}

pub fn install() {
    install_status_scripts!(
        lucario_escape_air_end
    );
}