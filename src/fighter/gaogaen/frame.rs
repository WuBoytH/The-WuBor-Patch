use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smashline::*,
    wubor_utils::{
        wua_bind::*,
        vars::*,
        table_const::*
    }
};

#[fighter_frame( agent = FIGHTER_KIND_GAOGAEN )]
fn gaogaen_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        
        // Darkest Lariat Jump Cancel

        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
        && StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_N
        && WorkModule::is_flag(fighter.module_accessor, FIGHTER_STATUS_WORK_ID_FLAG_JUMP_CANCEL)
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        && !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
        && !fighter.global_table[IN_HITLAG].get_bool() {
            FGCModule::jump_cancel_check_exception(fighter);
        }

        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_S_LARIAT
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        && WorkModule::get_int(fighter.module_accessor, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_INT_REVENGE) > 0
        && WorkModule::get_int(fighter.module_accessor, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_INT_REVENGE) < 3
        && AttackModule::get_power(fighter.module_accessor, 0, false, 1.0, false) >= 20.0 {
            MiscModule::critical_zoom(fighter, 0, 2.0, 1.5);
            WorkModule::set_int(fighter.module_accessor, 3, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_INT_REVENGE);
        }

        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_LW
        && !fighter.global_table[IN_HITLAG].get_bool() {
            if WorkModule::get_int(fighter.module_accessor, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_INT_REVENGE) == 2 {
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