use crate::error::{Error, self};
use colored::*;

pub fn get_arr_from_letter(letter: String) -> Result<[u8; 26], Error>{
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let cleaned_letter = letter.to_lowercase();
    if letter.is_empty(){
        return Err(error::new_error("Letter can not be empty".red().to_string()))
    } else if !alphabet.contains(&cleaned_letter){
        return Err(error::new_error(format!("Letter {} can not be found in the alphabet", cleaned_letter).red().to_string()));
    }
    let index = alphabet.find(&cleaned_letter).unwrap();
    let mut letter_arr: [u8; 26] = [0; 26];
    letter_arr[index] = 1;
    Ok(letter_arr)
}