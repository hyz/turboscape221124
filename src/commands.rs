use flatbuffers::Vector;
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use std::{
    collections::{HashMap, HashSet},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex, Once,
    },
};
use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};
use tauri::http::method::Method;
use tauri::http::MimeType;
use tauri::window;

use serde::{Deserialize, Serialize};
use serde_json::{json, to_writer, Serializer};
use serialize_to_javascript::{DefaultTemplate, RawValue};
use tauri::{Manager, State, Window};
use url::Url;

#[path = "prettyformatter.rs"]
mod prettyformatter;
use crate::protocols::query_generated::query;
use crate::protocols::query_generated::query::Pair;
use crate::protocols::query_generated::query::PairArgs;
use crate::protocols::query_generated::query::QueryArgs;
use crate::protocols::query_generated::query::ResponseArgs;
use crate::protocols::query_generated::{self, query::RequestArgs};
use crate::{template, Database};

// use rustpython_pylib as pylib;
// use rustpython_stdlib as stdlib;
// use rustpython_vm as vm;
// use std::process::ExitCode;

//const SCRIPT_PATH: &str = "src/scripts/perl.py";
const SRC_1: &str = r#"
import embed_import
count=0
ctx = embed_import.context()
count += 1
print(f"SOURCE#{count}", __name__, ctx)
"#;

#[derive(Debug, thiserror::Error)] //, Serialize
                                   //#[serde(tag = "kind")] //#[serde(tag = "t", content = "c")] #[serde(untagged)] //
pub enum Error {
    #[error("io(read/write) failed: {source}")]
    Io {
        #[from]
        source: std::io::Error,
    },
    #[error("serde(decode/encode) failed: {source}")]
    Json { source: serde_json::Error },
}

#[derive(Debug, Serialize)]
#[serde(tag = "result")] //#[serde(untagged)] //#[serde(tag = "t", content = "c")]
pub enum Return<T: Serialize, E: Serialize> {
    Ok(T),
    Err(E),
}

impl<T: Serialize, E: Serialize> From<Result<T, E>> for Return<T, E> {
    fn from(r: Result<T, E>) -> Self {
        match r {
            Ok(x) => Return::Ok(x),
            Err(x) => Return::Err(x),
        }
    }
}
impl<T: Serialize> From<Option<T>> for Return<T, serde_json::Value> {
    fn from(r: Option<T>) -> Self {
        match r {
            Some(x) => Return::Ok(x),
            None => Return::Err(json!("")),
        }
    }
}

enum Scripts {
    AjaxHook(template::AjaxHook),
}
// #[derive(Deserialize)]
// #[serde(untagged)] //#[serde(tag = "name")] //#[serde(tag = "t", content = "c")]
// pub enum Require {
//     Script { tag: String, location: String },
// }

type Value = serde_json::Value;
type Object = serde_json::value::Map<String, serde_json::Value>;

// static INIT: Once = Once::new();

static GLOBAL_DATA: Lazy<Mutex<HashMap<i32, String>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(13, "chain".to_string());
    m.insert(74, "queue".to_string());
    Mutex::new(m)
});

fn static_props_() -> &'static Mutex<HashMap<i32, String>> {
    static INSTANCE: OnceCell<Mutex<HashMap<i32, String>>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert(13, "sky".to_string());
        m.insert(74, "walker".to_string());
        Mutex::new(m)
    })
}
const QCC_SEARCHMULTI_PATH: i32 = 1;

// fn static_props(i: i32) -> Option<&'static String> {
//     let mx: &'static _ = static_props_();

//     mx.lock().unwrap(). //.get(&i) //.map(|x| x.as_str())
// }

////body > div > div.app-search > div.container.m-t > div.adsearch-list > nav > ul > li.active
///body > div > div.app-search > div.container.m-t > div.adsearch-list > nav > ul > li.active > a
// #[derive(Debug, Deserialize)]
// pub struct Props<'a> {
//     // url,status, content, contenttype, contentlength, headers
//     origin: &'a str,
//     method: &'a str,
//     url: &'a str,
//     status: i32,
//     content: &'a str,
//     ctype: &'a str,
//     headers: serde_json::Value,
// }
use std::cell::Cell;

// static a_str: Cell<String> = Cell::new(String::new());
static mut LAST_URLPATH_SEARCH: OnceCell<String> = OnceCell::new();

