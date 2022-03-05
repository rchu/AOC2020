use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

use anyhow::Result;
use anyhow::bail;
use array_tool::vec::Shift;
use crate::puzzle::Puzzle;

fn game_part1(mut player1: Vec<u8>, mut player2: Vec<u8>) -> Vec<u8> {
    while !player1.is_empty() && !player2.is_empty() {
        match (player1.shift(), player2.shift()) {
            (Some(a), Some(b)) if a > b => { player1.push(a); player1.push(b); },
            (Some(a), Some(b)) if a < b => { player2.push(b); player2.push(a); },
            _ => {},
        }
    }
    if player1.is_empty() {player2} else {player1}
}

fn game_part2(mut player1: Vec<u8>, mut player2: Vec<u8>) -> (usize, Vec<u8>) {
    let mut prev_decks: HashSet<u64> = HashSet::new();
    loop {
        let mut hasher = DefaultHasher::new();
        player1.hash(&mut hasher);
        player2.hash(&mut hasher);
        if !prev_decks.insert(hasher.finish()) {
            return (1,player1);
        }
        
        let (card1, card2) = (player1.shift().unwrap() as usize, player2.shift().unwrap() as usize);
      
        if card1 <= player1.len() && card2 <= player2.len() {
            if let (1,_) = game_part2(player1.iter().take(card1).copied().collect(), player2.iter().take(card2).copied().collect()) {
                player1.extend([card1 as u8, card2 as u8]);
                if player2.is_empty() {return (1,player1); }
            } else {
                player2.extend([card2 as u8, card1 as u8]);
                if player1.is_empty() {return (2,player2); }
            }
        } else if card1 > card2 {
            player1.extend([card1 as u8, card2 as u8]);
            if player2.is_empty() {return (1,player1); }
        } else {
            player2.extend([card2 as u8, card1 as u8]);
            if player1.is_empty() {return (2,player2); }
        }
        
    }
}
fn score(winner: &[u8]) -> usize {
    winner
        .iter().enumerate()
        .map(|(i,x)| (winner.len()-i) * (*x as usize) )
        .sum::<usize>()
}
impl Puzzle {
    pub fn day22(&mut self) -> Result<()> {
        let input = &mut self.input.iter();
        
        let mut player1: Vec<u8> = Vec::new();
        if input.next()!= Some(&"Player 1:".to_string()) { bail!("Expected 'Player 1'")}
        for line in input.by_ref() {
            if line.is_empty() {break}
            player1.push(line.parse()?)
        }
        let mut player2: Vec<u8> = Vec::new();
        if input.next()!= Some(&"Player 2:".to_string()) { bail!("Expected 'Player 2'")}
        for line in input {
            if line.is_empty() {break}
            player2.push(line.parse()?)
        }

        self.set_answer_a(score( &game_part1(player1.clone(), player2.clone())));
        self.set_answer_b(score( &game_part2(player1.clone(), player2.clone()).1));
        Ok(())
    }
}