use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Clone, Debug)]
struct Combat {
    player1: Vec<usize>,
    player2: Vec<usize>,
    played_states: HashSet<String>,
    rounds: usize,
    winner: Option<usize>,
}
impl Combat {
    fn tick(&mut self) {
        if self.player1.len() == 0 {
            self.winner = Some(2);
            return;
        } else if self.player2.len() == 0 {
            self.winner = Some(1);
            return;
        }

        self.rounds += 1;
        self.played_states.insert(self.to_string());

        let card1 = self.player1.remove(0);
        let card2 = self.player2.remove(0);

        if card1 > card2 {
            self.player1.push(card1);
            self.player1.push(card2);
        } else if card2 > card1 {
            self.player2.push(card2);
            self.player2.push(card1);
        }
    }

    fn play(&mut self) {
        while self.winner == None {
            self.tick();
        }
    }

    fn tick2(&mut self) {
        if self.player1.len() == 0 {
            self.winner = Some(2);
            return;
        } else if self.player2.len() == 0 {
            self.winner = Some(1);
            return;
        }

        let state = self.to_string();
        if self.played_states.contains(&state) {
            self.winner = Some(1);
            return;
        }
        self.played_states.insert(state);

        self.rounds += 1;

        let card1 = self.player1.remove(0);
        let card2 = self.player2.remove(0);

        if card1 <= self.player1.len() && card2 <= self.player2.len() {
            let mut recursion = self.clone();
            recursion.player1.truncate(card1);
            recursion.player2.truncate(card2);
            recursion.played_states = HashSet::new();
            recursion.play2();
            match recursion.winner {
                Some(1) => {
                    self.player1.push(card1);
                    self.player1.push(card2);
                }
                _some2 => {
                    self.player2.push(card2);
                    self.player2.push(card1);
                }
            }
        } else if card1 > card2 {
            self.player1.push(card1);
            self.player1.push(card2);
        } else if card2 > card1 {
            self.player2.push(card2);
            self.player2.push(card1);
        }
    }

    fn play2(&mut self) {
        while self.winner == None {
            self.tick2();
        }
    }

    fn score(&self) -> usize {
        let cards = match self.winner {
            Some(1) => &self.player1,
            _some2 => &self.player2,
        };
        let mut i: usize = 0;
        let mut score: usize = 0;
        for card in cards {
            score += (cards.len() - i) * card;
            i += 1;
        }
        score
    }

    fn to_string(&self) -> String {
        let mut s = String::new();

        s.push_str("Player 1: ");
        let cards1: String = self
            .player1
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        s.push_str(&cards1);

        s.push_str("\nPlayer 2: ");
        let cards2: String = self
            .player2
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        s.push_str(&cards2);

        s
    }
}

pub fn run(filename: &str) {
    let player_cards: Vec<String> = read_to_string(filename)
        .unwrap()
        .trim()
        .split("\n\n")
        .map(String::from)
        .collect();
    let player1: Vec<usize> = player_cards[0]
        .lines()
        .map(String::from)
        .collect::<Vec<String>>()[1..]
        .iter()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let player2: Vec<usize> = player_cards[1]
        .lines()
        .map(String::from)
        .collect::<Vec<String>>()[1..]
        .iter()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let mut combat = Combat {
        player1,
        player2,
        played_states: HashSet::new(),
        rounds: 0,
        winner: None,
    };
    let mut combat2 = combat.clone();

    combat.play();
    println!("{}", combat.score());

    // part 2

    combat2.play2();
    println!("{}", combat2.score());
}
