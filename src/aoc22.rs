use std::collections::{VecDeque, HashSet};

pub fn advent() {
    //let data = parse_data();
    let player1: VecDeque<usize> = vec!(
        29, 25, 9, 1, 17, 28, 12, 49, 8, 15, 41, 31, 39, 24, 40, 23, 6, 21, 13, 45, 20, 2, 42, 47, 10
    ).into_iter().collect();
    let player2: VecDeque<usize> = vec!(
        46, 27, 44, 18, 30, 50, 37, 11, 43, 35, 34, 4, 22, 7, 33, 16, 36, 26, 48, 19, 38, 14, 5, 3, 32
    ).into_iter().collect();

    let (result1, result2) = play_game(player1.clone(), player2.clone());
    println!("Player 1's deck: {:?}\nPlayer 2's deck: {:?}", result1, result2);
    let winner = if result2.is_empty() { result1 } else { result2 };
    println!("Score: {}\n", score(&winner));

    let (result1, result2) = play_recursive_game(player1.clone(), player2.clone());
    println!("Player 1's deck: {:?}\nPlayer 2's deck: {:?}", result1, result2);
    let winner = if result2.is_empty() { result1 } else { result2 };
    println!("Score: {}", score(&winner));
}

fn score(deck: &VecDeque<usize>) -> usize {
    deck.iter().rev().enumerate().map(|(idx, n)| (idx+1)*n).sum::<usize>()
}

fn play_game(mut player1: VecDeque<usize>, mut player2: VecDeque<usize>) -> (VecDeque<usize>, VecDeque<usize>) {
    while !player1.is_empty() && !player2.is_empty() {
        play_hand(&mut player1, &mut player2);
    }
    (player1, player2)
}

fn play_hand(player1: &mut VecDeque<usize>, player2: &mut VecDeque<usize>) {
    let card1 = player1.pop_front().unwrap();
    let card2 = player2.pop_front().unwrap();
    if card1 > card2 {
        player1.push_back(card1);
        player1.push_back(card2);
    } else if card2 > card1 {
        player2.push_back(card2);
        player2.push_back(card1);
    } else { panic!() }
}

fn play_recursive_game(mut player1: VecDeque<usize>, mut player2: VecDeque<usize>) -> (VecDeque<usize>, VecDeque<usize>) {
    let mut seen_states: HashSet<(Vec<usize>, Vec<usize>)> = HashSet::new();
    while !player1.is_empty() && !player2.is_empty() {
        if !seen_states.insert((player1.iter().copied().collect(), player2.iter().copied().collect())) {
            // player 1 wins if we see a game state again
            return (player1, VecDeque::new());
        }
        play_recursive_hand(&mut player1, &mut player2);
    }
    (player1, player2)
}

fn play_recursive_hand(player1: &mut VecDeque<usize>, player2: &mut VecDeque<usize>) {
    //println!("---{}---\nPlayer 1's deck: {:?}\nPlayer 2's deck: {:?}\n---", player1.len()+player2.len(), player1, player2);
    let card1 = player1.pop_front().unwrap();
    let card2 = player2.pop_front().unwrap();
    if card1 <= player1.len() && card2 <= player2.len() {
        let (result1, result2) =
            play_recursive_game(
                player1.iter().take(card1).copied().collect(),
                player2.iter().take(card2).copied().collect());
        if result2.is_empty() {
            player1.push_back(card1);
            player1.push_back(card2);
        } else if result1.is_empty() {
            player2.push_back(card2);
            player2.push_back(card1);
        } else { panic!() }
    } else {
        if card1 > card2 {
            player1.push_back(card1);
            player1.push_back(card2);
        } else if card2 > card1 {
            player2.push_back(card2);
            player2.push_back(card1);
        } else { panic!() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combat() {
        let player1: VecDeque<usize> = vec!(9,2,6,3,1).into_iter().collect();
        let player2: VecDeque<usize> = vec!(5,8,4,7,10).into_iter().collect();
        let (result1, result2) = play_game(player1, player2);
        assert!(result1.is_empty());
        assert_eq!(result2, [3, 2, 10, 6, 8, 5, 9, 4, 7, 1]);
        assert_eq!(score(&result2), 306);
    }

    #[test]
    fn recursive_combat() {
        let player1: VecDeque<usize> = vec!(9,2,6,3,1).into_iter().collect();
        let player2: VecDeque<usize> = vec!(5,8,4,7,10).into_iter().collect();
        let (result1, result2) = play_recursive_game(player1, player2);
        assert!(result1.is_empty());
        assert_eq!(result2, [7, 5, 6, 2, 4, 1, 10, 8, 9, 3]);
        assert_eq!(score(&result2), 291);
    }

    #[test]
    fn infinite_recursion() {
        let player1: VecDeque<usize> = vec!(43, 19).into_iter().collect();
        let player2: VecDeque<usize> = vec!(2, 29, 14).into_iter().collect();

        // Problem statement doesn't actually provide results for this example (and it's not really
        // interesting, player1 wins by default essentially) we really just care that it terminates.
        play_recursive_game(player1, player2);
    }
}
