//mod phtml;
//use std::error::Error;

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::fs;
use std::path::Path;
use tauri::{ Manager, State};
use std::ffi::OsStr;
use ::phf::{Map, phf_map};
use serde_json::json;

use std::thread;
use std::time::Duration;

use tauri_plugin_store::{Store, StoreBuilder};
use std::sync::Mutex;

mod phtml;
mod csv_translate;

static LANG: Map<&str, &str> = phf_map!{
    "Auto Detect" => "",
    "Japanese" => "JA",
    "Korean" => "KO", 
    "English" => "EN",
    "Chinese" => "ZH",
    "Arabic" => "AR",
    "Bulgarian" => "BG",
    "Czech" => "CS",
    "Danish" => "DA",
    "German" => "DE",
    "Greek" => "EL",
    "Spanish" => "ES",
    "Estonian" => "ET",
    "Finnish" => "FI",
    "French" => "FR",
    "Hungarian" => "HU",
    "Indonesian" => "ID",
    "Italian" => "IT",
    "Lithuanian" => "LT",
    "Latvian" => "LV",
    "Norwegian Bokmål" => "NB",
    "Dutch" => "NL",
    "Polish" => "PL",
    "Portuguese" => "PT-PT",
    "Portuguese (Brazilian)" => "PT-BR",
    "Romanian" => "RO",
    "Russian" => "RU",
    "Slovak" => "SK",
    "Slovenian" => "SL",
    "Swedish" => "SV",
    "Turkish" => "TR",
    "Ukrainian" => "UK"};

#[derive(Clone, serde::Serialize)]
struct Payload{
    message: Vec<String>
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//
#[tauri::command]
fn save_api_key(key: String, app_handle: tauri::AppHandle) -> Result<(), String>{
    let mut store = StoreBuilder::new(app_handle, "key.json".parse().unwrap()).build();
    let _ = store.insert("api_key".parse().unwrap(), json!(key));
    store.save().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn get_api_key(app_handle: tauri::AppHandle) -> Result<String, String> {
    let mut store = StoreBuilder::new(app_handle, "key.json".parse().unwrap()).build();
    let _ = store.load().expect("");
    if let Some(key) = store.get("api_key"){
        Ok(key.as_str().unwrap().to_string())
    }
    else {
        Ok("".to_string())
    }
   
}

// TODO in here add the service when needed
fn compose_api_key(app_handle: tauri::AppHandle) -> String{
    let key = get_api_key(app_handle).unwrap();
    let mut key_prefix = String::from("DeepL-Auth-Key ");
    key_prefix.push_str(&key);
    key_prefix
}

#[tauri::command]
fn translate_task(files:Vec<String>, file_names:Vec<&str>, output_path:&str, languages: Vec<String>, source_lang: &str, app_handle: tauri::AppHandle){

    let api_key = compose_api_key(app_handle.clone());

    let mut source_lang_key: &str = "";
    if LANG.contains_key(source_lang){
        source_lang_key = LANG.get(source_lang).expect("");
    }
    // println!("{:#?}, {:#?}, {:?}", languages, files, source_lang);
    for (idx, path) in files.iter().enumerate(){

        if !path.ends_with(".csv"){
            for (index, lang) in languages.clone().iter().enumerate(){
                if !LANG.contains_key(&lang) {
                    continue;
                }
                let lang_key:&str = LANG.get(&lang).expect("");

                translate_phtml(&path, output_path, file_names[idx], lang_key, source_lang_key, &api_key);
                if index == languages.len() - 1{
                    
                    app_handle.emit_all("file_handled", path).unwrap();
                }   
            }
        }
        else if path.ends_with(".csv"){
            let mut out_path:String = String::from(output_path);
            out_path.push_str(file_names[idx]);
            translate_csv(&path, &out_path, false, &api_key);
            app_handle.emit_all("file_handled", path).unwrap();
        }
    }
}

fn translate_csv(path: &str, out_path:&str, generate_new:bool, api_key:&str){
    tauri::async_runtime::block_on(async move {
        let awnswer = csv_translate::read_csv(path, out_path, generate_new, api_key).await;
        match awnswer{
            Ok(_) => {
                println!("successfully executed csv translation");
            }
            Err(err) =>{
                println!("{err}");
            }
        }
    });
}

fn translate_phtml(path: &str, out_path:&str, out_name:&str, lang_key: &str, source_lang_key: &str, api_key:&str){
    tauri::async_runtime::block_on(async move{
        let mut lang_loc = lang_key.to_lowercase();
        lang_loc.push('/');
        let mut target_loc = String::from(out_path);
        target_loc.push_str(&lang_loc);
        target_loc.push_str(out_name);
        let awnswer = phtml::translate_text(path, &target_loc, lang_key, source_lang_key, api_key).await;
        match awnswer{
            Ok(_) => {
                println!("successfully executed translation");
            }
            Err(err)=>{
                println!("{}", err);
            }
        }
    });
}

//const invoke = window.__TAURI__.invoke
#[tauri::command]
fn get_all_files(paths: Vec<&str>, app_handle: tauri::AppHandle){
    for path in paths{
        let _path = Path::new(path);
        let all_files = get_all_files_recursive(_path);
        if _path.is_file(){
            let all_file_names = vec![_path.file_name().unwrap().to_string_lossy().to_string()];
            app_handle.emit_all("files_listed", [all_files, all_file_names]).unwrap();
        }
        else {
            let all_file_names = all_files
                                    .iter()
                                    .map(|x| x.replace(path, ""))
                                    .collect::<Vec<_>>();
            app_handle.emit_all("files_listed", [all_files, all_file_names]).unwrap();
        }
    }
}

fn get_all_files_recursive(path: &Path) -> Vec<String>{
   if path.is_file(){
       if let Some(ext) = path.extension(){

       if ext == OsStr::new("csv") || ext == OsStr::new("phtml") || ext == OsStr::new("php"){
           return vec![path.to_string_lossy().into_owned()];
       }
       else {
           return Vec::new();
       }
       
      }
   }
   else {
        let mut result: Vec<String> = Vec::new();
        let paths_to_cicle_through = fs::read_dir(path).unwrap();
        for next_path in paths_to_cicle_through{
            let tmp_vec = get_all_files_recursive(&next_path.unwrap().path());
            let mut filtered_vec = tmp_vec.into_iter().filter(|x| !result.contains(x)).collect::<Vec<_>>();
            result.append(&mut filtered_vec); 
        }
        return result;
   }
   return Vec::new();
}
//#[tauri::command]
//async fn translate(path: &str, dir: bool, target_lang: u16)->Result<(), Box<dyn Error>>{
//    phtml::translate_text().await?;
//    println!("test test test");
//    Ok(())
//}

//fn translate_phtml(path: &str, target_lang: &str){
//
//}

fn main() {

    tauri::Builder::default()
        .setup(|app| {
            let mut store = StoreBuilder::new(app.handle(), "key.json".parse()?).build();
            let _ = store.load();
            let _ = store.save();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![translate_task, get_all_files, get_api_key, save_api_key])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
