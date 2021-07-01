use crate::Solution;

impl Solution {
    pub fn number_of_rounds(start_time: String, finish_time: String) -> i32 {
        let start_time = start_time.split(":").map(|x| {x.parse::<i32>().unwrap()}).collect::<Vec<i32>>();
        let mut finish_time = finish_time.split(":").map(|x| {x.parse::<i32>().unwrap()}).collect::<Vec<i32>>();
        if finish_time[0] < start_time[0] {
            finish_time[0] += 24;
        } else if finish_time[0] == start_time[0] && finish_time[1] < start_time[1] {
            finish_time[0] += 24;
        }
        let start_time = start_time[0] * 60 + if start_time[1] == 0 {
            0
        } else if start_time[1] > 0 && start_time[1] <= 15 {
            15
        } else if start_time[1] > 15 && start_time[1] <= 30 {
            30
        } else if start_time[1] > 30 && start_time[1] <= 45 {
            45
        } else {
            60
        };
        let finish_time = finish_time[0] * 60 + if finish_time[1] >= 0 && finish_time[1] < 15 {
            0
        } else if finish_time[1] >= 15 && finish_time[1] < 30 {
            15
        } else if finish_time[1] >= 30 && finish_time[1] < 45 {
            30
        } else {
            45
        };
        let result = (finish_time - start_time) / 15;
        return if result < 0 { 0 } else { result };
    }
}