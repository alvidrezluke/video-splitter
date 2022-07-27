use std::env;

pub fn get_args() -> Result<(String, [u8; 26]), &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("No filepath or letter provided.");
    } else if args.len() < 3 {
        return Err("Invalid parameters. Goes path then letter")
    }
    Ok((args[1].clone(), get_arr_from_letter(args[2].clone()).expect("Could not get arr")))
}

fn get_arr_from_letter(letter: String) -> Result<[u8; 26], &'static str>{
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let cleaned_letter = letter.to_lowercase();
    if letter.len() < 1 || !alphabet.contains(&cleaned_letter){
        return Err("Invalid letter");
    }
    let index = alphabet.find(&cleaned_letter).expect("Not in alphabet");
    let mut letter_arr: [u8; 26] = [0; 26];
    letter_arr[index] = 1;
    Ok(letter_arr)
}