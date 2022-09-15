/// Stores ax + b. Query: given x0, maximizes a * x0 + b
pub struct ConvexSet {
    hull: Vec<(i64, i64)>,
}

/// left.0 < between.0 < right.0
fn line_not_required_between(between: &(i64, i64), left: &(i64, i64), right: &(i64, i64)) -> bool {
    (right.0 - between.0) * (left.1 - between.1) >= (between.0 - left.0) * (between.1 - right.1)
}

/// left.0 < right.0
fn point_to_the_left_of_intersection(x: i64, left: &(i64, i64), right: &(i64, i64)) -> bool {
    (right.0 - left.0) * x <= (left.1 - right.1)
}

impl ConvexSet {
    /// Elements are to be given strictly increasing by a
    pub fn new(lines: &Vec<(i64, i64)>) -> Self {
        let mut hull = Vec::new();
        for line in lines {
            while hull.len() > 1
                && line_not_required_between(&hull[hull.len() - 1], &hull[hull.len() - 2], line)
            {
                hull.pop();
            }
            hull.push(line.to_owned());
        }
        Self { hull }
    }

    pub fn query(&self, x: i64) -> (i64, i64) {
        let mut left = 0;
        let mut right = self.hull.len();
        while left + 1 < right {
            let mid = (left + right) >> 1;
            if point_to_the_left_of_intersection(x, &self.hull[mid - 1], &self.hull[mid]) {
                right = mid;
            } else {
                left = mid;
            }
        }
        self.hull[left]
    }

    pub fn monotonic_query(&self) -> MonotonicQuery {
        MonotonicQuery {
            conv_set: self,
            pos: 0,
        }
    }
}

pub struct MonotonicQuery<'a> {
    conv_set: &'a ConvexSet,
    pos: usize,
}

impl<'a> MonotonicQuery<'a> {
    pub fn query(&mut self, x: i64) -> (i64, i64) {
        let hull = &self.conv_set.hull;
        while self.pos + 1 < hull.len()
            && !point_to_the_left_of_intersection(x, &hull[self.pos], &hull[self.pos + 1])
        {
            self.pos += 1;
        }
        hull[self.pos]
    }
}
