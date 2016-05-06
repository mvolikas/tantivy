#![allow(dead_code)]

mod skiplist_builder;
mod skiplist;

pub use self::skiplist_builder::SkipListBuilder;
pub use self::skiplist::SkipList;



#[cfg(test)]
mod tests {

    use super::*;
    use std::io::{Write, Read};
    use std::io::Seek;

    #[test]
    fn test_skiplist() {
        let mut output: Vec<u8> = Vec::new();
        let mut skip_list_builder: SkipListBuilder<u32> = SkipListBuilder::new(10);
        skip_list_builder.insert(2, &3).unwrap();
        skip_list_builder.write::<Vec<u8>>(&mut output).unwrap();
        let mut skip_list: SkipList<u32> = SkipList::read(&mut output);
        assert_eq!(skip_list.next(), Some((2, 3)));
    }

    #[test]
    fn test_skiplist2() {
        let mut output: Vec<u8> = Vec::new();
        let skip_list_builder: SkipListBuilder<u32> = SkipListBuilder::new(10);
        skip_list_builder.write::<Vec<u8>>(&mut output).unwrap();
        let mut skip_list: SkipList<u32> = SkipList::read(&mut output);
        assert_eq!(skip_list.next(), None);
    }

    #[test]
    fn test_skiplist3() {
        let mut output: Vec<u8> = Vec::new();
        let mut skip_list_builder: SkipListBuilder<()> = SkipListBuilder::new(2);
        skip_list_builder.insert(2, &()).unwrap();
        skip_list_builder.insert(3, &()).unwrap();
        skip_list_builder.insert(5, &()).unwrap();
        skip_list_builder.insert(7, &()).unwrap();
        skip_list_builder.insert(9, &()).unwrap();
        skip_list_builder.write::<Vec<u8>>(&mut output).unwrap();
        let mut skip_list: SkipList<()> = SkipList::read(&mut output);
        assert_eq!(skip_list.next().unwrap(), (2, ()));
        assert_eq!(skip_list.next().unwrap(), (3, ()));
        assert_eq!(skip_list.next().unwrap(), (5, ()));
        assert_eq!(skip_list.next().unwrap(), (7, ()));
        assert_eq!(skip_list.next().unwrap(), (9, ()));
        assert_eq!(skip_list.next(), None);
    }

    #[test]
    fn test_skiplist4() {
        let mut output: Vec<u8> = Vec::new();
        let mut skip_list_builder: SkipListBuilder<()> = SkipListBuilder::new(2);
        skip_list_builder.insert(2, &()).unwrap();
        skip_list_builder.insert(3, &()).unwrap();
        skip_list_builder.insert(5, &()).unwrap();
        skip_list_builder.insert(7, &()).unwrap();
        skip_list_builder.insert(9, &()).unwrap();
        skip_list_builder.write::<Vec<u8>>(&mut output).unwrap();
        let mut skip_list: SkipList<()> = SkipList::read(&mut output);
        assert_eq!(skip_list.next().unwrap(), (2, ()));
        skip_list.seek(5);
        assert_eq!(skip_list.next().unwrap(), (5, ()));
        assert_eq!(skip_list.next().unwrap(), (7, ()));
        assert_eq!(skip_list.next().unwrap(), (9, ()));
        assert_eq!(skip_list.next(), None);
    }

    #[test]
    fn test_skiplist5() {
        let mut output: Vec<u8> = Vec::new();
        let mut skip_list_builder: SkipListBuilder<()> = SkipListBuilder::new(3);
        skip_list_builder.insert(2, &()).unwrap();
        skip_list_builder.insert(3, &()).unwrap();
        skip_list_builder.insert(5, &()).unwrap();
        skip_list_builder.insert(6, &()).unwrap();
        skip_list_builder.write::<Vec<u8>>(&mut output).unwrap();
        let mut skip_list: SkipList<()> = SkipList::read(&mut output);
        assert_eq!(skip_list.next().unwrap(), (2, ()));
        skip_list.seek(6);
        assert_eq!(skip_list.next().unwrap(), (6, ()));
        assert_eq!(skip_list.next(), None);
    }

    #[test]
    fn test_skiplist6() {
        let mut output: Vec<u8> = Vec::new();
        let mut skip_list_builder: SkipListBuilder<()> = SkipListBuilder::new(2);
        skip_list_builder.insert(2, &()).unwrap();
        skip_list_builder.insert(3, &()).unwrap();
        skip_list_builder.insert(5, &()).unwrap();
        skip_list_builder.insert(7, &()).unwrap();
        skip_list_builder.insert(9, &()).unwrap();
        skip_list_builder.write::<Vec<u8>>(&mut output).unwrap();
        let mut skip_list: SkipList<()> = SkipList::read(&mut output);
        assert_eq!(skip_list.next().unwrap(), (2, ()));
        skip_list.seek(10);
        assert_eq!(skip_list.next(), None);
    }

    #[test]
    fn test_skiplist7() {
        let mut output: Vec<u8> = Vec::new();
        let mut skip_list_builder: SkipListBuilder<()> = SkipListBuilder::new(3);
        for i in 0..1000 {
            skip_list_builder.insert(i, &()).unwrap();
        }
        skip_list_builder.insert(1004, &()).unwrap();
        skip_list_builder.write::<Vec<u8>>(&mut output).unwrap();
        let mut skip_list: SkipList<()> = SkipList::read(&mut output);
        assert_eq!(skip_list.next().unwrap(), (0, ()));
        skip_list.seek(431);
        assert_eq!(skip_list.next().unwrap(), (431,()) );
        skip_list.seek(1003);
        assert_eq!(skip_list.next().unwrap(), (1004,()) );
        assert_eq!(skip_list.next(), None);
    }

    #[test]
    fn test_skiplist8() {
        let mut output: Vec<u8> = Vec::new();
        let mut skip_list_builder: SkipListBuilder<u32> = SkipListBuilder::new(10);
        skip_list_builder.insert(2, &3).unwrap();
        skip_list_builder.write::<Vec<u8>>(&mut output).unwrap();
        assert_eq!(output.len(), 13);
        assert_eq!(output[0], 1);
    }

    #[test]
    fn test_skiplist9() {
        let mut output: Vec<u8> = Vec::new();
        let mut skip_list_builder: SkipListBuilder<u32> = SkipListBuilder::new(3);
        for i in 0..9 {
            skip_list_builder.insert(i, &i).unwrap();
        }
        skip_list_builder.write::<Vec<u8>>(&mut output).unwrap();
        assert_eq!(output.len(), 117);
        assert_eq!(output[0], 3);
    }

    #[test]
    fn test_skiplist10() {
        // checking that void gets serialized to nothing.
        let mut output: Vec<u8> = Vec::new();
        let mut skip_list_builder: SkipListBuilder<()> = SkipListBuilder::new(3);
        for i in 0..9 {
            skip_list_builder.insert(i, &()).unwrap();
        }
        skip_list_builder.write::<Vec<u8>>(&mut output).unwrap();
        assert_eq!(output.len(), 81);
        assert_eq!(output[0], 3);
    }

}
