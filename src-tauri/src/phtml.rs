use scraper::{Html, Selector};
use std::collections::VecDeque;
use std::fs::File;
use std::collections::HashMap;
use std::error::Error;
use reqwest::Client;
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use tokio;
use std::fs;
use std::path::{ Path, PathBuf };
use std::cmp;
use std::io::Write;
use serde_json::json;
use std::time::Instant;
use regex::{Regex, Captures, RegexBuilder};

static KEYWORDS:[&'static str; 2] = [
    "world creator",
    "worldcreator"
];
 
static PHPSTART:&str = "<?php";
static PHPEND:&str = "?>";

static SIGNIN:&str = "<[span translate='no']>";

const PHP_SIG:&str = "!!!!PHP!!!!";
// TODO if this is to become a full fledge tool, the user needs to be able to define this
const IGNORE_IN_TRANSLATION:[&str; 28] = [
    "&nbsp",
    "BiteTheBytes GmbH", 
    "world creator", 
    "worldcreator", 
    "Arnold",
    "Blender",
    "Cinema 4D",
    "Clarisse",
    "Cry Engine",
    "DAZ Studio",
    "Flax Engine",
    "Godot Engine",
    "Houdini",
    "Lightwave",
    "Lumberyard Engine",
    "3DS Max",
    "Maya",
    "MudBox",
    "Octane",
    "Photoshop",
    "Quixel Mixer",
    "Substance 3D Designer",
    "Unity Engine",
    "Unigine Engine",
    "Unreal Engine",
    "Unreal",
    "Discord",
    "Windows"
];

