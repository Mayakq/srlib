#[cfg(test)]
mod tests {
    use crate::search::Search;

    #[test]
    fn test_absolute_name() {
        let result = Search::new(
            "./".to_string(),
            crate::search::Pallet::Name(".gitignore".to_string()),
            crate::search::Searching::Absolute,
        );
        let res = result.start();
        assert_eq!(*res.unwrap().get(0).unwrap(), "./.gitignore".to_string());
    }
    #[test]
    fn test_relative_name() {
        let result = Search::new(
            "./".to_string(),
            crate::search::Pallet::Name(".gitignor".to_string()),
            crate::search::Searching::Relative,
        );
        let res = result.start();
        assert_eq!(*res.unwrap().get(0).unwrap(), "./.gitignore".to_string());
    }
}
