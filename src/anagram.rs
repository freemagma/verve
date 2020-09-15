use std::collections::HashMap;

use crate::word::*;
use crate::Verve;

pub struct AnagramR {
    alpha_to_words : HashMap<Word, Vec<Id>>,
    alpha_trie : TrieNode,
}

#[derive(Debug)]
pub enum TrieNode {
    Empty,
    Full { next : [Box<TrieNode> ; 26], alpha : Option<Word> },
}

impl Default for TrieNode {
    fn default() -> Self { TrieNode::Empty }
}


impl TrieNode {
    pub fn new() -> TrieNode {
        TrieNode::Empty
    }
    pub fn new_full() -> TrieNode {
        TrieNode::Full { 
            next  : Default::default(),
            alpha : None
        }
    }
}

enum StopStatus { Stop, Equal, Less }

impl AnagramR {
    pub fn new(verve : &Verve) -> Self {
        let mut alpha_to_words = HashMap::new();
        let mut alpha_trie = TrieNode::new_full();
        for word_datum in verve.word_data.iter() {

            let alpha_key = alphatize(&word_datum.word);

            let mut current = &mut alpha_trie;
            for (i, letter) in alpha_key.letters.iter().enumerate() {
                let letter_index = *letter as usize;
                if let TrieNode::Full { next, alpha : _ } = current {
                    if let TrieNode::Empty = &mut *next[letter_index] {
                        next[letter_index] = Box::new(TrieNode::new_full());
                    }
                    current = &mut *next[letter_index];
                    
                }
                if let TrieNode::Full { next : _, alpha } = current {
                    if i == alpha_key.letters.len() - 1 {
                        alpha.get_or_insert_with(|| alpha_key.clone());
                    } 
                }                
            }

            let s = alpha_to_words
                .entry(alpha_key)
                .or_insert(Vec::new());
            s.push(word_datum.id);
        }
        return AnagramR {
            alpha_to_words,
            alpha_trie,
        };
    }
    pub fn exact_anagrams(&self, word : &Word) -> Vec<Id> {
        let mut vector = Vec::new();
        let alpha_word = alphatize(word);
        if self.alpha_to_words.contains_key(&alpha_word) {
            vector.push(self.alpha_to_words.get(&alpha_word).unwrap());
        }
        return vector.into_iter().flat_map(|s| s.iter()).map(|id| *id).collect();
    }
    pub fn anagrams(&self, word : &Word) -> Vec<Id> {
        return self.anagrams_helper(word, None);
    }
    fn alphagrams_helper(&self, word : &Word, stop_alpha : Option<&Word>) -> Vec<&Word> {
        let alpha = alphatize(word);
        let mut stack : Vec<(&TrieNode, bool, bool, usize, usize)> = Vec::new();
        let mut alphagrams = Vec::new();
        stack.push((&self.alpha_trie, true, stop_alpha.is_none(), 0, 0));
        while !stack.is_empty() {
            let (node, new_node, less, index, length) = stack.pop()
                .expect("stack is empty somehow");
            if let TrieNode::Full { next, alpha : node_alpha } = node {
                if new_node && node_alpha.is_some() {
                    alphagrams.push(node_alpha.as_ref().unwrap());
                }
                if index < alpha.letters.len() {
                    let letter = *alpha.letters.get(index)
                        .expect("index out of bounds");
                    let letter_index = letter as usize;
                    let mut new_less = less;
                    if !less {
                        match AnagramR::stop_status(letter, stop_alpha, length) {
                            StopStatus::Stop => continue,
                            StopStatus::Less => new_less = true,
                            _ => {}
                        };
                    }
                    let mut new_index = index + 1;
                    while new_index < alpha.letters.len() {
                        let next_letter = *alpha.letters.get(new_index)
                            .expect("index out of bounds");
                        if letter != next_letter {
                            break
                        } 
                        new_index += 1;
                    }
                    stack.push((node, false, new_less, new_index, length));
                    stack.push((&next[letter_index], true, new_less, index + 1, length + 1));
                }
            }
        }
        return alphagrams;
    }
    fn anagrams_helper(&self, word : &Word, stop_alpha : Option<&Word>) -> Vec<Id> {
        self.alphagrams_helper(word, stop_alpha).into_iter()
            .flat_map(|a| self.alpha_to_words.get(&a).unwrap().iter())
            .map(|id| *id)
            .collect()
    }
    fn stop_status(test_letter : Letter, stop_alpha : Option<&Word>, stop_index : usize) -> StopStatus {
        if let Some(stop) = stop_alpha {
            if stop_index >= stop.letters.len() {
                return StopStatus::Stop;
            }
            let stop_letter = *stop.letters.get(stop_index)
                .expect("index out of bounds");
            if stop_letter < test_letter {
                return StopStatus::Stop;
            } else if stop_letter == test_letter {
                return StopStatus::Equal;
            }
        }
        StopStatus::Less
    }
    pub fn multigrams(&self, word : &Word, word_limit : Option<usize>) -> Vec<Vec<Id>> {
        let alpha = alphatize(word);
        let mut stack : Vec<(Word, Vec<&Word>)> = Vec::new();
        let mut alpha_lists : Vec<Vec<&Word>> = Vec::new();
        stack.push((alpha, Vec::new()));
        while !stack.is_empty() {
            let (left, current) = stack.pop()
                .expect("stack is empty somehow");
            for next_alpha in self.alphagrams_helper(&left, current.last().copied()) {
                let mut next_vec = current.clone();
                next_vec.push(next_alpha);
                let next_left = alpha_subtract(&left, &next_alpha);
                if next_left.letters.len() == 0 {
                    alpha_lists.push(next_vec)
                } else if word_limit.is_none() || next_vec.len() < word_limit.unwrap() {
                    stack.push((next_left, next_vec));
                }
            }
        }
        let mut stack : Vec<(Vec<&Word>, Vec<Vec<Id>>)> = Vec::new();
        let mut finals = Vec::new();
        for list in alpha_lists.into_iter() {
            stack.push((list, vec!(Vec::new())));
        }
        while !stack.is_empty() {
            let (mut alphas, products) = stack.pop()
                .expect("stack is empty somehow");
            let cur_alpha = alphas.pop()
                .expect("no alphas left");
            let mut new_products = Vec::new();
            for word in self.alpha_to_words.get(&cur_alpha).unwrap().iter() {
                for product in products.iter() {
                    let mut new_product = product.clone();
                    new_product.push(*word);
                    new_products.push(new_product);
                }
            }
            if alphas.len() == 0 {
                finals.extend(new_products.into_iter());
            } else {
                stack.push((alphas, new_products));
            }
        }
        return finals;
    }
}

fn alphatize(word : &Word) -> Word {
    let mut alpha_key = word.clone();
    alpha_key.letters.sort();
    return alpha_key;
}

fn alpha_subtract(source : &Word, to_subtract : &Word) -> Word {
    let mut alpha : Vec<Letter> = Vec::new();
    let mut sub_iter = to_subtract.letters.iter();
    let mut sub_option = sub_iter.next();
    for letter in source.letters.iter() {
        match sub_option {
            Some(sub) => {
                if letter != sub {
                    alpha.push(*letter);
                } else {
                    sub_option = sub_iter.next();
                }
            },
            None => alpha.push(*letter)
        };
    }
    Word { letters : alpha }
}