pub async fn translate_text(file_path: &str, save_path: &str, target_lang_key: &str, source_lang_key: &str, api_key: &str) -> Result<(), Box<dyn Error>>{
    println!("started");

    let mut max_length:usize = 0;
    for x in &KEYWORDS {
        if x.len() > max_length {
            max_length = x.len();
        }
    }

    let contents: String = fs::read_to_string(file_path).expect("should be able to be parsed");
    let mut translated_text:String = contents.clone();
    let texts_to_translate_php:Vec<String> = extract_php_text(&contents);
    let (mut word_buffer ,replaced_text) = replace_between_tags(&contents, max_length);
    let selector = Selector::parse("div").unwrap();
    let document: Html = Html::parse_document(&replaced_text);
    let mut items_map:HashMap<&str, usize> = HashMap::new();
    println!("parsed");
    //let mut texts_to_translate:Vec<String> = php_texts.iter().map(|s| s.as_str()).collect::<Vec<_>>();

    let mut texts_to_translate:Vec<String> = Vec::new();
    for node in document.select(&selector){
        let texts:Vec<&str> = node.text().collect::<Vec<_>>();
        for text in texts{
            let lines = text.lines();
            for line in lines{
                let t: &str = line.trim();
                if t.is_empty(){
                    continue;
                }
                let count = items_map.entry(t).or_insert(0);
                *count += 1;
                if *count >= 2{
                    continue;
                }
                if t.contains("&nbsp"){
                    continue;
                }
                texts_to_translate.push(t.to_string());   
            }
        }
    }
    // texts_to_translate_php.sort_by(|a, b| b.len().cmp(&a.len()));
    let mut texts_to_translate_php_replaced:Vec<String> = Vec::new();
    for text in &texts_to_translate_php{
        let replaced_text = set_ignore_translate(text, false);
        texts_to_translate_php_replaced.push(replaced_text);
    }

    // texts_to_translate.sort_by(|a, b| b.len().cmp(&a.len()));
    let mut texts_to_translate_replaced:Vec<String> = Vec::new();
    for text in &texts_to_translate{
        let replaced_text = set_ignore_translate(text, false);
        texts_to_translate_replaced.push(replaced_text);
    }
    
    let mut translated_texts_php:Vec<String> = translate_vec_req(texts_to_translate_php_replaced, target_lang_key, source_lang_key, api_key).await.unwrap();
    let mut translated_texts:Vec<String> = translate_vec_req(texts_to_translate_replaced.clone(), target_lang_key, source_lang_key, api_key).await.unwrap();
    //let re_delimitors = Regex::new(r"(?s)>(.*?)<").unwrap();
    translated_texts.append(&mut translated_texts_php);
    translated_texts.sort_by(|a, b| b.len().cmp(&a.len()));

    for (idx ,t) in translated_texts.iter().enumerate(){
        let post_translation = set_ignore_translate(t, true);
        let pre_translation = texts_to_translate.get(idx).unwrap();
        let re_escape_pre_translation = regex::escape(&pre_translation);
        let re_final_pattern = format!(r"(?s)(\b|>|\r\n|\W)({})(\b|<|\r\n|\W)", re_escape_pre_translation);
        //let re_final_pattern = format!(r"(?s)>{}<", re_escape_pre_translation);
        let regex_final = Regex::new(&re_final_pattern).unwrap();

        //test 
        let re_tags = Regex::new(r"(?s)>(.*?)<([^>]*?)").unwrap();
        for caps in re_tags.captures_iter(&translated_text.clone()){
            let text = caps.get(0).unwrap().as_str();
            if !text.contains(pre_translation) || text.contains(r"/script") || text.contains('{') || text.contains('}'){
                continue;
            }
            let rep_text = regex_final.replace_all(text, |capst:&Captures|{
                let full_match = capst.get(0).unwrap().as_str();
                let full_match_str = full_match.replace(pre_translation, &post_translation);
                full_match_str
                //println!("{full_match}");
                //if full_match.starts_with('>') && full_match.ends_with('<'){
                //    format!(">{}<", &post_translation)
                //}
                //else if full_match.starts_with('>'){
                //    if full_match.ends_with("\r\n"){
                //        format!(">{}\r\n", &post_translation)
                //    }
                //    else {
                //        format!(">{}", &post_translation)
                //    }
                //}
                //else if full_match.ends_with('<'){
                //    if full_match.starts_with("\r\n"){
                //        format!("\r\n{}<", &post_translation)
                //    }
                //    else {
                //        format!("{}<", &post_translation)
                //    } 
                //}
                //else if full_match.starts_with("\r\n") && full_match.ends_with("\r\n"){
                //    format!("\r\n{}\r\n", &post_translation)
                //}
                //else if full_match.starts_with("\r\n"){
                //    format!("\r\n{}", &post_translation)
                //}
                //else if full_match.ends_with("\r\n"){
                //    format!("{}\r\n", &post_translation)
                //}
                //else{
                //    post_translation.to_string()
                //}
            }).to_string();
            translated_text = translated_text.replace(text, &rep_text);
        }

        // translated_text = regex_final.replace_all(&translated_text, |caps:&Captures|{
        //     let full_match = caps.get(0).unwrap().as_str();
        //     if full_match.starts_with('>') && full_match.ends_with('<'){
        //         format!(">{}<", &post_translation)
        //     }
        //     else if full_match.starts_with('>'){
        //         if full_match.ends_with("\r\n"){
        //             format!(">{}\r\n", &post_translation)
        //         }
        //         else {
        //             format!(">{}", &post_translation)
        //         }
        //     }
        //     else if full_match.ends_with('<'){
        //         if full_match.starts_with("\r\n"){
        //             format!("\r\n{}<", &post_translation)
        //         }
        //         else {
        //             format!("{}<", &post_translation)
        //         } 
        //     }
        //     else if full_match.starts_with("\r\n") && full_match.ends_with("\r\n"){
        //         format!("\r\n{}\r\n", &post_translation)
        //     }
        //     else if full_match.starts_with("\r\n"){
        //         format!("\r\n{}", &post_translation)
        //     }
        //     else if full_match.ends_with("\r\n"){
        //         format!("{}\r\n", &post_translation)
        //     }
        //     else{
        //         post_translation.to_string()
        //     }
        // }).to_string();

        // TODO this was not the solution. attack here tomorrow
        // let _ = re_delimitors.replace_all(&translated_text.clone(), |caps: &Captures| {
        //     let matched = &caps[1];
        //     let replaced = matched.replace(pre_translation, &post_translation);
        //     let rep_res = format!(">{}<", replaced);
        //     translated_text = translated_text.replace(&caps[0], &rep_res);
        //     ""
        // });

        // translated_text = translated_text.replace(pre_translation, &post_translation);
    }
    // for t in texts_to_translate{
    //     //translate t; 
    //     //push t in Vec
    //     //let text_to_translate = set_ignore_translate(t, false);
    //     let res: String = translate_req(&t, target_lang_key, source_lang_key, api_key).await.unwrap();
    //     let complete_res: String = set_ignore_translate(&res, true);
    //     // let ts: String = t.to_string();
    //     // let mut res: String = t.to_string();
    //     
    //     translated_text = translated_text.replace(t.trim(), &complete_res);
    //     // let res:String = t.replace(SIGNIN, word_buffer.pop().expect("").as_str());
    //     //println!("{}", res);
    //     //pair replaced words with t buffer
    //     //replace text snippets
    //     //
    //     //
    //     //
    //     // let (word_buffer, replaced_text):(Vec<String>, String) = replace_between_tags(t, max_length);
    //     // println!("{:?}", word_buffer);
    // }
    println!("translated");

    // replace the html language code 
    let mut target_lang_code = target_lang_key.to_lowercase();
    let mut source_lang_code = source_lang_key.to_lowercase();
    if target_lang_code.contains('-'){
        target_lang_code = target_lang_code.split('-').collect::<Vec<&str>>()[0].to_string();
    }
    if source_lang_code.contains('_'){
        source_lang_code = source_lang_code.split('-').collect::<Vec<&str>>()[0].to_string();
    }

    let lang_code_string = format!(r#"lang="{source_lang_code}""#);
    let lang_code_replace = format!(r#"lang="{target_lang_code}""#);

    
    translated_text = translated_text.replace(&lang_code_string, &lang_code_replace);
        
    // let testtext = "I<< fucking <?php hate is no solution ?> every<?phpthing?> world creator world creaworldcreator";
    // replace_between_tags(testtext, max_length);
    let path_to_save = Path::new(save_path);
    let prefix = path_to_save.parent().unwrap();

    std::fs::create_dir_all(prefix).unwrap();
    //let path_to_read = Path::new(file_path);
    //let name_to_save = path_to_read.file_name().unwrap();
    //let mut path_buf = PathBuf::from(path_to_save);
    //path_buf.push(name_to_save);
    //path_to_save = path_buf.as_path();

    let mut new_file = File::create(path_to_save).unwrap();
    new_file.write_all(translated_text.as_bytes()).unwrap();
    let _ = new_file.flush();
    drop(new_file);
    Ok(())
}


// ----------------------------------------------------------------------------------
// TODO for now something like <p> ... <?php echo'sim'; ?> ... </p> is not supported
fn extract_php_text(php: &str) -> Vec<String>{
    let mut extracted_texts: Vec<String> = Vec::new();

    let re = Regex::new(r"echo\s+[^;]+").unwrap();
   // for caps in re.captures_iter(php){
   //     extracted_texts.push(value)   
   // }
    let extracted_echo:Vec<&str> = re.captures_iter(php).map(|x| x.get(0).unwrap().as_str()).collect();
    
    let re_echo = Regex::new(r"(?s)[^-^\[][>'](.*?)[<']").unwrap();
    'outer: for echo in extracted_echo{
        let mut echo_chars = echo.chars();
        let mut skip: bool = false;
        let mut symb:char = ' ';
        while symb != '\'' && symb != '"'{
            if let Some(tmp_char) = echo_chars.next(){
                symb = tmp_char;
            }
            else{
                println!("echo: {echo} => No text found, maybe it is hidden in a variable! this tool does not currently support the translation of variables");
                continue 'outer;
            }
        }
        // let re_escape_pattern = format!(r"{}.*?[^\\]{}", symb, symb);
        let re_escape_pattern = format!("{}", symb);
        let re_escape = Regex::new(&re_escape_pattern).unwrap();
        let replaced_echo = echo.replace(r"\'", "!!!THING!!!");
        for caps in re_echo.captures_iter(&replaced_echo){
            let extracted_text:&str = caps.get(1).unwrap().as_str().trim();
            if extracted_text.contains(">") || extracted_text.contains('<') || extracted_text.contains('$'){
                continue;
            }
            if extracted_text.contains("&nbsp"){
                continue;
            }
            let mut  text_to_translate = extracted_text.to_string();// .chars().collect::<Vec<_>>()[1..].iter().collect::<String>();
            // text_to_translate = text_to_translate.trim().to_string();
            if text_to_translate.is_empty() || text_to_translate == "'"{
                continue;
            }
            //text_to_translate = text_to_translate.replace(r"\'", "!!!THING!!!");
            let resulting_strings = re_escape.split(&text_to_translate).collect::<Vec<_>>();
            for (result_idx, result) in resulting_strings.iter().enumerate(){
                if result_idx % 2 != 0 || result.is_empty(){
                    continue;
                }
                let full_result = result.replace("!!!THING!!!", r"\'");
                extracted_texts.push(full_result);
            }
            //let mut result = re_escape.replace_all(&text_to_translate, "").to_string();
            // result = result.replace(r"\", "");
            // extracted_texts.push(result);
        }

    }
    extracted_texts
}

//TODO if this becomes a more general tool, somehow the ' after translation for php parts 
//needs to be replaced 
fn replace_with_sign(input_text: &str, elements: &mut Vec<String>, sig: &str, re: Regex) -> String{
    let mut text = input_text.to_string();
    if elements.is_empty(){
        text = re.replace_all(&text, |caps: &Captures| {
            elements.push(caps[0].to_string());
            sig.to_string()
        }).to_string();
    }
    else {
        let re_res = Regex::new(sig).unwrap();
        text = re_res.replace_all(&text, |_caps: &Captures| {
            elements.remove(0)
        }).to_string();
    }
    text
}

fn replace_php(html: &str, php_elements: &mut Vec<String>) -> String {
    
    let mut html = html.to_string();

    if php_elements.is_empty() {
        let re = Regex::new(r"(?s)<\?php.*?\?>").unwrap();
        html = re.replace_all(&html, |caps: &Captures| {
            php_elements.push(caps[0].to_string());
            PHP_SIG.to_string()
        }).to_string();
        
        // Uncomment and adjust the below section as per your IGNORE_IN_TRANSLATION tags
        // for (i, ignore_case) in IGNORE_IN_TRANSLATION.iter().enumerate() {
        //     let pattern = Regex::new(ignore_case).unwrap();
        //     html = pattern.replace_all(&html, &format!("!!!!{}!!!!", i)).to_string();
        // }
    } else {
        let re = Regex::new(PHP_SIG).unwrap();
        html = re.replace_all(&html, |_caps: &Captures| {
            php_elements.remove(0)
        }).to_string();

        // Uncomment and adjust the below section as per your IGNORE_IN_TRANSLATION tags
        // for (i, ignore_case) in IGNORE_IN_TRANSLATION.iter().enumerate() {
        //     let pattern = Regex::new(&format!("!!!!{}!!!!", i)).unwrap();
        //     html = pattern.replace_all(&html, *ignore_case).to_string();
        // }
    }

    html
}

// TODO this function is not even needed anymore if the one above is implemented more intelligently
fn set_ignore_translate(html: &str, reaply: bool) -> String {
    let mut html = html.to_string();

    let mut _tags: Vec<&str> = IGNORE_IN_TRANSLATION.to_vec();
    _tags.push(PHP_SIG);
    if !reaply {
        for ignore_case in _tags.iter() {
            let pattern = RegexBuilder::new(ignore_case).case_insensitive(true).build().unwrap();
            html = pattern.replace_all(&html, |caps: &Captures| {
                format!("<p translate='no'>{}</p>", caps[0].to_string())
            }).to_string();        }
    } else {
        for ignore_case in _tags.iter() {
            let pattern = RegexBuilder::new(&format!("<p translate='no'>{}</p>", ignore_case))
                .case_insensitive(true)
                .build()
                .unwrap();
            html = pattern.replace_all(&html, |caps: &Captures| {
                let mut caps_text = caps.get(0).unwrap().as_str().to_string();
                caps_text = caps_text.replace("<p translate='no'>", "");
                caps_text = caps_text.replace("</p>", "");
                
                caps_text
            }).to_string();
        }
    }
    html
}

// ----------------------------------------------------------------------------------


fn replace_between_tags(text:&str, max_length:usize) -> (Vec<String>,String){

    let mut first: Vec<usize> = Vec::new(); 
    let mut second: Vec<usize> = Vec::new();

    let mut word_buffer: Vec<String> = Vec::new();
    let count: bool = false;
    let mut char_buffer:VecDeque<&str> = VecDeque::new();
    let text_copy = text.to_string();
         // check how to change string without interrupting the loop
    
    
    let mut resulttwo = replace_php(text, &mut word_buffer);
    return (word_buffer, resulttwo); 
    for (index, char) in text.clone().chars().enumerate(){


        // check if it is the beginning of a signin
        let (mut f, mut s):(usize, usize) = replace_keywords(&text_copy, index, max_length);
        //if f == s{
        //    (f, s) = replace_tags(&text_copy, index);
        //}
        if f != s {
            first.push(f);
            second.push(s);
        }
        // if yes then count until end and remove
        // if no check if it is a php tag and do the same than. 
        // both elements can go into the same VecDeque and be replaced by the php signin becuase
        // in the end it des not really make any difference !
    }


    let mut result:String = String::from(text);
    
    // replace the passages not to translate 
    while let Some(first_idx) = first.pop(){
        if let Some(secont_idx) = second.pop() {
            let mut text_cut:String = String::from(&result[first_idx..=secont_idx]);
            // remove php portions
            if text_cut.starts_with(PHPSTART){
                result.replace_range(first_idx..=secont_idx, "");
            }
            else{
                word_buffer.push(text_cut);
                result.replace_range(first_idx..=secont_idx, SIGNIN);
            }
        }
    } 

    (word_buffer, result)
    // let char_vec = text.chars();
    // for char in char_vec
    // {
    //     if(chars_to_extract.contains(&char)){
    //         char_buffer.push_back(char);
    //     }
    //     //println!("{}", char);
    // }
    
    //while let Some(e) = char_buffer.pop_front(){
    //    println!("{}", e);
    //}
}

fn replace_keywords(text:&str, idx:usize, max_length:usize) -> (usize, usize){
    let text_char_vec = text.to_string().chars().collect::<Vec<_>>();
    let end_idx:usize = cmp::min(idx+max_length, text_char_vec.len());
    let text_copy:String = text_char_vec[idx..end_idx].iter().cloned().collect::<String>().to_lowercase();
    for keyword in KEYWORDS {
        if text_copy.starts_with(keyword){
            return (idx, idx+keyword.len() - 1);
        }
    }
    (0,0)
}

fn replace_tags(text:&str, idx:usize) -> (usize, usize){
    let text_char_vec = text.to_string().chars().collect::<Vec<_>>();
    let text_copy:String = text_char_vec[idx..].iter().cloned().collect::<String>();
    if !text_copy.starts_with(PHPSTART){
        return (0,0);
    }

    for n in 0..(text_copy.len() - 1){
        let c_1:char = text_copy.chars().nth(n).expect("should have been a char");
        let c_2:char = text_copy.chars().nth(n+1).expect("should have been a char");
        let s:&str = &format!("{}{}", c_1, c_2);
        if PHPEND == s{
            return(idx, idx+n+1);
        }
    } 
    (0,0) 
}

// async fn translate_text(text:&str) -> Result<(), Box<dyn Error>>{
//     let tcp = TcpStream::connect("127.0.0.1:5982").await?;
//     let (h2, connection) = client::handshake(tcp).await?;
//     tokio::spawn(async move {
//         connection.await.unwrap();
//     });
//     let mut h2 = h2.ready().await?;
// 
//     let mut request = Request::builder()
//         .method(Method::POST)
//         .uri("https://api.deepl.com/v2/translate")
//         .header("Authorization", "abaa5e16-ba5e-4dec-83fa-44<lable>Select Source Language: </lable>575f26d498:fx")
//         .header("Content-Type", "application/json")
//         .body("{text: 'whatever it takes',target_language: 'DE'}")?;
// 
//     let (response, _) = h2.send_request(request, true).unwrap();
//     let (head, mut body) = response.await?.into_parts();
//     
//     println!("received response: {:?}", body);
// 
//     let mut flow_control = body.flow_control().clone();
//     while let Some(chunk) = body.data().await{
//         let chunk = chunk?;
//         println!("RX: {:?}", chunk);
// 
//         let _ = flow_control.release_capacity(chunk.len());
//     }
// 
//     Ok(()) 
// }

// TODO look up requwest library
//
async fn translate_req(text:&str, target_lang: &str, source_lang:&str, api_key: &str) -> Result<String, Box<dyn Error>>{
    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", api_key.parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let mut source_lang_str: String = String::from("\"source_lang\":\"");
    source_lang_str.push_str(source_lang);
    source_lang_str.push_str("\",");
    if source_lang.is_empty(){
        source_lang_str = String::new();
    }
    
    let mut body_string:String = format!("{{{:?}: {target_lang:?}, ", "target_lang");

    if !source_lang_str.is_empty(){
        body_string.push_str(&source_lang_str);
    }
    body_string.push_str("\"text\": [\"");
    body_string.push_str(text);
    body_string.push_str("\"], \"formality\": \"prefer_more\", \"tag_handling\": \"html\"}");
    let mut res: serde_json::Value = json!(null);
    println!("{body_string}");
    match client.post("https://api.deepl.com/v2/translate")
        .headers(headers)
        .body(format!("{}", body_string.clone()))
        .send()
        .await?
        .json()
        .await {
            Ok (x) => {
                res = x;
            }
            Err(err) => {
                println!("{err}");
            }
        }    
    //let result:String = res["translations"][0]["text"].as_str().unwrap().to_string();
    //let result = res["translations"][0]["text"].as_str();
    if let Some(result) = res["translations"][0]["text"].as_str(){
        //println!("{}", result.to_string());
        return Ok(result.to_string());
    }
    Ok(String::from(text))
}

