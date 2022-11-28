import { fs, tauri } from "tauri-apps-api"; //"tauri-apps-api-dist";
import init, { flatbuffers_examples, greet } from "../pkg";
import * as flatbuffers from "flatbuffers";
import { Color, Equipment, Monster, Vec3, Weapon } from "./monster_generated";

// import "../pkg/index_bg.wasm";
// console.log(1, 2, 3, fs.exists, tauri.invoke, init, greet);

const _name__ = "flatbuffers";

fs.readBinaryFile(`apps/${_name__}/dist/index_bg.wasm`)
	.then(async (bytes) => {
		let blob = new Blob([bytes], { type: "application/wasm" });
		let url = URL.createObjectURL(blob);
		await init(url);

		greet("lucky day");

		bytes = flatbuffers_exa(createMonstor());
		console.log("------------------------------------------");
		bytes = flatbuffers_examples(bytes);
		flatbuffers_exa(bytes);
	})
	.catch(console.error);

function flatbuffers_exa(bytes: Uint8Array) {
	let buf = new flatbuffers.ByteBuffer(bytes);
	let monster = Monster.getRootAsMonster(buf);

	// Get access to the root:
	// let monster = flatbuffers:: root::<Monster>(buf).unwrap();
	console.log(monster);

	// Get and test some scalar types from the FlatBuffer.
	let hp = monster.hp();
	let mana = monster.mana();
	let name = monster.name();

	// assert_eq!(hp, 80);
	// assert_eq!(mana, 150); // default
	// assert_eq!(name, Some("Orc"));

	// // Get and test a field of the FlatBuffer's `struct`.
	// assert!(monster.pos().is_some());
	let pos = monster.pos()!; //.unwrap();
	let x = pos.x();
	let y = pos.y();
	let z = pos.z();
	// assert_eq!(x, 1.0f32);
	// assert_eq!(y, 2.0f32);
	// assert_eq!(z, 3.0f32);

	// Get an element from the `inventory` FlatBuffer's `vector`.
	// assert!(monster.inventory().is_some());
	let third_item = monster.inventory(2)!; //.unwrap();
	// let third_item = inv.get(2);
	// assert(third_item === 2);

	// Get and test the `weapons` FlatBuffers's `vector`.
	// assert(monster.weapons());
	// let weps = monster.weapons().unwrap();
	//let weps_len = weps.len();
	let wep2 = monster.weapons(1)!;
	let second_weapon_name = wep2.name()!;
	let second_weapon_damage = wep2.damage()!;
	// assert(second_weapon_name === "Axe");
	// assert(second_weapon_damage === 5);

	// Get and test the `Equipment` union (`equipped` field).
	// assert(monster.equippedType()! === Equipment.Weapon);

	let weapon = new Weapon();
	monster.equipped(weapon); //.name(); // 'Axe'
	// assert(weapon.name()! === "Axe");
	// assert(weapon.damage()! === 5);

	// Get and test the `path` FlatBuffers's `vector`.
	//assert_eq!(monster.path().unwrap().len(), 2);
	//assert_eq!(monster.path().unwrap()[0].x(), 1.0);
	//assert_eq!(monster.path().unwrap()[1].x(), 4.0);

	console.log("The FlatBuffer was successfully created and accessed!");
	console.log(weapon, wep2, third_item, pos, name, mana, hp);

	return bytes;
}
function createMonstor() {
	// Build up a serialized buffer algorithmically.
	// Initialize it with a capacity of 1024 bytes.
	let builder = new flatbuffers.Builder(1024); //::FlatBufferBuilder::with_capacity(1024);

	// Serialize some weapons for the Monster: A 'sword' and an 'axe'.
	let weapon_one_name = builder.createString("Sword");
	let weapon_two_name = builder.createString("Axe");
	let monsterName = builder.createString("Orc");
	// Use the `Weapon::create` shortcut to create Weapons with named field
	// arguments.
	let sword = Weapon.createWeapon(builder, weapon_one_name, 3);

	let axe = Weapon.createWeapon(builder, weapon_two_name, 5);

	// Name of the Monster.
	// let nameOrc = builder.createString("Orc");

	// Inventory.
	// let inventory = builder.create_vector(& [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
	let inventory = Monster.createInventoryVector(builder, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
	// Create a FlatBuffer `vector` that contains offsets to the sword and axe
	// we created above.
	// let weapons = builder.create_vector(& [sword, axe]);
	let weapons = Monster.createWeaponsVector(builder, [sword, axe]);
	// let posOrc = Vec3.createVec3(builder, 1.0, 2.0, 3.0);

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
	// let orc = Monster:: create(
	//     & mut builder,
	//     & MonsterArgs {
	// 		pos: Some(& Vec3:: new (1.0f32, 2.0f32, 3.0f32)),
	// 		mana: 150,
	// 		hp: 80,
	// 		name: Some(name),
	// 		inventory: Some(inventory),
	// 		color: Color:: Red,
	// 		weapons: Some(weapons),
	// 		equipped_type: Equipment:: Weapon,
	// 		equipped: Some(axe.as_union_value()),
	// 		//path: Some(path),
	// 		..Default::default ()
	// 	},
	// );
	// Monster.createMonster(
	// 	builder,
	// 	posOrc,
	// 	150,
	// 	80,
	// 	nameOrc,
	// 	inventory,
	// 	Color.Red,
	// 	weapons,
	// 	Equipment.Weapon,
	// 	axe,
	// 	undefined,
	// );
	Monster.startMonster(builder);
	Monster.addPos(builder, Vec3.createVec3(builder, 1.0, 2.0, 3.0));
	Monster.addHp(builder, 80);
	Monster.addColor(builder, Color.Red);
	Monster.addName(builder, monsterName);
	Monster.addInventory(builder, inventory);
	Monster.addWeapons(builder, weapons);
	Monster.addEquippedType(builder, Equipment.Weapon);
	Monster.addEquipped(builder, axe);
	// Monster.addPath(builder, path);
	let orc = Monster.endMonster(builder);

	// Serialize the root of the object, without providing a file identifier.
	builder.finish(orc);
	console.log(builder);

	// We now have a FlatBuffer we can store on disk or send over a network.

	// ** file/network code goes here :) **

	// Instead, we're going to access it right away (as if we just received it).
	// This must be called after `finish()`.
	// let buf = builder.finished_data(); // Of type `&[u8]`
	let bytes = builder.asUint8Array();
	// writeFileSync('monster.bin', buf, 'binary');

	return bytes;
}
