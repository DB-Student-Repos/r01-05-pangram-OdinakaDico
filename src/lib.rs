/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut alphabet = [false; 26]; 
    for c in sentence.chars().filter(|c| c.is_ascii_alphabetic()) {
        let index = c.to_ascii_lowercase() as usize - 'a' as usize;
        alphabet[index] = true;
    }
    alphabet.iter().all(|&present| present)
}
