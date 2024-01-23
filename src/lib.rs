#![feature(slice_flatten)]

use rand::random;
use rand::seq::SliceRandom;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Contribution {
    pub recipient: String,
    pub amount: f64,
    pub sender: String,
}

trait Random {
    fn rnd() -> Self;
}

impl Random for Contribution {
    fn rnd() -> Self {
        let names = ["Alice", "Bob", "Thomas", "Ben", "Jason", "Mary"];
        let rand_names = names
            .choose_multiple(&mut rand::thread_rng(), 2)
            .collect::<Vec<&&str>>();
        let amount: f64 = random();
        Contribution {
            sender: format!("0x{}", rand_names[0]),
            recipient: format!("0x{}", rand_names[1]),
            amount,
        }
    }
}

#[derive(Debug)]
pub struct Matcha {
    recipient: String,
    matcha: f64,
}

#[derive(Default)]
pub enum MatchingCapStrategy {
    #[default]
    Cap,
    Redistribute,
}

#[derive(Default)]
pub struct LinearQfOptions {
    pub matching_cap_amount: Option<f64>,
    pub matching_cap_strategy: MatchingCapStrategy,
    pub upscale: bool,
}

pub fn calculate_linear_qf(
    contributions: Vec<Contribution>,
    matching_pot: f64,
    options: LinearQfOptions,
) -> Vec<Matcha> {
    let mut total_match = 0f64;
    let mut has_saturated = false;
    let mut contributions_by_recipient: HashMap<String, HashMap<String, Contribution>> =
        HashMap::new();

    let mut distributions: Vec<Matcha> = vec![];

    // pivot the contributions by recipient
    for contribution in contributions {
        if !contributions_by_recipient.contains_key(&contribution.recipient) {
            contributions_by_recipient.insert(contribution.recipient.clone(), HashMap::new());
        }

        if let Some(existing_contribution_map) =
            contributions_by_recipient.get_mut(&contribution.recipient)
        {
            if let Some(existing_contribution) =
                existing_contribution_map.get_mut(&contribution.sender)
            {
                existing_contribution.amount += contribution.amount;
            } else {
                existing_contribution_map.insert(contribution.sender.clone(), contribution);
            }
        }
    }

    for details in contributions_by_recipient {
        let mut sum_of_sqrt_contrib = 0f64;
        let mut sum_of_contrib = 0f64;
        for contribution in details.1.values() {
            sum_of_sqrt_contrib += contribution.amount.powf(0.5);
            sum_of_contrib += contribution.amount;
        }

        let matcha = sum_of_sqrt_contrib.powf(2f64) - sum_of_contrib;

        distributions.push(Matcha {
            recipient: details.0,
            matcha,
        });

        total_match += matcha;
    }

    if total_match > matching_pot {
        has_saturated = true;
    }

    if has_saturated {
        for distribution in &mut distributions {
            distribution.matcha = (distribution.matcha * matching_pot) / total_match;
        }
    }

    if options.upscale && total_match < matching_pot {
        let upscale_factor = matching_pot / total_match;
        for distribution in &mut distributions {
            distribution.matcha *= upscale_factor;
        }
    }

    // Apply matching cap and redistribute if necessary
    if let Some(cap) = options.matching_cap_amount {
        let mut overflow_total = 0f64;
        let mut eligible_for_redistribution = 0usize;

        // First pass to apply cap and calculate overflow
        for matcha in &mut distributions {
            if matcha.matcha > cap {
                overflow_total += matcha.matcha - cap;
                matcha.matcha = cap;
            } else {
                eligible_for_redistribution += 1;
            }
        }

        // Redistribution logic
        if matches!(
            options.matching_cap_strategy,
            MatchingCapStrategy::Redistribute
        ) && overflow_total > 0f64
        {
            let redistribution_amount = overflow_total / eligible_for_redistribution as f64;
            for matcha in &mut distributions {
                if matcha.matcha < cap {
                    matcha.matcha += redistribution_amount;
                    // Ensure not to exceed the cap after redistribution
                    if matcha.matcha > cap {
                        matcha.matcha = cap;
                    }
                }
            }
        }
    }

    distributions
}

#[cfg(test)]
mod tests {
    use super::*;
    use arr_macro::arr;
    use rand::*;

    fn generate_contributions() -> Vec<Contribution> {
        arr![Contribution::rnd(); 10].to_vec()
    }

    #[test]
    fn test_upscale_to_pot() {
        let contributions = arr![Contribution::rnd(); 50].to_vec();
        let matching_pot = 1000.0;
        let options = LinearQfOptions {
            upscale: true,
            ..Default::default()
        };

        let distributions = calculate_linear_qf(contributions, matching_pot, options);

        let total_distributed: f64 = distributions.iter().map(|d| d.matcha).sum();
        assert!(total_distributed <= matching_pot);
        assert!(total_distributed >= matching_pot * 0.99); // Allow a small margin for rounding errors
    }

    #[test]
    fn test_add() {
        let mut rng = rand::thread_rng();

        let a_contribs = arr![Contribution {
                recipient: "A".into(),
                amount: 200f64,
                sender: rng.gen::<char>().into(),
            }; 5]
        .to_vec();
        let b_contribs = arr![Contribution {
                recipient: "B".into(),
                amount: 500f64,
                sender: rng.gen::<char>().into(),
            }; 2]
        .to_vec();
        let c_contribs = arr![Contribution {
                recipient: "C".into(),
                amount: 50f64,
                sender: rng.gen::<char>().into(),
            }; 20]
        .to_vec();
        let contributions = vec![a_contribs, b_contribs, c_contribs]
            .into_iter()
            .flatten()
            .collect::<Vec<Contribution>>();
        calculate_linear_qf(contributions, 10_000f64, LinearQfOptions::default());
    }

    #[test]
    fn test_redistribution_strategy() {
        let mut rng = rand::thread_rng();

        let a_contribs = arr![Contribution {
                recipient: "A".into(),
                amount: 200f64,
                sender: rng.gen::<char>().into(),
            }; 5]
        .to_vec();
        let b_contribs = arr![Contribution {
                recipient: "B".into(),
                amount: 500f64,
                sender: rng.gen::<char>().into(),
            }; 2]
        .to_vec();
        let c_contribs = arr![Contribution {
                recipient: "C".into(),
                amount: 50f64,
                sender: rng.gen::<char>().into(),
            }; 20]
        .to_vec();
        let contributions = vec![a_contribs, b_contribs, c_contribs]
            .into_iter()
            .flatten()
            .collect::<Vec<Contribution>>();

        let matching_pot = 10000.0;
        let cap = 10.0; // Set a cap that would force redistribution

        let options = LinearQfOptions {
            matching_cap_amount: Some(cap),
            matching_cap_strategy: MatchingCapStrategy::Redistribute,
            upscale: false, // Upscaling not relevant for this test
        };

        let distributions = calculate_linear_qf(contributions, matching_pot, options);

        // Verify that none of the distributions exceed the cap
        assert!(
            distributions.iter().all(|d| d.matcha <= cap),
            "All distributions must be within the cap."
        );

        // Calculate total distributed amount to ensure it's within the matching pot
        let total_distributed: f64 = distributions.iter().map(|d| d.matcha).sum();

        // The total distributed amount should be less than or equal to the matching pot
        assert!(
            total_distributed <= matching_pot,
            "Total distributed amount must not exceed the matching pot."
        );
    }
}
