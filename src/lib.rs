use wasm_bindgen::prelude::*;

// pub fn solve(params: &str) -> String {
  
//     // return String::from("not real numbers");
//     // return serde_json::to_string(&solution).unwrap();
// }


const BASE64_ALPHABET: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '-',
];

#[wasm_bindgen]
pub fn base64_encode(content: &str) -> String {
    let characters: &[u8] = content.as_bytes();
    let mut base64_output = Vec::with_capacity((characters.len() / 3 + 1) * 4);

    let mut counter = 0;
    while counter + 3 <= characters.len() {
        let first_base64_character = extract_first_character_bits(characters[counter]);
        let second_base64_character =
            extract_second_character_bits(characters[counter], characters[counter + 1]);
        let third_base64_character =
            extract_third_character_bits(characters[counter + 1], characters[counter + 2]);
        let fourth_base64_character = characters[counter + 2] & 0b00111111;

        base64_output.append(&mut vec![
            BASE64_ALPHABET[first_base64_character as usize],
            BASE64_ALPHABET[second_base64_character as usize],
            BASE64_ALPHABET[third_base64_character as usize],
            BASE64_ALPHABET[fourth_base64_character as usize],
        ]);

        counter += 3;
    }

    if counter + 1 == characters.len() {
        let first_base64_character = extract_first_character_bits(characters[counter]);
        let second_base64_character = extract_second_character_bits(characters[counter], 0);

        base64_output.append(&mut vec![
            BASE64_ALPHABET[first_base64_character as usize],
            BASE64_ALPHABET[second_base64_character as usize],
            '=',
            '=',
        ]);
    } else if counter + 2 == characters.len() {
        let first_base64_character = extract_first_character_bits(characters[counter]);
        let second_base64_character =
            extract_second_character_bits(characters[counter], characters[counter + 1]);
        let third_base64_character = extract_third_character_bits(characters[counter + 1], 0);

        base64_output.append(&mut vec![
            BASE64_ALPHABET[first_base64_character as usize],
            BASE64_ALPHABET[second_base64_character as usize],
            BASE64_ALPHABET[third_base64_character as usize],
            '=',
        ]);
    }

    base64_output.into_iter().collect::<String>()
}


fn extract_first_character_bits(first_byte: u8) -> u8 {
    (first_byte & 0b1111100) >> 2
}

fn extract_second_character_bits(first_byte: u8, second_byte: u8) -> u8 {
    (first_byte & 0b00000011) << 4 | ((second_byte & 0b11110000) >> 4)
}

fn extract_third_character_bits(second_byte: u8, third_byte: u8) -> u8 {
    (second_byte & 0b00001111) << 2 | ((third_byte & 0b11000000) >> 6)
}


#[wasm_bindgen]
pub fn base64_decode(base64: &str) -> String {
  if base64.len() % 4 != 0 {
      panic!("A base64 string contains a multiple of 4 characters");
  }

  let base64_bits: Vec<u8> = base64
      .chars()
      .map(|character| {
          // The bit value is the character value minus ascii value plus offset in base64 alphabet.
          if character.is_ascii_uppercase() {
              (character as u32) - 65
          } else if character.is_ascii_lowercase() {
              (character as u32) - 97 + 26
          } else if character.is_numeric() {
              (character as u32) - 48 + 52
          } else if character == '=' {
              return 255;
          } else {
              panic!("All base64 characters need to be in [A-Za-z0-9]");
          }
      })
      .map(|character| character as u8)
      .collect::<Vec<u8>>();
  let chunks: Vec<&[u8]> = base64_bits.chunks(4).collect();
  let mut output = String::new();

  for chunk in &chunks {
      let mut character_bits: u32 = 0;
      character_bits |= (chunk[0] as u32) << 18;
      character_bits |= (chunk[1] as u32) << 12;

      let character_bytes;
      if chunk[2] == 255 {
          character_bytes = character_bits.to_be_bytes()[1..2].to_vec();
      } else if chunk[3] == 255 {
          character_bits |= (chunk[2] as u32) << 6;

          character_bytes = character_bits.to_be_bytes()[1..3].to_vec();
      } else {
          character_bits |= (chunk[2] as u32) << 6;
          character_bits |= chunk[3] as u32;

          character_bytes = character_bits.to_be_bytes()[1..4].to_vec();
      }

      let characters = std::str::from_utf8(&character_bytes);
      match characters {
          Ok(characters) => output.push_str(characters),
          Err(_) => panic!("The base64 encoded value does not contain a valid utf8 string."),
      }
  }

  output
}

#[wasm_bindgen]
pub fn ceaser_7(params: &str) -> String {
    fn add_char(c: char) -> char {
        std::char::from_u32(c as u32 + 7).unwrap_or(c)
    }
    return params.chars().map(add_char).collect()
}

