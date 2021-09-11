use crate::sets::set1;
use crate::sets::tochar;

fn test_equal(message: &str, first: &str, second: &str) {
    print!("{} ", message);
    if first == second {
        println!("\u{001b}[32;1mPASS\u{001b}[0m");
    } else {
        println!("\u{001b}[31;1mFAIL\u{001b}[0m")
    }
}

pub fn challenge1test() {
    let hexencstring = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    test_equal(
        "Testing Challenge 1...",
        &set1::hex2base64(&hexencstring),
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
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
    test_equal("Testing Challenge 2...", &hexresult, "746865206b696420646f6e277420706c6179");
}

pub fn challenge3test() {
    let val = set1::break_single_char_xor(
        &hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
            .expect("no"),
    );
    test_equal("Testing Challenge 3...", &tochar(val), "Cooking MC's like a pound of bacon");
}
