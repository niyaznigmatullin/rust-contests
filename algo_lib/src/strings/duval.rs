use std::iter;
use crate::math::cmp::WithInfinity;

pub fn lex_min_suffix_length<T>(s: &[T]) -> usize
where
    T: Ord,
{
    let n = s.len();
    let mut i = 0;
    let mut j = 0;
    let mut k = 1;
    loop {
        if k == n || s[k] < s[j] {
            let len = k - j;
            while i <= j {
                i += len;
            }
            if i == n {
                return len;
            }
            j = i;
            k = i + 1;
        } else if s[j] == s[k] {
            j += 1;
            k += 1;
        } else {
            k += 1;
            j = i;
        }
    }
}

pub fn lex_min_cyclic_shift_pos<T>(a: &[T]) -> usize
where
    T: Ord,
{
    let s = a
        .iter()
        .chain(a.iter())
        .map(|x| WithInfinity::Value(x))
        .chain(iter::once(WithInfinity::PlusInfinity))
        .collect::<Vec<_>>();
    s.len() - lex_min_suffix_length(&s)
}

pub fn lex_min_cyclic_shift<T>(mut a: Vec<T>) -> Vec<T>
where
    T: Ord,
{
    let pos = lex_min_cyclic_shift_pos(&a);
    a.rotate_left(pos);
    a
}

#[cfg(test)]
mod tests {
    use crate::strings::duval::{
        lex_min_cyclic_shift, lex_min_cyclic_shift_pos, lex_min_suffix_length,
    };

    #[test]
    fn test_ababb() {
        let a = "ababb".to_string().into_bytes();
        assert_eq!(lex_min_suffix_length(&a), 5);
        assert_eq!(lex_min_cyclic_shift_pos(&a), 0);
    }

    #[test]
    fn test_abab() {
        let a = "abab".to_string().into_bytes();
        assert_eq!(lex_min_suffix_length(&a), 2);
    }

    #[test]
    fn test_abbaba() {
        let a = "abbaba".to_string().into_bytes();
        assert_eq!(lex_min_suffix_length(&a), 1);
        assert_eq!(lex_min_cyclic_shift_pos(&a), 5);
        assert_eq!(lex_min_cyclic_shift(a), "aabbab".to_string().into_bytes());
    }

    #[test]
    fn test_babbaba() {
        let a = "babbaba".to_string().into_bytes();
        assert_eq!(lex_min_suffix_length(&a), 1);
        assert_eq!(lex_min_cyclic_shift_pos(&a), 4);
        assert_eq!(lex_min_cyclic_shift(a), "abababb".to_string().into_bytes());
    }
}
