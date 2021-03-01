// use smash::phx::Hash40;
use smash::lua2cpp::L2CAgentBase;
// use smash::app::sv_animcmd;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
// use smash_script::macros;

static mut UP_SPECIAL_CANCEL : [bool; 8] = [false; 8];

#[fighter_frame( agent = FIGHTER_KIND_EDGE )]
unsafe fn edge_frame(fighter: &mut L2CAgentBase) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    // Cancel Frames

    if MotionModule::motion_kind(boma) == smash::hash40("special_hi")
    || MotionModule::motion_kind(boma) == smash::hash40("special_hi1") {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            UP_SPECIAL_CANCEL[entry_id] = true;
        }
    }
    if StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_SPECIAL_HI {
        UP_SPECIAL_CANCEL[entry_id] = false;
    }
    if UP_SPECIAL_CANCEL[entry_id] == true {
        CancelModule::enable_cancel(boma);
    }
}

pub fn install() {
    smash_script::replace_fighter_frames!(
        edge_frame
    );
}