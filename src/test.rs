use crate::Quote;

#[cfg(test)]

use super::*;
static BINARY_TEST_DATA: &str = include_str!("../data/test.json");

#[test]
fn from_json_method(){        
    let quotes: Vec<Quote> = Quote::from_json(BINARY_TEST_DATA).unwrap();

    let test_quote_data_1: Quote = Quote{
        author: "bahrom04".to_string(), 
        quotes: vec!["oʻzbekchasi yoʻq ekan".to_string(), "asahi oʻrnatsammikan?".to_string()] 
    };
    let test_quote_data_2: Quote = Quote{
        author: "orzklv".to_string(), 
        quotes: vec!["cooked".to_string(), "koʻkaldosh".to_string()] 
    };

    assert_eq!(quotes, vec![test_quote_data_1, test_quote_data_2])

}
