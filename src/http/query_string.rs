use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}
//a=1&b=2&c&d&e===&d=abc
impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}
impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";

            //check if the substring has an equal signe
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }

            //adding the values to the data collection
            //check if key existe in the collection
            data.entry(key)
                .and_modify(|existing_value| match existing_value {
                    Value::Single(old_value) => {
                        *existing_value = Value::Multiple(vec![old_value, val]);
                    }
                    Value::Multiple(old_vec) => old_vec.push(val),
                })
                .or_insert(Value::Single(val));
        }

        QueryString { data }
    }
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}
