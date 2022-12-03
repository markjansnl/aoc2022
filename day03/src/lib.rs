pub mod input;

pub fn unique_to_priority(&unique: &u8) -> usize {
    if unique >= 97 {
        unique as usize - 96
    } else {
        unique as usize - 38
    }
}
