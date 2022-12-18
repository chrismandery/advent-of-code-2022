use anyhow::{Context, Result};
use regex::Regex;
use std::collections::{BTreeSet, HashMap};
use std::fs::read_to_string;
use std::path::Path;

struct Valve {
    flow_rate: u32,
    tunnels_to: Vec<String>
}

type ValveSet = HashMap<String, Valve>;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Position {
    pos_self: String,
    pos_elephant: Option<String>
}

struct CurrentState {
    opened_valves: BTreeSet<String>,
    achieved_pressure_release: u32
}

/// Maps states to already released pressure: At each time for each puzzle state, we only have to keep the best path that is associated
/// with the maximum achieved pressure release.
type PuzzleState = HashMap<Position, Vec<CurrentState>>;

fn calc_max_releasable_pressure(valves: &ValveSet, start_pos: &str, minutes: u32, include_elephant: bool) -> u32 {
    // Initialize structure to track all states that can be reached in each iteration
    let mut states = PuzzleState::new();
    states.insert(
        Position {
            pos_self: start_pos.into(),
            pos_elephant: include_elephant.then_some(start_pos.into())
        },
        vec!(
            CurrentState {
                opened_valves: BTreeSet::new(),
                achieved_pressure_release: 0
            }
        )
    );

    let mut max_achieavable_flow = 0;

    for minute in 0..minutes {
        let total_states_count: usize = states.values().map(|s| s.len()).sum();
        println!("{} minute(s) have passed: Tracking {} states for {} positions. Maximum pressure release is {}.",
            minute, total_states_count, states.len(), max_achieavable_flow);

        // Loop over all current positions and states and generates states reachable from there in the next minute
        let mut next_states = PuzzleState::new();

        for (cur_pos, states_list) in states {
            for state in states_list {
                let pressure_released_this_minute: u32 = state.opened_valves
                    .iter()
                    .map(|v| valves.get(v).unwrap().flow_rate)
                    .sum();
                let total_pressure_released = state.achieved_pressure_release + pressure_released_this_minute;

                // Option 1: Open valve at current position
                if !state.opened_valves.contains(&cur_pos.pos_self) {
                    let mut opened_valves = state.opened_valves.clone();
                    opened_valves.insert(cur_pos.pos_self.clone());

                    track_successor_state(&mut next_states, cur_pos.clone(), opened_valves, total_pressure_released);
                }

                // Option 2: Move to another location
                for target in &valves.get(&cur_pos.pos_self).unwrap().tunnels_to {
                    let new_pos = Position { pos_self: target.clone(), pos_elephant: None };  // TODO
                    track_successor_state(&mut next_states, new_pos, state.opened_valves.clone(), total_pressure_released);
                }
            }
        }

        // Update states and calculate current best maximum achievable flow
        states = next_states;
        max_achieavable_flow = states.values().map(|s| s.iter().map(|t| t.achieved_pressure_release).max().unwrap()).max().unwrap();

        // Starting in minute 10, we are using a heuristics to prune "bad" states to reduce the computational effort. We are pruning all
        // states that have considerably less than the current best achievable flow. (This might make us miss the true solution though. In
        // case that happens, this part has to be adjusted.)
        if minute > 10 {
            let prune_threshold = max_achieavable_flow - 100;
            for states_list in states.values_mut() {
                states_list.retain(|s| s.achieved_pressure_release >= prune_threshold);
            }
        }
    }

    // Return maximum achievable flow from best path
    max_achieavable_flow
}

fn main() -> Result<()> {
    let valves = read_input_file("../inputs/day16_input.txt")?;
    println!("Maximum releasable pressure in 30 minutes: {}", calc_max_releasable_pressure(&valves, "AA", 30, false));

    Ok(())
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<ValveSet> {
    let re = Regex::new(r"^Valve (?P<valve_code>[A-Z]{2}) has flow rate=(?P<flow_rate>\d+); tunnels? leads? to valves? (?P<tunnels_to>[A-Z ,]+)$")?;

    let input = read_to_string(input_path).context("Could not read input file!")?;
    let res = input
        .lines()
        .map(|l| {
            let caps = re.captures(l).expect("Could not parse input line!");

            let valve_code = caps.name("valve_code").unwrap().as_str().to_string();
            let flow_rate = caps
                .name("flow_rate")
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
            let tunnels_to = caps
                .name("tunnels_to")
                .unwrap()
                .as_str()
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();

            (valve_code, Valve { flow_rate, tunnels_to })
        })
        .collect();

    Ok(res)
}

fn track_successor_state(states: &mut PuzzleState, cur_pos: Position, opened_valves: BTreeSet<String>, achieved_pressure_release: u32) {
    if let Some(existing_states) = states.get_mut(&cur_pos) {
        // Check if we already have a state that is strictly better (then we do not need to save this one)
        for existing_state in existing_states.iter() {
            if existing_state.opened_valves.is_superset(&opened_valves) &&
                existing_state.achieved_pressure_release >= achieved_pressure_release {
                return;
            }
        }

        // Delete existing states that are strictly inferior and can be deleted
        existing_states.retain(|possibly_delete_state| {
            !opened_valves.is_superset(&possibly_delete_state.opened_valves) ||
                achieved_pressure_release < possibly_delete_state.achieved_pressure_release
        });

        // Add new state
        existing_states.push(CurrentState {
            opened_valves,
            achieved_pressure_release
        })
    } else {
        states.insert(cur_pos, vec!(CurrentState {
            opened_valves,
            achieved_pressure_release
        }));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let valves = read_input_file("../inputs/day16_example.txt").unwrap();
        assert_eq!(calc_max_releasable_pressure(&valves, "AA", 30, false), 1651);
    }
}
