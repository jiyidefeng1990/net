extern {fn hello();}
extern {fn bye();}

fn main() {
    unsafe{
        hello();
        bye();
    }
}
