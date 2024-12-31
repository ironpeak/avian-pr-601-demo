use avian3d::prelude::PhysicsLayer;

#[derive(PhysicsLayer, Default, Debug)]
pub enum GameLayer {
    #[default]
    Wall,
    Player,
    Enemy,
}

fn main() {
    let layer = GameLayer::Wall;
    println!("Hello, {:?}!", layer);
}
