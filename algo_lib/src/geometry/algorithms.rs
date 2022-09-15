use crate::geometry::point_32::Point32;

macro_rules! swap {
    ($x:expr, $y:expr) => {
        let t = $x;
        $x = $y;
        $y = t
    };
}

pub fn convex_hull(mut p: Vec<Point32>) -> Vec<Point32> {
    let n = p.len();
    for i in 1..n {
        if p[i].x < p[0].x || p[i].x == p[0].x && p[i].y < p[0].y {
            swap!(p[i], p[0]);
        }
    }
    let first = p[0].clone();
    p[1..].sort_by(|a, b| {
        let d1 = *a - first;
        let d2 = *b - first;
        let dir = d1.vmul(&d2);
        if dir == 0 {
            d1.len_squared().cmp(&d2.len_squared())
        } else {
            dir.cmp(&0).reverse()
        }
    });
    let mut res = Vec::new();
    res.push(first);
    for e in p.into_iter().skip(1) {
        while res.len() > 1 && (e - res[res.len() - 2]).vmul(&(e - res[res.len() - 1])) <= 0 {
            res.pop();
        }
        res.push(e);
    }
    res
}
