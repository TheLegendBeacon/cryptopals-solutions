pub fn fixed_xor(buffer1: Vec<u8>, buffer2: Vec<u8>) -> Vec<u8> {
    let mut encoded_vec = Vec::new();
    let iterator = buffer1.iter().zip(buffer2.iter());
    for (_, (index1, index2)) in iterator.enumerate() {
            encoded_vec.push(index1 ^ index2);
        }

    encoded_vec
}