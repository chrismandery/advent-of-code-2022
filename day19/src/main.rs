use anyhow::{Context, Result};
use regex::Regex;
use std::cmp::max;
use std::fs::read_to_string;
use std::ops::{Add, Sub};
use std::path::Path;

#[derive(Clone, Debug)]
struct ResourceList {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32
}

impl Add for &ResourceList {
    type Output = ResourceList;

    fn add(self, other: Self) -> ResourceList {
        ResourceList {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
            geode: self.geode + other.geode,
        }
    }
}

impl Sub for &ResourceList {
    type Output = ResourceList;

    fn sub(self, other: Self) -> ResourceList {
        ResourceList {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.geode - other.geode,
        }
    }
}

impl ResourceList {
    fn contains(&self, other: &Self) -> bool {
        self.ore >= other.ore && self.clay >= other.clay && self.obsidian >= other.obsidian && self.geode >= other.geode
    }
}

#[derive(Debug)]
struct Blueprint {
    ore_robot_cost: ResourceList,
    clay_robot_cost: ResourceList,
    obsidian_robot_cost: ResourceList,
    geode_robot_cost: ResourceList,
}

fn get_max_geode_count(bp: &Blueprint, time_minutes: u8) -> i32 {
    let mut global_max_seen_final_geode_count = 0;  // Used internally for pruning

    // We are limiting the number of robots based on the blueprint: It does not make sense to have more robots of a specific kind (except
    // geode robots) than are necessary to produce the maximum that can be spend by any robot construction recipe.
    let robot_limits_for_blueprint = ResourceList {
        ore: max(max(bp.ore_robot_cost.ore, bp.clay_robot_cost.ore),
            max(bp.obsidian_robot_cost.ore, bp.geode_robot_cost.ore)),
        clay: max(max(bp.ore_robot_cost.clay, bp.clay_robot_cost.clay),
            max(bp.obsidian_robot_cost.clay, bp.geode_robot_cost.clay)),
        obsidian: max(max(bp.ore_robot_cost.obsidian, bp.clay_robot_cost.obsidian),
            max(bp.obsidian_robot_cost.obsidian, bp.geode_robot_cost.obsidian)),
        geode: 0  // not used
    };

    get_max_geode_count_recurse(
        bp,
        &robot_limits_for_blueprint,
        time_minutes,
        ResourceList {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0
        },
        ResourceList {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0
        },
        &mut global_max_seen_final_geode_count
    )
}

/// This function recurses into possible option for this turn, but also includes some pruning to make the problem tractable. Regarding the
/// robot limits, see comment in get_max_geode_count().
fn get_max_geode_count_recurse(bp: &Blueprint, robot_limits_for_blueprint: &ResourceList, minutes_left: u8, cur_resources: ResourceList,
    cur_robots: ResourceList, global_max_seen_final_geode_count: &mut i32) -> i32 {
    if minutes_left == 0 {
        cur_resources.geode
    } else {
        let next_resources = &cur_resources + &cur_robots;

        let mut best = -1;

        // Calculate how many final geodes this state will generate if we would be building no more geode robots or if we would be building
        // only geode robots from now on (which is the most optimistic possible case to maximize the final geode number)
        let minutes_left_m1 = minutes_left as i32 - 1;
        let this_state_final_geode_count_most_pessimistic =
            cur_resources.geode +  // geodes that are already there
            minutes_left as i32 * cur_robots.geode;  // geodes produces by the robots that are already there
        let this_state_final_geode_count_most_optimistic =
            this_state_final_geode_count_most_pessimistic +
            ((minutes_left_m1 * minutes_left_m1) + minutes_left_m1) / 2;  // geodes produces by new robots, sum formula with N = (minutes_left - 1)

        if this_state_final_geode_count_most_optimistic < *global_max_seen_final_geode_count {
            // This state can no longer help us reach a new maximum
            return -1;
        } else if this_state_final_geode_count_most_pessimistic > *global_max_seen_final_geode_count {
            *global_max_seen_final_geode_count = this_state_final_geode_count_most_pessimistic
        };
        
        // Build geode robot (there is no limit for geode robots)
        if cur_resources.contains(&bp.geode_robot_cost) {
            best = max(best, get_max_geode_count_recurse(
                bp,
                robot_limits_for_blueprint,
                minutes_left - 1,
                &next_resources - &bp.geode_robot_cost,
                &cur_robots + &ResourceList { ore: 0, clay: 0, obsidian: 0, geode: 1 },
                global_max_seen_final_geode_count
            ));
        }

        // Build obsidian robot
        if cur_robots.obsidian < robot_limits_for_blueprint.obsidian && cur_resources.contains(&bp.obsidian_robot_cost) {
            best = max(best, get_max_geode_count_recurse(
                bp,
                robot_limits_for_blueprint,
                minutes_left - 1,
                &next_resources - &bp.obsidian_robot_cost,
                &cur_robots + &ResourceList { ore: 0, clay: 0, obsidian: 1, geode: 0 },
                global_max_seen_final_geode_count
            ));
        }

        // Build clay robot
        if cur_robots.clay < robot_limits_for_blueprint.clay && cur_resources.contains(&bp.clay_robot_cost) {
            best = max(best, get_max_geode_count_recurse(
                bp,
                robot_limits_for_blueprint,
                minutes_left - 1,
                &next_resources - &bp.clay_robot_cost,
                &cur_robots + &ResourceList { ore: 0, clay: 1, obsidian: 0, geode: 0 },
                global_max_seen_final_geode_count
            ));
        }

        // Build ore robot
        if cur_robots.ore < robot_limits_for_blueprint.ore && cur_resources.contains(&bp.ore_robot_cost) {
            best = max(best, get_max_geode_count_recurse(
                bp,
                robot_limits_for_blueprint,
                minutes_left - 1,
                &next_resources - &bp.ore_robot_cost,
                &cur_robots + &ResourceList { ore: 1, clay: 0, obsidian: 0, geode: 0 },
                global_max_seen_final_geode_count
            ));
        }

        // Do nothing and wait
        best = max(best, get_max_geode_count_recurse(
            bp,
            robot_limits_for_blueprint,
            minutes_left - 1,
            &cur_resources + &cur_robots,
            cur_robots,
            global_max_seen_final_geode_count
        ));

        best
    }
}

