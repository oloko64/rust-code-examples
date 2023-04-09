use serde_json::Value;

struct Ipv4(u8, u8, u8, u8);

/// You can call this function with the following:
///
/// ```rust
/// use serde_json::Value;
///
/// let ipv4 = Ipv4(127, 0, 0, 1);
///
/// let value: String = ipv4.into();
///
/// assert_eq!(value, "127.0.0.1".to_string());
/// ```
// impl Into<String> for Ipv4 {
//     fn into(self) -> String {
//         format!("{}.{}.{}.{}", self.0, self.1, self.2, self.3)
//     }
// }

/// This is the same as the above, but using the std::string::String type.
/// One should always prefer implementing From over Into because implementing From automatically provides one with an implementation of Into thanks to the blanket implementation in the standard library.
///
/// You can call this function with the following:
///
/// ```rust
///
/// let ipv4 = Ipv4(127, 0, 0, 1);
///
/// let value = String::from(ipv4);
///
/// // Or
/// // let value: String = ipv4.into();
///
/// assert_eq!(value, "127.0.0.1".to_string());
///
/// ```
impl From<Ipv4> for String {
    fn from(ipv4: Ipv4) -> Self {
        format!("{}.{}.{}.{}", ipv4.0, ipv4.1, ipv4.2, ipv4.3)
    }
}

/// This is the same as the above, but using the serde_json::Value type.
///
/// This is a good example of how to use the Into trait to convert a type into another type.
///
/// You can call this function with the following:
///
/// ```rust
/// use serde_json::Value;
///
/// let ipv4 = Ipv4(127, 0, 0, 1);
///
/// let value: Value = ipv4.into();
/// ```
impl Into<Value> for Ipv4 {
    fn into(self) -> Value {
        Value::Array(vec![
            Value::Number(self.0.into()),
            Value::Number(self.1.into()),
            Value::Number(self.2.into()),
            Value::Number(self.3.into()),
        ])
    }
}
