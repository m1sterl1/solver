use std::iter::zip;


/// Return number ofequal elements in the same position
/// for two strings
pub fn get_coincidence(s1: &str, s2: &str) -> usize{
    zip(s1.chars(), s2.chars())
    .fold(
        0,
        |acc, (c1, c2)|{
            if c1.eq(&c2){
                acc+1
            }
            else{
                acc
            }
        }
    )
}

#[cfg(test)]
mod test{
    use super::get_coincidence;

    #[test]
    fn test_compare(){
        let s1 = "processor";
        let s2 = "durasteel";
        assert_eq!(get_coincidence(s1, s2), 0);

        let s2 = "consisted";
        assert_eq!(get_coincidence(s1, s2), 1);
    }
}