use super::*;

#[no_mangle]
unsafe extern "C" fn look_up_rv_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_SquatRv()
}

#[no_mangle]
unsafe extern "C" fn look_up_rv_main_common(fighter: &mut L2CFighterCommon, start_status: L2CValue, wait_status: L2CValue) -> L2CValue {
    fighter.status_SquatRv_param(start_status, wait_status, hash40("look_up_rv").into());
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_SquatRv_Main as *const () as _))
}