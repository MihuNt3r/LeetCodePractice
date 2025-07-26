fn main() {
    let vec = vec![120,126,180,231,31,228,220,166,39,69,125,29,97,215,50,187,194,235,159,244,182,151,48,113,97,38,259,72,136,36,4,241,110,169,47,127,140,143,21,258,191,111,95,154,63,212,112,155,7,217,34,128,74,73,207,126,122,111,202,245,202,28,230,54,225,77,218,65,146,11,101,218,31,104,152,60,256,180,100,258,257,230,22,105,117,70,159];
    let a = 114;
    let b = 24;
    let c = 41;

    println!("{}", Solution::count_good_triplets(vec, a, b, c));
}

struct Solution;

impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        // Create tuples-triplets
        let mut triplets = Vec::new();

        for (i, a) in arr.iter().enumerate() {
            for (j, b) in arr.iter().enumerate().skip(i + 1) {
                for (k, c) in arr.iter().enumerate().skip(j + 1) {
                    triplets.push((a, b, c));
                }
            }
        }

        println!("{:?}", triplets);

        // filter tuples-triplets

        println!("-------------------------------------");

        let good_triplets: Vec<_> = triplets
            .iter()
            .filter(|&&(x, y, z)| is_good_triplet(x, y, z, a, b, c))
            .collect();

        println!("{:?}", good_triplets);

        // return count

        good_triplets.len() as i32
    }
}

fn is_good_triplet(x: &i32, y: &i32, z: &i32, a: i32, b: i32, c: i32) -> bool {
        (x - y).abs() <= a &&
        (y - z).abs() <= b &&
        (x - z).abs() <= c
}