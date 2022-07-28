pub fn get_arr_from_letter(letter: String) -> Result<[u8; 26], &'static str>{
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let cleaned_letter = letter.to_lowercase();
    if letter.is_empty() || !alphabet.contains(&cleaned_letter){
        return Err("Invalid letter");
    }
    let index = alphabet.find(&cleaned_letter).expect("Not in alphabet");
    let mut letter_arr: [u8; 26] = [0; 26];
    letter_arr[index] = 1;
    Ok(letter_arr)
}