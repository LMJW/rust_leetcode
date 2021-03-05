struct Solution;
impl Solution {
    // pub fn merge_alternately(word1: String, word2: String) -> String {
    //     let (mut i, mut j) = (0,0);
    //     let mut res = String::new();
    //     let (mut word1, mut word2) = (word1.chars().collect::<Vec<_>>(), word2.chars().collect::<Vec<_>>());

    //     loop {
    //         match (word1.get(i), word2.get(j)){
    //             (Some(a), Some(b)) => {
    //                 if a > b {
    //                     res.push(*b);
    //                     j += 1;
    //                 }else {
    //                     res.push(*a);
    //                     i += 1;
    //                 }
    //                 dbg!(a, b, &res);
    //             },
    //             (None, Some(b)) => {
    //                 res.push(*b);
    //                 j += 1;
    //                 dbg!( b, &res);

    //             },
    //             (Some(a),None) => {
    //                 res.push(*a);
    //                 i += 1;
    //                 dbg!(a,  &res);

    //             },
    //             (None, None) => {break;},
    //         }
    //     }
    //     return res;
    // }
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