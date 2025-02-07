//TODO: RTO-4
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

    let mut teams: Vec<SprintTeam> = Vec::new();
    teams.push(SprintTeam {name: "Team A".to_string(), spaces_remaining: 7});
    teams.push(SprintTeam {name: "Team B".to_string(), spaces_remaining: 7});
    teams.push(SprintTeam {name: "Team C".to_string(), spaces_remaining: 6});
    teams.sort_by_key(|x| x.spaces_remaining);
    teams.reverse();

    for student in students {
        loop {
            let team = &mut teams[0];
            team.spaces_remaining -= 1;
            println!(
                "Student: {:<25}  Team: {:<10}  Remaining Spaces: {}",
                student,
                team.name,
                team.spaces_remaining
            );
            teams.sort_by_key(|x| x.spaces_remaining);
            teams.reverse();
            break;
        }
    }
}