use std::io;
use std::collections::HashMap;
fn main (){
    let mut s = String::new();
    println!("숫자를 입력하시오");
    io::stdin().read_line(&mut s).expect("입력오류");

    let mut numbers:Vec<u32>=s
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();
    let sum:u32 =numbers.iter().sum();
    let avg:f64 =(sum as f64)/(numbers.len() as f64 );
    println!("average value: {:.2}",avg);

    numbers.sort();
    
    let n=numbers.len();
    if n%2==0{
        println!("middle value : {}",numbers[n/2]);
    }
    else{
        println!("middle value : {}",numbers[(n-1)/2]);
    }
    let mut h:HashMap<u32,u32>=HashMap::new();
    let mut max_value=0;
    let mut max_value_num:&u32=&0;
    for i in numbers.iter(){
        h.entry(*i).and_modify(|e|*e+=1).or_insert(1);
        if h.get(i).copied().unwrap_or(0)>max_value{
            max_value=h.get(i).copied().unwrap();
            max_value_num=i;
        }
    }
    println!("max value : {}",max_value_num);
}

