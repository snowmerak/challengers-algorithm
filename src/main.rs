pub mod N1903_largest_odd_number;
pub mod N1904_number_of_rounds;
pub mod N1905_count_sub_islands;
pub mod N1906_min_difference;
pub mod N1512_num_identical_pairs;
pub mod N1137_tribonacci;
pub mod N1713_min_operations;

fn main() {
    let rs = Solution::min_difference(vec![4,5,2,2,7,10], vec![vec![2,3],vec![0,2],vec![0,5],vec![3,5]]);
    println!("{:?}", rs);
}

struct Solution {}
