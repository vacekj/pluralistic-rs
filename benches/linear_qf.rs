#[path = "../src/lib.rs"]
mod lib;

use crate::lib::{calculate_linear_qf, Contribution, LinearQfOptions, MatchingCapStrategy};
use criterion::{criterion_group, criterion_main, Criterion};
use serde::Deserialize;
use std::fs;
use std::str::FromStr;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Vote {
    amount_round_token: String,
    grant_address: String,
    voter: String,
}

impl From<Vote> for Contribution {
    fn from(vote: Vote) -> Contribution {
        Contribution {
            sender: vote.voter,
            amount: f64::from_str(&vote.amount_round_token).unwrap(),
            recipient: vote.grant_address,
        }
    }
}

fn my_benchmark_function(c: &mut Criterion) {
    let data = fs::read_to_string(format!(
        "{}/benches/votes.json",
        std::env::current_dir().unwrap().display()
    ))
    .expect("Unable to read votes file");

    let votes: Vec<Vote> = serde_json::from_str(&data).expect("JSON does not have correct format.");
    println!("total votes: {}", votes.len());
    let contributions: Vec<Contribution> = votes.into_iter().map(Contribution::from).collect();

    c.bench_function("linear_qf", |b| {
        b.iter(|| {
            calculate_linear_qf(
                contributions.clone(),
                350000000000000000000000.0,
                LinearQfOptions {
                    matching_cap_strategy: MatchingCapStrategy::Cap,
                    matching_cap_amount: Some(14000000000000000000000.0),
                },
            )
        });
    });
}

criterion_group!(benches, my_benchmark_function);
criterion_main!(benches);
