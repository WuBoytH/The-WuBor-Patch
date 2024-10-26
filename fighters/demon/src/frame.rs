use super::*;

extern "C" {
    #[link_name = "common_fighter_frame"]
    pub fn common_fighter_frame(fighter: &mut L2CFighterCommon);
}

unsafe extern "C" fn autoturn_on_cancel_frame(fighter: &mut L2CFighterCommon) {
    let new_lr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    let lr = PostureModule::lr(fighter.module_accessor);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && CancelModule::is_enable_cancel(fighter.module_accessor)
    && new_lr != lr {
        let status = if [
            *FIGHTER_STATUS_KIND_ATTACK_LW3,
            *FIGHTER_STATUS_KIND_ATTACK_LW4,
            *FIGHTER_DEMON_STATUS_KIND_ATTACK_SQUAT_1,
            *FIGHTER_DEMON_STATUS_KIND_ATTACK_SQUAT_2,
            *FIGHTER_DEMON_STATUS_KIND_ATTACK_SQUAT_3,
            *FIGHTER_DEMON_STATUS_KIND_ATTACK_SQUAT_4,
        ].contains(&StatusModule::status_kind(fighter.module_accessor)) {
            FIGHTER_DEMON_STATUS_KIND_SQUAT_TURN_AUTO
        }
        else {
            FIGHTER_DEMON_STATUS_KIND_TURN_AUTO
        };
        fighter.change_status(status.into(), false.into());
    }
}

unsafe extern "C" fn on_main(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    autoturn_on_cancel_frame(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, on_main);
}