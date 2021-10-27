use smash::{
    lua2cpp::L2CFighterCommon,
    lib::{lua_const::*, L2CValue}
};
use smashline::*;
use crate::{
    vars::*,
    common_funcs::*,
    mario::mario_speciallw_restrict,
    kirby::kirby_specialn_restrict,
    daisy::{
        // daisy_speciallw_restrict,
        daisy_itemtoss_special
    },
    ken::ken_speciallw_restrict,
    kamui::kamui_escapeair_restrict,
    lucina::{
        yu_specialns_restrict,
        yu_speciallw_restrict
    },
    lucario::lucario_specialhi_restrict
};

unsafe extern "C" fn specialn_restrict_generic(fighter: &mut L2CFighterCommon) -> L2CValue {
    if DISABLE_SPECIAL_N[entry_id(fighter.module_accessor)] {
        return 0.into();
    }
    1.into()
}

unsafe extern "C" fn specials_restrict_generic(fighter: &mut L2CFighterCommon) -> L2CValue {
    if DISABLE_SPECIAL_S[entry_id(fighter.module_accessor)] {
        return 0.into();
    }
    1.into()
}

unsafe extern "C" fn specialhi_restrict_generic(fighter: &mut L2CFighterCommon) -> L2CValue {
    if DISABLE_SPECIAL_HI[entry_id(fighter.module_accessor)] {
        return 0.into();
    }
    1.into()
}

unsafe extern "C" fn speciallw_restrict_generic(fighter: &mut L2CFighterCommon) -> L2CValue {
    if DISABLE_SPECIAL_LW[entry_id(fighter.module_accessor)] {
        return 0.into();
    }
    1.into()
}

#[fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = smash::app::utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind == *FIGHTER_KIND_MARIO {
            fighter.global_table[0x3b].assign(&L2CValue::Ptr(mario_speciallw_restrict as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_KIRBY {
            fighter.global_table[0x38].assign(&L2CValue::Ptr(kirby_specialn_restrict as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_DAISY {
            // fighter.global_table[0x26].assign(&L2CValue::Bool(false));
            fighter.global_table[0x28].assign(&L2CValue::Ptr(daisy_itemtoss_special as *const () as _));
            fighter.global_table[0x2d].assign(&L2CValue::Ptr(daisy_itemtoss_special as *const () as _));
            fighter.global_table[0x39].assign(&L2CValue::Ptr(specials_restrict_generic as *const () as _));
            // fighter.global_table[0x3b].assign(&L2CValue::Ptr(daisy_speciallw_restrict as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_PEACH {
            fighter.global_table[0x28].assign(&L2CValue::Ptr(daisy_itemtoss_special as *const () as _));
            fighter.global_table[0x2d].assign(&L2CValue::Ptr(daisy_itemtoss_special as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_GANON {
            fighter.global_table[0x38].assign(&L2CValue::Ptr(specialn_restrict_generic as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_LUCINA {
            fighter.global_table[0x38].assign(&L2CValue::Ptr(yu_specialns_restrict as *const () as _));
            fighter.global_table[0x39].assign(&L2CValue::Ptr(yu_specialns_restrict as *const () as _));
            fighter.global_table[0x3b].assign(&L2CValue::Ptr(yu_speciallw_restrict as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_LUCARIO {
            fighter.global_table[0x3a].assign(&L2CValue::Ptr(lucario_specialhi_restrict as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_SHULK {
            fighter.global_table[0x3b].assign(&L2CValue::Ptr(speciallw_restrict_generic as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_KEN {
            fighter.global_table[0x3b].assign(&L2CValue::Ptr(ken_speciallw_restrict as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_KAMUI {
            fighter.global_table[0x2f].assign(&L2CValue::Ptr(kamui_escapeair_restrict as *const () as _));
        }
        else if fighter_kind == *FIGHTER_KIND_RICHTER {
            fighter.global_table[0x3a].assign(&L2CValue::Ptr(specialhi_restrict_generic as *const () as _));
        }
    }
}

pub fn install() {
    install_agent_init_callbacks!(
        agent_init
    );
}