fn pack_request<'a>(
    builder: &'a mut flatbuffers::FlatBufferBuilder<'a>,
    method: &'a str,
    url: &'a str,
    body: Option<&'a str>,
    headers: &'a serde_json::Value,
) -> flatbuffers::WIPOffset<query::Request<'a>> {
    let method = builder.create_string(method);
    let url = Some(builder.create_string(url));
    let body = body.map(|s| builder.create_vector(s.as_bytes()));

    let items = headers[0].as_object().unwrap();
    let items: Vec<_> = items
        .iter()
        .filter(|(k, val)| val.is_string())
        .map(|(k, val)| {
            let key = Some(builder.create_string(k));
            let value = Some(builder.create_string(val.as_str().unwrap()));
            Pair::create(builder, &PairArgs { key, value })
        })
        .collect();
    let headers = builder.create_vector(&items);

    query::Request::create(
        builder,
        &RequestArgs {
            method: query::Method::Get,
            url,
            body,
            headers: Some(headers),
        },
    )
}
fn pack_response<'a>(
    builder: &'a mut flatbuffers::FlatBufferBuilder<'a>,
    status: i32,
    content: &'a str,
    ctype: &'a str,
    headers: &'a serde_json::Value,
) -> flatbuffers::WIPOffset<query::Response<'a>> {
    let content = Some(builder.create_vector(content.as_bytes()));
    let contenttype = Some(builder.create_string(ctype));

    let items = headers[1].as_object().unwrap();
    let items: Vec<_> = items
        .iter()
        .filter(|(k, val)| val.is_string())
        .map(|(k, val)| {
            let key = Some(builder.create_string(k));
            let value = Some(builder.create_string(val.as_str().unwrap()));
            Pair::create(builder, &PairArgs { key, value })
        })
        .collect();
    let headers = Some(builder.create_vector(&items));

    query::Response::create(
        builder,
        &ResponseArgs {
            status: status as i16,
            content,
            contenttype,
            headers,
        },
    )
}

