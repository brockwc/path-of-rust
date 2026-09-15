pub fn banner() -> &'static str {
    "Path of Rust"
}

fn main() {
    println!("{}: the dungeon is not built yet.", banner());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn banner_is_set() {
        assert_eq!(banner(), "Path of Rust");
    }
}
