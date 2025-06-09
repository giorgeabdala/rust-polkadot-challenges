pub trait Basic{
    fn test(&self);
}

pub struct StructBasic{
     string_test: String,
}


impl Basic for StructBasic {
    fn test(&self){
        println!("{}", &self.string_test);
    }
}


pub struct Controler<T: Basic> {
    basic: T
}

impl<T: Basic> Controler<T> {
    fn do_test(&self) {
        self.basic.test();
    }

}


mod tests {
    use super::*;

    fn test() {
        let struct_basic = StructBasic{
            string_test: String::from("testing...")
        };
    
        let controler = Controler {
            basic : struct_basic
        };
        controler.do_test();
    }
    

}
















