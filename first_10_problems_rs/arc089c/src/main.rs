use std::io::stdin;

fn main() {
    let mut s = String::new();

    stdin().read_line(&mut s).ok();
    let n: i32 = s.trim().parse().unwrap();
    s.clear();

    let mut former_t = 0;
    let mut former_travel_plan = vec![0, 0, 0];
    let mut travel_plan: Vec<i32>;

    let mut res = "Yes";

    for _i in 0..n {
        stdin().read_line(&mut s).ok();
        travel_plan = s
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let t_diff = travel_plan[0] - former_t;
        let place_diff = (travel_plan[1] - former_travel_plan[1]).abs()
            + (travel_plan[2] - former_travel_plan[2]).abs();
        if (t_diff >= place_diff) & ((t_diff - place_diff) % 2 == 0){
            former_t = travel_plan[0];
            former_travel_plan = travel_plan;
            s.clear();
        } else {
            res = "No";
            break;
        }
    }

    println!("{}", res);
//     let mut travel_plan = Vec::<(i32, i32, i32)>::new();
//     travel_plan.push((0, 0, 0));
// 
//     for _i in 0..n {
//         stdin().read_line(&mut s).ok();
//         let travel_plan_unit: Vec<i32> = s
//             .trim()
//             .split_whitespace()
//             .map(|x| x.parse().unwrap())
//             .collect();
//         travel_plan.push((
//             travel_plan_unit[0],
//             travel_plan_unit[1],
//             travel_plan_unit[2],
//         ));
//         s.clear();
//     }
}
