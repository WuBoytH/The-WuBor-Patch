mod special_n;
mod breath;

pub fn install(agent : &mut smashline::Agent) {
    special_n::install(agent);
    breath::install(agent);
}