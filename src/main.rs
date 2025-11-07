use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, (update_name, greet_people).chain()))
        .run();
}

fn hello_world() {
    println!("Hello, World!");
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("David Tenant".to_string())));
    commands.spawn((Person, Name("Matt Smith".to_string())));
    commands.spawn((Person, Name("Peter Capaldi".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query{
        println!("Hello, {}!", name.0);
    }
}

fn update_name(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query{
        if name.0 == "David Tenant"{
            name.0 = "Dave Tents".to_string();
            break;
        }
    }
}
