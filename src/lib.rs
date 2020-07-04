mod quantity;
mod defs;

pub use quantity::Quantity;

#[cfg(test)]
mod test {

    use crate::quantity::Quantity;

    #[test]
    fn retrieve_and_add() {
        let m = Quantity::from_unit("m").unwrap();
        let inch = Quantity::from_unit("in").unwrap();
        let sum = m.add(&inch);
        println!("m + ft = {:?}", sum);
    } 
}