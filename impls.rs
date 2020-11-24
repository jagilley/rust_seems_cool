// well this is annoying how much rust cares about camelcase

struct MyStruct {
    my_data_thingy: i32,
}

impl MyStruct {
    fn foobar(&self) -> i32 {
        return self.my_data_thingy;
    }
}

fn main(){
    let me = MyStruct {
        my_data_thingy: 42,
    };
    println!("my data thingy is {}", me.foobar());
}