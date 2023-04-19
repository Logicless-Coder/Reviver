use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("logs.txt").expect("Opening file");
    let mut logs = String::new();
    file.read_to_string(&mut logs).expect("Reading file");

    let mut redo_list: Vec<u32> = Vec::new();
    let mut undo_list: Vec<u32> = Vec::new();

    let check_list: Vec<u32>;
    for line in logs.lines().rev() {
        if line.starts_with("checkpoint") {
            let mut trans = line.split(" ");
            trans.next();

            check_list = trans
                .map(|x| x.strip_prefix("T").unwrap().parse().expect("Cannot parse"))
                .collect();
            for t in check_list {
                if !redo_list.contains(&t) {
                    undo_list.push(t);
                }
            }
            break;
        }

        let mut parts = line.split('.');
        let left = parts.next();
        let right = parts.next();

        let typ: &str;
        match right {
            Some(t) => typ = t,
            None => panic!("Cannot happen"),
        }

        // if right.is_some_and(|x| x == "commit") {
        if typ == "commit" {
            let num: u32 = left
                .unwrap()
                .strip_prefix("T")
                .unwrap()
                .parse()
                .expect("Should be parsable");
            redo_list.push(num);
        }
        // else if right.is_some_and(|x| x == "start") {
        else if typ == "start" {
            let num: u32 = left
                .unwrap()
                .strip_prefix("T")
                .unwrap()
                .parse()
                .expect("Should be parsable");
            if !redo_list.contains(&num) {
                undo_list.push(num);
            }
        }
    }

    println!("Redo: {:?}", redo_list);
    println!("Undo: {:?}", undo_list);
}
