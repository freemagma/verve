
use std::fmt;
use std::str::FromStr;
use std::string::ParseError;

// WORD
#[derive(
    Clone, Debug, PartialEq,
    Eq, Hash, PartialOrd, Ord
)]
pub struct Word {
    pub letters: Vec<Letter>,
}
pub type Id = usize;

#[derive(Debug)]
pub struct WordDatum {
    pub word: Word,
    pub id: Id,
}

impl Word {
    const UTF_A : u8 = 97;
}

impl FromStr for Word {
    type Err = ParseError;
    fn from_str(s : &str) -> Result<Self, Self::Err> {
        let letters = s.bytes()
            .map(|x| 
                Letter::from(x - Word::UTF_A).
                    expect("faulty letter")
            )
            .collect();
        Ok ( Word { letters } )
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.letters.iter().fold(Ok(()), |r, letter| {
            r.and_then(|_| write!(f, "{:?}", letter))
        })
    }
}

// LETTER
#[derive(Copy, Clone, Debug, PartialEq,
    Eq, Hash, PartialOrd, Ord)
]
pub enum Letter {
    A = 00, B = 01, C = 02, D = 03, E = 04,
    F = 05, G = 06, H = 07, I = 08, J = 09,
    K = 10, L = 11, M = 12, N = 13, O = 14,
    P = 15, Q = 16, R = 17, S = 18, T = 19,
    U = 20, V = 21, W = 22, X = 23, Y = 24,
    Z = 25,
}

impl Letter {
    pub fn iter() -> LetterIter {
        return LetterIter { cur : 0 };
    }
    pub fn from(i : u8) -> Option<Self>  {
        let result = match i {
            00 => Letter::A, 01 => Letter::B, 02 => Letter::C, 03 => Letter::D,
            04 => Letter::E, 05 => Letter::F, 06 => Letter::G, 07 => Letter::H,
            08 => Letter::I, 09 => Letter::J, 10 => Letter::K, 11 => Letter::L,
            12 => Letter::M, 13 => Letter::N, 14 => Letter::O, 15 => Letter::P,
            16 => Letter::Q, 17 => Letter::R, 18 => Letter::S, 19 => Letter::T,
            20 => Letter::U, 21 => Letter::V, 22 => Letter::W, 23 => Letter::X,
            24 => Letter::Y, 25 => Letter::Z,
            _  => return None,
        };
        return Some(result);
    }
}

pub struct LetterIter {
    cur : u8,
}

impl Iterator for LetterIter {
    type Item = Letter;
    fn next(&mut self) -> Option<Letter> {
        let result = Letter::from(self.cur);
        self.cur += 1;
        return result;
    }
}