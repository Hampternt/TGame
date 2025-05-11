use getset::{Getters, Setters, MutGetters};


#[derive(Default, Getters, Setters, MutGetters)]
pub struct CharacterStats {
    #[getset(get = "pub", set = "pub")]
    name: String,
    #[getset(get = "pub", set = "pub")]
    willpower: i32,
    #[getset(get = "pub", set = "pub")]
    attributes: Attributes,
    #[getset(get = "pub", set = "pub")]
    skills: Skills,
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    equipment: Vec<Equipment>,
}

#[derive(Getters,Setters)]
struct Equipment {
    #[getset(get = "pub", set = "pub")]
    name: String,
    #[getset(get = "pub", set = "pub")]
    descriptoin: String,
    #[getset(get = "pub", set = "pub")]
    value: String,
}

#[derive(Default, Getters, Setters)]
struct Attributes {
    #[getset(get = "pub", set = "pub")]
    strength: i32,
    #[getset(get = "pub", set = "pub")]
    dexterity: i32,
    #[getset(get = "pub", set = "pub")]
    stamina: i32,
    #[getset(get = "pub", set = "pub")]
    charisma: i32,
    #[getset(get = "pub", set = "pub")]
    manipulation: i32,
    #[getset(get = "pub", set = "pub")]
    composure: i32,
    #[getset(get = "pub", set = "pub")]
    inteligence: i32,
    #[getset(get = "pub", set = "pub")]
    wits: i32,
    #[getset(get = "pub", set = "pub")]
    resolve: i32,
}

#[derive(Default, Getters, Setters)]
pub struct Skills {
    #[getset(get = "pub", set = "pub")]
    alertness: i32,
    #[getset(get = "pub", set = "pub")]
    art: i32,
}
