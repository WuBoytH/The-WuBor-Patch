use super::*;

unsafe extern "C" fn attack_s3_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_AttackS3()
}

unsafe extern "C" fn attack_s3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_ATTACK {
        let motion = MotionModule::motion_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new_raw(motion), true);
        if frame < cancel_frame {
            VarModule::on_flag(fighter.module_accessor, vars::demon::status::flag::ATTACK_S3_FROM_JAB);
        }
    }
    fighter.status_AttackS3()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_ATTACK_S3, attack_s3_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_S3, attack_s3_main);
}