#[tauri::command] //(rename_all = "snake_case")
pub fn sample(
    origin: &str,
    method: &str,
    url: &str,
    body: Option<&str>,
    status: i32,
    content: &str,
    ctype: &str, // clength: ...,
    mut headers: Vec<serde_json::Value>,
    // window: Window,paging: tauri::State<'_, Database>,
) -> (String, Vec<u8>) {
    assert!(headers.len() == 2);

    //invoke('consume', { url: config.url, status, response }).then(console.log);
    //await __TAURI__.tauri.invoke('consume',{response:'hello world',url:'/path/to',status:200})
    //JSON.parse(temp1.response, (k, v) => { if (typeof v === 'string') return v.replace('<em>', '').replace('</em>', ''); return v })
    if !true || url.len() < 1 || ctype.len() < 1 || content.len() < 10 || status != 200 {
        return (String::new(), vec![]); //json!({"err":-1,"hint":"invalid args"});
    }
    // let headers = headers.as_array().expect("headers:[]");

    let url_arg = url;
    let url = if url.starts_with("/") {
        let b = Url::parse(origin).ok();
        Url::options().base_url(b.as_ref()).parse(url).expect(url)
    } else {
        Url::parse(url).expect(url)
    };
    let url_path = Path::new(url.path());
    let hash_index = format!("{}{}", url.host_str().unwrap(), url.path());
    let mime_type = if ctype.contains("application/json") {
        "application/json".into()
    } else {
        MimeType::parse(content.as_bytes(), url_arg)
    };

    let clen = content.len();
    let content = content.trim_start();
    // let cont = &content[..256.min(content.len())]; //.get(..256).unwrap_or(content); //
    // let u_ = &url[..128.min(ulen)];
    dbg!(format!(
        "consume___{method}/{status} {mime_type} {hash_index} {clen}:..",
    ));

    // let now = chrono::offset::Local::now();
    // let today = now.format("_%y%m%d___");

    let mut file_path = PathBuf::from("sites");
    if !file_path.exists() {
        println!("`sites` not exists");
    }

    file_path.push(url.host_str().expect(url_arg));
    file_path.extend(url_path.components().skip(1));
    dbg!((url_path, &file_path)); // ___

    let site_dir = file_path
        .parent()
        .and_then(|d| {
            _ = fs::create_dir_all(d);
            Some(d.to_string_lossy().to_string())
        })
        .unwrap();

    // file_path.set_file_name(file_name)
    file_path.set_extension("tmp");
    if let Ok(mut file) = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&file_path)
    {
        let map_err = &|_| -> io::Error { io::ErrorKind::Other.into() };
        if let Err(err) = io::Result::Ok(())
            .and_then(|_| writeln!(file, "{origin}"))
            .and_then(|_| writeln!(file, ""))
            .and_then(|_| writeln!(file, "{method}"))
            .and_then(|_| writeln!(file, "{url_arg}"))
            .and_then(|_| to_writer(&mut file, &headers[0]).map_err(map_err))
            .and_then(|_| writeln!(file, ""))
            .and_then(|_| body.map(|body| writeln!(file, "{body}")).unwrap_or(Ok(())))
            .and_then(|_| writeln!(file, ""))
            .and_then(|_| writeln!(file, "{status}"))
            .and_then(|_| writeln!(file, "{ctype}"))
            .and_then(|_| to_writer(&mut file, &headers[1]).map_err(map_err))
            .and_then(|_| writeln!(file, ""))
            .and_then(|_| writeln!(file, "{}", content.len()))
            .and_then(|_| file.write_all(content.as_bytes()))
            .and_then(|_| writeln!(file, ""))
        {
            dbg!(format!("{err:?}"));
        }
    }
    dbg!(format!("saved___ {}", file_path.display()));

    let builder = &mut flatbuffers::FlatBufferBuilder::with_capacity(1024);
    let hd2 = headers.pop().unwrap();
    let hd1 = headers.pop().unwrap();
    drop(headers);

    let bytes = {
        let origin_key = builder.create_string("origin");
        let origin = builder.create_string(origin);
        let origin = Pair::create(
            builder,
            &PairArgs {
                key: Some(origin_key),
                value: Some(origin),
            },
        );
        let context = Some(builder.create_vector(&[origin]));
        let request = {
            // let method = builder.create_string(method);
            let url = Some(builder.create_string(url_arg));
            let body = body.map(|s| builder.create_vector(s.as_bytes()));

            let items = hd1.as_object().unwrap();
            let items: Vec<_> = items
                .iter()
                .filter(|(k, val)| val.is_string())
                .map(|(k, val)| {
                    let key = Some(builder.create_string(k));
                    let value = Some(builder.create_string(val.as_str().unwrap()));
                    Pair::create(builder, &PairArgs { key, value })
                })
                .collect();
            let headers = builder.create_vector(&items);

            fn as_method(one: &str) -> query::Method {
                match one {
                    "Method_" => query::Method::Method_,
                    "Get" => query::Method::Get,
                    "Head" => query::Method::Head,
                    "Post" => query::Method::Post,
                    "Put" => query::Method::Put,
                    "Delete" => query::Method::Delete,
                    _ => query::Method::Method_,
                }
            }
            query::Request::create(
                builder,
                &RequestArgs {
                    method: as_method(method), //query::Method::Get,
                    url,
                    body,
                    headers: Some(headers),
                },
            )
        };
        // Some(pack_request( builder,  method, url, body, &headers[0],));
        let response = {
            let content = Some(builder.create_vector(content.as_bytes()));
            let contenttype = Some(builder.create_string(ctype));

            let items = hd2.as_object().unwrap();
            let items: Vec<_> = items
                .iter()
                .filter(|(k, val)| val.is_string())
                .map(|(k, val)| {
                    let key = Some(builder.create_string(k));
                    let value = Some(builder.create_string(val.as_str().unwrap()));
                    Pair::create(builder, &PairArgs { key, value })
                })
                .collect();
            let headers = Some(builder.create_vector(&items));

            query::Response::create(
                builder,
                &ResponseArgs {
                    status: status as i16,
                    content,
                    contenttype,
                    headers,
                },
            )
        };
        let query = query::Query::create(
            builder,
            &QueryArgs {
                context,
                request: Some(request),
                response: Some(response),
            },
        );

        builder.finish(query, None);
        builder.finished_data()
    };

    let query = flatbuffers::root::<query::Query>(bytes).unwrap();
    dbg!(query);

    let host = url.host_str().expect(url_arg).to_string();
    let uuid = uuid::Uuid::new_v4().to_string();
    let index = "packages/flatcollect/dist/index.js".into();
    let index_bg = "packages/flatcollect/dist/index_bg.wasm".into();
    let js = template::Forward {
        host,
        uuid,
        site_dir,
        index,
        index_bg,
    }
    .render_default(&Default::default())
    .unwrap()
    .into_string();
    // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    // window.emit("jedi", Some(js)).expect("emit-jedi");

    return (js, bytes.into());
    // return (String::new(), vec![]); //json!({"code":0,"hint":"invalid args"});
}

