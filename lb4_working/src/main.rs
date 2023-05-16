fn rand(seed: &mut u32, min_rand: u32, max_rand: u32) -> u32 {
    let mut tmp: u32 = *seed;
    *seed = *seed+1;

    for i in 1..10 {
        tmp *= *seed+i;
    }

    tmp = tmp % (max_rand - min_rand) + min_rand;

    tmp
}

fn swap_arr(arr: &mut [i32], i: usize, j:usize){
    let tmp = arr[i];
    arr[i] = arr[j];
    arr[j] = tmp;
}

fn rand_perm(arr: &mut[i32], seed: &mut u32){
    let mut index = 0;
    for i in 0..arr.len() {
        let tmp = rand(seed, 0, (arr.len() - 1) as u32);
        swap_arr(arr, index, tmp as usize);
        index += 1;
    }
}

fn main() {
    let mut seed: u32 = 5;
    let mut arr = [1, 2, 3, 4, 5];
    for i in 1..10{
        println!("{}", rand(&mut seed, 8, 100));
    }
    swap_arr(&mut arr, 2, 4);
    for i in 0..6 {
        println!("{}", arr[i]);
    }

    rand_perm(&mut arr, &mut seed);
    for i in 0..6 {
        println!("{}", arr[i]);
    }


}