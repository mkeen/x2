use tinyvec::ArrayVec;

pub fn csv<'a, T>(list: &[T]) -> String
where
    T: Clone + Into<&'static str> + super::EnumCount,
{
    list.iter()
        .map(|t| t.clone().into())
        .collect::<ArrayVec<[&str; 24]>>() // todo choose a more accurate max array size based on T
        .join(",")
}

#[cfg(test)]
mod tests {
    use crate::model::users::Field as UserField;

    use super::*;

    #[test]
    fn csv_length() {
        let test_input = [
            UserField::CreatedAt,
            UserField::Description,
            UserField::Entities,
            UserField::Id,
        ];

        let c = csv(&test_input);

        assert_eq!(c.split(",").collect::<Vec<&str>>().len(), test_input.len());
    }
}
