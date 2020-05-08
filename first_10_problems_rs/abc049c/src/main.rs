use std::io::stdin;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s = s.trim().to_string();

    let mut i = 0;
    let mut was_str = "er";
    let mut res = "YES";
    loop {
        // successfully finished
        if s.len() == i {
            break;
        }

        if s.len() >= i + 5 {
            if ["dream", "erase"].contains(&&s[i..i+5]) {
                was_str = &s[i..i+5];
                i += 5;
                continue;
            }
        }
        if (s.len() >= i + 2) & (was_str == "dream") {
            if &s[i..i+2] == "er" {
                was_str = &s[i..i+2];
                i += 2;
                continue;
            }
        }
        if (s.len() >= i + 1) & (was_str == "erase") {
            if &s[i..i+1] == "r" {
                was_str = &s[i..i+1];
                i += 1;
                continue;
            }
        }

        res = "NO";
        break;
    }
    
    println!("{}", res);
}
