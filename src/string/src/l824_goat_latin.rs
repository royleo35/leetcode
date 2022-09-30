pub struct Solution;
impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        let vowels = ['a','e','i','o','u','A','E','I','O','U'];
        let mut res = Vec::new();
        for (i,word) in sentence.split(' ').enumerate() {
            let new_word = match word.starts_with(&vowels[..]) {
                true => word.to_string(),
                false => word[1..].to_string()+&word[..1],
            } + "ma" + &"a".repeat(i+1);
            res.push(new_word);
        }
        res.join(" ")
    }
}