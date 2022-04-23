pub trait TennisGame {
    fn clear(&mut self);
    fn won_point(&mut self, player_name: &str);
    fn get_score(&self) -> String;
    fn play(&mut self, p1: u8, p2: u8);
}

#[derive(Default)]
pub struct TennisGameScoreKeeper {
    score1: u8,
    score2: u8,
    _player1_name: String,
    _player2_name: String,
}
impl TennisGameScoreKeeper {
    pub fn new() -> Self {
        TennisGameScoreKeeper::default()
    }

    fn same_score(score: u8) -> String {
        match score {
            0 => return "Love-All".to_owned(),
            1 => return "Fifteen-All".to_owned(),
            2 => return "Thirty-All".to_owned(),
            _ => return "Deuce".to_owned(),
        }
    }

    fn player_advantage(score1: u8, score2: u8) -> String {
        let minus_result = score1 as i8 - score2 as i8;
        if minus_result == 1 {
            return "Advantage player1".to_owned();
        } else if minus_result == -1i8 {
            return "Advantage player2".to_owned();
        } else if minus_result >= 2 {
            return "Win for player1".to_owned();
        }
        "Win for player2".to_owned()
    }
}
impl TennisGame for TennisGameScoreKeeper {
    fn clear(&mut self) {
        self.score1 = 0;
        self.score2 = 0;
    }
    fn won_point(&mut self, player_name: &str) {
        if player_name == "player1" {
            self.score1 += 1;
        } else {
            self.score2 += 1;
        }
    }
    fn get_score(&self) -> String {
        match (self.score1, self.score2) {
            (a, b) if a == b => Self::same_score(a),
            (a, b) if a >= 4 || b >= 4 => Self::player_advantage(self.score1, self.score2),
            _ => {
                let mut temp_score: u8;
                let mut score = String::new();
                for i in 1..3 {
                    if i == 1 {
                        temp_score = self.score1;
                    } else {
                        score.push_str("-");
                        temp_score = self.score2;
                    }
                    match temp_score {
                        0 => score.push_str("Love"),
                        1 => score.push_str("Fifteen"),
                        2 => score.push_str("Thirty"),
                        3 => score.push_str("Forty"),
                        _ => {}
                    }
                }
                return score;
            }
        }
    }

    fn play(&mut self, p1: u8, p2: u8) {
        let highest_score = u8::max(p1, p2);
        for i in 0..highest_score {
            if i < p1 {
                self.won_point("player1")
            }
            if i < p2 {
                self.won_point("player2")
            }
        }
    }
}

