use super::*;

unsafe extern "C" fn dolly_super_special2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, vars::dolly::instance::flag::SPECIAL_CANCEL) {
        VarModule::on_flag(fighter.module_accessor, vars::dolly::status::flag::IS_SPECIAL_CANCEL);
        VarModule::off_flag(fighter.module_accessor, vars::dolly::instance::flag::SPECIAL_CANCEL);
    }
    let original = original_status(Main, fighter, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2);
    original(fighter)
}

unsafe extern "C" fn dolly_super_special2_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2_BLOW {
        VarModule::off_flag(fighter.module_accessor, vars::dolly::status::flag::IS_SPECIAL_CANCEL);
        ArticleModule::remove_exist(
            fighter.module_accessor,
            *FIGHTER_DOLLY_GENERATE_ARTICLE_FIRE,
            ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)
        );
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("dolly_buster_punch"), true, true);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("dolly_buster_dash"), true, true);
    }
    0.into()
}

unsafe extern "C" fn dolly_super_special2_blow_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status != *FIGHTER_STATUS_KIND_WAIT
    && status != *FIGHTER_STATUS_KIND_FALL
    && status != *FIGHTER_STATUS_KIND_CATCH_CUT {
        CatchModule::catch_cut(fighter.module_accessor, false, false);
    }
    if ![
        vars::dolly::status::ATTACK_DASH_COMMAND
    ].contains(&status) {
        VarModule::off_flag(fighter.module_accessor, vars::dolly::instance::flag::RISING_FORCE);
        EffectModule::clear_screen(fighter.module_accessor, 1, 0x14);
    }
    else {
        VarModule::on_flag(fighter.module_accessor, vars::dolly::instance::flag::RISING_FORCE);
    }
    VarModule::off_flag(fighter.module_accessor, vars::dolly::status::flag::IS_SPECIAL_CANCEL);
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2, dolly_super_special2_main);
    agent.status(End, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2, dolly_super_special2_end);

    agent.status(End, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2_BLOW, dolly_super_special2_blow_end);
}