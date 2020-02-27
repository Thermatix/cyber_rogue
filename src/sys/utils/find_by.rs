#[derive(PartialEq, Debug)]
pub enum FindBy<'f> {
    U(&'f usize),
    S(&'f str),
    ST(&'f String),
}

impl<'f> From<&'f usize> for FindBy<'f> {
    fn from(v: &'f usize) -> Self {
        Self::U(v)
    }
}

impl<'r, 'f: 'r> From<FindBy<'f>> for &'r usize {
    fn from(v: FindBy<'f>) -> &'r usize {
        match v {
            FindBy::U(n) => n,
            _ => panic!("{:?} was not a usize", v),
        }
    }
}

impl<'f> From<&'f str> for FindBy<'f> {
    fn from(v: &'f str) -> Self {
        Self::S(v)
    }
}

impl<'r, 'f: 'r> From<FindBy<'f>> for &'r str {
    fn from(v: FindBy<'f>) -> &'r str {
        match v {
            FindBy::S(s) => s,
            _ => panic!("{:?} was not a str", v),
        }
    }
}

impl<'f> From<&'f String> for FindBy<'f> {
    fn from(v: &'f String) -> Self {
        Self::ST(v)
    }
}

impl<'r, 'f: 'r> From<FindBy<'f>> for &'r String {
    fn from(v: FindBy<'f>) -> &'r String {
        match v {
            FindBy::ST(st) => st,
            _ => panic!("{:?} was not a usize", v),
        }
    }
}