#[tauri::command] //(rename_all = "snake_case")
pub fn _consume_0(
    origin: &str,
    method: &str,
    url: &str,
    body: Option<&str>,
    status: i32,
    content: &str,
    ctype: &str,                     // clength: ...,
    headers: Vec<serde_json::Value>, //[Vec<Object>],
    window: Window,
    paging: tauri::State<'_, Database>,
) -> (String, Vec<u8>) {
    //invoke('consume', { url: config.url, status, response }).then(console.log);
    //await __TAURI__.tauri.invoke('consume',{response:'hello world',url:'/path/to',status:200})
    //JSON.parse(temp1.response, (k, v) => { if (typeof v === 'string') return v.replace('<em>', '').replace('</em>', ''); return v })
    assert!(headers.len() == 2);
    if !true || url.len() < 1 || ctype.len() < 1 || content.len() < 10 || status != 200 {
        return (String::new(), vec![]); //json!({"err":-1,"hint":"invalid args"});
    }
    // let headers = headers.as_array().expect("headers:[]");

    let url_arg = url;
    let url = if url.starts_with("/") {
        let b = Url::parse(origin).ok();
        Url::options().base_url(b.as_ref()).parse(url).expect(url)
    } else {
        Url::parse(url).expect(url)
    };
    let url_path = Path::new(url.path());
    let hash_index = format!("{}{}", url.host_str().unwrap(), url.path());
    let mime_type = if ctype.contains("application/json") {
        "application/json".into()
    } else {
        MimeType::parse(content.as_bytes(), url_arg)
    };

    let clen = content.len();
    // let ulen = url_arg.len();
    let content = content.trim_start();
    //let cont = &content[..256.min(content.len())]; //.get(..256).unwrap_or(content); //
    // let u_ = &url[..128.min(ulen)];
    dbg!(format!(
        "consume___{method}/{status} {mime_type} {hash_index} {clen}:..",
    ));

    // let now = chrono::offset::Local::now();
    // let today = now.format("_%y%m%d___");

    let mut file_path = PathBuf::from("sites");
    if !file_path.exists() {
        println!("`sites` not exists");
    }

    file_path.push(url.host_str().expect(url_arg));
    file_path.extend(url_path.components().skip(1));
    dbg!((url_path, &file_path)); // ___

    // "searchKey"
    let post_body = body.and_then(|body| {
        if mime_type.contains("/json") {
            serde_json::from_str::<Value>(body).ok()
        } else {
            None
        }
    });
    let keyword = post_body
        .as_ref()
        .and_then(|v| v.get("searchKey").and_then(|x| x.as_str()));
    keyword.map(|k| file_path.push(k));
    dbg!(("___", keyword, &file_path));

    fs::create_dir_all(file_path.parent().unwrap()).expect(url_arg);

    // match tokio::fs::read(path).await;
    //             response.mimetype(&mime_type).body(data)
    if mime_type.contains("application/json") {
        let json_content: serde_json::Value = serde_json::from_str(&content).unwrap();
        if let Some(Value::Object(obj)) = json_content.get("Paging") {
            let index = obj.get("PageIndex").and_then(|x| x.as_i64()).unwrap_or(0);
            let size = obj.get("PageSize").and_then(|x| x.as_i64()).unwrap_or(0);
            let total = obj
                .get("TotalRecords")
                .and_then(|x| x.as_i64())
                .unwrap_or(0);

            let mut mutex = paging.0.lock().unwrap();
            let hs = mutex
                .entry(hash_index.clone())
                .or_insert_with(|| numbs_init(file_path.parent().unwrap()));
            hs.insert(index as i32);
            // numbs_save( index as i32, &hash_index, file_path.parent(),  &paging, );

            if origin.contains("qcc.com") && url_arg.contains("searchMulti") {
                file_path.set_extension(format!("{index}-{size}-{total}.json"));
                //.append(true) ////.create(true) //.create_new(true)
                if let Ok(mut file) = fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&file_path)
                {
                    let fmt = prettyformatter::PrettyFormatter::new();
                    let mut ser = Serializer::with_formatter(&mut file, fmt);
                    //serde_json::to_writer_pretty(&mut file, &json_value)
                    if let Err(___) = json_content.serialize(&mut ser) {
                        dbg!((___, url_arg));
                    }
                }

                let window = window.clone();
                let keyword = keyword.unwrap_or("").to_string();
                let hash_index = hash_index.clone();
                tauri::async_runtime::spawn(async move {
                    let js = template::SearchQcc {
                        uuid: uuid::Uuid::new_v4().to_string(),
                        keyword,
                        hash_index,
                        numb: index as i32,
                    }
                    .render_default(&Default::default())
                    .unwrap()
                    .into_string();

                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    window.emit("jedi", Some(js)).expect("emit-jedi");
                    dbg!("___ emit-jedi qcc");
                });
            }
        }
    }

    // file_path.set_file_name(file_name)
    file_path.set_extension("tmp");
    if let Ok(mut file) = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&file_path)
    {
        let map_err = &|_| -> io::Error { io::ErrorKind::Other.into() };
        if let Err(err) = io::Result::Ok(())
            .and_then(|_| writeln!(file, "{origin}"))
            .and_then(|_| writeln!(file, ""))
            .and_then(|_| writeln!(file, "{method}"))
            .and_then(|_| writeln!(file, "{url_arg}"))
            .and_then(|_| to_writer(&mut file, &headers[0]).map_err(map_err))
            .and_then(|_| writeln!(file, ""))
            .and_then(|_| body.map(|body| write!(file, "{body}")).unwrap_or(Ok(())))
            .and_then(|_| writeln!(file, ""))
            .and_then(|_| writeln!(file, "{status}"))
            .and_then(|_| writeln!(file, "{ctype}"))
            .and_then(|_| to_writer(&mut file, &headers[1]).map_err(map_err))
            .and_then(|_| writeln!(file, ""))
            .and_then(|_| writeln!(file, "{}", content.len()))
            .and_then(|_| file.write_all(content.as_bytes()))
            .and_then(|_| writeln!(file, ""))
        {
            dbg!(format!("{err:?}"));
        }
    }

    dbg!(format!("saved___ {}", file_path.display()));

    if origin.contains("baidu.com/s") {
        //https://www.baidu.com/s?ie=utf-8&f=3&...
        //"/s?ie=utf-8&mod=1&isbd=1&isid=d66d0937000cc171&wd=starcraft&pn=750&oq=starcraft&ie=utf-8&usm=5&fenlei=256&rsv_idx=1&rsv_pq=d66d0937000cc171&rsv_t=b69fIiJ8FZEFlPA0Oc0OxH%2BFfyr7TzVvzjLM5caVFsIjxah7gJswBV20IkA&bs=starcraft&rsv_sid=36546_37689_37771_37303_37722_37799_36805_37662_37538_37716_37743_26350_37791&_ss=1&clist=cf46c4f8bcbae28b%09cf46c405bbc1e601%09cf46c312bac8e977%09cf46c21fb9cfeced%09cf46c12cb8d6f063%09cf46c039b7ddf3d9%09cef5c8c4bfb3dbc6%09cef5c7d1bebadf3c&hsug=&f4s=1&csor=9&_cr1=37672"
        //"/s?ie=utf-8&csq=1&pstg=20&mod=2&isbd=1&cqid=cda8d2040008637e&istc=969&ver=RgseJ2UFOu4aje7jp_XXm39Z1bh9WCGVEYm&chk=63771663&isid=d66d0937000cc171&wd=starcraft&pn=750&oq=starcraft&ie=utf-8&usm=5&fenlei=256&rsv_idx=1&rsv_pq=d66d0937000cc171&rsv_t=b69fIiJ8FZEFlPA0Oc0OxH%2BFfyr7TzVvzjLM5caVFsIjxah7gJswBV20IkA&bs=starcraft&f4s=1&_ck=1718.0.-1.-1.-1.-1.-1&rsv_isid=36546_37689_37771_37303_37722_37799_36805_37662_37538_37716_37743_26350_37791&isnop=0&rsv_stat=1_4_4_6_15.4.19.4.12.4.4.4.9.4.4.4"
        //https://ug.baidu.com/mcp/pc/pcsearch
        let re = regex::Regex::new(r"baidu.com/.*search$").unwrap();
        if re.is_match(url_arg) {
            let window = window.clone();
            tauri::async_runtime::spawn(async move {
                let js = template::SearchBaidu {
                    hash_index,
                    uuid: uuid::Uuid::new_v4().to_string(),
                }
                .render_default(&Default::default())
                .unwrap()
                .into_string();

                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                window.emit("jedi", Some(js)).expect("emit-jedi");
                dbg!("___ emit-jedi baidu");
            });
        }
    }

    return (String::new(), vec![]); //json!({"code":0,"hint":"invalid args"});
}

