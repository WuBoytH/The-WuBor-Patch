use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::Hash40,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smashline::*,
    wubor_utils::{vars::*, table_const::*},
    super::{agent_init::*, vars::*}
};

// Notes:
// vc_ken_special_l01 is "I hit my boiling point!"
// vc_ken_special_l02 is "Shoryureppa"

unsafe fn dolly_reset_vars(fighter: &mut L2CFighterCommon) {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if [
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH
    ].contains(&status)  {
        WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLOAT_GO_METER);
    }
}

unsafe fn dolly_super_special_aura(fighter: &mut L2CFighterCommon) {
    if WorkModule::get_float(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLOAT_GO_METER) >= 200.0{
        let eff = WorkModule::get_int(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_SUPER_SPECIAL_AURA) as u32;
        if !EffectModule::is_exist_effect(fighter.module_accessor, eff) {
            EffectModule::req_follow(
                fighter.module_accessor,
                Hash40::new("sys_aura_light"),
                Hash40::new("hip"),
                &ZERO_VECTOR,
                &ZERO_VECTOR,
                4.0,
                false,
                0,
                0,
                0,
                0,
                0,
                false,
                false
            );
            let eff = EffectModule::get_last_handle(fighter.module_accessor) as u32;
            EffectModule::set_rgb(fighter.module_accessor, eff, 1.0, 1.0, 0.2);
            WorkModule::set_int(fighter.module_accessor, eff as i32, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_SUPER_SPECIAL_AURA);
        }
        let eff = WorkModule::get_int(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_SUPER_SPECIAL_AURA2) as u32;
        if !EffectModule::is_exist_effect(fighter.module_accessor, eff) {
            EffectModule::req_follow(
                fighter.module_accessor,
                Hash40::new("sys_aura_light"),
                Hash40::new("hip"),
                &ZERO_VECTOR,
                &ZERO_VECTOR,
                4.0,
                false,
                0,
                0,
                0,
                0,
                0,
                false,
                false
            );
            let eff = EffectModule::get_last_handle(fighter.module_accessor) as u32;
            EffectModule::set_rgb(fighter.module_accessor, eff, 1.0, 1.0, 0.2);
            WorkModule::set_int(fighter.module_accessor, eff as i32, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_SUPER_SPECIAL_AURA2);
        }
    }
    else {
        let eff = WorkModule::get_int(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_SUPER_SPECIAL_AURA) as u32;
        let eff2 = WorkModule::get_int(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_SUPER_SPECIAL_AURA2) as u32;
        if EffectModule::is_exist_effect(fighter.module_accessor, eff) {
            EffectModule::kill(fighter.module_accessor, eff, true, false);
            EffectModule::kill(fighter.module_accessor, eff2, true, false);
        }
    }
}

unsafe fn dolly_super_super_cancels(fighter: &mut L2CFighterCommon) {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status == *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2_BLOW
    && MotionModule::frame(fighter.module_accessor) < 10.0 {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL) {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
        }
        dolly_check_special_command(fighter);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_CANCEL);
    }
    if status == *FIGHTER_STATUS_KIND_ATTACK_DASH
    && fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2_BLOW
    && !fighter.global_table[IN_HITLAG].get_bool()
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
        fighter.sub_transition_group_check_ground_special();
    }
}

#[fighter_frame( agent = FIGHTER_KIND_DOLLY )]
fn dolly_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        dolly_reset_vars(fighter);
        dolly_super_special_aura(fighter);
        dolly_super_super_cancels(fighter);
    }
}

pub fn install() {
    install_agent_frames!(
        dolly_frame
    );
}