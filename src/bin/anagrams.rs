

fn is_anagram(word1: &str, word2: &str) -> bool {
    let mut remaining_letters = String::from(word2);

    if word1.len() != word2.len() {
        return false
    }

    for character in word1.chars() {
        if !remaining_letters.contains(character) {
            return false
        }

        remaining_letters.remove(remaining_letters.chars().position(|x| x == character).expect("letter exist"));
    }

    return true
}

fn anagrams(word: &str, words: &[String]) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let remaining = words
        .iter()
        .filter(|x| is_anagram(x, word))
        .collect::<Vec<&String>>();
    
    for r in remaining {
        result.push(r.clone())
    }
    return result
}


fn main() {
    let result = anagrams("abba", &["aabb".to_string(), "abcd".to_string(), "bbaa".to_string(), "dada".to_string()]);
    println!("{}", result.len());
}
