use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct TeamStats {
    name: String,
    mp: i8,
    w: i8,
    l: i8,
    d: i8,
    p: i8
}

impl TeamStats {
    /// Creates a new TeamStats struct
    fn new(team_name: String) -> TeamStats {
        TeamStats { name: team_name, mp: 0, w: 0, l: 0, d: 0, p: 0 }
    }
}

pub fn tally(match_results: &str) -> String {
    let mut table = "Team                           | MP |  W |  D |  L |  P".to_string();
    let mut team_stats: HashMap<&str, TeamStats> = HashMap::new();

    if match_results.len() > 0 {
        let local_copy = match_results.lines();
        
        // typical input ===> "Allegoric Alaskans;Blithering Badgers;win"
        for line in local_copy {
            let res: Vec<&str> = line.split(";").collect();
            
            for team in &res[..2] {
                if !team_stats.contains_key(team) {
                    team_stats.entry(team).or_insert(TeamStats::new(team.to_string()));
                }
            }

            match res[2] {
                "win" => {
                    team_stats.entry(res[0]).and_modify(|v| v.w += 1);
                    team_stats.entry(res[1]).and_modify(|v| v.l += 1);

                    team_stats.entry(res[0]).and_modify(|v| v.p += 3);

                    team_stats.entry(res[0]).and_modify(|v| v.mp += 1);
                    team_stats.entry(res[1]).and_modify(|v| v.mp += 1);
                }
                "loss" => {
                    team_stats.entry(res[0]).and_modify(|v| v.l += 1);
                    team_stats.entry(res[1]).and_modify(|v| v.w += 1);

                    team_stats.entry(res[1]).and_modify(|v| v.p += 3);

                    team_stats.entry(res[0]).and_modify(|v| v.mp += 1);
                    team_stats.entry(res[1]).and_modify(|v| v.mp += 1);
                }
                "draw" => {
                    team_stats.entry(res[0]).and_modify(|v| v.d += 1);
                    team_stats.entry(res[1]).and_modify(|v| v.d += 1);

                    team_stats.entry(res[0]).and_modify(|v| v.p += 1);
                    team_stats.entry(res[1]).and_modify(|v| v.p += 1);

                    team_stats.entry(res[0]).and_modify(|v| v.mp += 1);
                    team_stats.entry(res[1]).and_modify(|v| v.mp += 1);
                }
                _ => continue
            }
        }


        let mut unsorted: Vec<&TeamStats> = team_stats.values().collect();
        unsorted.sort_by(|a, b| if a.p != b.p {
            b.p.cmp(&a.p)
        } else {
            a.name.cmp(&b.name)
        });

        for item in unsorted.iter() {
            let string = format!("\n{:<31}|  {} |  {} |  {} |  {} |  {}", item.name, item.mp, item.w, item.d, item.l, item.p);
            table.push_str(&string);
        }
    }
    
    table
}