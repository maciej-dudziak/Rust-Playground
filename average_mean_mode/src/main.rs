use std::collections::HashMap;

fn main() {
    let mut v = vec![4,53,8,8,10,7,3,9,6,3,9,9,9];
    let av = average(&v);
    println!("The average of the vector is {}", av);

    let med = median(&mut v);
    println!("The median is {}", med);

    let mode = mode(&v);
    println!("The mode is {:?}", mode);

}

fn average(v: &Vec<u32>) -> f32 {
    let mut sum = 0;
    let mut lenght = 0;
    for i in v{
        sum = sum + *i;
        lenght=lenght+1;
    }
    sum as f32/lenght as f32
}

fn median(v: &mut Vec<u32>) -> f32 {
    v.sort_unstable();
    let size  = v.len();
    if size%2 == 0 {
        (v[size/2 -1 as usize] as f32 + v[size/2 as usize] as f32) / 2 as f32
    } else {
        v[size/2 as usize] as f32
    }
}

fn mode (v: &Vec<u32>) -> Vec<u32> {
    let mut map = HashMap::new();
    for element in v {
        let count = map.entry(element).or_insert(0);
        *count += 1;
    }
    let mut res : Vec<u32> = Vec::new();
    let mut comp = 0;
    for (key, value) in &map {
        if *value == comp {
            comp = *value;
            res.push(**key);
        } else if *value > comp {
            comp = *value;
            res.clear();
            res.push(**key);
        }
    }
    res
}