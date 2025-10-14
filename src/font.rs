pub const SYMBOL_WIDTH: usize = 6;
pub const SYMBOL_HEIGHT: usize = 5;

#[derive(Debug, Clone, Copy)]
pub struct CharSymbol(pub &'static str);

impl CharSymbol {
    pub fn new(symbol: &'static str) -> Self {
        Self(symbol)
    }
    pub fn height(&self) -> usize {
        self.0.lines().count() - 1
    }
    pub fn width(&self) -> usize {
        let mut line_length: Option<usize> = None;
        for line in self.0.lines() {
            if line == "\n" {
                continue;
            }
            if line.chars().count() > 0 {
                if line_length.is_some_and(|len| len != line.chars().count()) {
                    panic!(
                        "Inconsistent line length in CharSymbol! ({} != {}) \n{}",
                        line_length.unwrap(),
                        line.chars().count(),
                        self.0
                    )
                } else {
                    line_length = Some(line.chars().count())
                }
            }
        }
        line_length.unwrap()
    }
    pub fn get(char: char) -> Self {
        match char {
            // Uppercase Alphabet
            'A' => Self(A),
            'B' => Self(B),
            'C' => Self(C),
            'D' => Self(D),
            'E' => Self(E),
            'F' => Self(F),
            'G' => Self(G),
            'H' => Self(H),
            'I' => Self(I),
            'J' => Self(J),
            'K' => Self(K),
            'L' => Self(L),
            'M' => Self(M),
            'N' => Self(N),
            'O' => Self(O),
            'P' => Self(P),
            'Q' => Self(Q),
            'R' => Self(R),
            'S' => Self(S),
            'T' => Self(T),
            'U' => Self(U),
            'V' => Self(V),
            'W' => Self(W),
            'X' => Self(X),
            'Y' => Self(Y),
            'Z' => Self(Z),
            // Digits
            '0' => Self(ZERO),
            '1' => Self(ONE),
            '2' => Self(TWO),
            '3' => Self(THREE),
            '4' => Self(FOUR),
            '5' => Self(FIVE),
            '6' => Self(SIX),
            '7' => Self(SEVEN),
            '8' => Self(EIGHT),
            '9' => Self(NINE),
            // Punctuation
            ' ' => Self(BACKSPACE),
            // Unknown char
            _ => Self(UNKNOWN),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::font::{CharSymbol, SYMBOL_HEIGHT, SYMBOL_WIDTH, UNKNOWN};

    #[test]
    fn chars_consistency() {
        let tested_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 ";
        for char in tested_chars.chars() {
            let symbol = CharSymbol::get(char);
            assert_ne!(symbol.0, UNKNOWN);
            assert_eq!(
                symbol.height(),
                SYMBOL_HEIGHT,
                "height mismatch (got {} instead of {}) {}",
                symbol.height(),
                SYMBOL_HEIGHT,
                symbol.0
            );
            assert_eq!(
                symbol.width(),
                SYMBOL_WIDTH,
                "width mismatch (got {} instead of {}) {}",
                symbol.width(),
                SYMBOL_WIDTH,
                symbol.0
            );
        }
    }
}

pub const UNKNOWN: &str = "
■■■■■
■■■■■
■■■■■
■■■■■
■■■■■";

// Uppercase Letters

pub const A: &str = "
 ■■   
■  ■  
■■■■■ 
■    ■
■    ■";

pub const B: &str = "
■■■■  
■    ■
■■■■  
■    ■
■■■■  ";

pub const C: &str = "
 ■■■  
■    ■
■     
■    ■
 ■■■  ";

pub const D: &str = "
■■■■  
■    ■
■    ■
■    ■
■■■■  ";

pub const E: &str = "
■■■■■ 
■     
■■■■  
■     
■■■■■ ";

pub const F: &str = "
■■■■■ 
■     
■■■■  
■     
■     ";

pub const G: &str = "
 ■■■  
■     
■  ■■ 
■    ■
 ■■■  ";

pub const H: &str = "
■    ■
■    ■
■■■■■ 
■    ■
■    ■";

pub const I: &str = "
  ■■■ 
   ■  
   ■  
   ■  
  ■■■ ";

pub const J: &str = "
  ■■■ 
   ■  
   ■  
■  ■  
 ■■   ";

pub const K: &str = "
■    ■
■  ■  
■■■   
■  ■  
■    ■";

pub const L: &str = "
■     
■     
■     
■     
■■■■■ ";

pub const M: &str = "
■    ■
■■  ■■
■ ■■ ■
■    ■
■    ■";

pub const N: &str = "
■    ■
■■   ■
■ ■  ■
■  ■ ■
■   ■■";

pub const O: &str = "
 ■■■  
■   ■ 
■   ■ 
■   ■ 
 ■■■  ";

pub const P: &str = "
■■■■  
■   ■ 
■■■■  
■     
■     ";

pub const Q: &str = "
 ■■■  
■    ■
■  ■ ■
■   ■ 
 ■■■■ ";

pub const R: &str = "
■■■■  
■   ■ 
■■■■  
■  ■  
■   ■ ";

pub const S: &str = "
 ■■■■ 
■     
 ■■■  
    ■ 
■■■■  ";

pub const T: &str = "
■■■■■ 
  ■   
  ■   
  ■   
  ■   ";

pub const U: &str = "
■    ■
■    ■
■    ■
■    ■
 ■■■■ ";

pub const V: &str = "
■    ■
■    ■
 ■  ■ 
 ■  ■ 
  ■■  ";

pub const W: &str = "
■    ■
■    ■
■ ■■ ■
■■  ■■
■    ■";

pub const X: &str = "
■    ■
 ■  ■ 
  ■■  
 ■  ■ 
■    ■";

pub const Y: &str = "
■    ■
 ■  ■ 
  ■■  
  ■   
  ■   ";

pub const Z: &str = "
■■■■■ 
   ■  
  ■   
 ■    
■■■■■ ";

// Digits

pub const ZERO: &str = "
 ■■■  
■   ■ 
■   ■ 
■   ■ 
 ■■■  ";

pub const ONE: &str = "
  ■   
 ■■   
  ■   
  ■   
 ■■■  ";

pub const TWO: &str = "
 ■■■  
■   ■ 
  ■■  
 ■    
■■■■■ ";

pub const THREE: &str = "
 ■■■  
■   ■ 
  ■■  
■   ■ 
 ■■■  ";

pub const FOUR: &str = "
  ■■  
 ■ ■  
■  ■  
■■■■■ 
   ■  ";

pub const FIVE: &str = "
■■■■■ 
■     
■■■■  
    ■ 
■■■■  ";

pub const SIX: &str = "
 ■■■  
■     
■■■■  
■   ■ 
 ■■■  ";

pub const SEVEN: &str = "
■■■■■ 
   ■  
  ■   
 ■    
■     ";

pub const EIGHT: &str = "
 ■■■  
■   ■ 
 ■■■  
■   ■ 
 ■■■  ";

pub const NINE: &str = "
 ■■■  
■   ■ 
 ■■■■ 
    ■ 
 ■■■  ";

// Punctuation

const BACKSPACE: &str = "
      
      
      
      
      ";
