use std::collections::HashMap;

use day20::input;

fn grove_coordinates(input: &str) -> i16 {
    let sequence: Vec<i16> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut mixed = sequence.clone();

    let mut map: HashMap<i16, (Vec<usize>, usize)> = HashMap::new();
    for (index, number) in sequence.iter().copied().enumerate() {
        map.entry(number)
            .and_modify(|number_vec| number_vec.0.push(index))
            .or_insert((vec![index], 0));
    }

    for number in sequence.iter() {
        let index;
        let next_index;

        {
            let map_item = map.get_mut(number).unwrap();
            index = map_item.0[map_item.1];

            let next_index_i16 = (index as i16 + number) % (sequence.len() as i16 - 1);
            next_index = if next_index_i16 < 0 {
                sequence.len() - 1 - (next_index_i16.abs() as usize % (sequence.len() - 1))
            } else {
                next_index_i16 as usize
            };
        }


        assert!(index < sequence.len());
        assert!(next_index < sequence.len());
        if next_index > index {
            for i in index + 1..=next_index {
                map.entry(mixed[i]).and_modify(|(rotated_number_vec, _)| {
                    for rotated_index in rotated_number_vec.iter_mut() {
                        if *rotated_index == i {
                            *rotated_index -= 1;
                        }
                    }
                });
            }
            map.entry(*number).and_modify(|map_item| map_item.0[map_item.1] = next_index);
            mixed[index..=next_index].rotate_left(1);
        } else if index > next_index {
            for i in next_index..index {
                map.entry(mixed[i]).and_modify(|(rotated_number_vec, _)| {
                    for rotated_index in rotated_number_vec.iter_mut() {
                        if *rotated_index == i {
                            *rotated_index += 1;
                        }
                    }
                });
            }
            map.entry(*number).and_modify(|map_item| map_item.0[map_item.1] = next_index);
            mixed[next_index..=index].rotate_right(1);
        }
        map.entry(*number).and_modify(|map_item| map_item.1 += 1);
    }

    let index0 = *map.get(&0).unwrap().0.first().unwrap();
    mixed[(index0 + 1000) % sequence.len()]
        + mixed[(index0 + 2000) % sequence.len()]
        + mixed[(index0 + 3000) % sequence.len()]
}

fn main() {
    println!("{}", grove_coordinates(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(3, grove_coordinates(input::EXAMPLE));
}
