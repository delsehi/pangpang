use bevy::prelude::*;



pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, &mut app: App) {
        app.insert_resource(Bullet)
        ;
    }
}


fn spawn_bullet(commands: Commands, asset_server: Res<AssetServer>, query: Query<&Player>) {

}