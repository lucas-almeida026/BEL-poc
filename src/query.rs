use crate::data::Entry;

#[derive(Debug)]
pub enum QueryError {
    InvalidQuery,
    InvalidQueryCmd,
    InvalidGetQuery,
    InvalidGetQueryK,
    InvalidGetQueryId,
    InvalidPutQuery,
    InvalidPutQueryList,
}

#[derive(Debug)]
pub enum QueryKind {
    Get { id: u32, top_k: Option<u32> },
    Put(Vec<Entry>),
}

pub fn parse_query(query: &str) -> Result<QueryKind, QueryError> {
    let words = query.trim().split(" ").collect::<Vec<_>>();

    if words.len() < 2 {
        return Err(QueryError::InvalidQuery);
    }

    if words[0] == "get" {
        if words.len() != 5 {
            return Err(QueryError::InvalidGetQuery); //"[Invalid query](get) expecting: get top <k> from <id>"
        }
        if words[1] != "top" || words[3] != "from" {
            return Err(QueryError::InvalidGetQuery); //"[Invalid query](get) expecting: get top <k> from <id>"
        }

        let top_k = match words[2].parse::<u32>() {
            Ok(n) => n,
            Err(_) => return Err(QueryError::InvalidGetQueryK), //"[Invalid query](get) expecting positive integer <k> at ...top <k>..."
        };
        

        let id = match words[4].parse::<u32>() {
            Ok(n) => n,
            Err(_) => return Err(QueryError::InvalidGetQueryId),
        };

        Ok(QueryKind::Get {
            id,
            top_k: if top_k == 0 { None } else { Some(top_k) },
        })
    } else if words[0] == "put" {
        if words.len() != 2 {
            return Err(QueryError::InvalidPutQuery);
        }

        let items = words[1].split(",").collect::<Vec<_>>();
        let mut entries = Vec::new();

        // println!("items = {items:?}");
        for item in items {
            let item = item.trim();
            if !item.starts_with("(") || !item.ends_with(")") {
                return Err(QueryError::InvalidPutQueryList);
            }
            let substring = &item[1..(item.len() - 1)];
            let parts = substring.split(":").collect::<Vec<_>>();

            if !parts.len() == 2 {
                return Err(QueryError::InvalidPutQueryList);
            }

            let id = match parts[0].parse::<u32>() {
                Ok(n) => n,
                Err(_) => return Err(QueryError::InvalidPutQueryList),
            };

            let group = match parts[1].parse::<u32>() {
                Ok(n) => n,
                Err(_) => return Err(QueryError::InvalidPutQueryList),
            };

            entries.push(Entry::new(id, group));
        }

        return Ok(QueryKind::Put(entries));
    } else {
        return Err(QueryError::InvalidQueryCmd); //"[Invalid query] expecting: \"get\" or \"put\" commands"
    }
}
