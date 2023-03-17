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

#[fighter_frame( agent = FIGHTER_KIND_GAOGAEN, main )]
fn gaogaen_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        
        // Darkest Lariat Jump Cancel

        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
        && StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_N
        && VarModule::is_flag(fighter.battle_object, fighter::status::flag::JUMP_CANCEL)
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        && !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
        && !fighter.global_table[IS_STOP].get_bool() {
            FGCModule::jump_cancel_check_exception(fighter);
        }

        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_S_LARIAT
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        && VarModule::get_int(fighter.battle_object, gaogaen::instance::int::REVENGE) > 0
        && VarModule::get_int(fighter.battle_object, gaogaen::instance::int::REVENGE) < 3
        && AttackModule::get_power(fighter.module_accessor, 0, false, 1.0, false) >= 20.0 {
            MiscModule::critical_zoom(fighter, 0, 2.0, 1.5);
            VarModule::set_int(fighter.battle_object, gaogaen::instance::int::REVENGE, 3);
        }

        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_LW
        && !fighter.global_table[IS_STOP].get_bool() {
            if VarModule::get_int(fighter.battle_object, gaogaen::instance::int::REVENGE) == 2 {
                fighter.change_status(FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_S_LARIAT.into(), true.into());
            }
        }
    }
}

pub fn install() {
    install_agent_frames!(
        gaogaen_frame
    );
}