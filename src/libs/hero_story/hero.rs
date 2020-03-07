extern crate flatbuffers;


#[allow(dead_code, unused_imports)]
#[path = "./hero_generated.rs"]
mod hero_generated;

pub use hero_generated::my_game::sample::{
    get_root_as_monster,
    Color, Equipment,
    Monster, MonsterArgs,
    Vec3,
    Weapon, WeaponArgs,
};

pub fn test()
{
    let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(1024);

    // Serialize some weapons for the Monster: A 'sword' and an 'axe'.
    let weapon_one_name = builder.create_string("Sword");
    let weapon_two_name = builder.create_string("Axe");

    // Use the `Weapon::create` shortcut to create Weapons with named field
    // arguments.
    let sword = Weapon::create(&mut builder, &WeaponArgs{
        name: Some(weapon_one_name),
        damage: 3,
    });
    let axe = Weapon::create(&mut builder, &WeaponArgs{
        name: Some(weapon_two_name),
        damage: 5,
    });

    // Name of the Monster.
    let name = builder.create_string("Orc");
    // Inventory.
    let inventory = builder.create_vector(&[0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    // Create a FlatBuffer `vector` that contains offsets to the sword and axe
    // we created above.
    let weapons = builder.create_vector(&[sword, axe]);


    // Create the path vector of Vec3 objects.
    let x = Vec3::new(1.0, 2.0, 3.0);
    let y = Vec3::new(4.0, 5.0, 6.0);

    // Note that, for convenience, it is also valid to create a vector of
    // references to structs, like this:
    // `let path = builder.create_vector(&[&x, &y]);`
    let path = builder.create_vector(&[x, y]);

    // Create the monster using the `Monster::create` helper function. This
    // function accepts a `MonsterArgs` struct, which supplies all of the data
    // needed to build a `Monster`. To supply empty/default fields, just use the
    // Rust built-in `Default::default()` function, as demonstrated below.
    let orc = Monster::create(&mut builder, &MonsterArgs{
        pos: Some(&Vec3::new(1.0f32, 2.0f32, 3.0f32)),
        mana: 150,
        hp: 80,
        name: Some(name),
        inventory: Some(inventory),
        color: Color::Red,
        weapons: Some(weapons),
        equipped_type: Equipment::Weapon,
        equipped: Some(axe.as_union_value()),
        path: Some(path),
        ..Default::default()
    });

    builder.finish(orc, None);
    // This must be called after `finish()`.
    // `finished_data` returns a byte slice.
    let buf = builder.finished_data();// Of type `&[u8]`
    // Now you can write the bytes to a file, send them over the network.. **Make sure your file mode (or transfer protocol) is set to BINARY, not text.`** If you transfer a FlatBuffer in text mode, the buffer will be corrupted, which will lead to hard to find problems when you read the buffer.


    // Reading Orc FlatBuffers
    // Get an accessor to the root object inside the buffer.
    let monster = get_root_as_monster(buf);

    // Get and test some scalar types from the FlatBuffer.
    let hp = monster.hp();
    let mana = monster.mana();
    let name = monster.name();

    // To access sub-objects, in the case of our pos, which is a Vec3:
    let pos = monster.pos().unwrap();
    let x = pos.x();
    let y = pos.y();
    let z = pos.z();

    // Get a test an element from the `inventory` FlatBuffer's `vector`.
    let inv = monster.inventory().unwrap();

    // Note that this vector is returned as a slice, because direct access for
    // this type, a `u8` vector, is safe on all platforms:
    let third_item = inv[2];

    // Get and test the `weapons` FlatBuffers's `vector`.
    let weps = monster.weapons().unwrap();
    let weps_len = weps.len();

    let wep2 = weps.get(1);
    let second_weapon_name = wep2.name();
    let second_weapon_damage = wep2.damage();

    // Get and test the `Equipment` union (`equipped` field).
    // `equipped_as_weapon` returns a FlatBuffer handle much like normal table
    // fields, but this will return `None` is the union is not actually of that
    // type.
    if monster.equipped_type() == Equipment::Weapon {
        let equipped = monster.equipped_as_weapon().unwrap();
        let weapon_name = equipped.name();
        let weapon_damage = equipped.damage();
    }
}