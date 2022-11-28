extern crate cfg_if;
extern crate flatbuffers;
extern crate wasm_bindgen;

// extern crate greet;

mod utils;

use cfg_if::cfg_if;
use gloo_console::{console, console_dbg};
use wasm_bindgen::prelude::*;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    // fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    // let win = web_sys::window().unwrap();
    //win.alert_with_message(&format!("Greet,{}", name))
    _ = console_dbg!("Greet,{}", name);
    // _ = dbg!(alert(&format!("Greet,{}", name)));
}

#[allow(dead_code, unused_imports)]
#[allow(clippy::all)]
mod rust_generated;
pub use rust_generated::my_game::sample::{
    Color, Equipment, Monster, MonsterArgs, Vec3, Weapon, WeaponArgs,
};

#[wasm_bindgen]
#[allow(clippy::float_cmp)]
pub fn flatbuffers_examples(bytes: &[u8]) -> Vec<u8> {
    fn check(monster: Monster) {
        // Get access to the root:
        console_dbg!(monster);

        // Get and test some scalar types from the FlatBuffer.
        let hp = monster.hp();
        let mana = monster.mana();
        let name = monster.name();

        // assert_eq!(hp, 80);
        assert_eq!(mana, 150); // default
        assert_eq!(name, Some("Orc"));

        // Get and test a field of the FlatBuffer's `struct`.
        assert!(monster.pos().is_some());
        let pos = monster.pos().unwrap();
        let x = pos.x();
        let y = pos.y();
        let z = pos.z();
        assert_eq!(x, 1.0f32);
        assert_eq!(y, 2.0f32);
        assert_eq!(z, 3.0f32);

        // Get an element from the `inventory` FlatBuffer's `vector`.
        assert!(monster.inventory().is_some());
        let inv = monster.inventory().unwrap();
        let third_item = inv.get(2);
        assert_eq!(third_item, 2);

        // Get and test the `weapons` FlatBuffers's `vector`.
        assert!(monster.weapons().is_some());
        let weps = monster.weapons().unwrap();
        //let weps_len = weps.len();
        let wep2 = weps.get(1);
        let second_weapon_name = wep2.name();
        let second_weapon_damage = wep2.damage();
        assert_eq!(second_weapon_name, Some("Axe"));
        assert_eq!(second_weapon_damage, 5);

        // Get and test the `Equipment` union (`equipped` field).
        assert_eq!(monster.equipped_type(), Equipment::Weapon);
        let equipped = monster.equipped_as_weapon().unwrap();
        let weapon_name = equipped.name();
        let weapon_damage = equipped.damage();
        assert_eq!(weapon_name, Some("Axe"));
        assert_eq!(weapon_damage, 5);

        // Get and test the `path` FlatBuffers's `vector`.
        //assert_eq!(monster.path().unwrap().len(), 2);
        //assert_eq!(monster.path().unwrap()[0].x(), 1.0);
        //assert_eq!(monster.path().unwrap()[1].x(), 4.0);

        // console_dbg!("The FlatBuffer was successfully created and accessed!");
        console_dbg!(monster, "<<<>>>");
    }
    // Build up a serialized buffer algorithmically.
    // Initialize it with a capacity of 1024 bytes.
    let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);

    // Serialize some weapons for the Monster: A 'sword' and an 'axe'.
    let weapon_one_name = builder.create_string("Sword");
    let weapon_two_name = builder.create_string("Axe");

    // Use the `Weapon::create` shortcut to create Weapons with named field
    // arguments.
    let sword = Weapon::create(
        &mut builder,
        &WeaponArgs {
            name: Some(weapon_one_name),
            damage: 3,
        },
    );
    let axe = Weapon::create(
        &mut builder,
        &WeaponArgs {
            name: Some(weapon_two_name),
            damage: 5,
        },
    );

    // Name of the Monster.
    let name = builder.create_string("Orc");

    // Inventory.
    let inventory = builder.create_vector(&[0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    // Create a FlatBuffer `vector` that contains offsets to the sword and axe
    // we created above.
    let weapons = builder.create_vector(&[sword, axe]);

    // Create the path vector of Vec3 objects:
    //let x = Vec3::new(1.0, 2.0, 3.0);
    //let y = Vec3::new(4.0, 5.0, 6.0);
    //let path = builder.create_vector(&[x, y]);

    // Note that, for convenience, it is also valid to create a vector of
    // references to structs, like this:
    // let path = builder.create_vector(&[&x, &y]);

    // Create the monster using the `Monster::create` helper function. This
    // function accepts a `MonsterArgs` struct, which supplies all of the data
    // needed to build a `Monster`. To supply empty/default fields, just use the
    // Rust built-in `Default::default()` function, as demonstrated below.
    let orc = Monster::create(
        &mut builder,
        &MonsterArgs {
            pos: Some(&Vec3::new(1.0f32, 2.0f32, 3.0f32)),
            mana: 150,
            hp: 80,
            name: Some(name),
            inventory: Some(inventory),
            color: Color::Red,
            weapons: Some(weapons),
            equipped_type: Equipment::Weapon,
            equipped: Some(axe.as_union_value()),
            //path: Some(path),
            ..Default::default()
        },
    );

    // Serialize the root of the object, without providing a file identifier.
    builder.finish(orc, None);

    // We now have a FlatBuffer we can store on disk or send over a network.

    // ** file/network code goes here :) **

    // Instead, we're going to access it right away (as if we just received it).
    // This must be called after `finish()`.
    let buf_1 = builder.finished_data(); // Of type `&[u8]`

    // decode(buf_1);
    let mon1 = flatbuffers::root::<Monster>(buf_1).unwrap();
    check(mon1);
    let mon2 = flatbuffers::root::<Monster>(bytes).unwrap();
    check(mon2);
    // !! assert_eq!(mon1, mon2);
    console_dbg!("--------==========----------");
    return buf_1.into();
}

#[cfg(test)]
#[test]
fn test_main() {
    // flatbuffers_examples()
}
