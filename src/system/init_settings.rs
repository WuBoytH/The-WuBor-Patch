use {
    smash::{
        app::{lua_bind::*, *},
        lib::lua_const::*,
    },
    custom_var::*
};

#[skyline::hook( replace = StatusModule::init_settings )]
pub unsafe fn init_settings_replace(
    module_accessor: *mut BattleObjectModuleAccessor,
    situation: SituationKind,
    kinetic: i32,
    correct: u32,
    cliff_check: GroundCliffCheckKind,
    jostle: bool,
    keep_flag: i32,
    keep_int: i32,
    keep_float: i32,
    arg10: i32,
) {
    let status = StatusModule::status_kind(module_accessor);
    let status_prev = StatusModule::prev_status_kind(module_accessor, 0);
    let object_id = (*module_accessor).battle_object_id;
    let status_pair = VarModule::check_reset_statuses(object_id, status, status_prev);
    let mut mask = 0;
    if keep_flag != *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG
    && !status_pair {
        mask += VarModule::RESET_STATUS_FLAG;
    }
    if keep_int != *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT
    && !status_pair {
        mask += VarModule::RESET_STATUS_INT;
        mask += VarModule::RESET_STATUS_INT64;
    }
    if keep_float != *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT
    && !status_pair {
        mask += VarModule::RESET_STATUS_FLOAT;
    }
    VarModule::reset(module_accessor, mask);
    original!()(
        module_accessor,
        situation,
        kinetic,
        correct,
        cliff_check,
        jostle,
        keep_flag,
        keep_int,
        keep_float,
        arg10
    )
}

pub fn install() {
    skyline::install_hooks!(
        init_settings_replace
    );
    
}