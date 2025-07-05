use std::iter::Peekable;

use crate::java_type::JavaType;

//TODO: probably use global err
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodDescriptorErr {
    ShouldStartWithParentheses,
    MissingClosingParenthesis,
    UnexpectedEnd,
    InvalidType,
    TrailingCharacters,
}

/// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.3.2
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodDescriptor {
    params: Vec<JavaType>,
    ret: JavaType,
}

impl MethodDescriptor {
    fn parse_type<I>(it: &mut Peekable<I>) -> Result<JavaType, MethodDescriptorErr>
    where
        I: Iterator<Item = char>,
    {
        let c = it.next().ok_or(MethodDescriptorErr::UnexpectedEnd)?;

        if let Ok(base) = JavaType::try_from(c) {
            return Ok(base);
        }

        match c {
            'L' => {
                let mut instance_name = String::new();
                while let Some(&next) = it.peek() {
                    it.next();
                    if next == ';' {
                        return Ok(JavaType::Instance(instance_name));
                    }
                    instance_name.push(next);
                }
                Err(MethodDescriptorErr::UnexpectedEnd)
            }
            '[' => {
                let elem = Self::parse_type(it)?;
                Ok(JavaType::Array(Box::new(elem)))
            }
            _ => Err(MethodDescriptorErr::InvalidType),
        }
    }
}

impl TryFrom<&str> for MethodDescriptor {
    type Error = MethodDescriptorErr;

    fn try_from(desc: &str) -> Result<Self, Self::Error> {
        let mut chars = desc.chars().peekable();

        if chars.next() != Some('(') {
            return Err(MethodDescriptorErr::ShouldStartWithParentheses);
        }

        let mut params = Vec::new();
        loop {
            match chars.peek() {
                Some(')') => {
                    chars.next();
                    break;
                }
                Some(_) => params.push(Self::parse_type(&mut chars)?),
                None => return Err(MethodDescriptorErr::MissingClosingParenthesis),
            }
        }

        let ret = Self::parse_type(&mut chars)?;

        if chars.next().is_some() {
            return Err(MethodDescriptorErr::TrailingCharacters);
        }

        Ok(MethodDescriptor { params, ret })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void add(int, int)
    #[test]
    fn parse_two_ints_void() {
        // given
        let signature = "(II)V";
        let expected_param = vec![JavaType::Int, JavaType::Int];
        let expected_ret = JavaType::Void;

        // when
        let md = MethodDescriptor::try_from(signature).unwrap();

        // then
        assert_eq!(md.params, expected_param);
        assert_eq!(md.ret, expected_ret);
    }

    // Java: int getCount()
    #[test]
    fn parse_no_params_int_return() {
        // given
        let signature = "()I";
        let expected_param: Vec<JavaType> = Vec::new();
        let expected_ret = JavaType::Int;

        // when
        let md = MethodDescriptor::try_from(signature).unwrap();

        // then
        assert_eq!(md.params, expected_param);
        assert_eq!(md.ret, expected_ret);
    }

    // Java: String echo(String)
    #[test]
    fn parse_string_param_string_return() {
        // given
        let signature = "(Ljava/lang/String;)Ljava/lang/String;";
        let expected_param = vec![JavaType::Instance("java/lang/String".into())];
        let expected_ret = JavaType::Instance("java/lang/String".into());

        // when
        let md = MethodDescriptor::try_from(signature).unwrap();

        // then
        assert_eq!(md.params, expected_param);
        assert_eq!(md.ret, expected_ret);
    }

    // Java: int[] process(int, String[])
    #[test]
    fn parse_array_param_and_return() {
        // given
        let signature = "(I[Ljava/lang/String;)[I";
        let expected_param = vec![
            JavaType::Int,
            JavaType::Array(Box::new(JavaType::Instance("java/lang/String".into()))),
        ];
        let expected_ret = JavaType::Array(Box::new(JavaType::Int));

        // when
        let md = MethodDescriptor::try_from(signature).unwrap();

        // then
        assert_eq!(md.params, expected_param);
        assert_eq!(md.ret, expected_ret);
    }

    // Java: Object[][] flatten(Object[][])
    #[test]
    fn parse_multi_dimensional_arrays() {
        // given
        let signature = "([[Ljava/lang/Object;)[[Ljava/lang/Object;";
        let obj = JavaType::Instance("java/lang/Object".into());
        let two_d_obj = JavaType::Array(Box::new(JavaType::Array(Box::new(obj.clone()))));
        let expected_param = vec![two_d_obj.clone()];
        let expected_ret = two_d_obj;

        // when
        let md = MethodDescriptor::try_from(signature).unwrap();

        // then
        assert_eq!(md.params, expected_param);
        assert_eq!(md.ret, expected_ret);
    }
}
