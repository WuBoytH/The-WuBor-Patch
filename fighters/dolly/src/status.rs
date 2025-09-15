use super::*;

pub unsafe extern "C" fn dolly_super_special_end_helper(fighter: &mut L2CFighterCommon, hash: L2CValue) {
    let param = hash.get_u64();
    let map_coll_joint = WorkModule::get_param_int64(fighter.module_accessor, param, hash40("map_coll_joint"));
    let offx = WorkModule::get_float(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SUPER_SPECIAL_WORK_FLOAT_MAP_COLL_OFFSET_X);
    let offy = WorkModule::get_float(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SUPER_SPECIAL_WORK_FLOAT_MAP_COLL_OFFSET_Y);
    let offz = WorkModule::get_float(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SUPER_SPECIAL_WORK_FLOAT_MAP_COLL_OFFSET_Z);
    GroundModule::set_shape_data_rhombus_modify_node_offset(fighter.module_accessor, Hash40::new_raw(map_coll_joint), &Vector3f{x: offx, y: offy, z: offz});
}

// mod wait;

// mod walk;

mod dash_back;

// mod squat_wait;

mod landing;

mod guard_on;
mod guard_off;

mod escape;

mod attack;

mod attack_dash;

mod attack_s3;

mod attack_hi3;

mod attack_lw3;
mod attack_lw32;

mod attack_air;

mod special_n;

mod special_s;

mod special_hi;

mod special_lw;
mod special_lw_attack;
mod special_lw_breaking;

mod super_special;
mod super_special2;

mod appeal;

pub fn install(agent: &mut Agent) {
    // wait::install(agent);

    // walk::install(agent);

    dash_back::install(agent);

    // squat_wait::install(agent);

    landing::install(agent);

    guard_on::install(agent);
    guard_off::install(agent);

    escape::install(agent);

    attack::install(agent);

    attack_dash::install(agent);

    attack_s3::install(agent);

    attack_hi3::install(agent);

    attack_lw3::install(agent);
    attack_lw32::install(agent);

    attack_air::install(agent);

    special_n::install(agent);

    special_s::install(agent);

    special_hi::install(agent);

    special_lw::install(agent);
    special_lw_attack::install(agent);
    special_lw_breaking::install(agent);

    super_special::install(agent);
    super_special2::install(agent);

    appeal::install(agent);
}