use {
    smash::{
        lua2cpp::{L2CFighterCommon, L2CFighterBase},
        app::lua_bind::*,
        lib::lua_const::*
    },
    smashline::*,
    super::super::element::helper::*,
    wubor_utils::{
        wua_bind::*,
        vars::*,
        table_const::*
    }
};

#[fighter_frame( agent = FIGHTER_KIND_EFLAME )]
fn eflame_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_S
        && MotionModule::frame(fighter.module_accessor) >= 14.0 {
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                WorkModule::enable_transition_term(fighter.module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
                WorkModule::enable_transition_term(fighter.module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
                WorkModule::enable_transition_term(fighter.module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH);
            }
            if ControlModule::check_button_trigger(fighter.module_accessor,*CONTROL_PAD_BUTTON_SPECIAL)
            && !WorkModule::is_flag(fighter.module_accessor, FIGHTER_EFLAME_SPECIAL_S_FLAG_ROTATE) {
                WorkModule::on_flag(fighter.module_accessor, FIGHTER_EFLAME_SPECIAL_S_FLAG_ROTATE);
            }
        }

        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FUNNY) {
            if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_LW
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_FINAL
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_EFLAME_STATUS_KIND_FINAL_READY
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_EFLAME_STATUS_KIND_FINAL_SCENE01
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_EFLAME_STATUS_KIND_FINAL_SCENE02
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_EFLAME_STATUS_KIND_FINAL_END
            && MiscModule::is_damage_check(fighter.module_accessor, false) == false {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
            }
        }

        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC) {
            element_fgc(fighter);
        }
    }
}

#[weapon_frame( agent = WEAPON_KIND_EFLAME_ESWORD )]
fn eflame_esword_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        if StatusModule::status_kind(weapon.module_accessor) == *WEAPON_EFLAME_ESWORD_STATUS_KIND_SPECIAL_S_FLY {
            let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
            let oboma = smash::app::sv_battle_object::module_accessor(otarget_id);
            if smash::app::utility::get_kind(&mut *oboma) == *FIGHTER_KIND_EFLAME
            && WorkModule::is_flag(oboma, FIGHTER_EFLAME_SPECIAL_S_FLAG_ROTATE) {
                StatusModule::change_status_request_from_script(weapon.module_accessor, *WEAPON_EFLAME_ESWORD_STATUS_KIND_SPECIAL_S_ROTATE, true);
            }
        }
    }
}

pub fn install() {
    install_agent_frames!(
        eflame_frame,
        eflame_esword_frame
    );
}