use crate::imports::status_imports::*;

pub unsafe extern "C" fn pickel_attack_que(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status = fighter.global_table[PREV_STATUS_KIND].get_i32();
    if !FighterSpecializer_Pickel::is_status_kind_attack(prev_status) {
        fighter.sub_GetLightItemImm(L2CValue::Void());
        if StatusModule::status_kind_que_from_script(fighter.module_accessor) as i32 != *STATUS_KIND_NONE {
            return true.into();
        }
    }
    false.into()
}

pub unsafe extern "C" fn pickel_attack_catch_item(fighter: &mut L2CFighterCommon) {
    let catch_frame_param = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        hash40("item_catch_frame_attack_3")
    }
    else {
        hash40("item_air_catch_frame")
    };
    let catch_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), catch_frame_param);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_ATTACK_FRAME) <= catch_frame {
        fighter.sub_GetLightItemImm(L2CValue::Void());
    }
}
