use super::*;

pub unsafe extern "C" fn demon_attack_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_GetLightItemImm(L2CValue::Void());
    (StatusModule::status_kind_que_from_script(fighter.module_accessor) as i32 != *STATUS_KIND_NONE).into()
}