pub mod player_data;

pub struct Situation {
    players: Vec<player_data::PlayerData>,
    tournament: TournamentData,
    teams: Vec<TeamData>,
}

impl Situation {
    pub fn new(
        players: Vec<player_data::PlayerData>,
        tournament: TournamentData,
        teams: Vec<TeamData>,
    ) -> Self {
        Self {
            players,
            tournament,
            teams,
        }
    }
}
