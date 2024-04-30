// find all subsequences of seq in s and return a vector of tuples containing the start position
// and the found subsequences as string slices
// ignore overlaps: if a subsequence is found, the search must continue from the next character
// missing lifetimes: the result string slices depend only from one input parameter, which one?

// suggestion: write a function find_sub(&str, &str) -> Option<(usize, &str)> that finds the first subsequence in a string, you can use it in all the following functions



pub fn find_sub<'a,'b>(s: &'a str, seq: &'b str) -> Option<(usize, &'a str) > {
    let enc_seq: Vec<(char,u32,u32)> = seq.split(',')
                .map(|subs| 
                    {let subs = subs.replace("-","");
                    let mut iter = subs.chars();
                    (iter.next().unwrap(),iter.next().unwrap().to_digit(10).unwrap(),iter.next().unwrap().to_digit(10).unwrap())
                    }
                )
                .collect();
    for (i,_) in s.chars().enumerate(){
        let mut s2 = &s[i..];
        let mut final_idx: usize = i;
        let mut ok = false;
        for (c,min,max) in &enc_seq {
            let len = s2.strip_suffix(s2.trim_start_matches(*c)).unwrap().chars().count() as u32;
            if len >= *min{
                final_idx += if len < *max {len} else {*max} as usize;
                s2 = &s[final_idx..];
            }
            else{
                // to skip at the next char
                ok = true;
                break;
            }
        }
        if ok{
            continue;
        }
        return Some((i,&s[i..final_idx]));
    }
    None
    
}


fn subsequences1<'a,'b>(s: &'a str, seq : &'b str) -> Vec<(usize, &'a str)> {
    let mut subseqs: Vec<(usize, &'a str)> = Vec::new();
    let mut last_idx = 0;
    while last_idx < s.len() {
        let res = find_sub(&s[last_idx..], seq);
        match res {
            Some((i,subs)) => {subseqs.push((i + last_idx,subs)); last_idx += i + subs.len();}
            None => {last_idx += 1;}
        }
    }
    subseqs
}


pub fn demo1() {
    let a = "AACCGGTAACC".to_string();
    let seq = "A1-1,C2-4";

    for (off, sub) in subsequences1(&a, seq) {
        println!("Found subsequence at position {}: {}", off, sub);
    }
}


// Now we want to find different subsequences at the same time, seq is a vector of string slices with many subsequence to search
// For each subsequence find all the matches and to the results (there may be overlaps, ignore them), but in this way you can reuse the previous solution
// The result will contain: the start position in s, the found subsequence as string slice and the mached subsequence in seq
// Now the string slices in the rsult depend from two input parameters, which ones?


fn subsequences2<'a>(s: &'a str, seqs: &'a [&'a str]) -> Vec<(usize, &'a str, &'a str)> {
    let mut subseqs2: Vec<(usize, &'a str, &'a str)> = Vec::new();
    for seq in seqs {
        let res = subsequences1(s, seq);
        if !res.is_empty() {
            for (i,slice) in res {
                subseqs2.push((i,slice,seq));
            }
        }
    }
    subseqs2
}

pub fn demo2() {
    let a = "AACCGGTAACC".to_string();
    let seqs = ["A1-1,C2-4", "G1-1,T1-4"];

    for (off, matched, sub) in subsequences2(&a, &seqs) {
        println!("Found subsequence {} at position {}: {}", matched, off, sub);
    }
}

// Now we want to do some DNA editing! Therefore we receive a mutable string and we'd like to return a vector of mutable string slices
// Follow this steps:
// 1. adjust the lifetimes without any implementation yet: does it compile?
// 2. try to implement the function: does it compile?
// 3. if it doesn't compile, try to understand why from the compiler errors and draw all the necessary lifetimes
// 4. Spoiler: basically it's not possibile to return more then one mutable reference to the same data
// 5. Try this workaround: return a vector of indexes (first solution) and let the caller extract the mutable references
// 7. (later in the course you will learn about smart pointers, which can be used to solve this kind of problems in a more elegant way)
fn subsequences3(s: &str, seq: &str) -> Vec<(usize, usize)> {
    let mut subseqs: Vec<(usize, usize)> = Vec::new();
    let mut last_idx = 0;
    while last_idx < s.len() {
        let res = find_sub(&s[last_idx..], seq);
        match res {
            Some((i,subs)) => {subseqs.push((i + last_idx, i + last_idx + subs.len())); last_idx += i + subs.len();}
            None => {last_idx += 1;}
        }
    }
    subseqs
}

pub fn demo3() {
    let mut a = "AACCGGTAACC".to_string();
    let seq = "A1-1,C1-4";

    for (off, end_off) in subsequences3(&a, seq) {
        println!("Found subsequence at position: {} to {}", off, end_off);
        let mut b = &mut a[off..end_off];
        println!("Mutable ref to: {}", b);
    }
}


