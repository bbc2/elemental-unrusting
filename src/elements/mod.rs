extern crate im;
extern crate serde;
extern crate serde_json;

use std::iter::FromIterator;

use self::im::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Element {
    pub symbol: String,
    pub atomic_number: u32,
}

#[derive(Deserialize)]
struct JsonElement {
    number: u32,
    symbol: String,
}

#[derive(Deserialize)]
struct Json {
    elements: Vec<JsonElement>,
}

fn json_element_to_element(json_element: JsonElement) -> Element {
    return Element {
        atomic_number: json_element.number,
        symbol: json_element.symbol,
    };
}

fn json_to_elements(json: Json) -> Vector<Element> {
    return Vector::from_iter(json.elements.into_iter().map(json_element_to_element));
}

pub fn load() -> Vector<Element> {
    let string = String::from(include_str!("periodic_table.json"));
    let json_elements = serde_json::from_str(&string).unwrap();
    return json_to_elements(json_elements);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        let result = load();

        assert_eq!(119, result.len());
    }
}
