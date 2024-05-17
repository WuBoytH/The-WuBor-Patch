use super::*;

unsafe extern "C" fn trail_landing_attack_air_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    let mut motion_rate: f32 = 1.0;
    let landing_param_type;
    let mut landing_param : u64 = 0;
    let landing_mot;
    if mot == hash40("attack_air_n") {
        landing_param_type = hash40("landing_attack_air_frame_n");
        landing_mot = hash40("landing_air_n");
    }
    else if mot == hash40("attack_air_f") {
        landing_param_type = hash40("landing_attack_air_frame_f");
        landing_mot = hash40("landing_air_f");
    }
    else if mot == hash40("attack_air_b") {
        landing_param_type = hash40("landing_attack_air_frame_b");
        landing_mot = hash40("landing_air_b");
    }
    else if mot == hash40("attack_air_hi") {
        landing_param_type = hash40("landing_attack_air_frame_hi");
        landing_mot = hash40("landing_air_hi");
    }
    else if mot == hash40("attack_air_lw") {
        landing_param_type = hash40("landing_attack_air_frame_lw");
        landing_mot = hash40("landing_air_lw");
    }
    else {
        landing_param_type = hash40("param_private");
        let luaconst;
        let temp1;
        if mot == hash40("attack_air_n2") {
            landing_param = hash40("landing_attack_air_frame_n2");
            landing_mot = hash40("landing_air_n2");
            temp1 = hash40("landing_attack_air_frame_n");
            luaconst = *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLOAT_LANDING_ATTACK_AIR_FRAME_N;
        }
        else if mot == hash40("attack_air_n3") {
            landing_param = hash40("landing_attack_air_frame_n3");
            landing_mot = hash40("landing_air_n3");
            temp1 = hash40("landing_attack_air_frame_n");
            luaconst = *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLOAT_LANDING_ATTACK_AIR_FRAME_N;
        }
        else if mot == hash40("attack_air_f2") {
            landing_param = hash40("landing_attack_air_frame_f2");
            landing_mot = hash40("landing_air_f2");
            temp1 = hash40("landing_attack_air_frame_f");
            luaconst = *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLOAT_LANDING_ATTACK_AIR_FRAME_F;
        }
        else  {
            landing_param = hash40("landing_attack_air_frame_f3");
            landing_mot = hash40("landing_air_f3");
            temp1 = hash40("landing_attack_air_frame_f");
            luaconst = *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLOAT_LANDING_ATTACK_AIR_FRAME_F;
        }
        let lag = WorkModule::get_param_float(fighter.module_accessor, temp1, 0);
        let other_lag = WorkModule::get_float(fighter.module_accessor, luaconst);
        motion_rate = lag / other_lag;
    }
    let mut landing_lag = WorkModule::get_param_float(fighter.module_accessor, landing_param_type, landing_param);
    landing_lag *= motion_rate;
    if landing_lag != 0.0 {
        motion_rate = fighter.sub_get_landing_motion_rate(landing_mot.into(), landing_lag.into()).get_f32();
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_CANCEL);
    }
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new_raw(landing_mot),
        0.0,
        motion_rate,
        false,
        0.0,
        false,
        false
    );
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RUN_FALL) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_RUN_FALL);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RUN_FALL);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_LANDING_CLIFF_STOP) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_LANDING_CLIFF_STOP);
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, trail_landing_attack_air_init);
}