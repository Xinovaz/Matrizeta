#![allow(non_snake_case)]
#![allow(dead_code)]

#[macro_export]
macro_rules! func {
    ( $v:ident ; $def:block ; $s:expr  ) => {
        Box::new(ZetaObject::Function((move |$v: &mut ZetaObject| { $def }), $s.to_string()))
    };
}

#[macro_export]
macro_rules! omega {
    () => {
        Box::new(Failure())
    }
}

#[macro_export]
macro_rules! empty {
    () => {
        Box::new(Success())
    }
}

#[macro_export]
macro_rules! zn {
    ( $n:expr ) => {
        Box::new(ZetaObject::from_int($n))
    }
}

use std::fmt;

struct ZetaResult {
    operator: ZetaObject,
    operand: ZetaObject,
    failures: isize,
    successes: isize,
    result: ZetaObject,
}

impl fmt::Display for ZetaResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tried:\n{}\nWith:\n{}\nSuccesses: {}\nFailures: {}\nResult:\n{}", self.operator, self.operand, self.successes, self.failures, self.result)
    }
}

type ZetaFunction = fn(&mut ZetaObject) -> ZetaObject;

#[derive(Clone)]
enum ZetaObject {
    Matrix(Vec<Vec<Box<ZetaObject>>>),
    Null,
    Function(ZetaFunction, String),
}

impl ZetaObject {
    fn is_null(&self) -> bool {
        match self {
            ZetaObject::Null => true,
            _ => false,
        }
    }

    fn is_matrix(&self) -> bool {
        match self {
            ZetaObject::Matrix(_) => true,
            _ => false,
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            ZetaObject::Matrix(mtx) => {
                let mut empty = true;
                for row in mtx {
                    if empty {
                        empty = row.len() != 0;
                    }
                }
                empty
            },
            _ => false,
        }
    }

    fn is_function(&self) -> bool {
        match self {
            ZetaObject::Function(_, _) => true,
            _ => false,
        }
    }

    fn from_int(n: isize) -> ZetaObject {
        let mut v: Vec<Vec<Box<ZetaObject>>> = Vec::new();
        for _ in 0..n.abs() {
            v.push(vec![omega!()]);
        }
        if n < 0 {
            v.push(vec![empty!()]);
        }
        ZetaObject::Matrix(v)
    }

    fn to_int(&self) -> isize {
        match self {
            ZetaObject::Matrix(mtx) => {
                let mut count: isize = 0;
                for row in mtx {
                    if row.get(0).unwrap().is_null() {
                        count += 1;
                    }
                }
                if mtx.len() >= 1 {
                    if mtx.get(mtx.len() - 1)
                        .unwrap().get(0)
                        .unwrap().is_matrix() {
                        count *= -1;
                    }
                }
                count
            },
            _ => isize::MIN,
        }
    }

    fn evaluate_object(&self, object: &mut ZetaObject) -> ZetaObject {
        match self {
            ZetaObject::Function(f, _) => {
                f(object)
            },
            _ => self.clone(),
        }
    }

    fn evaluate(self, object: &mut ZetaObject) -> ZetaResult {
        let operator = self.clone();
        match self {
            ZetaObject::Matrix(mtx) => {

                let mut row: usize = 0;
                let mut col: usize = 0;

                let mut result = Success();

                while row < mtx.len() && col < mtx.get(0).unwrap().len() {
                    let ptr = mtx.get(row).unwrap().get(col).unwrap();
                    result = ptr.evaluate_object(object);
                    if result.is_null() {
                        row += 1;
                    } else {
                        col += 1;
                    }
                }

                ZetaResult {
                    operator,
                    operand: object.clone(),
                    failures: row.try_into().unwrap(),
                    successes: col.try_into().unwrap(),
                    result,
                }

            },
            _ => ZetaResult { operator, operand: ZetaObject::Null, failures: 0, successes: 0, result: self},
        }
    }
}

impl fmt::Display for ZetaObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result: String = match self {
            ZetaObject::Null => {
                "*O*".to_string()
            },
            ZetaObject::Matrix(M) => {
                let mut s = String::from("⎡ ");
                if M.len() <= 1 {
                    s = String::from("[ ");
                }
                let mut rn: usize = 0;
                let mut colsizes = Vec::new();
                for _ in 0..M.get(0).unwrap().len() {
                    colsizes.push(0);
                }
                #[allow(unused_assignments)]
                let mut coln = 0; //haha colon
                for row in M {
                    coln = 0;
                    for item in row {
                        let mut clen = format!("{}", item).chars().count();
                        match &**item {
                            ZetaObject::Matrix(mtx) => {
                                if mtx.len() > 1 {
                                    clen = 3;
                                }
                            },
                            _ => {}
                        }
                        if clen > colsizes[coln] {
                            colsizes[coln] = clen;
                        }
                        let mut spaces = "".to_string();
                        for _ in (clen-1)..colsizes[coln] {
                            spaces.push(' ');
                        }
                        coln += 1;
                    }
                }
                for row in M {
                    coln = 0;
                    for item in row {
                        let clen = format!("{}", item).chars().count();
                        let mut spaces = "".to_string();
                        for _ in (clen-1)..colsizes[coln] {
                            spaces.push(' ');
                        }
                        let istr = match &**item {
                            ZetaObject::Matrix(mtx) => {
                                if mtx.len() > 1 {
                                    String::from("*Z* ")
                                } else {
                                    format!("{}{}", item, spaces)
                                }
                            },
                            _ => format!("{}{}", item, spaces)
                        };
                        s.push_str(istr.as_str());
                        coln += 1;
                    }
                    if rn + 1 == M.len() {
                        if M.len() <= 1 {
                            s.push_str("]");
                        } else {
                            s.push_str("⎦");
                        }
                    } else if rn == 0 {
                        s.push_str("⎤\n");
                    } else {
                        s.push_str("⎥\n");
                    }

                    if rn + 2 == M.len() {
                        s.push_str("⎣ ");
                    } else if rn + 2 < M.len() {
                        s.push_str("⎢ ");
                    }

                    rn += 1;
                }
                s
            },
            ZetaObject::Function(_, s) => {
                let mut end = String::from("{");
                end.push_str(s);
                end.push_str("}");
                end
            },
        };
        write!(f, "{}", result)
    }
}