async fn translate_vec_req(texts:Vec<String>, target_lang: &str, source_lang:&str, api_key: &str) -> Result<Vec<String>, Box<dyn Error>>{
    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", api_key.parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let mut source_lang_str: String = String::from(r#""source_lang":""#);
    source_lang_str.push_str(source_lang);
    source_lang_str.push_str(r#"","#);
    if source_lang.is_empty(){
        source_lang_str = String::new();
    }
    
    let mut body_string:String = format!(r#"{{"target_lang": {target_lang:?}, "#);
    if !source_lang_str.is_empty(){
        body_string.push_str(&source_lang_str);
    }
    let json_text = r#""text": ["#;
    body_string.push_str(&format!("{}", json_text));
    for (idx, text) in texts.iter().enumerate(){
        // body_string.push('"');
        body_string.push_str(&format!("{:#?}",text));
        // body_string.push('"');
        if idx < texts.len()-1{
            body_string.push(',');
        }
    }
   
    let json_end = r#"], "formality": "prefer_more", "tag_handling": "html"}"#;
    body_string.push_str(&format!("{}", json_end));

    let mut res: serde_json::Value = json!(null);
    match client.post("https://api.deepl.com/v2/translate")
        .headers(headers)
        .body(format!("{}", body_string.clone()))
        .send()
        .await?
        .json()
        .await {
            Ok (x) => {
                res = x;
            }
            Err(err) => {
                println!("{err}");
            }
        }    

    //let resulting_translations = res["translations"].as_array();
    if let Some(resulting_translations) = res["translations"].as_array(){
        let mut results:Vec<String> = Vec::new();
        for translation in resulting_translations{
            if let Some(result) = translation["text"].as_str(){
                results.push(result.to_string());
            }
            else {
                return Ok(texts);
            }
        }
        return Ok(results);
    }
    Ok(texts)
}


//TODO  this can run better when chaining up api requests 

