

mod itemtype;
use itemtype::item_type;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        //! ::itemtype::item_type, first :: appear!
        let mut x : ::itemtype::item_type<String> = ::itemtype::item_type::new("abc".to_string(), "def".to_string());
        
        assert!(*x.get_name() == "abc".to_string());
        assert!(*x.get_value() == "def".to_string());
        let y : String = String::from("DEF");
        x.set_name("ABC".to_string());
        x.set_value(y); //"DEF".to_string());

        assert!(*x.get_name() == "ABC".to_string());
        assert!(*x.get_value() == "DEF".to_string());
    }
}
