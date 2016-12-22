#[derive(Debug)]
pub enum Item {
    Str(String),
    List<T>(Vec<T>),
    Single<T>(T),
    ItemName(Str),
    ItemDef(ItemName, List<ItemName>)
    Num(Str)
}

fn number(_input: &'a str, _super: &'b _Accepter, _pars: Vec<Box<_Containers>>) -> Result<&'a str> {
	let _capture = null;
	let _into = _super;
	let val = Rc<Item::Str(String::new())>;
	let _capture = val.clone();
}