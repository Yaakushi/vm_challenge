#[derive(Clone)]
struct Coin {
    name: String,
    value: i32,
}

fn find_permutations(set: &[Coin]) -> Vec<Vec<Coin>> {
    if set.is_empty() {
        return vec![vec![]];
    }

    let mut permutations = Vec::new();

    for (selected_index, coin) in set.iter().enumerate() {
        let mut new_set = set.to_vec();
        new_set.remove(selected_index);

        let subpermutations = find_permutations(&new_set);

        for mut permutation in subpermutations {
            permutation.insert(0, coin.clone());
            permutations.push(permutation);
        }
    }

    permutations
}

fn main() {
    let coins = vec![
        Coin {
            name: "red".to_owned(),
            value: 2,
        },
        Coin {
            name: "corroded".to_owned(),
            value: 3,
        },
        Coin {
            name: "blue".to_owned(),
            value: 9,
        },
        Coin {
            name: "shiny".to_owned(),
            value: 5,
        },
        Coin {
            name: "concave".to_owned(),
            value: 7,
        },
    ];

    let all_permutations = find_permutations(&coins);

    for perm in all_permutations {
        if (perm[0].value
            + perm[1].value * perm[2].value * perm[2].value
            + perm[3].value * perm[3].value * perm[3].value
            - perm[4].value)
            == 399
        {
            println!(
                "{} - {} - {} - {} - {}",
                perm[0].name, perm[1].name, perm[2].name, perm[3].name, perm[4].name
            );

            return;
        }
    }
}
