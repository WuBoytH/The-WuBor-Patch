use {
    smash::lua2cpp::L2CAgentBase,
    smash_script::*,
    smashline::*,
    custom_var::*,
    wubor_utils::vars::*
};

#[acmd_script( agent = "falco", scripts = [ "game_appeallwl", "game_appeallwr" ], category = ACMD_GAME)]
unsafe fn falco_appeallw(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, falco::instance::flag::KAA);
    }
}

pub fn install() {
    install_acmd_scripts!(
        falco_appeallw
    );
}