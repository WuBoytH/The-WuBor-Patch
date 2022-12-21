use {
    smash::lua2cpp::L2CAgentBase,
    smashline::*
};

#[acmd_script( agent = "snake_cypher", script = "game_detach", category = ACMD_GAME )]
unsafe fn snake_cypher_detach(_fighter: &mut L2CAgentBase) {
}

pub fn install() {
    install_acmd_scripts!(
        snake_cypher_detach
    );
}