// DNA strings may be very long and we can get a lot of matches.
// Therefore we want to process a subsequence as soon as we find it, without storing it in a vector
// A solution is to pass a closure to the function, which will be called for each match
// do you need to put lifetime annotations in the closure? why?

// i don't need lifetime in the closure because doesn't have any return, so the variable don't live after the function
fn subsequence4<F>(s: &str, seq: &str , mut f: F) where F: FnMut(usize,&str) {
    let mut last_idx = 0;
    while last_idx < s.len() {
        let res = find_sub(&s[last_idx..], seq);
        match res {
            Some((i,subs)) => {f(i + last_idx,subs); last_idx += i + subs.len();}
            None => {last_idx += 1;}
        }
    }
}

pub fn demo4() {
    let a = "AACCGGTAACC".to_string();
    let seq = "A1-1,C2-4";

    subsequence4(&a, seq, |pos, sub| {
        println!("Found subsequence at position {}: {}", pos, sub);
    });
}


// Now let's define a struct SimpleDNAIter (add the required lifetimes), memorizing a DNA sequence and the subsequence to search
// Then we add a next() method to the struct, which will return the next subsequence found in the DNA sequence after each call
// The result of next() is a tuple, but it's wrapped in an Option, because a call to next() may find no more subsequences in the DNA sequence
// In order to implement it, you may add any other attribute to the struct (remember: the struct is stateful and after each call to next() you must start from the last position found)
// The struct may be used as shown in the demo_SimpleDNAIter() function
// This approach is similar to the previous one, but it's more flexible and it can be used in more complex scenarios. For example you may interrupt it
// at any time and resume it later

struct SimpleDNAIter<'a,'b> {
    s: &'a str,
    seq: &'b str,
    idx: usize,
}

impl SimpleDNAIter<'_,'_>{
    pub fn new<'a,'b>(s: &'a str, seq: &'b str) -> SimpleDNAIter<'a,'b> {
        SimpleDNAIter { s: s, seq: seq, idx: 0 }
    }

    pub fn next(&mut self) -> Option<(usize, &str)> {
        let res = find_sub(&self.s[self.idx..], self.seq);
        if res.is_some() {
            let (i,slice) = res.unwrap();
            self.idx += i + slice.len();
            return Some((self.idx - slice.len(), slice));
        }
        None
    }
}

pub fn demo_SimpleDNAIter() {
    let mut dna_iter = SimpleDNAIter::new("ACGTACGTACGTACGT", "A1-1,C1-1");

    while let Some((pos, subseq)) = dna_iter.next() {
        println!("Found subsequence at position {}: {}", pos, subseq);
        // we can break and stop if we have found what we were looking for
    }
}


// finally we want to implement a real iterator, so that it can be used in a for loop and it may be combined we all the most common iterator methods
// The struct DNAIter is already defined, you have to implement the Iterator trait for it and add lifetimes
struct DNAIter<'a,'b> {
    s: &'a str,
    seq: &'b str,
    idx: usize
}

impl DNAIter<'_,'_>{
    pub fn new<'a,'b>(s: &'a str, seq: &'b str) -> DNAIter<'a,'b>{
        DNAIter {
            s: s,
            seq: seq,
            idx: 0
        }
    }
}

impl <'a>Iterator for DNAIter<'a, '_>{
    type Item = (usize, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let res = find_sub(&self.s[self.idx..], self.seq);
        if res.is_some() {
            let (i,slice) = res.unwrap();
            self.idx += i + slice.len();
            return Some((self.idx - slice.len(), slice));
        }
        None
    }
}

pub fn demo_dna_iter() {
    let dna_iter = DNAIter::new("ACGTACGTAAACCCGTACGT", "A1-3,C1-2");

    // now you can combine it with all the iterator modifiers!!!
    dna_iter
        .filter(|(pos, sub)| sub.len() >= 5)
        .for_each(|(pos, sub)| {
            println!(
                "Found subsequence at least long 5 at position {}: {}",
                pos, sub
            )
        });
}



// now let's return an iterator without defining a struct, just using a closure
// the std lib of rust support you with the std::from_fn() function
// we supply a skeleton implementation, you have to fill the closure
fn subsequence5_iter<'a,'b>(s: &'a str, seq: &'b str) -> impl Iterator<Item = (usize, &'a str)> where 'b: 'a{
    let mut pos = 0;
    // and any other necessary variable to remember the state
    std::iter::from_fn(move || {
        if let Some((i, slice)) = find_sub(&s[pos..], seq) {
            pos += i + slice.len();
            return  Some((pos - slice.len(), slice));
        } else {
            None
        }
    })
}

pub fn demo_dna_iter2() {
    subsequence5_iter("ACGTACGTAAACCGTACGT", "A1-3,C1-2")
        .filter(|(pos, sub)| sub.len() >= 5)
        .for_each(|(pos, sub)| {
            println!(
                "Found subsequence at least long 5 at position {}: {}",
                pos, sub
            )
        });
}