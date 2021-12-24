use bevy::prelude::*;
use components::GreetTimer;
use plugins::HelloPlugin;

mod components;
mod plugins;
mod systems;

pub fn game() -> AppBuilder {
    let mut builder = App::build();
    builder.add_plugins(DefaultPlugins).add_plugin(HelloPlugin);
    builder.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)));
    builder
}
