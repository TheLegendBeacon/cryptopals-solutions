pub fn repeating_key_xor(to_encrypt: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    let textlen: usize = to_encrypt.len();
    let keylen: usize = key.len();

    let mut result: Vec<u8> = Vec::new();

    for number in 0..textlen {
        result.push(to_encrypt[number] ^ key[number % keylen]);
    }

    return result
}