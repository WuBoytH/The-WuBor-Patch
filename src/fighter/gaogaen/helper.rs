use {
    smash::{
        hash40,
        app::{lua_bind::*, *}
    },
    crate::function_hooks::{
        get_active_battle_object_id_from_entry_id,
        get_battle_object_from_id
    },
    super::vars::*
};

pub unsafe fn handle_revenge(module_accessor: *mut BattleObjectModuleAccessor, attacker_ids: i32) {
    if (MotionModule::motion_kind(module_accessor) == hash40("special_lw_start")
    || MotionModule::motion_kind(module_accessor) == hash40("special_air_lw_start"))
    && WorkModule::get_int(module_accessor, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_INT_REVENGE) == 1 {
        WorkModule::set_int(module_accessor, 2, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_INT_REVENGE);
        for x in 0..8 {
            if attacker_ids & (1 << x) == 0 {
                continue;
            }
            if let Some(object_id) = get_active_battle_object_id_from_entry_id(x) {
                let object = get_battle_object_from_id(object_id);
                let attacker_boma = (*object).module_accessor;
                if PostureModule::pos_x(module_accessor) < PostureModule::pos_x(attacker_boma)
                && PostureModule::lr(module_accessor) == 1.0 {
                    PostureModule::reverse_lr(module_accessor);
                }
                else if PostureModule::pos_x(module_accessor) > PostureModule::pos_x(attacker_boma)
                && PostureModule::lr(module_accessor) == -1.0 {
                    PostureModule::reverse_lr(module_accessor);
                }
                break;
            }
        }
    }
}