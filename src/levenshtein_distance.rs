use std::cmp::max;

pub struct LevenshteinDistanceCalc {
    cache: Vec<usize>,
}

impl LevenshteinDistanceCalc {
    pub fn new() -> Self {
        Self { cache: Vec::new() }
    }

    fn build_cache(&mut self, target_len: usize) {
        let alloc_size = max(self.cache.len(), target_len);
        self.cache.reserve(alloc_size);
        self.cache.clear();

        for i in 0..=target_len {
            self.cache.push(i);
        }
    }

    pub fn calc(&mut self, source: &str, target: &str) -> usize {
        if source.is_empty() {
            return target.len();
        }

        if target.is_empty() {
            return source.len();
        }

        let target_len = target.len();

        self.build_cache(target_len);

        for (i, source_char) in source.chars().enumerate() {
            let mut next_dist = i + 1;

            for (j, target_char) in target.chars().enumerate() {
                let current_dist = next_dist;

                let mut dist_if_substitute = unsafe { self.cache.get_unchecked(j).clone() };
                if source_char != target_char {
                    dist_if_substitute += 1;
                }

                let dist_if_insert = current_dist + 1;
                let dist_if_delete = unsafe { self.cache.get_unchecked(j + 1).clone() } + 1;

                next_dist = min(dist_if_delete, min(dist_if_insert, dist_if_substitute));

                unsafe {
                    *self.cache.get_unchecked_mut(j) = current_dist;
                }
            }

            unsafe {
                *self.cache.get_unchecked_mut(target_len) = next_dist;
            }
        }

        unsafe { self.cache.get_unchecked(target_len).clone() }
    }
}

fn min(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}
