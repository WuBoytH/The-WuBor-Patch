use super::*;

pub unsafe extern "C" fn guard_cancel_appeal_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
    VarModule::on_flag(fighter.module_accessor, vars::fighter::instance::flag::BURNOUT);

    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_APPEAL);
    1.into()
}