fn Success() -> ZetaObject {
    ZetaObject::Matrix(vec![vec![]])
}

fn Failure() -> ZetaObject {
    ZetaObject::Null
}

fn add(N: ZetaObject, A: ZetaObject) -> ZetaObject {
    ZetaObject::from_int(N.to_int() + A.to_int())
}

fn sub(N: ZetaObject, A: ZetaObject) -> ZetaObject {
    ZetaObject::from_int(N.to_int() - A.to_int())
}

fn mul(N: ZetaObject, A: ZetaObject) -> ZetaObject {
    ZetaObject::from_int(N.to_int() * A.to_int())
}

fn div(N: ZetaObject, A: ZetaObject) -> ZetaObject {
    if A.is_empty() {
        Failure()
    } else {
        ZetaObject::from_int(N.to_int() / A.to_int())
    }
}

fn vertical_union(N: ZetaObject, A: ZetaObject) -> ZetaObject {
    let mut final_vector: Vec<Vec<Box<ZetaObject>>> = Vec::new();
    match N {
        ZetaObject::Matrix(ref Nmtx) => {
            match A {
                ZetaObject::Matrix(ref Amtx) => {
                    if Nmtx.get(0).unwrap().len() < Amtx.get(0).unwrap().len() {
                        for row in Nmtx {
                            let mut new_row = row.to_vec();
                            for _ in Nmtx.get(0).unwrap().len()..Amtx.get(0).unwrap().len() {
                                new_row.push(omega!());
                            }
                            final_vector.push(new_row);
                        }
                        for row in Amtx {
                            final_vector.push(row.to_vec());
                        }
                    } else if Nmtx.get(0).unwrap().len() > Amtx.get(0).unwrap().len() {
                        for row in Nmtx {
                            final_vector.push(row.to_vec());
                        }
                        for row in Amtx {
                            let mut new_row = row.to_vec();
                            for _ in Amtx.get(0).unwrap().len()..Nmtx.get(0).unwrap().len() {
                                new_row.push(omega!());
                            }
                            final_vector.push(new_row);
                        }
                    } else {
                        final_vector = Nmtx.clone();
                        for row in Amtx {
                            final_vector.push(row.to_vec());
                        }
                    }
                    ZetaObject::Matrix(final_vector)
                },
                _ => vertical_union(N, ZetaObject::Matrix(vec![vec![Box::new(A)]]))
            }
        },
        _ => vertical_union(ZetaObject::Matrix(vec![vec![Box::new(N)]]), A)
    }
}

fn horizontal_union(N: ZetaObject, A: ZetaObject) -> ZetaObject {
    let mut final_vector: Vec<Vec<Box<ZetaObject>>> = Vec::new();
    match N {
        ZetaObject::Matrix(ref Nmtx) => {
            match A {
                ZetaObject::Matrix(ref Amtx) => {
                    if Nmtx.len() < Amtx.len() {
                        for i in 0..Nmtx.len() {
                            let mut new_row = Nmtx.get(i).unwrap().to_vec();
                            for item in Amtx.get(i).unwrap() {
                                new_row.push(item.clone());
                            }
                            final_vector.push(new_row);
                        }
                        for i in Nmtx.len()..Amtx.len() {
                            let mut new_row = Vec::new();
                            for _ in 0..Nmtx.get(0).unwrap().len() {
                                new_row.push(omega!());
                            }
                            for item in Amtx.get(i).unwrap() {
                                new_row.push(item.clone());
                            }
                            final_vector.push(new_row);
                        }
                    } else if Amtx.len() < Nmtx.len() {
                        for i in 0..Amtx.len() {
                            let mut new_row = Nmtx.get(i).unwrap().to_vec();
                            for item in Amtx.get(i).unwrap() {
                                new_row.push(item.clone());
                            }
                            final_vector.push(new_row);
                        }
                        for i in Amtx.len()..Nmtx.len() {
                            let mut new_row = Nmtx.get(i).unwrap().to_vec();
                            for _ in 0..Amtx.get(0).unwrap().len() {
                                new_row.push(omega!());
                            }
                            final_vector.push(new_row);
                        }
                    } else {
                        for i in 0..Nmtx.len() {
                            let mut new_row = Nmtx.get(i).unwrap().to_vec();
                            for item in Amtx.get(i).unwrap() {
                                new_row.push(item.clone());
                            }
                            final_vector.push(new_row);
                        }
                    }
                    ZetaObject::Matrix(final_vector)
                },
                _ => vertical_union(N, ZetaObject::Matrix(vec![vec![Box::new(A)]]))
            }
        },
        _ => vertical_union(ZetaObject::Matrix(vec![vec![Box::new(N)]]), A)
    }
}

fn main() {
    let mut a = ZetaObject::Matrix(vec![vec![empty!()]]);
    //let adder = move |v: &mut ZetaObject| add(v.clone(), ZetaObject::from_int(3));
    let mtx = ZetaObject::Matrix(vec![
    vec![func!(A;{horizontal_union(ZetaObject::Matrix(vec![vec![empty!()], vec![empty!()], vec![empty!()]]),A.clone())};"HU([ [ ]x3 ], A)")],
    ]);
    println!("{}", mtx.evaluate(&mut a));
}