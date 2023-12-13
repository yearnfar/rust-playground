use std::collections::HashMap;

fn run() {
    let mut scores = HashMap::new();
    scores.insert("k2", "v2");
    scores.insert("k3", "v3");

    scores.entry("k2").or_insert("v2_2");
    scores.entry("k1").or_insert("v1");

    println!("{:?}", scores);

    for (k, v) in scores {
        println!("{k} {v}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        run();
    }
}