fn get_quality_level_sum(bps: &[Blueprint], time_minutes: u8) -> i32 {
    bps.iter().enumerate().map(|(i, bp)| {
        println!("Running blueprint {}...", (i + 1));
        (i + 1) as i32 * get_max_geode_count(bp, time_minutes)
    }).sum()
}

fn main() -> Result<()> {
    let blueprints = read_input_file("../inputs/day19_input.txt")?;

    println!("First part - Sum of quality levels: {}", get_quality_level_sum(&blueprints, 24));

    let geode_bp1 = get_max_geode_count(&blueprints[0], 32);
    println!("Second part - Blueprint 1: {}", geode_bp1);

    let geode_bp2 = get_max_geode_count(&blueprints[1], 32);
    println!("Second part - Blueprint 2: {}", geode_bp2);

    let geode_bp3 = get_max_geode_count(&blueprints[2], 32);
    println!("Second part - Blueprint 3: {}", geode_bp3);

    println!("Second part - Product is: {}", geode_bp1 * geode_bp2 * geode_bp3);

    Ok(())
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<Vec<Blueprint>> {
    let re = Regex::new(r"^Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$")?;

    let input = read_to_string(input_path).context("Could not read input file!")?;
    let res = input
        .lines()
        .enumerate()
        .map(|(num, l)| {
            let caps = re.captures(l).expect("Could not parse input line!");
            let numbers: Vec<i32> = (1..=7).map(|i| caps
                .get(i)
                .unwrap()
                .as_str()
                .parse()
                .expect("Could not parse number!")
            ).collect();

            if numbers[0] as usize != (num + 1) {
                panic!("Unexpected number of blueprint!");
            }

            Blueprint {
                ore_robot_cost: ResourceList { ore: numbers[1], clay: 0, obsidian: 0, geode: 0 },
                clay_robot_cost: ResourceList { ore: numbers[2], clay: 0, obsidian: 0, geode: 0 },
                obsidian_robot_cost: ResourceList { ore: numbers[3], clay: numbers[4], obsidian: 0, geode: 0 },
                geode_robot_cost: ResourceList { ore: numbers[5], clay: 0, obsidian: numbers[6], geode: 0 },
            }
        })
        .collect();

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let blueprints = read_input_file("../inputs/day19_example.txt").unwrap();
        assert_eq!(get_max_geode_count(&blueprints[0], 24), 9);
        assert_eq!(get_max_geode_count(&blueprints[1], 24), 12);
        assert_eq!(get_quality_level_sum(&blueprints, 24), 33);
    }

    #[test]
    fn example_part2() {
        let blueprints = read_input_file("../inputs/day19_example.txt").unwrap();
        assert_eq!(get_max_geode_count(&blueprints[0], 32), 56);
        assert_eq!(get_max_geode_count(&blueprints[1], 32), 62);
    }
}