#[tauri::command]
pub fn numb_exists(numb: i32, hashindex: &str, paging: tauri::State<'_, Database>) -> bool {
    let mutex = paging.0.lock().unwrap();
    let found = mutex.get(hashindex).and_then(|h| Some(h.contains(&numb)));
    dbg!(found.unwrap_or(false)) //("exists___", )
}

// fn numbs_save(index: i32, hash_key: &String, dir: &Path, paging: &tauri::State<'_, Database>) {
//     let mut mutex = paging.0.lock().unwrap();
//     let nums = mutex
//         .entry(hash_key.to_string())
//         .or_insert_with(|| nums_init(dir));
//     // INIT.call_once(|| {});// INIT.call_once_force
//     nums.insert(index);
// }
fn numbs_init(dir: &Path) -> HashSet<i32> {
    let mut index_set = HashSet::<i32>::default();
    if let Ok(entities) = fs::read_dir(dir) {
        for entry in entities {
            if let Ok(entry) = entry {
                if let Ok(ft) = entry.file_type() {
                    if ft.is_file() {
                        //let fp = entry.path();
                        let name = entry.file_name();
                        let name = name.to_string_lossy();
                        let re = regex::Regex::new(r"\.(\d+)-\d{2}-\d+\.json$").unwrap();
                        if let Some(caps) = re.captures(&name) {
                            let ix = &caps[1];
                            let index: i32 = ix.parse().unwrap();
                            index_set.insert(index);
                        }
                    }
                }
            }
        }
    }
    dbg!(("paging_init___", dir, &index_set));
    index_set
}

