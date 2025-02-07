// TODO: RTO-4
use rand::Rng;

struct SprintTeam {
    name: String,
    spaces_remaining: u8,
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
        spaces_remaining: 5,
    };

    let sprint_team_b = SprintTeam { 
        name: "Team B".to_string(), 
        spaces_remaining: 5,
    };

    let sprint_team_c = SprintTeam { 
        name: "Team C".to_string(), 
        spaces_remaining: 5,
    };

    let sprint_team_d = SprintTeam {
        name: "Team D".to_string(),
        spaces_remaining: 5,
    };


    let mut teams = [sprint_team_a, sprint_team_b, sprint_team_c, sprint_team_d];


    let mut rng = rand::thread_rng();
    let mut count = 0;

    for student in students {
        loop {
            let random_number = rng.gen_range(0..teams.len());
            let team = &mut teams[random_number];
            if team.spaces_remaining > 0 {
                team.spaces_remaining -= 1;
                println!(
                    "Student: {:<25}  Team: {:<10}  Remaining Spaces: {}",
                    student,
                    team.name,
                    team.spaces_remaining
                );
                
                if team.spaces_remaining == 1{
                    println!("One Spot remaining on this team!")
                }

                count += 1;
                if count < students.len(){
                    println!("Next Student is...");
                }
 
                break;
                

            }
        }
    }
}
