use crate::levenshtein_distance::LevenshteinDistanceCalc;

mod levenshtein_distance;

fn main() {
    let lines: Vec<&str> = include_str!("../sample.txt").split('\n').collect();

    let mut leven_dist_calc = LevenshteinDistanceCalc::new();

    use std::time::Instant;
    let now = Instant::now();

    {
        for _ in 0..10000 {
            let mut last_value = "";
            for line in &lines {
                leven_dist_calc.calc(last_value, line);
                last_value = line;
            }
        }
    }

    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    print!("{}", sec);

    // check
    let answers: Vec<String> = (0..lines.len() - 1)
        .map(|i| leven_dist_calc.calc(lines[i], lines[i + 1]))
        .map(|dist| dist.to_string())
        .collect();
    eprintln!("{}", answers.join(","));
}