pub fn on_page_load(_: &str, window: &tauri::Window) {
    let js = template::AjaxHook
        .render_default(&Default::default())
        .unwrap()
        .into_string();
    if let Err(err) = window.eval(&js) {
        dbg!(format!("template:Hook eval: {err}"));
    }

    let js = template::PageOnLoad
        .render_default(&Default::default())
        .unwrap()
        .into_string();
    if let Err(err) = window.eval(&js) {
        dbg!(format!("on_page_load template eval: {err}"));
    }

    // // await __TAURI__.invoke('trigger', {request:{url:'asset://localhost/hook'},response:{},body:''})
    // if let Some(Value::String(url)) = request.get("url") {
    //     match url.as_str() {
    //         "asset://localhost/hook" => {
    //             // let s = template::AjaxHook
    //             //     .render_default(&Default::default())
    //             //     .unwrap()
    //             //     .into_string();
    //             // if let Err(err) = window.eval(&s) {
    //             //     eprint!("template::AjaxHook eval: {err}")
    //             // }
    //             return vec![format!("console.log('{url}')")];
    //         }
    //         _ => {
    //             let s = template::AjaxHook {}
    //                 .render_default(&Default::default())
    //                 .unwrap()
    //                 .into_string();
    //             if let Err(err) = window.eval(&s) {
    //                 eprint!("template::AjaxHook eval: {err}")
    //             }
    //             return vec![format!("console.log('trigger {url}')")];
    //         }
    //     }
    // } else {
    //     return vec!["console.log('trigger fail: {request:{url:...}}')".into()];
    // }
    //return Return::Err(json!({}));

    // match tokio::fs::read(path).await
    // {
    //     Ok(data) => {
    //         let mime_type = MimeType::parse(&data, uri);
    //         response.mimetype(&mime_type).body(data)
    //     }
    //     Err(e) => {
    //         debug_eprintln!("Failed to read file: {}", e);
    //         response.status(404).body(Vec::new())
    //     }
    // }
}

