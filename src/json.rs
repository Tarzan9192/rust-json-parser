pub mod person;
pub mod remote_jobs;

use person::Person;
use remote_jobs::RemoteJobs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
/// Represents the json strucures that can be parsed by the program.
/// Need to use the `untagged` option for serde.
///
/// ## From the serde docs for `untagged`:
/// *There is no explicit tag identifying which variant the data contains. Serde will try to match the data against each variant in order and the first one that deserializes successfully is the one returned. This representation can handle enums containing any type of variant.*
///
/// See further details -> [here](https://serde.rs/enum-representations.html)
#[derive(PartialEq)]
pub enum Parsed {
    Person(Person),
    RemoteJobs(RemoteJobs),
}

pub fn parse<R>(reader: R) -> Option<Parsed>
where
    R: std::io::Read,
{
    let deserialized: Result<Parsed, _> = serde_json::from_reader(reader);
    match deserialized {
        Ok(Parsed::Person(person)) => Some(Parsed::Person(person)),
        Ok(Parsed::RemoteJobs(jobs)) => Some(Parsed::RemoteJobs(jobs)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {

    use remote_jobs::{RemoteJobCategory, Website};
    use std::io::BufReader;

    use super::*;

    #[test]
    fn parse_not_parsable() {
        // arrange
        let json = "{ key: \"some value\" }";
        let reader = BufReader::new(json.as_bytes());

        // act
        let result = parse(reader);

        // assert
        assert!(result.is_none())
    }

    #[test]
    fn parse_person() {
        let json = "{ 
            \"name\": \"John Doe\",
            \"age\": 30,
            \"email\": \"john@example.com\",
            \"phones\": [
                \"+1-555-555-1234\",
                \"+1-555-555-5678\"
            ]
        }";
        let reader = BufReader::new(json.as_bytes());

        let result = parse(reader);

        // assert
        assert_eq!(
            result,
            Some(Parsed::Person(Person {
                name: "John Doe".to_string(),
                age: 30,
                email: Some("john@example.com".to_string()),
                phones: vec!["+1-555-555-1234".to_string(), "+1-555-555-5678".to_string()]
            }))
        );
    }

    #[test]
    fn parse_remote_jobs() {
        let category = "Major Job Platforms with Strong Remote Filters";
        let name = "LinkedIn";
        let url = "https://www.linkedin.com";
        let json = format!(
            "{{ 
            \"remote_sites\": [
                    {{
                        \"category\": \"{}\",
                        \"sites\": [
                            {{
                                \"name\": \"{}\",
                                \"url\": \"{}\"
                            }}
                        ]
                    }}
                ]
            }}",
            category, name, url
        );
        let reader = BufReader::new(json.as_str().as_bytes());

        let result = parse(reader);

        // assert
        assert_eq!(
            result,
            Some(Parsed::RemoteJobs( RemoteJobs {
                remote_sites: vec![RemoteJobCategory {
                    category: category.to_string(),
                    sites: vec![Website {
                        name: name.to_string(),
                        url: url.to_string()
                    }]
                }]
            }))
        );
    }
}
