// TODO: RTO-4
use rand::Rng;

struct SprintTeam {
    name: String,
    spaces_remaining: u8,
    team_color: String,
} 

fn main() {
    let students = [
        "Simon William Benjamin",
        "Julian A Brito-Hanley",
        "Nick Clarkson",
        "Sam Ethan Fortin",
        "Jack Allen Frambes",
        "Evan Thomas Gunnulfsen",
        "Jack Robert Hoeting",
        "Julia Lacy Hootman",
        "Calvin William LaHuis",
        "Adam Joseph Maida",
        "Brian Donovan Matton",
        "Katelyn Quach-Giang",
        "Gavin Ryan Reed",
        "Nicolas Frank Salvador",
        "Cory Davis Schaefer",
        "Carmen Atticus Shero",
        "Lance Stephen Silliman",
        "Kyle R Totorica",
        "Khanh Tran",
        "Alex K Welch",
    ];
  
    let sprint_team_a  = SprintTeam { 
        name: "Team A".to_string(), 
        spaces_remaining: 7,
        team_color: "Blue".to_string(),
    };

    let sprint_team_b = SprintTeam { 
        name: "Team B".to_string(), 
        spaces_remaining: 7,
        team_color: "Yellow".to_string(),
    };

    let sprint_team_c = SprintTeam { 
        name: "Team C".to_string(), 
        spaces_remaining: 6,
        team_color: "Pink".to_string(),
    };

    let mut teams = [sprint_team_a, sprint_team_b, sprint_team_c];

    let mut rng = rand::thread_rng();

    for student in students {
        loop {
            let random_number = rng.gen_range(0..teams.len());
            let team = &mut teams[random_number];
            if team.spaces_remaining > 0 {
                team.spaces_remaining -= 1;
                println!(
                    "Student: {:<25}  Team: {:<10}  Color: {:<6}  Remaining Spaces: {}",
                    student,
                    team.name,
                    team.team_color,
                    team.spaces_remaining
                );
                break;
            }
        }
    }
}
