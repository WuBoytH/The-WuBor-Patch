use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smashline::*,
    wubor_utils::table_const::*,
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

unsafe fn dolly_super_super_cancels(fighter: &mut L2CFighterCommon) {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status == *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2_BLOW
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL)
    && MotionModule::frame(fighter.module_accessor) < 10.0 {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
        dolly_check_super_special_command(fighter);
    }
}

#[fighter_frame( agent = FIGHTER_KIND_DOLLY )]
fn dolly_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        dolly_reset_vars(fighter);
        dolly_super_super_cancels(fighter);
    }
}

pub fn install() {
    install_agent_frames!(
        dolly_frame
    );
}