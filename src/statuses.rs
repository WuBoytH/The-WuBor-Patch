use smash::hash40;
use smash::lua2cpp::L2CFighterCommon;
use smash::lib::L2CValue;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::*;
use smashline::*;
use smash_script::*;
use crate::IS_FUNNY;
use crate::commonfuncs::*;
use crate::globals::*;

#[common_status_script(status = FIGHTER_STATUS_KIND_ESCAPE_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END,
    symbol = "_ZN7lua2cpp16L2CFighterCommon20status_end_EscapeAirEv")]
pub fn escapeair_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    unsafe {
        let status_kind = fighter.global_table[STATUS_KIND].clone();
        if status_kind == FIGHTER_STATUS_KIND_FALL || status_kind == FIGHTER_STATUS_KIND_LANDING {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
                use interpolation::Lerp;
                let current_frame = MotionModule::frame(fighter.module_accessor);
                let end_frame = MotionModule::end_frame(fighter.module_accessor);
                let progress = current_frame / end_frame;
                let escape_air_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("landing_frame_escape_air_slide"));
                let escape_air_end_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("landing_frame_escape_air_slide_max"));
                let landing_frame = escape_air_frame.lerp(&escape_air_end_frame, &progress);
                WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                let speed_max = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("escape_air_slide_landing_speed_max"));
                let speed_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("landing_speed_mul_escape_air_slide"));
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
                let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent) * speed_mul;
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
                let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
                let speed_x = if speed_max < speed_x.abs() { speed_max * speed_x.signum() } else { speed_x };
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, speed_y);
                sv_kinetic_energy::set_speed(fighter.lua_state_agent);
            } else {
                let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("landing_frame_escape_air"));
                WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            }
            if status_kind == FIGHTER_STATUS_KIND_LANDING {
                if !MotionModule::is_end(fighter.module_accessor) {
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_TURN);
                }
                if IS_FUNNY[get_player_number(&mut *fighter.module_accessor)] == false {
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_LANDING_CLIFF_STOP);
                }
            }
        }
    }
    0.into()
}

pub fn install() {
    smashline::install_status_scripts!(
        escapeair_end
    );
}