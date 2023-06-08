use enum_matching::EnumTryFrom;

fn main() {
    let var = MyEnum::try_from(1);
    assert_eq!(var, Ok(MyEnum::First));
}

#[derive(EnumTryFrom, PartialEq, Eq, Debug)]
enum MyEnum {
    First = 1,
    Second,
    Third,
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_number() {
        let var = MyEnum::try_from(1);
        assert_eq!(var.unwrap(), MyEnum::First);
        let var = MyEnum::try_from(2);
        assert_eq!(var.unwrap(), MyEnum::Second);
        let var = MyEnum::try_from(3);
        assert_eq!(var.unwrap(), MyEnum::Third);
    }
    #[test]
    fn invalid_number() {
        let var = MyEnum::try_from(4);
        let expected = Err(enum_matching::Error { num: 4 });
        assert_eq!(expected, var);
    }
}
