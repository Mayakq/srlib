#[cfg(test)]
mod tests {
    use regex::Regex;

    use crate::search::Search;

    #[test]
    fn absolute_name() {
        let result = Search::new(
            "./".to_string(),
            crate::search::Pallet::Name(".gitignore".to_string()),
            crate::search::Searching::Absolute,
        );
        let res = result.start();
        assert_eq!(*res.unwrap().get(0).unwrap(), "./.gitignore".to_string());
    }
    #[test]
    fn relative_name() {
        let result = Search::new(
            "./".to_string(),
            crate::search::Pallet::Name(".gitignor".to_string()),
            crate::search::Searching::Relative,
        );
        let res = result.start();
        assert_eq!(*res.unwrap().get(0).unwrap(), "./.gitignore".to_string());
    }
    #[test]
    fn regex1() {
        let result = Search::new(
            "./tests/".to_string(),
            crate::search::Pallet::Regular(Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()),
            crate::search::Searching::Absolute,
        );
        let res = result.start();
        assert_eq!(*res.unwrap().get(0).unwrap(), "./tests/2010-03-14".to_string());
    }
    #[test]
    fn regex2() {
        let result = Search::new(
            "./tests/".to_string(),
            crate::search::Pallet::Regular(Regex::new(r"^\d{4}-\d{2}-\d{2}$*\.txt").unwrap()),
            crate::search::Searching::Absolute,
        );
        let res = result.start();
        assert_eq!(*res.unwrap().get(0).unwrap(), "./tests/2010-03-14.txt".to_string());
    }
}
