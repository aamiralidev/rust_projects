
fn main() {
    println!("Hello, world!");
    let perm: [u32; 9] = [1, 5, 9, 14, 25, 36, 47, 18, 29];
    let winner = deal(perm);
    println!("{:?}", winner);
}

// enum CardType {Club, Diamond, Heart, Spade, None}

fn get_card_type(num:u32) -> u32 {
        if num > 0 && num <= 13 {
            return 0
        } else if num > 13 && num <=26 {
            return 1
        } else if num > 26 && num <=39 {
            return 2
        } else if num > 39 && num <= 52 {
            return 3
        } else {
            panic!("Card Number out of bond");
        }
}

fn get_card_type_c(num:u32) -> String {
    if num > 0 && num <= 13 {
        return "C".to_string()
    } else if num > 13 && num <=26 {
        return "D".to_string()
    } else if num > 26 && num <=39 {
        return "H".to_string()
    } else if num > 39 && num <= 52 {
        return "S".to_string()
    } else {
        panic!("Card Number out of bond");
    }
}

fn get_card_number(num:u32) -> u32 {
    let mut number = num;
    number = number%13;
    if number == 0 {
        number = 13;
    }
    number
}

fn get_card_rank(num:u32) -> u32{
    let mut rank = get_card_number(num);
    if rank==1 {
        rank = 14;
    }
    rank
}

fn is_royal_flush(hand:Vec<u32>) -> Vec<u32>{
    let mut count:Vec<u32> = vec![];
    let mut temp_vec:Vec<u32> = vec![];
    for i in 0..7 {
        if get_card_rank(hand[i]) >= 10 {
            count.push(hand[i]);
        }
    }
    if count.len() >= 5 {
        let card_type = get_card_type(count[0]);
        while count.len() > 0 {
            for i in 0..count.len() {
                if get_card_type(count[i]) != card_type {
                    temp_vec.push(count[i]);
                    count.remove(i);
                }
            }
            if count.len() == 5 {
                count.sort();
                return count
            }
            count.clear();
            count = temp_vec.clone();
            temp_vec.clear();
        }
    }
    temp_vec
}

fn  is_straight_flush(hand:Vec<u32>) -> Vec<u32> {
    let mut count = 1;
    let mut index = 0;
    let mut temp_vec:Vec<u32> = vec![];
    for i in 1..7 {
        if count < 5 && hand[i]-1 == hand[i-1] {
            count+=1;
            index = i;
        } else if count < 5 {
            count = 1;
        }
    }
    if count >= 5 {
        for i in 0..5 {
            temp_vec.push(hand[index-i]);
        }
        temp_vec.sort();
    }
    temp_vec
}


fn divide_by_number(hand:Vec<u32>) -> (Vec<u32>, Vec<Vec<u32>>) {
    let mut count:Vec<u32> = vec![];
    let mut temp_vec:Vec<Vec<u32>> = vec![];
    for i in 0..hand.len() {
        let num = get_card_number(hand[i]);
        if count.contains(&num){
            let index = count.iter().position(|&r| r == num).unwrap();
            temp_vec[index].push(hand[i]);
        } else {
            count.push(num);
            temp_vec.push(vec![hand[i]]);
        }
    }
    (count, temp_vec)
}

fn divide_by_type(hand:Vec<u32>) -> (Vec<u32>, Vec<Vec<u32>>){
    let mut count:Vec<u32> = vec![];
    let mut temp_vec:Vec<Vec<u32>> = vec![];
    for i in 0..hand.len() {
        let num = get_card_type(hand[i]);
        if count.contains(&num){
            let index = count.iter().position(|&r| r == num).unwrap();
            temp_vec[index].push(hand[i]);
        } else {
            count.push(num);
            temp_vec.push(vec![hand[i]]);
        }
    }
    (count, temp_vec)
}

fn is_four_of_a_kind(hand:Vec<u32>) -> Vec<u32> {
    let (mut count, temp_vec) = divide_by_number(hand.clone());
    for mut v in temp_vec {
        if v.len() == 4 {
            let mut max = 2;
            for i in hand {
                if get_card_rank(i) > get_card_rank(max) && i!=v[0] {
                    max = i
                }
            }
        } else if v.len() > 4 {
            v.truncate(5);
        }
        return v
    }
    count.clear();
    return count
}

fn is_full_house(hand:Vec<u32>) -> Vec<u32>{
    let (mut count, temp_vec) = divide_by_number(hand);
    let mut index3 = 100;
    let mut index2 = 100;
    for v in 0..temp_vec.len(){
        if temp_vec[v].len() == 3 {
            if index3 == 100 {
                index3 = v;
            } else {
                if get_card_rank(temp_vec[index3][0]) < get_card_rank(temp_vec[v][0]) {
                    index2 = index3;
                    index3 = v;
                } else {
                    index2 = v;
                }
            }
        } else if temp_vec[v].len() == 2 {
            index2 = v;
        }
    }
    if index2 != 100 && index3 != 100 {
        let mut s = temp_vec[index3].clone();
        s.append(&mut temp_vec[index2].clone());
        return s
    }
    count.clear();
    return count;
}

fn is_flush(hand:Vec<u32>) -> Vec<u32> {
    let (mut count, temp_vec) = divide_by_type(hand);
    for v in temp_vec {
        if v.len() >= 5 {
            return v
        }
    }
    count.clear();
    return count
}

