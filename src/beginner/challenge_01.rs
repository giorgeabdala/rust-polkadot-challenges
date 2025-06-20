#![allow(dead_code)]
#![allow(unused_imports)]
use std::fmt;
use std::fmt::Formatter;

pub struct Player {
    name: String,
    score: u32
}


impl Player {
    fn new(name: String) -> Self {
        Player {name, score: 0 }
    }

    fn add_score(&mut self, points: u32) {
        self.score = self.score.saturating_add(points);
    }

    fn get_info(&self) -> String {
        format!("{}: {} points", self.name, self.score)
    }

    fn reset_score(&mut self) {
        self.score = 0;
    }

}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Player(Display): {} - Points: {}", self.name, self.score)
    }
}

fn transfer_player(player: Player) -> Player {
    player
}

fn clone_player(player: &Player) -> Player {
    let mut new_player = Player::new(player.name.clone());
    new_player.score = player.score;
    new_player
}

mod tests {
    use crate::beginner::challenge_01::{clone_player, transfer_player, Player};

    #[test]
    fn new_player_test(){
        let name = "Alice".to_string();
        let player = Player::new(name.clone());
        assert_eq!(name, player.name);
        assert_eq!(0, player.score);
    }
    #[test]
    fn add_score_test() {
        let name = "Alice".to_string();
        let mut player = Player::new(name.clone());
        player.add_score(10);
        assert_eq!(10, player.score);
    }

    #[test]
    fn get_info_test() {
        let name = "Alice".to_string();
        let player = Player::new(name.clone());
        let info = player.get_info();
        assert_eq!(info, "Alice: 0 points");
    }

    #[test]
    fn transfer_player_test() {
        let name = "Alice".to_string();
        let player = Player::new(name.clone());
        let transferred_player = transfer_player(player);
        assert_eq!(transferred_player.name, name);
        assert_eq!(transferred_player.score, 0);

        // error[E0382]: borrow of moved value: `player`
        // println!("Nome do player original: {}", player.name);
    }
    #[test]
    fn clone_player_test() {
        let name = "Alice".to_string();
        let player = Player::new(name.clone());
        let cloned_player = clone_player(&player);
        assert_eq!(cloned_player.name, name);
        assert_eq!(cloned_player.score, 0);

        assert_eq!(player.name, name);
        assert_eq!(player.score, 0);
    }

    #[test]
    fn reset_score_test() {
        let name = "Alice".to_string();
        let mut player = Player::new(name.clone());
        player.add_score(10);
        assert_eq!(10, player.score);
        player.reset_score();
        assert_eq!(0, player.score);
    }



}
