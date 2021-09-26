use crate::sets::set1;
use crate::sets::tochar;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn test_equal(message: &str, condition: bool) {
    print!("{} ", message);
    if condition {
        println!("\u{001b}[32;1mPASS\u{001b}[0m");
    } else {
        println!("\u{001b}[31;1mFAIL\u{001b}[0m")
    }
}

pub fn challenge1test() {
    let hexencstring = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    test_equal(
        "Testing Challenge 1...",
        &set1::hex2base64(&hexencstring)
            == "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
    );
}

pub fn challenge2test() {
    let hexdecstr1 = hex::decode("1c0111001f010100061a024b53535009181c");
    let hexdecstr2 = hex::decode("686974207468652062756c6c277320657965");
    let result = set1::fixed_xor(
        &hexdecstr1.expect("Failed to get Vec."),
        &hexdecstr2.expect("Failed to get Vec."),
    );
    let hexresult = hex::encode(result);
    test_equal(
        "Testing Challenge 2...",
        &hexresult == "746865206b696420646f6e277420706c6179",
    );
}

pub fn challenge3test() {
    let val = set1::break_single_char_xor(
        &hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
            .expect("no"),
    );
    test_equal(
        "Testing Challenge 3...",
        &tochar(&val) == "Cooking MC's like a pound of bacon",
    );
}

pub fn challenge4test() {
    let filepath = Path::new("src/txts/4.txt");
    let file = File::open(&filepath).expect("go cry");
    let filecontent = BufReader::new(file);
    let lines: Vec<_> = filecontent
        .lines()
        .map(|item| item.expect("cry").as_bytes().to_vec())
        .map(|item| hex::decode(item).expect("cry more"))
        .collect();
    test_equal(
        "Testing Challenge 4...",
        hex::encode(set1::find_single_char_xor(&lines))
            == "7b5a4215415d544115415d5015455447414c155c46155f4058455c5b523f",
    );
    return ();
}

pub fn challenge5test() {
    let stringbytes: Vec<u8> = format!("Burning 'em, if you ain't quick and nimble{}I go crazy when I hear a cymbal", tochar(&[10])).as_bytes().to_vec();
    let key: Vec<u8> = "ICE".as_bytes().to_vec();
    let xored_val: Vec<u8> = set1::repeating_key_xor(&stringbytes, &key);
    test_equal("Testing Challenge 5...", hex::encode(xored_val) == "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f")

}