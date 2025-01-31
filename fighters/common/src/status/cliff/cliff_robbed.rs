use super::*;

pub static CLIFF_ROBBED_FALL_FRAME : f32 = 18.0;

#[skyline::hook(replace = L2CFighterCommon_status_CliffRobbed)]
unsafe extern "C" fn status_cliffrobbed(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_CLIFF_ROBBED);

    let motion = if fighter.global_table[KIND].get_i32() == *FIGHTER_KIND_KOOPAG {
        Hash40::new("fall")
    }
    else {
        Hash40::new("damage_air_2")
    };

    let cliff_robbed_no_control_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("cliff_robbed_no_control_frame"));
    let rate = fighter.sub_get_adjust_rate_from_cancel_frame(motion.into(), cliff_robbed_no_control_frame.into()).get_f32();

    MotionModule::change_motion(
        fighter.module_accessor,
        motion,
        0.0,
        rate,
        false,
        0.0,
        false,
        false
    );

    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);

    let cliff_robbed_speed_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("cliff_robbed_speed_x"));
    let cliff_robbed_speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("cliff_robbed_speed_y"));

    let air_brake_x = WorkModule::get_param_float(fighter.module_accessor, hash40("air_brake_x"), 0);

    let lr = PostureModule::lr(fighter.module_accessor);

    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        ENERGY_STOP_RESET_TYPE_AIR,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0
    );

    let speed_x = (cliff_robbed_speed_x - 0.5 * air_brake_x * (CLIFF_ROBBED_FALL_FRAME * CLIFF_ROBBED_FALL_FRAME)) / CLIFF_ROBBED_FALL_FRAME;

    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        speed_x,
        -1.0
    );

    sv_kinetic_energy!(
        set_limit_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        speed_x,
        -1.0
    );

    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        speed_x * -lr,
        0.0
    );

    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);

    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
    let speed_y = (cliff_robbed_speed_y - 0.5 * -air_accel_y * (CLIFF_ROBBED_FALL_FRAME * CLIFF_ROBBED_FALL_FRAME)) / CLIFF_ROBBED_FALL_FRAME;

    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        speed_y
    );

    sv_kinetic_energy!(
        set_limit_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        speed_y
    );

    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        speed_y
    );

    let cliff_release_disable_wall_jump_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("cliff_release_disable_wall_jump_frame"));
    WorkModule::set_int(fighter.module_accessor, cliff_release_disable_wall_jump_frame, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_WALL_JUMP_FRAME);

    // ShakeModule::req(
    //     fighter.module_accessor,
    //     Hash40::new("damage_air"),
    //     5,
    //     false,
    //     &Vector2f{x: 0.0, y: 1.0},
    //     1.0,
    //     0.0,
    //     false,
    //     false
    // );

    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_cliff_robbed_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_cliff_robbed_uniq as *const () as _));

    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_CliffRobbed_Main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_sub_cliff_robbed_uniq)]
unsafe extern "C" fn sub_cliff_robbed_uniq(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        let cliff_robbed_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_CLIFF_ROBBED);
        let cliff_robbed_no_control_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("cliff_robbed_no_control_frame"));
        if cliff_robbed_frame == CLIFF_ROBBED_FALL_FRAME as i32 {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
            sv_kinetic_energy!(
                reset_energy,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
                0.0,
                speed_y,
                0.0,
                0.0,
                0.0
            );
        }
        if cliff_robbed_frame == cliff_robbed_no_control_frame {
            sv_kinetic_energy!(
                reset_energy,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0
            );

            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        
            fighter.sub_air_check_fall_common_pre();
        }
    }
    else {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_CLIFF_ROBBED);
    }

    fighter.sub_fall_common_uniq(param_1);

    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_cliffrobbed,
            sub_cliff_robbed_uniq
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}