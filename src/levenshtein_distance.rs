pub struct LevenshteinDistanceCalc {
    cache: Vec<usize>,
}

impl LevenshteinDistanceCalc {
    pub fn new() -> Self {
        Self { cache: Vec::new() }
    }

    pub fn calc(&mut self, source: &str, target: &str) -> usize {
        if source.is_empty() {
            return target.len();
        }

        if target.is_empty() {
            return source.len();
        }

        let target_len = target.len();

        self.cache = (0..=target_len).collect();

        for (i, source_char) in source.chars().enumerate() {
            let mut next_dist = i + 1;

            for (j, target_char) in target.chars().enumerate() {
                let current_dist = next_dist;

                let mut dist_if_substitute = self.cache[j];
                if source_char != target_char {
                    dist_if_substitute += 1;
                }

                let dist_if_insert = current_dist + 1;
                let dist_if_delete = self.cache[j + 1] + 1;

                next_dist = min(dist_if_delete, min(dist_if_insert, dist_if_substitute));

                self.cache[j] = current_dist;
            }

            self.cache[target_len] = next_dist;
        }

        self.cache[target_len]
    }
}

fn min(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}
