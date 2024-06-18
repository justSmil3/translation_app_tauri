use std::error::Error;
use ::phf::{Map, phf_map};
use std::vec::Vec;
use std::collections::VecDeque;
use std::fs::{self, File};
use std::io::{LineWriter, Write};
use reqwest::Client;
use reqwest::header::{HeaderMap,CONTENT_TYPE};
use std::path::Path;
use tokio;


static LANG: Map<&str, &str> = phf_map!{
    "Default Culture" => "EN",
    "ja-jp"=> "JA",
    "zh-hans"=> "ZH",
    "zh-hant"=> "ZH",
    "ko-KR"=> "KO",
    };

static SPLITTER:&str = "!!!S!!!";
static WC_SIGNIN:&str = "!!!W!!!";

//#[tokio::main]
//async fn main() -> Result<(), Box<dyn Error>>{
//    let _ = read_csv(false).await?;
//    Ok(())
//}


pub async fn read_csv(file_path:&str, save_path:&str, generate_new:bool, api_key: &str) -> Result<(), Box<dyn Error>>{
    // let mut reader = csv::ReaderBuilder::new()
    //     .from_path("./localization.csv")?;
    let contents: String= fs::read_to_string(file_path).expect("");
    let num_fields = LANG.len();
    let csv_char_vec = contents.chars().collect::<Vec<_>>();
    let mut ignore_chars:bool = false;
    let mut field_count: usize = 0;
    let mut prev_idx:usize = 0;
    let mut csv_vec:Vec<Vec<String>> = Vec::new();
    csv_vec.push(Vec::new());
    for (idx, char) in contents.clone().chars().enumerate(){
        if char == '\"'{
            ignore_chars = !ignore_chars;  
        }
        if ignore_chars{
            continue;
        }
        match char{
            ';' => {
                field_count+=1;
                let csv_vec_idx = csv_vec.len()-1;
                csv_vec[csv_vec_idx].push(csv_char_vec[prev_idx..idx].iter().cloned().collect::<String>());
                prev_idx = idx+1;
            },
            '\r' => {
                if(field_count < num_fields){
                    continue;
                }
                field_count = 0;
                let csv_vec_idx = csv_vec.len()-1;
                csv_vec[csv_vec_idx].push(csv_char_vec[prev_idx..idx].iter().cloned().collect::<String>());
                prev_idx = idx+2;
                csv_vec.push(Vec::new());
            },
            _ => {}
        }
    }
    // throw an error when csv has less than 2 rows
   
    // for x in reader.records(){
    //     println!("{x:?}");
    // }
    
    let cloned_csv_vec = csv_vec.clone();
    let lv:Vec<&str> = cloned_csv_vec[0].iter().map(|s| s.as_str()).collect::<Vec<_>>();
    let mut lines = vec!["".to_string(); csv_vec.len()];
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("Authorization", api_key.parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    let mut word_to_translate:Vec<String> = Vec::new(); 
    for (idx, &current_header) in lv.iter().enumerate(){
        let b_contains_key:bool = LANG.contains_key(current_header);
        let lang_key:&str = 
            if b_contains_key { 
                LANG.get(current_header).expect("") 
            }else{
                ""
            };
        let mut idx_stack:VecDeque<usize> = VecDeque::new();
        let mut word_stack:VecDeque<String> = VecDeque::new();
        let mut ignore_stack:VecDeque<String> = VecDeque::new();
        
        for (res_idx, result) in csv_vec.clone().iter().enumerate(){
            if result.is_empty(){
                continue;
            }
            println!("{result:?}");
            let word:&str = &result[idx];
            if res_idx == 0 || !b_contains_key{
                if !lines[res_idx].is_empty(){
                    lines[res_idx].push_str(";");
                }
                lines[res_idx].push_str(word);
                continue;
            }
            if lang_key == "EN"{
                word_to_translate.push(word.to_string());
                if !lines[res_idx].is_empty(){
                    lines[res_idx].push_str(";");
                }
                lines[res_idx].push_str(word);
                continue;
            }
            //if document des not need to be filled
            if !generate_new{
                if !word.is_empty(){
                    idx_stack.push_back(res_idx);
                    ignore_stack.push_back(word.to_string());
                    continue;
                }
            }
            if word_to_translate.len() >= res_idx {
                word_stack.push_back(word_to_translate[res_idx-1].clone().replace("\r\n", SPLITTER).replace("World Creator", WC_SIGNIN));
            } else {
                word_stack.push_back(String::new());
            }
            
        
            // for x in record.iter(){
            //     println!("{x:?}");
            //}
        }
        
        // in here formulate the requests
        let res:serde_json::Value = client.post("https://api-free.deepl.com/v2/translate")
            .headers(headers.clone())
            .body(format!("{{\"source_lang\": \"EN\", \"target_lang\": {lang_key:?}, \"text\": {word_stack:?}}}"))
            .send()
            .await?
            .json()
            .await?;

        if !&res["translations"].is_null(){
            word_stack.clear();
            let objs = res["translations"].as_array().expect("");
            for obj in objs{
                word_stack.push_back(obj["text"].as_str().expect("").to_string());
            }
        }
        //for obj in objs{
        //    println!("{:?}", obj["text"]);
        //}

        //replug things
        let mut i:usize = 1;
        while let Some(idx) = idx_stack.pop_front(){
            while i < idx{
                if let Some(word) = word_stack.pop_front(){
                    lines[i].push_str(";");
                    let translate_safe_word = &word.replace(SPLITTER, "\r\n")
                                                    .replace(WC_SIGNIN, "World Creator");
                    lines[i].push_str(translate_safe_word);
                }
                i+=1;
            }
            if let Some(word) = ignore_stack.pop_front(){
                lines[i].push_str(";");
                lines[i].push_str(&word);
            }
            i+=1;            
        }
        while let Some(word) = word_stack.pop_front(){
            lines[i].push_str(";");
            let translate_safe_word = &word.replace(SPLITTER, "\r\n")
                                            .replace(WC_SIGNIN, "World Creator");
            lines[i].push_str(translate_safe_word);
            i+=1;
        }
    }
    
    let mut path_to_save = Path::new(save_path);
    let prefix = path_to_save.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();
    let new_file = File::create(path_to_save)?;
    let mut new_file = LineWriter::new(new_file);

    for line in lines{
        new_file.write_all(line.as_bytes());
        new_file.write_all(b"\r\n");
    }
    new_file.flush()?;


    // for 
    // let res: serde_json::Value = client.post("https://api-free.deepl.com/v2/translate")
    //     .headers(headers)
    //     .body(format!("{{\"target_lang\": \"{}\", ""}}"))
    // println!("fuck me ");
    // println!("{:?}, {:?}", word_stack, idx_stack.len());
    
    Ok(())
}


// this is something for tomorrow in the morning but check wth the iteration just stops in the
// 522nd line of the csv ???? 
