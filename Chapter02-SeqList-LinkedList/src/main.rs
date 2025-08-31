mod seq_list_impl;

use crate::seq_list_impl::SeqList;

fn main() {
    let mut a: SeqList<&str> = SeqList::new();
    a.push_back("XiaoMing");
    a.print();
}
