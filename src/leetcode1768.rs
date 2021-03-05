struct Solution;
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut res = String::new();

        let mut word2 = word2.chars();
        for letter in word1.chars(){
            res.push(letter);
            match word2.nth(0){
                Some(i) => {res.push(i);},
                None => {continue;},
            }
        }
        while let Some(a) = word2.nth(0){
            res.push(a);
        }

        res
    }
}
#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_1768() {
        assert_eq!(Solution::merge_alternately("abc".to_owned(), "pqr".to_owned()), "apbqcr".to_owned());
        assert_eq!(Solution::merge_alternately("ab".to_owned(), "pqrs".to_owned()), "apbqrs".to_owned());
    }
}