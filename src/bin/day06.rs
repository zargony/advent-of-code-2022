use advent_of_code_2022::Input;
use itertools::Itertools;
use std::{error, io::Read};

#[derive(Debug)]
struct MarkerDetector<R: Read>(R);

fn detect(size: usize, s: &str) -> &str {
    for (i, window) in s.as_bytes().windows(size).enumerate() {
        if window.iter().unique().count() == window.len() {
            return &s[0..i + size];
        }
    }
    &s[0..0]
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = Input::day(6)?.lines().next().ok_or("Missing input")??;

    let packet_prefix_len = detect(4, &input).len();
    println!("Packet starts after {} characters", packet_prefix_len);

    let message_prefix_len = detect(14, &input).len();
    println!("Message starts after {} characters", message_prefix_len);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(detect(4, "mjqjpqmgbljsphdztnvjfqwrcgsmlb").len(), 7);
        assert_eq!(detect(4, "bvwbjplbgvbhsrlpgdmjqwftvncz").len(), 5);
        assert_eq!(detect(4, "nppdvjthqldpwncqszvftbrmjlhg").len(), 6);
        assert_eq!(detect(4, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").len(), 10);
        assert_eq!(detect(4, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").len(), 11);
    }

    #[test]
    fn part_2() {
        assert_eq!(detect(14, "mjqjpqmgbljsphdztnvjfqwrcgsmlb").len(), 19);
        assert_eq!(detect(14, "bvwbjplbgvbhsrlpgdmjqwftvncz").len(), 23);
        assert_eq!(detect(14, "nppdvjthqldpwncqszvftbrmjlhg").len(), 23);
        assert_eq!(detect(14, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").len(), 29);
        assert_eq!(detect(14, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").len(), 26);
    }
}
