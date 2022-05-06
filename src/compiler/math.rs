pub fn sum(a: &String, b: &String) -> String {
    a.to_owned() + "\n" + b + "\n" + "f64.add"
}

pub fn sub(a: &String, b: &String) -> String {
    a.to_owned() + "\n" + b + "\n" + "f64.sub"
}

pub fn mul(a: &String, b: &String) -> String {
    a.to_owned() + "\n" + b + "\n" + "f64.mul"
}

pub fn div(a: &String, b: &String) -> String {
    a.to_owned() + "\n" + b + "\n" + "f64.div"
}