fn is_straight(hand:Vec<u32>) -> Vec<u32>{
    let mut count = 1;
    let mut index = 0;
    let mut temp_vec:Vec<u32> = vec![];
    let mut win_hand:Vec<u32> = vec![];

    let mut numbered_hand: Vec<u32> = vec![];
    for i in &hand {
        numbered_hand.push(get_card_number(*i));
    }
    numbered_hand.sort();
    for i in 1..7 {
        if numbered_hand[i] == numbered_hand[i-1]{
            count+=1;
            index = i;
        } else if count < 5 {
            count = 1;
        }
    }

    if count >= 5 {
        for i in 0..5 {
            temp_vec.push(numbered_hand[index-i]);
        }
        for i in 0..7 {
            if numbered_hand.contains(&get_card_number(hand[i])) {
                win_hand.push(hand[i]);
            }
        }
        return win_hand
    }
    temp_vec.clear();
    return temp_vec;
}

fn is_three_of_a_kind(hand:Vec<u32>) -> Vec<u32> {
    let (mut count, mut temp_vec) = divide_by_number(hand.clone());
    for i in 0..temp_vec.len() {
        if temp_vec[i].len() == 3 {
            let mut max = 2;
            let mut s_max = 2;
            for j in hand {
                if get_card_rank(j) > get_card_rank(max) && j!=temp_vec[i][0] {
                    s_max = max;
                    max = j;
                }
            }
            temp_vec[i].push(max);
            temp_vec[i].push(s_max);
            return  temp_vec[i].clone()
        }
    }
    count.clear();
    return count;
}

fn is_two_pair(hand:Vec<u32>) -> Vec<u32> {
    let (mut count, temp_vec) = divide_by_number(hand.clone());
    let mut index1 = 100;
    let mut index2 = 100;

    for i in 0..temp_vec.len() {
        if temp_vec[i].len() == 2 {
            if index1 == 100 { index1 = i }
            else if index2 == 100 {
                if get_card_rank(temp_vec[i][0]) > get_card_rank(temp_vec[index1][0]) {
                    index2 = index1;
                    index1 = i;
                }else {
                    index2 = i;
                }
            } else {
                if get_card_rank(temp_vec[i][0]) > get_card_rank(temp_vec[index1][0]) {
                    index2 = index1;
                    index1 = i;
                } else if get_card_rank(temp_vec[i][0]) > get_card_rank(temp_vec[index2][0]) {
                    index2 = i;
                }
            }
        }
    }
    if index2 != 100 && index1 != 100 {

        let mut s = temp_vec[index1].clone();
        s.append(&mut temp_vec[index2].clone());

        let mut max = 2;
        for i in hand {
            if get_card_rank(i) > get_card_rank(max) && i!=temp_vec[index1][0]
            && i!=temp_vec[index2][0]{
                max = i
            }
        }
        s.push(max);
        return s
    }
    count.clear();
    return count
}

fn is_one_pair(hand:Vec<u32>) -> Vec<u32>{
    let (mut count, mut temp_vec) = divide_by_number(hand.clone());
    for i in 0..temp_vec.len() {
        if temp_vec[i].len() == 2 {
            let mut temp_hand = hand.clone();
            for _j in 0..3 {
               let s = *temp_hand.iter().max().unwrap();
                temp_vec[i].push(s);
                let index = temp_hand.iter().position(|&r| r == s).unwrap();
                temp_hand.remove(index);
            }
            return temp_vec[i].clone()
        }
    }
    count.clear();
    return count
}

fn is_high_card(hand:Vec<u32>) -> Vec<u32> {
    let mut numbered_hand = vec![];
    let mut temp_vec = vec![];
    for i in &hand {
        numbered_hand.push(get_card_number(*i));
    }
    numbered_hand.sort();
    numbered_hand.remove(0);
    numbered_hand.remove(0);
    for i in hand {
        if numbered_hand.contains(&get_card_number(i)) {
            temp_vec.push(i);
        }
    }
    temp_vec
}

enum Hand{}
// {RoyalFlush, StraightFlush, FourOfAKind, FullHouse, Flush,
//     Straight, ThreeOfAKind, TwoPair, OnePair, HighCard}


impl Hand {
    fn new(hand:Vec<u32>) -> (u32, Vec<u32>){
        let mut rank:u32 = 0;
        let mut p_hand = vec![];
        let fn_vec = [is_royal_flush, is_straight_flush,
        is_four_of_a_kind, is_full_house, is_flush, is_straight, is_three_of_a_kind,
        is_two_pair, is_one_pair, is_high_card];

        for i in 0..10 {
            p_hand = fn_vec[i](hand.clone());
            if p_hand.len() == 5 {
                rank = (10 - i) as u32;
                break;
            }
        }
        (rank, p_hand)
    }
}

fn map_to_strings(hand:Vec<u32>) -> Vec<String> {
    let mut win_hand:Vec<String> = vec![];
    let mut temp;
    for i in hand {
        temp = get_card_type_c(i);
        temp.push_str(&get_card_number(i).to_string());
        win_hand.push(temp);
    }
    return win_hand;
}


pub fn deal(perm:[u32; 9]) -> Vec<String> {
    let mut player1 = vec![perm[0], perm[2]];
    let mut player2 = vec![perm[1], perm[3]];
    let mut pool = vec![];
    for i in 4..9 {
        pool.push(perm[i]);
    }
    let pool1 = pool.clone();
    player1.extend(pool1);
    let pool1 = pool.clone();
    player2.extend(pool1);
    player1.sort();
    player2.sort();

    let player1_details = Hand::new(player1);
    let player2_details = Hand::new(player2);
    let winning_hand:Vec<String>;

    if player1_details.0 == player2_details.0 {
        println!("tie");
        winning_hand = map_to_strings(player1_details.1);
    }else if player1_details.0 > player2_details.0 {
        println!("{}", player1_details.0);
        winning_hand = map_to_strings(player1_details.1);
    } else {
        println!("{}", player2_details.0);
        winning_hand = map_to_strings(player2_details.1);
    }
    return winning_hand
}
