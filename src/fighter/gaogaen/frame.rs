use {
    smash::{
        lua2cpp::*,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smashline::*,
    custom_var::*,
    wubor_utils::{
        wua_bind::*,
        vars::*,
        table_const::*
    }
};

#[line("gaogaen", main)]
unsafe fn gaogaen_frame(fighter: &mut L2CFighterCommon) {
        
        // Darkest Lariat Jump Cancel

        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
        && StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_N
        && VarModule::is_flag(fighter.battle_object, fighter::status::flag::JUMP_CANCEL)
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        && !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
        && !fighter.global_table[IS_STOP].get_bool() {
            FGCModule::jump_cancel_check_exception(fighter);
        }
}

pub fn install() {
    gaogaen_frame::install();
}