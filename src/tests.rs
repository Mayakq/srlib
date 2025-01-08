#[cfg(test)]
mod tests {
    use regex::Regex;

    use crate::search::Search;

    #[test]
    fn absolute_name() {
        let binding = crate::search::Pallet::Name(".gitignore".to_string());
        let path = "./".to_string();
        let result = Search::new(&path, &binding, &crate::search::Searching::Absolute);
        let res = result.start();
        assert_eq!(*res.unwrap().get(0).unwrap(), "./.gitignore".to_string());
    }
    #[test]
    fn relative_name() {
        let binding = crate::search::Pallet::Name(".gitignor".to_string());
        let path = "./".to_string();
        let result = Search::new(&path, &binding, &crate::search::Searching::Relative);
        let res = result.start();
        assert_eq!(*res.unwrap().get(0).unwrap(), "./.gitignore".to_string());
    }
    #[test]
    fn regex1() {
        let binding = crate::search::Pallet::Regular(Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap());
        let path = "./tests/".to_string();
        let result = Search::new(&path, &binding, &crate::search::Searching::Absolute);
        let res = result.start();
        assert_eq!(
            *res.unwrap().get(0).unwrap(),
            "./tests/2010-03-14".to_string()
        );
    }
    #[test]
    fn regex2() {
        let binding =
            crate::search::Pallet::Regular(Regex::new(r"^\d{4}-\d{2}-\d{2}$*\.txt").unwrap());
        let path = "./tests/".to_string();
        let result = Search::new(&path, &binding, &crate::search::Searching::Absolute);
        let res = result.start();
        assert_eq!(
            *res.unwrap().get(0).unwrap(),
            "./tests/2010-03-14.txt".to_string()
        );
    }
}
