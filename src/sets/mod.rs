pub mod set1;
pub mod tests;

pub fn tochar(vec: Vec<u8>) -> String {
    vec.iter().map(|v| *v as char).collect::<String>()
}
