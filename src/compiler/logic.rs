pub fn eq(a: &String, b: &String) -> String {
    a.to_owned() + "\n" + b + "\n" + "f64.eq"
}

pub fn ne(a: &String, b: &String) -> String {
    a.to_owned() + "\n" + b + "\n" + "f64.ne"
}

pub fn gt(a: &String, b: &String) -> String {
    a.to_owned() + "\n" + b + "\n" + "f64.gt_s"
}

pub fn ls(a: &String, b: &String) -> String {
    a.to_owned() + "\n" + b + "\n" + "f64.ls_s"
}

pub fn ge(a: &String, b: &String) -> String {
    a.to_owned() + "\n" + b + "\n" + "f64.ge_s"
}

pub fn le(a: &String, b: &String) -> String {
    a.to_owned() + "\n" + b + "\n" + "f64.le_s"
}

pub fn and(a: &String, b: &String) -> String {
    a.to_owned() + "\n" + b + "\n" + "i32.and"
}

pub fn or(a: &String, b: &String) -> String {
    a.to_owned() + "\n" + b + "\n" + "i32.or"
}