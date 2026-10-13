#![allow(unused)]
use std::fmt::Debug;

use enum_display::EnumDisplay;

#[derive(EnumDisplay)]
pub enum Dtype {
    Plus,
    Minus,
    Fail,
}
fn split_words(data : String) -> Vec<String>{
   let mut parts:Vec<String> = data.split(' ').map(String::from).collect();
//    for i in 0..parts.len(){
//         if  parts[i].len() == 1 {
//             continue;
//         } 
//         let k:String = match parts[i].to_string() {
//            _ => parts[i].to_string(), 
//         };
//         if k == parts[i] {
//             continue;
//         }
//    }
   return parts; 
}



pub fn lexer(data: String) -> Vec<Dtype> {
    let spw : Vec<String> = split_words(data);
    println!("{:?}\nLen is :{}",spw,spw.len());
    let mut values : Vec<Dtype> = Vec::new();  
    for i in 0..(spw.len()){
        println!("At : {}\n",i);
        values[i] = match (spw[i].as_str()) {
            "+" => Dtype::Plus,
            "-" => Dtype::Minus,
            &_ => Dtype::Fail,
        };
    }
    return values;
}
