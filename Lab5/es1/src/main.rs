use clap::Parser;
use itertools::Itertools;
use std::{collections::HashSet,thread, time::Instant};

#[derive(Parser, Debug)]
struct Args {
    seq: String,
}



fn main() {
    let op = vec!['+','-','x','/'];
    let args = Args::parse();
    let seq: Vec<_>= args.seq.rsplit(' ').collect();
    let v2: Vec<i32> = seq.iter()
        .map(|s| s.chars().next().expect("No value").to_digit(10).unwrap() as i32)
        .collect();

    let perm = v2.iter().permutations(5).collect_vec();
    let comb = op.iter().combinations_with_replacement(4).collect_vec();
    
    // let brute_force = perm.iter().flat_map(|s| comb.iter().map(|o| (s,o))).collect(); // using iterator
    let brute_force = perm.iter().cartesian_product(comb.iter()).collect_vec();
    
    let num_threads = 4;
    
    let start = Instant::now();
    let t_results = thread::scope(|s| {
        let mut threads = vec![]; 
        
        for i in 0..num_threads{
            let slice = &brute_force[i*(brute_force.len()/num_threads)..if i==num_threads-1 {brute_force.len()}else {(i+1)*(brute_force.len()/num_threads)}];
            threads.push(s.spawn(|| {
                solve(slice)
            }));
        }
        let mut results = HashSet::new();
        for t in threads {
            for res in t.join().unwrap(){
                results.insert(res);
            }
        }
        results
    });

    println!("Stringa: {}\nNum_threads: {}\nNum_sol: {}\nTime: {} ms\nSol: {:?}",args.seq, num_threads, t_results.len(), start.elapsed().as_millis(), t_results);
    
}

fn compute_operation(str: (&Vec<&i32>,&Vec<&char>)) -> Option<(i32,String)> {
    let (seq,op) = str;
    let mut init_value = *seq[0];
    let mut initial_string = seq[0].to_string();
    for (i,value) in seq[1..].iter().enumerate(){
        initial_string.push_str(format!(" {} {}",op[i],value).as_str());
        match op[i] {
            '+' => init_value += **value,
            '-' => init_value -= **value,
            'x' => init_value *= **value,
            '/' => {if **value == 0 || init_value % **value != 0 { return None;} else {init_value /= **value}},
             _ => panic!("Operazione non supportata"),
        }
    }
    Some((init_value,initial_string))
}

fn solve(slice: &[(&Vec<&i32>, &Vec<&char>)]) -> Vec<String>{
    let mut col: Vec<String> = vec![];
    for str in slice {
        let res = compute_operation(*str);
        if res.is_some(){
            let (value, string) = res.unwrap();
            if value == 10 {
                col.push(string);
            }
        }
    }
    col
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_division_zero() {
        let result = compute_operation((&vec![&2,&0,&2,&2,&1], &vec![&'/',&'-',&'x',&'+']));
        assert!(result.is_none());
    }

    #[test]
    fn test_not_integer_division() {
        let result = compute_operation((&vec![&2,&3,&2,&2,&1], &vec![&'/',&'-',&'x',&'+']));
        assert!(result.is_none());
    }

    #[test]
    fn test_operation_negative() {
        let result = compute_operation((&vec![&2,&7,&2,&2,&1], &vec![&'-',&'-',&'x',&'+']));
        assert_eq!(result.unwrap(), (-13, "2 - 7 - 2 x 2 + 1".to_string()));
    }

    #[test]
    fn test_operation_10() {
        let result = compute_operation((&vec![&7,&2,&1,&2,&2], &vec![&'-',&'-',&'x',&'+']));
        assert_eq!(result.unwrap(), (10, "7 - 2 - 1 x 2 + 2".to_string()));
    }

}
