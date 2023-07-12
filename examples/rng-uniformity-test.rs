use twothousand_forty_eight::random::{lcg_sane, Pickable};

fn main() {
    const OPTIONS: [usize; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut seed = 0;
    let mut count = [0; OPTIONS.len()];
    for _i in 0..1000000 {
        let n = OPTIONS.pick_lcg(&mut seed);
        count[*n] += 1;
    }
    println!("{:?}", count);

    // 2d noise
    let mut seed = 0;
    for _i in 0..20 {
        for _j in 0..80 {
            //let seed = i * j;
            let n = OPTIONS.pick_lcg(&mut seed);
            let to_print = match n {
                0 => " ",
                1 => ".",
                2 => ":",
                3 => ";",
                4 => "+",
                5 => "=",
                6 => "x",
                7 => "X",
                8 => "$",
                9 => "&",
                _ => "",
            };
            print!("{}", to_print);
        }
        println!();
    }
    let mut state = 0;
    for i in 0..100 {
        let result = lcg_sane(&mut state);
        println!("{}, {}", i, result);
    }
}
