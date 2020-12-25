pub fn advent() {
    let (card_key, door_key) = (3248366, 4738476);
    let card_loop = find_loop_size(card_key);
    let door_loop = find_loop_size(door_key);
    println!("Card Loop: {}, Door Loop: {}", card_loop, door_loop);
    let key = generate_key(card_loop, door_key);
    assert_eq!(key, generate_key(door_loop, card_key));
    println!("Encryption Key: {}", key);

}

fn find_loop_size(public_key: i64) -> i64 {
    let subject = 7;
    let mut result = 1;
    for i in 1.. {
        result = (subject * result) % 20201227;
        if result == public_key {
            return i;
        }
    }
    unreachable!();
}

fn generate_key(loop_size: i64, subject: i64) -> i64 {
    let mut result = 1;
    for _ in 0..loop_size {
        result = (subject * result) % 20201227;
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let (card_key, door_key) = (5764801, 17807724);
        let card_loop = find_loop_size(card_key);
        let door_loop = find_loop_size(door_key);
        assert_eq!(card_loop, 8);
        assert_eq!(door_loop, 11);
        assert_eq!(generate_key(card_loop, door_key), 14897079);
        assert_eq!(generate_key(door_loop, card_key), 14897079);
    }
}