#[tauri::command]
pub fn greet(name: &str) -> (String, Vec<u8>) {
    //await __TAURI__.tauri.invoke('greet', {name:"joe"})
    _ = dbg!(format!("greet: {name}"));
    // serde_json::json!({"greet":"Greet! You've been greeted from Rust!","name":name})
    return (r#"console.log"#.into(), "nice, greet".into());
}

#[tauri::command]
pub fn monster(/* !!!Error... bytes: &[u8] */) -> (String, Vec<u8>) {
    // let bytes = bytes.as_bytes();
    use crate::monster_generated::my_game::sample::*;

    fn check(monster: Monster) {
        // Get access to the root:
        dbg!(monster);

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
        dbg!(monster, "<<< rust checked >>>");
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
    dbg!("--------==== check monster from rust ====----------");
    check(mon1);
    // dbg!("--------==== check monster from unknown ====----------");
    // let mon2 = flatbuffers::root::<Monster>(bytes).unwrap();
    // check(mon2);
    dbg!("");
    // !! assert_eq!(mon1, mon2);

    return (String::from(""), buf_1.into());
}

#[derive(Debug, Serialize)]
pub struct Help {
    usage: String,
}

#[tauri::command]
pub fn help() -> Return<Help, serde_json::Value> {
    // await __TAURI__.tauri.invoke('help')
    Return::Ok(Help {
        usage: "bob".into(),
    })
}
#[tauri::command]
pub fn ask(question: Option<&str>) -> Return<Help, serde_json::Value> {
    // await __TAURI__.tauri.invoke('ask',{question:'hello'})
    match question {
        Some(q) => Return::Ok(Help { usage: q.into() }),
        None => Return::Err(json!({"hint":"question required"})),
    }
}

#[tauri::command]
async fn menu_toggle(window: tauri::Window) {
    window.menu_handle().toggle().unwrap();
}

#[derive(Debug, Serialize)]
#[serde(untagged)] //#[serde(tag = "result")] //#[serde(tag = "t", content = "c")]
pub enum Script<T: Serialize> {
    Some(T),
    None,
}

//impl From<MyResult<T, E>> for Return<T, E> {}

#[derive(Debug, Serialize)]
enum Json {}

// #[tauri::command] //(rename_all = "snake_case")
// pub fn mimetype(name: &str) -> String {
//     match tokio::fs::read(path).await {
//         Ok(data) => {
//             let mime_type = MimeType::parse(&data, uri);
//             response.mimetype(&mime_type).body(data)
//         }
//         Err(e) => {
//             debug_eprintln!("Failed to read file: {}", e);
//             response.status(404).body(Vec::new())
//         }
//     }
// }
//
// fn py_main(interp: &vm::Interpreter) -> vm::PyResult<vm::PyObjectRef> {
//     interp.enter(|vm| {
//         vm.insert_sys_path(vm.new_pyobj("src/scripts"))
//             .expect("add path");
//         let _result = {
//             {
//                 let scope = vm.new_scope_with_builtins();
//                 let value = vm.ctx.new_str(format!("{}", "世界")).into();
//                 scope.globals.set_item("_hi___", value, vm).unwrap();
//                 vm.run_script(scope, SCRIPT_PATH)?;
//                 // let src = "print('foo-bar')";
//                 // vm.run_code_string(vm.new_scope_with_builtins(), src, "<...>".to_owned())?;
//             }
//             let pycode = vm
//                 .compile(SRC_1, vm::compiler::Mode::Exec, "<embedded>".to_owned())
//                 .map_err(|err| vm.new_syntax_error(&err))?;
//             vm.run_code_obj(pycode, vm.new_scope_with_builtins())?
//         };
//         let result = {
//             let module = vm.import("embed_import", None, 0)?;
//             let name_func = module.get_attr("context", vm)?;
//             let result = vm.invoke(&name_func, ())?;
//             result.get_attr("name", vm)?
//             //; let result:: vm::builtins::PyStrRef = result.try_into_value(vm)?;
//         };
//         return vm::PyResult::Ok(result);
//     })
// }
