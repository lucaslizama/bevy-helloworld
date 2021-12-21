use bevy::prelude::*;

struct Person;
struct Name(String);
struct GreetTimer(Timer);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(add_people.system())
            .add_system(greet_people.system());
    }
}

fn hello_world() {
    println!("hello world");
}

fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Elaina Proctor".to_string()));

    commands
        .spawn()
        .insert(Person)
        .insert(Name("Renzo Hume".to_string()));

    commands
        .spawn()
        .insert(Person)
        .insert(Name("Zayna Nieves".to_string()));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("hello {}!", name.0);
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}
