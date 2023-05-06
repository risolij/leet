use std::ops::Not;

pub fn simplify_path(path: String) -> String {
    let mut paths = vec![];

    path.split('/')
        .filter(|&dir| dir.eq(".").not())
        .filter(|&dir| dir.is_empty().not())
        .for_each(|dir| {
            match dir {
                ".." => { paths.pop(); },
                _    => paths.push(dir.to_owned()),
            }
        });

    format!("/{}", paths.join("/"))
}

fn main() {
    println!("{}", simplify_path("/test/../home".to_string()));
}
