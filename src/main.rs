trait NegativeIndex<T> {
    fn pos(&self, i32) -> T;
}

impl<T> NegativeIndex<T> for [T]
where
    T: Copy,
{
    fn pos(&self, mut index: i32) -> T {
        let length = self.len() as i32;

        while index >= length {
            index -= length;
        }

        while index < 0 {
            index += length;
        }
        self[index as usize]
    }
}

fn main() {
    let a = [1, 2, 4, 6, 8, 9, 3];
    println!("Array  : {:?}", a);
    println!("[2]    : {}", a.pos(2));
    println!("[-3]   : {}", a.pos(-3));
    println!("[21]   : {}", a.pos(21));
    println!("[-100] : {}", a.pos(-100));
}
