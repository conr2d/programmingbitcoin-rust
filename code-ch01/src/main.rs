mod ecc;

use ecc::FieldElement;

fn main() {
    let mut a: FieldElement;
    let mut b: FieldElement;
    let mut c: FieldElement;

    a = FieldElement::new(7, 13);
    b = FieldElement::new(6, 13);
    println!("{}", a == b);
    println!("{}", a == a);

    a = FieldElement::new(7, 13);
    b = FieldElement::new(12, 13);
    c = FieldElement::new(6, 13);
    println!("{}", a + b == c);

    a = FieldElement::new(3, 13);
    b = FieldElement::new(12, 13);
    c = FieldElement::new(10, 13);
    println!("{}", a * b == c);

    a = FieldElement::new(3, 13);
    b = FieldElement::new(1, 13);
    println!("{}", a.pow(3) == b);

    a = FieldElement::new(7, 13);
    b = FieldElement::new(8, 13);
    println!("{}", a.pow(-3) == b);
}
