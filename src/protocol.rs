use std::{
    io::{Read, Seek, SeekFrom},
    path::{Path, PathBuf},
};

use tauri::{
    http::{HttpRange, MimeType, Request, ResponseBuilder},
    utils::debug_eprintln,
    AppHandle,
};

type Result = std::result::Result<tauri::http::Response, Box<(dyn std::error::Error + 'static)>>;

async fn _i__(uri: &str) -> Result {
    let mut response = ResponseBuilder::new();
    let path = uri;

    //tauri::async_runtime::safe_block_on(async move { })
    match tokio::fs::read(path).await {
        Ok(data) => {
            let mime_type = MimeType::parse(&data, uri);
            response.mimetype(&mime_type).body(data)
        }
        Err(e) => {
            debug_eprintln!("Failed to read file: {}", e);
            response.status(404).body(Vec::new())
        }
    }
}

pub fn bat_protocol(_app: &AppHandle, request: &Request) -> Result {
    let mut response = ResponseBuilder::new();
    //let _: serde_json::Value = serde_json::from_slice(request.body().as_slice()).unwrap();

    //let path = request.uri().strip_prefix("bat://localhost/");
    // if cfg!(windows) {
    //     request.uri().strip_prefix("bat://localhost/")
    // } else {
    // request.uri().strip_prefix("bat://localhost/")
    // }
    let bat_file = if let Some(path) = request.uri().strip_prefix("bat://localhost/") {
        percent_encoding::percent_decode(path.as_bytes())
            .decode_utf8_lossy()
            .to_string()
    } else {
        return response.mimetype("video/mp4").status(404).body(vec![]);
    };
    let bat_file = PathBuf::from(bat_file);
    dbg!(&bat_file, request.uri()); //___

    // if path != "foo/test_video.mp4" {
    //     return response.mimetype("text/plain").status(404).body(Vec::new());
    // }

    let mut content = std::fs::File::open(&bat_file)?;
    let mut buf = Vec::new();

    let mut status_code = 200;

    // if the webview sent a range header, we need to send a 206 in return
    // Actually only macOS and Windows are supported. Linux will ALWAYS return empty headers.
    if let Some(range) = request.headers().get("range") {
        let file_size = content.metadata().unwrap().len();

        let range = HttpRange::parse(range.to_str().unwrap(), file_size).unwrap();
        // let support only 1 range for now
        let first_range = range.first();
        if let Some(range) = first_range {
            let mut real_length = range.length;

            // prevent max_length; specially on webview2
            if range.length > file_size / 3 {
                // max size sent (400ko / request)
                // as it's local file system we can afford to read more often
                real_length = (1024 * 400).min(file_size - range.start);
            }

            // last byte we are reading, the length of the range include the last byte
            // who should be skipped on the header
            let last_byte = range.start + real_length - 1;
            // partial content
            status_code = 206;

            // Only macOS and Windows are supported, if you set headers in linux they are ignored
            response = response
                .header("Connection", "Keep-Alive")
                .header("Accept-Ranges", "bytes")
                .header("Content-Length", real_length)
                .header(
                    "Content-Range",
                    format!("bytes {}-{}/{}", range.start, last_byte, file_size),
                );

            // FIXME: Add ETag support (caching on the webview)

            content.seek(SeekFrom::Start(range.start))?;
            content.take(real_length).read_to_end(&mut buf)?;
        } else {
            content.read_to_end(&mut buf)?;
        }
    }
    response.mimetype("video/mp4").status(status_code).body(buf)
}

#[tauri::command]
pub fn video_uri() -> (&'static str, std::path::PathBuf) {
    ("bat", "test_video.mp4".into())
    //("asset", "test_video.mp4".into())
    // if cfg!(feature = "protocol-asset") {
    // let mut path = std::env::current_dir().unwrap();
    // path.push("test_video.mp4");
    // ("asset", path)
    // } else {
    //     ("bat", "foo/test_video.mp4".into())
    // }
}

//let video_file = PathBuf::from("test_video.mp4");
// let video_url =
//     "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4";
// if !video_file.exists() {
//     // Downloading with curl this saves us from adding
//     // a Rust HTTP client dependency.
//     println!("Downloading {}", video_url);
//     let status = Command::new("curl")
//         .arg("-L")
//         .arg("-o")
//         .arg(&video_file)
//         .arg(video_url)
//         .stdout(Stdio::inherit())
//         .stderr(Stdio::inherit())
//         .output()
//         .unwrap();
//     assert!(status.status.success());
//     assert!(video_file.exists());
// }

fn pack(
    origin: &str,
    method: &str,
    url: &str,
    body: Option<&str>,
    status: i32,
    content: &str,
    ctype: &str, // clength: ...,
    headers: Vec<serde_json::Value>, //[Vec<Object>],
                 // window: Window,
                 // paging: tauri::State<'_, Database>,
) -> Vec<u8> {
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
