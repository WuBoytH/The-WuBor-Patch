use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    custom_var::*,
    wubor_utils::{vars::*, table_const::*}
};

unsafe extern "C" fn kirby_specialn_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_COPY) {
        return 1.into();
    }
    else {
        let copy_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA);
        if copy_kind == *FIGHTER_KIND_ROSETTA {
            let rosetta_interval = WorkModule::get_int(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_ROSETTA_SPECIAL_N_INTERVAL);
            if rosetta_interval <= 0 {
                return 1.into();
            }
            else {
                return 0.into();
            }
        }
        if copy_kind == *FIGHTER_KIND_GANON {
            if VarModule::is_flag(fighter.battle_object, commons::instance::flag::DISABLE_SPECIAL_N) {
                return 0.into();
            }
            else {
                return 1.into();
            }
        }
        if copy_kind != *FIGHTER_KIND_PIT {
            if copy_kind != *FIGHTER_KIND_PITB {
                if copy_kind == *FIGHTER_KIND_INKLING {
                    let inkling_ink = WorkModule::get_float(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLOAT_INKLING_SPECIAL_N_INK);
                    if inkling_ink > 0.0 {
                        return 1.into();
                    }
                    else {
                        return 0.into();
                    }
                }
                return 1.into();
            }
        }
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_COPY_STRANS_OFF) {
            return 1.into();
        }
    }
    0.into()
}

#[fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_KIRBY {
            return;
        }
        fighter.global_table[CHECK_SPECIAL_N_UNIQ].assign(&L2CValue::Ptr(kirby_specialn_pre as *const () as _));
    }
}

pub fn install() {
    install_agent_init_callbacks!(
        agent_init
    );
}
