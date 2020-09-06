use std::io::stdin;
use std::str::FromStr;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn read_line<T>() -> Vec<T>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn read_str_as_char_vec() -> Vec<char> {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim().chars().collect()
}

fn read_str_as_T<T>() -> T
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim().parse().unwrap()
}

// fn dfs(mut hopped_rooms: &mut Vec<usize>,
//        the_room: usize,
//        path_map: &HashMap<usize, Vec<usize>>,
//        mut accessed_rooms: &mut HashSet<usize>) -> (i64, Vec<usize>) {
//     if the_room == 1 && hopped_rooms.len() > 0{
//         ;
//     }
//     let mut best_hop_num: i64 = ;
//     let mut best_hopped_rooms: Vec<usize> = Vec::new();
//     for &next_room in path_map.get(&the_room).unwrap() {
//         if !accessed_rooms.contains(&next_room) {
//             hopped_rooms.push(next_room);
//             accessed_rooms.insert(next_room);
//             let (temp_hop_num, temp_hopped_rooms) = dfs(&mut hopped_rooms, next_room, &path_map, &mut accessed_rooms);
//             if temp_hop_num < best_hop_num {
//                 best_hop_num = temp_hop_num;
//                 best_hopped_rooms = temp_hopped_rooms;
//             }
//         }
//     }
//     (best_hop_num, hopped_rooms)
// }

fn dfs(target_room: usize, mut res_vec: &mut Vec<usize>, path_map: &HashMap<usize, Vec<usize>>, mut accessed_rooms: &mut HashSet<usize>) -> (bool, Vec<usize>) {
    if accessed_rooms.len() == res_vec.len() {
        return (true, res_vec.to_vec());
    }
    for &next_room in path_map.get(&target_room).unwrap() {
         if !accessed_rooms.contains(&next_room) {
             accessed_rooms.insert(next_room);
             res_vec[next_room - 2] = target_room;
             let (temp_res, temp_res_vec) = dfs(next_room, &mut res_vec, &path_map, &mut accessed_rooms);
             if temp_res {
                 return (temp_res, temp_res_vec)
             }
         }
    }
    (false, vec![])
}

// fn bfs(target_room: usize, mut res_vec: &mut Vec<usize>, path_map: &HashMap<usize, Vec<usize>>, mut accessed_rooms: &mut VecDeque<usize>) -> (bool, Vec<usize>) {
//     if accessed_rooms.len() == res_vec.len() {
//         return (true, res_vec.to_vec());
//     }
//     for &next_room in path_map.get(&target_room).unwrap() {
//          if !accessed_rooms.contains(&next_room) {
//              accessed_rooms.insert(next_room);
//              res_vec[next_room - 2] = target_room;
//              let (temp_res, temp_res_vec) = dfs(next_room, &mut res_vec, &path_map, &mut accessed_rooms);
//              if temp_res {
//                  return (temp_res, temp_res_vec)
//              }
//          }
//     }
//     (false, vec![])
// }

fn main() {
    let in_vec: Vec<usize> = read_line();
    let n = in_vec[0];
    let m = in_vec[1];

    let mut path_map: HashMap<usize, Vec<usize>> = HashMap::new();
    for _i in 0..m {
        let in_vec: Vec<usize> = read_line();
        let a_i = in_vec[0];
        let b_i = in_vec[1];
        (*path_map.entry(a_i).or_insert(vec![])).push(b_i);
        (*path_map.entry(b_i).or_insert(vec![])).push(a_i);
    }

    let mut accessed_rooms: HashSet<usize> = HashSet::new();
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(1);
    accessed_rooms.insert(1);
    // let (res, res_vec) = dfs(0, 1, &path_map, &mut accessed_rooms);
    let mut res_vec = vec![0; n - 1];
    // let (res, res_vec) = dfs(1, &mut res_vec, &path_map, &mut accessed_rooms);
    while queue.len() > 0 {
        let target_room = queue.pop_front().unwrap();
        for &next_room in path_map.get(&target_room).unwrap(){
            if !accessed_rooms.contains(&next_room) {
                accessed_rooms.insert(next_room);
                res_vec[next_room - 2] = target_room;
                queue.push_back(next_room);
            } 
        }
    }

    if accessed_rooms.len() == n {
        println!("Yes");
        for res_i in res_vec {
            println!("{}", res_i);
        }
    } else {
        println!("No");
    }
}
