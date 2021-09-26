use crate::sets::set1::{break_single_char_xor, get_english_score};
use crate::sets::tochar;

fn char_printable(a: &u8) -> bool {
    match a {
        32..=126 => true,
        10 | 13 => true,
        _ => false,
    }
}

fn is_printable(string: &str) -> bool {
    for item in string.as_bytes().iter() {
        if !char_printable(item) {
            return false;
        }
    }
    true
}

pub fn find_single_char_xor(list_of_vecs: &[Vec<u8>]) -> Vec<u8> {
    let mut decoded_vecs: Vec<Vec<u8>> = Vec::new();

    for string in list_of_vecs {
        decoded_vecs.push(break_single_char_xor(&string));
    }

    let mut weights: Vec<f32> = Vec::new();

    for decoded_string in decoded_vecs {
        if is_printable(&tochar(&decoded_string)) {
            weights.push(get_english_score(&tochar(&decoded_string)));
        } else {
            weights.push(0.0)
        }
    }

    let (maxpos, _) = weights
        .iter()
        .enumerate()
        .reduce(|(max_pos, max_val), (current_pos, current_val)| {
            if current_val > max_val {
                (current_pos, current_val)
            } else {
                (max_pos, max_val)
            }
        })
        .expect("cry");

    let mut ret = vec![];
    ret.extend_from_slice(&list_of_vecs[maxpos]);
    ret
}
