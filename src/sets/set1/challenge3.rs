use std::collections::HashMap;

pub fn single_char_xor(buffer: &Vec<u8>, keynum: &u8) -> Vec<u8> {
    let mut encoded_vec = Vec::new();
    for byte in buffer {
        encoded_vec.push(byte ^ keynum)
    }
    encoded_vec
}

fn get_possible_iterations(buffer: &Vec<u8>) -> Vec<Vec<u8>> {
    let mut possible_iterations = Vec::new();
    for chr in 1..256 {
        possible_iterations.push(single_char_xor(buffer, &(chr as u8)));
    }

    possible_iterations
}

pub fn get_english_score(string: &str) -> f32 {
    let mut score: f32 = 0.0;

    let mut letters: Vec<_> = ('a'..='z').collect();
    letters.push(' ');
    let weightage_numbers: Vec<f32> = vec![
        8.2, 1.5, 2.8, 4.3, 13.0, 2.2, 2.0, 6.1, 7.0, 0.15, 0.77, 4.0, 2.4, 6.7, 7.5, 1.9, 0.095,
        6.0, 6.3, 9.1, 2.8, 0.98, 2.4, 0.15, 2.0, 0.074, 13.0,
    ];
    let letter_frequency_chart: HashMap<char, f32> = letters
        .into_iter()
        .zip(weightage_numbers)
        .collect::<HashMap<char, f32>>();

    for chr in string.chars() {
        if letter_frequency_chart.contains_key(&chr) {
            score += letter_frequency_chart
                .get(&chr)
                .expect("Unable to get frequency.")
        }
    }

    score
}

pub fn break_single_char_xor(buffer: &Vec<u8>) -> Vec<u8> {
    let total_vecs: Vec<Vec<u8>> = get_possible_iterations(buffer);
    let mut total_strings: Vec<String> = Vec::new();

    for vector in total_vecs {
        total_strings.push(crate::sets::tochar(vector));
    }

    let mut total_weights: Vec<f32> = Vec::new();

    for string in total_strings.iter() {
        total_weights.push(get_english_score(&string));
    }

    let (minpos, _) = total_weights
        .iter()
        .enumerate()
        .reduce(|(max_pos, max_val), (current_pos, current_val)| {
            if current_val > max_val {
                (current_pos, current_val)
            } else {
                (max_pos, max_val)
            }
        })
        .expect("no");

    let req_string = String::from(&total_strings[minpos]);
    let result_vec = req_string.into_bytes();
    result_vec
}
