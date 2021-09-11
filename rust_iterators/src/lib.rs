use std::collections::HashMap;
use std::convert::TryFrom;
use chrono::prelude::*;
use num_traits::FromPrimitive;


pub fn pairing(nums1: &Vec<i32>, nums2: &Vec<i32>) -> Vec<(i32, i32)> {
    nums1
        .iter()
        .flat_map(
            |n1_ref| nums2.iter().map(move |n2_ref| (n1_ref, n2_ref))
        )
        .map(|(x, y)| (*x, *y))
        .collect()
}


pub fn frequency_count(nums: &Vec<i32>) -> HashMap<i32, u32> {
    let mut map = HashMap::new();

    nums.iter().for_each(|num| {
        let count = map.entry(*num).or_insert(0);
        *count += 1;
    });

    map
}


fn get_days_in_month(year: i32, month: u32) -> i64 {
    NaiveDate::from_ymd(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )
    .signed_duration_since(NaiveDate::from_ymd(year, month, 1))
    .num_days()
}


pub fn calendar(year: i32, month: u32) {
    let date = NaiveDate::from_ymd(year, month, 1);
    let weekday = date.weekday();

    // weekday titles
    println!("Calendar for {:?} {}", Month::from_u32(date.month()).unwrap(), date.year());
    println!(
        "{} | {} | {} | {} | {} | {} | {}",
        Weekday::Mon, Weekday::Tue, Weekday::Wed, Weekday::Thu, Weekday::Fri, Weekday::Sat, Weekday::Sun
    );

    // padding at the start
    let mut days: Vec<String> = vec!["".to_string(); weekday.num_days_from_monday() as usize];
    // days in month
    let mut daystrings: Vec<String> = (1..(get_days_in_month(year, month) + 1))
        .collect::<Vec<i64>>()
        .iter()
        .map(|date| (*date).to_string())
        .collect();
    days.append(&mut daystrings);
    // padding at the end
    days.append(&mut vec!["".to_string(); usize::try_from((days.len() as f32 / 7f32).ceil() as u32 * 7 - days.len() as u32).unwrap()]);

    // print days
    days.chunks(7).for_each(
        |week| {
            println!(
                "{: <3} | {: <3} | {: <3} | {: <3} | {: <3} | {: <3} | {: <3}",
                week[0], week[1], week[2], week[3], week[4], week[5], week[6]
            );
        }
    );
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::{pairing, frequency_count, calendar};

    #[test]
    fn test_pairing_1() {
        assert_eq!(pairing(&vec![1, 2], &vec![3, 4]), vec![(1, 3), (1, 4), (2, 3), (2, 4)]);
    }

    #[test]
    fn test_pairing_2() {
        assert_eq!(
            pairing(&vec![-2, -3], &vec![-10, -20]),
            vec![(-2, -10), (-2, -20), (-3, -10), (-3, -20)]
        );
    }

    #[test]
    fn test_pairing_empty() {
        assert_eq!(
            pairing(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9], &vec![]),
            vec![]
        );
    }

    #[test]
    fn test_frequency_count_1() {
        let mut map = HashMap::new();
        map.insert(10, 2);
        map.insert(3, 1);

        assert_eq!(frequency_count(&vec![10,3,10]), map);
    }

    #[test]
    fn test_frequency_count_2() {
        let mut map = HashMap::new();
        map.insert(0, 10);

        assert_eq!(frequency_count(&vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), map);
    }

    #[test]
    #[should_panic]
    fn test_calendar_invalid_month() {
        calendar(2021, 0);
    }

    #[test]
    #[should_panic]
    fn test_calendar_out_of_range_date() {
        calendar(2021, 1000000);
    }

    #[test]
    fn test_calendar_success() {
        calendar(2021, 9);
    }

    #[test]
    fn test_frequency_count_empty() {
        assert_eq!(frequency_count(&vec![]), HashMap::new());
    }

    #[test]
    fn test_any() {
        // Vec
        assert_eq!(
            vec!["Hung", "Vu"]
                .iter()
                .any(|name| name.to_string() == "Hung".to_string()),
            true
            );
        assert_eq!(
            vec!["Hung", "Vu"]
                .iter()
                .any(|name| name.to_string() == "Ngan".to_string()),
            false
            );

        // HashMap
        let mut map = HashMap::new();
        map.insert("first", "Hung");
        map.insert("last", "Vu");
        assert_eq!(
            map
                .iter()
                .any(|keyval| keyval.1.to_string() == "Hung".to_string()),
            true
        );
        assert_eq!(
            map
                .iter()
                .any(|keyval| keyval.1.to_string() == "Ngan".to_string()),
            false
        );
    }

    #[test]
    fn test_chain() {
        // Vec
        let chain: Vec<&str> = vec!["hello", "world"].iter().chain(vec!["from", "CSC 253"]
            .iter())
            .map(|x| *x)
            .collect();
        assert_eq!(
            chain,
            ["hello", "world", "from", "CSC 253"]
        );

        // HashMap
        let mut map1 = HashMap::new();
        map1.insert("zero", 0);
        map1.insert("one", 1);

        let mut map2 = HashMap::new();
        map2.insert("two", 2);
        map2.insert("three", 3);

        let chain: HashMap<&str, i32> = map1
            .iter()
            .chain(map2.iter()).map(|(key, value)| (*key, *value))
            .collect();
        map1.extend(map2);
        assert_eq!(
            chain,
            map1
        )
    }

    #[test]
    fn test_partition() {
        // Vec
        let (positive, non_positive): (Vec<i32>, Vec<i32>) = vec![-1, -2, 1, -5, 100]
            .iter()
            .partition(|&x| x > &0);
        assert_eq!(positive, [1, 100]);
        assert_eq!(non_positive, [-1, -2, -5]);

        // HashMap
        let mut map = HashMap::new();
        map.insert("one", 1);
        map.insert("neg 5", -5);

        let (positive, non_positive): (HashMap<&str, i32>, HashMap<&str, i32>) = map
            .iter()
            .partition(|(_, &value)| &value > &0);

        let mut pos = HashMap::new();
        pos.insert("one", 1);
        let mut non_pos = HashMap::new();
        non_pos.insert("neg 5", -5);
        assert_eq!(positive, pos);
        assert_eq!(non_positive, non_pos);
    }

    #[test]
    fn test_try_fold() {
        // Vec
        assert_eq!(
            vec![-1, -2, 1, -5, 100]
                .iter()
                .try_fold(0i32, |acc, &elem| acc.checked_add(elem)),
            Some(93)
        );

        // HashMap
        let mut map = HashMap::new();
        map.insert("one", 1);
        map.insert("-5", -5);
        map.insert("neg 5", -5);
        assert_eq!(
            map.iter().try_fold(0i32, |acc, (_, &value)| acc.checked_add(value)),
            Some(-9)
        );
    }


    #[test]
    fn test_find_map() {
        // Vec
        assert_eq!(
            vec!["one", "1", "two", "2"]
                .iter()
                .find_map(|x| x.parse().ok()),
            Some(1)
        );

        // HashMap
        let mut map = HashMap::new();
        map.insert("one", 1);
        map.insert("-5", -5);
        map.insert("neg 5", -5);
        assert_eq!(
            map
                .iter()
                .find_map(|(key, _)| key.parse().ok()),
            Some(-5)
        );
    }

    #[test]
    fn test_rposition() {
        // Vec
        assert_eq!(
            vec!["one", "1", "two", "2"]
                .iter()
                .rposition(|x| x.parse::<i32>().is_ok()),
            Some(3)
        );

        // // HashMap
        // let mut map = HashMap::new();
        // map.insert("one", 1);
        // map.insert("-5", -5);
        // map.insert("neg 5", -5);
        // assert_eq!(
        //     map
        //         .iter()
        //         .rposition(|(key, value)| value.parse::<i32>().is_ok()),
        //     Some()
        // )
    }

    #[test]
    fn test_max_by() {
        // Vec
        assert_eq!(
            vec![-1, -2, 1, -5, 100]
                .iter()
                .max_by(|x, y| x.cmp(y)),
            Some(&100)
        );

        // HashMap
        let mut map = HashMap::new();
        map.insert("one", 1);
        map.insert("-5", -5);
        map.insert("neg 5", -5);

        assert_eq!(
            map
                .iter()
                .max_by(|(_, vx), (_, vy)| vx.cmp(vy)),
            Some((&"one", &1))
        )
    }

    #[test]
    fn test_unzip() {
        // Vec
        assert_eq!(
            vec![(1, 2), (10, 20)]
                .iter()
                .map(|x| *x)
                .unzip(),
            (vec![1, 10], vec![2, 20])
        );

        // HashMap
        let mut map1 = HashMap::new();
        map1.insert("neg 5", -5);

        let mut map2 = HashMap::new();
        map2.insert("three", 3);

        let (map_left, map_right): (HashMap<&str, i32>, HashMap<&str, i32>) = [(("neg 5", -5), ("three", 3))]
            .iter()
            .map(|x| *x)
            .unzip();

        assert_eq!(map1, map_left);
        assert_eq!(map2, map_right);
    }

    #[test]
    fn test_scan() {
        // Vec
        assert_eq!(
            vec![-1, -2, 1, -5, 100]
                .iter()
                .scan(0, |acc, &x| {
                    *acc = *acc + x;
                    Some(*acc)
                })
                .collect::<Vec<i32>>(),
            [-1, -3, -2, -7, 93]
        );

        // HashMap
        let mut map = HashMap::new();
        map.insert("one", 1);

        assert_eq!(
            map
                .iter()
                .scan(0, |acc, (_, x)| {
                    *acc = *acc + x;
                    Some(*acc)
                })
                .collect::<Vec<i32>>(),
            [1]
        )
    }

    #[test]
    fn test_flat_map() {
        // Vec
        assert_eq!(
            vec!["hello", "world", "from", "CSC253"]
                .iter()
                .flat_map(|s| s.chars())
                .collect::<String>(),
            "helloworldfromCSC253"
        );

        // HashMap
        let mut map = HashMap::new();
        map.insert(1, "hello");

        assert_eq!(
            map
                .iter()
                .flat_map(|(_, value)| value.chars())
                .collect::<String>(),
                "hello"
        )
    }

    #[test]
    fn test_cycle() {
        // Vec
        let players = vec!["player1", "player2"];
        let mut cycle = players.iter().cycle();

        assert_eq!(cycle.next(), Some(&"player1"));
        assert_eq!(cycle.next(), Some(&"player2"));
        assert_eq!(cycle.next(), Some(&"player1"));

        // HashMap
        let mut map = HashMap::new();
        map.insert(1, "hello");
        let mut cycle = map.iter().cycle();

        assert_eq!(cycle.next(), Some((&1, &"hello")));
        assert_eq!(cycle.next(), Some((&1, &"hello")));
        assert_eq!(cycle.next(), Some((&1, &"hello")));
    }
}
