use std::collections::HashMap;

use random_number::random;

pub const SQRT_5: f32 = 2.2360679775;

fn fibonash_bine(n: u8) -> f32 {
    let k: f32 = n.into();
    let phi: f32 = (1.0 + SQRT_5) / 2.0;
    ((phi.powf(k) - (-phi).powf(-k)) / (2.0 * phi - 1.0)).round()
}
fn f_c(f: i32) -> i32 {
    (f - 32) / 9 * 5
}
fn c_f(c: i32) -> i32 {
    c / 5 * 9 + 32
}
fn print_vector(vec: Vec<u32>) {
    for n in 0..vec.len() {
        print!("{} {}", vec[n], " ")
    }
}
fn twelve_days_of_christmas() {
    let count = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelth",
    ];
    let song = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five gold rings (five golden rings)",
        "Four calling birds",
        "Three French hens",
        "Two turtledoves",
    ];
    for i in 0..12 {
        println!(
            "{}{}{}{}{}",
            "On the", " ", count[i], " ", " day of Christmas, my true love sent to me"
        );
        for k in 0..i {
            println!("{}", song[k])
        }
        println!("A partridge in a pear tree");
        println!("")
    }
    println!("A partridge in a pear tree");
}
/*fn find_nlog_median(vect: &Vec<usize>) -> usize {
    let mut vec: &mut Vec<usize> = vect;
    vec.sort();
    if vec.len() % 2 == 1 {
        vec[vec.len() / 2]
    } else {
        (vec[vec.len() / 2 - 1] + vec[vec.len() / 2]) / 2
    }
}

fn pick(vec: &Vec<usize>) -> usize {
    if vec.len() < 5 {
        find_nlog_median(vec)
    } else {
        vec[0]
    }
}*/
fn vec_count(vec: &Vec<usize>) -> usize {
    let mut map = HashMap::new();
    for n in 0..vec.len() {
        let cou = map.entry(vec[n]).or_insert(0);
        *cou += 1;
    }
    map.iter().count()
}
fn find_median(vec: &Vec<usize>) -> usize {
    let length = vec.len();
    if length % 2 == 1 {
        quickselect(vec, length / 2)
    } else {
        (quickselect(vec, length / 2 - 1) + quickselect(vec, length / 2)) / 2
    }
}
fn find_summ(vec: &Vec<usize>) -> usize {
    vec.iter().sum()
}
fn quickselect(vec: &Vec<usize>, k: usize) -> usize {
    if vec.len() == 1 {
        vec[0]
    } else {
        let pivot = vec[random!(0..vec.len())];
        let lows: Vec<usize> = vec
            .iter()
            .filter(|x| **x < pivot)
            .copied()
            .collect::<Vec<_>>();
        let hight = vec
            .iter()
            .filter(|x| x > &&pivot)
            .copied()
            .collect::<Vec<_>>();
        let pivots = vec.iter().filter(|x| x == &&pivot).collect::<Vec<_>>();
        if k < lows.len() {
            quickselect(&lows, k)
        } else if k < lows.len() + pivots.len() {
            *pivots[0]
        } else {
            quickselect(&hight, k - lows.len() - pivots.len())
        }
    }
}
fn pig_latin(str: &String) -> String {
    let mut answer: String = "".to_string();
    for word in str.split_whitespace() {
        let mut n = 0;
        let mut second = word.chars();
        let mut w = word.chars().next().unwrap().to_string();
        if w == ("a")
            || w == ("e")
            || w == ("y")
            || w == ("u")
            || w == ("i")
            || w == ("o")
            || w == ("A")
            || w == ("E")
            || w == ("Y")
            || w == ("U")
            || w == ("I")
            || w == ("O")
        {
            for n in 0..word.chars().count() {
                answer.push_str(&second.next().unwrap().to_string());
            }
            answer.push_str("hay ");
        } else {
            second.next();
            for n in 0..word.chars().count() - 1 {
                answer.push_str(&second.next().unwrap().to_string());
            }
            {
                answer.push_str(&w);
            }
            answer.push_str("ay ");
        }
    }
    answer
}
fn main() {
    let text = "Another common use case for hash maps is to look up a key’s value and then update it based on the old value".to_string();
    print!("{}", pig_latin(&text))
    //let a = Vec::from([1, 2, 3]);
    //let m = find_summ(&a);
    //print!("{}", m);
    //let n: usize = 10;
    //let xs: Vec<u32> = random_vectot(n);
    //print_vector(xs);
    //twelve_days_of_christmas();
    //println!("{}", fibonash_bine(10)); //55
    //println!("{}", f_c(440));
    //println!("{}", c_f(-120));
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pig_latin() {
        let m = pig_latin(&"Another common use".to_string());
        print!("{}", m);
        assert_eq!(m, " Anotherhay ommoncay usehay".to_string())
    }
    #[test]
    fn test_median() {
        let a = Vec::from([
            4, 5, 3, 1, 2, 0, 6, 0, 6, 6, 6, 6, 0, 0, 0, 1, 2, 5, 4, 6, 0,
        ]);
        let m = find_median(&a);
        print!("{}", m);
        assert_eq!(m, 3)
    }
    #[test]
    fn test_count() {
        let a = Vec::from([
            4, 5, 3, 1, 2, 0, 6, 0, 6, 6, 6, 6, 0, 0, 0, 1, 2, 5, 4, 6, 0,
        ]);
        let b = vec_count(&a);
        print!("{:?}", b);
        assert_eq!(b, 7)
    }
    #[test]
    fn test_summ() {
        let a = Vec::from([4, 5, 3, 1, 2, 0, 6, 0, 6, 6, 6, 6, 0, 0, 0, 1, 2, 5, 4, 6]);
        let m = find_summ(&a);
        print!("{}", m);
        assert_eq!(m, 63)
    }
    #[test]
    fn test_fibonash_bine() {
        assert_eq!(fibonash_bine(10), 55.0)
    }
    #[test]
    fn test_f_c() {
        assert_eq!(40, f_c(104));
    }
    #[test]
    fn test_c_f() {
        assert_eq!(104, c_f(40));
    }
    #[test]
    fn test_f_c_plas_c_f() {
        assert_eq!(f_c(-40), f_c(-40));
    }
}
