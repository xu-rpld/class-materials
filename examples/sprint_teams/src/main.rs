// TODO: Assignment 4 Gavin Reed
use rand::Rng;

struct SprintTeam {
    name: String,
    spaces_remaining: u8,
    members: Vec<String>,
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
    let mut teams = vec![
        SprintTeam {
            name: "Team A".to_string(),
            spaces_remaining: 7,
            members: Vec::new(),
        },
        SprintTeam {
            name: "Team B".to_string(),
            spaces_remaining: 7,
            members: Vec::new(),
        },
        SprintTeam {
            name: "Team C".to_string(),
            spaces_remaining: 7,
            members: Vec::new(),
        },
    ];
    

    let mut rng = rand::thread_rng();

    for student in students.iter() {
        loop {
            let random_number = rng.gen_range(0..teams.len());
            let team = &mut teams[random_number];
            if team.spaces_remaining > 0 {
                team.spaces_remaining -= 1;
                team.members.push(student.to_string());
                break;
            }
        }
    }

    for team in &teams {
        println!("{}:", team.name);
        for member in &team.members {
            println!("  - {}", member);
        }
        println!();
    }
}
