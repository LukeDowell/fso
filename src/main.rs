use bevy::prelude::*;


struct Person;
struct Name(String);

fn add_people(commands: &mut Commands) {
    commands
        .spawn((Person, Name("Luke".to_string())))
        .spawn((Person, Name("Michelle".to_string())))
        .spawn((Person, Name("Garbage".to_string())));
}

struct GreetTimer(Timer);
fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if !timer.0.tick(time.delta_seconds()).just_finished() {
        return;
    }

    for name in query.iter() {
        println!("hello, {}", name.0)
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(add_people.system())
            .add_system(greet_people.system());
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
        .run();
}