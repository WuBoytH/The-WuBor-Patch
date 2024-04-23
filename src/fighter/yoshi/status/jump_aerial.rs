use crate::imports::*;

unsafe extern "C" fn yoshi_jump_aerial_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_JumpAerialSub(false.into(), false.into());
    // Removes Double Jump Armor
    // let reaction = WorkModule::get_float(fighter.module_accessor, *FIGHTER_YOSHI_INSTANCE_WORK_ID_FLOAT_AERIAL_DAMAGE_REACTION);
    // DamageModule::set_no_reaction_mode_status(
    //     fighter.module_accessor,
    //     DamageNoReactionMode(*DAMAGE_NO_REACTION_MODE_REACTION_VALUE),
    //     reaction,
    //     -1.0,
    //     -1
    // );
    // WorkModule::on_flag(fighter.module_accessor, *FIGHTER_YOSHI_INSTANCE_WORK_ID_FLAG_JUMP_AERIAL_ARMOR);
    let turn_value = WorkModule::get_param_float(fighter.module_accessor, hash40("jump_aerial"), hash40("turn_cont_value"));
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let lr = PostureModule::lr(fighter.module_accessor);
    let turn_count = if turn_value < stick_x * -lr {
        WorkModule::get_param_int(fighter.module_accessor, hash40("jump_aerial"), hash40("turn_count"))
    }
    else {
        0
    };
    WorkModule::set_int(fighter.module_accessor, turn_count, *FIGHTER_YOSHI_INSTANCE_WORK_ID_INT_AERIAL_TURN_COUNT);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_JumpAerial_Main as *const () as _))
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_JUMP_AERIAL, yoshi_jump_aerial_main);
}