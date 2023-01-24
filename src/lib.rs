pub fn a_b_c() {
    let abc = "A_B_C";
    println!("say {}", abc);
}

#[cfg(test)]
mod tests {

    use crate::a_b_c;

    #[test]
    fn test1() {
        a_b_c();
    }
}