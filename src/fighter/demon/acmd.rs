mod jabs;
mod normals;
mod smashes;
mod catch;
mod throws;
mod aerials;
mod specials;
mod stand;
mod step;
mod squat;
mod escape;
mod appeal;

pub fn install() {
    jabs::install();
    normals::install();
    smashes::install();
    catch::install();
    throws::install();
    aerials::install();
    specials::install();
    stand::install();
    step::install();
    squat::install();
    escape::install();
    appeal::install();
}