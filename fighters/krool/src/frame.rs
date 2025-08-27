use super::*;

extern "C" {
    #[link_name = "common_fighter_frame"]
    pub fn common_fighter_frame(fighter: &mut L2CFighterCommon);
}

unsafe extern "C" fn propellerpack_mash(fighter: &mut L2CFighterCommon) {
    if [
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI,
    ].contains(&fighter.global_table[STATUS_KIND].get_i32()) {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            VarModule::inc_int(fighter.module_accessor, vars::krool::status::int::SPECIAL_HI_MASH_COUNT);
        }
        let mash_count = VarModule::get_int(fighter.module_accessor, vars::krool::status::int::SPECIAL_HI_MASH_COUNT);
        if mash_count >= 3 {
            VarModule::set_int(fighter.module_accessor, vars::krool::status::int::SPECIAL_HI_MASH_COUNT, 0);
            ArticleModule::set_flag(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BACKPACK, true, 0x1000000C);
        }
    }
}

unsafe extern "C" fn on_main(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    propellerpack_mash(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, on_main);
}