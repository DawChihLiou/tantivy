extern crate tantivy;
extern crate byteorder;
use std::io::{Write, Seek};
use std::io::SeekFrom;
use tantivy::core::skip::*;
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

#[test]
fn test_skip_list_builder() {
    {
        let mut output: Vec<u8> = Vec::new();
        let mut skip_list_builder: SkipListBuilder = SkipListBuilder::new(10);
        skip_list_builder.insert(2, &3);
        skip_list_builder.write::<Vec<u8>>(&mut output);
        assert_eq!(output.len(), 16);
        assert_eq!(output[0], 0);
    }
    {
        let mut output: Vec<u8> = Vec::new();
        let mut skip_list_builder: SkipListBuilder = SkipListBuilder::new(3);
        for i in (0..9) {
            skip_list_builder.insert(i, &i);
        }
        skip_list_builder.write::<Vec<u8>>(&mut output);
        assert_eq!(output.len(), 120);
        assert_eq!(output[0], 0);
    }
    {
        // checking that void gets serialized to nothing.
        let mut output: Vec<u8> = Vec::new();
        let mut skip_list_builder: SkipListBuilder = SkipListBuilder::new(3);
        for i in (0..9) {
            skip_list_builder.insert(i, &());
        }
        skip_list_builder.write::<Vec<u8>>(&mut output);
        assert_eq!(output.len(), 84);
        assert_eq!(output[0], 0);
    }
}

#[test]
fn test_skip_list_reader() {
    {
        let mut output: Vec<u8> = Vec::new();
        let mut skip_list_builder: SkipListBuilder = SkipListBuilder::new(10);
        skip_list_builder.insert(2, &3);
        skip_list_builder.write::<Vec<u8>>(&mut output);
        let mut skip_list: SkipList<u32> = SkipList::read(&mut output);
        assert_eq!(skip_list.next(), Some((2, 3)));
        assert_eq!(skip_list.next(), None);
    }
    {
        let mut output: Vec<u8> = Vec::new();
        let mut skip_list_builder: SkipListBuilder = SkipListBuilder::new(10);
        skip_list_builder.write::<Vec<u8>>(&mut output);
        let mut skip_list: SkipList<u32> = SkipList::read(&mut output);
        assert_eq!(skip_list.next(), None);
    }
    {
        let mut output: Vec<u8> = Vec::new();
        let mut skip_list_builder: SkipListBuilder = SkipListBuilder::new(2);
        skip_list_builder.insert(2, &());
        skip_list_builder.insert(3, &());
        skip_list_builder.insert(5, &());
        skip_list_builder.insert(7, &());
        skip_list_builder.insert(9, &());
        skip_list_builder.write::<Vec<u8>>(&mut output);
        let mut skip_list: SkipList<()> = SkipList::read(&mut output);
        assert_eq!(skip_list.next().unwrap(), (2, ()));
        assert_eq!(skip_list.next().unwrap(), (3, ()));
        assert_eq!(skip_list.next().unwrap(), (5, ()));
        assert_eq!(skip_list.next().unwrap(), (7, ()));
        assert_eq!(skip_list.next().unwrap(), (9, ()));
        assert_eq!(skip_list.next(), None);
    }
    {
        let mut output: Vec<u8> = Vec::new();
        let mut skip_list_builder: SkipListBuilder = SkipListBuilder::new(2);
        skip_list_builder.insert(2, &());
        skip_list_builder.insert(3, &());
        skip_list_builder.insert(5, &());
        skip_list_builder.insert(7, &());
        skip_list_builder.insert(9, &());
        skip_list_builder.write::<Vec<u8>>(&mut output);
        let mut skip_list: SkipList<()> = SkipList::read(&mut output);
        assert_eq!(skip_list.next().unwrap(), (2, ()));
        skip_list.seek(5);
        assert_eq!(skip_list.next().unwrap(), (5, ()));
        assert_eq!(skip_list.next().unwrap(), (7, ()));
        assert_eq!(skip_list.next().unwrap(), (9, ()));
        assert_eq!(skip_list.next(), None);
    }
    {
        let mut output: Vec<u8> = Vec::new();
        let mut skip_list_builder: SkipListBuilder = SkipListBuilder::new(3);
        skip_list_builder.insert(2, &());
        skip_list_builder.insert(3, &());
        skip_list_builder.insert(5, &());
        skip_list_builder.insert(6, &());
        skip_list_builder.write::<Vec<u8>>(&mut output);
        let mut skip_list: SkipList<()> = SkipList::read(&mut output);
        assert_eq!(skip_list.next().unwrap(), (2, ()));
        skip_list.seek(6);
        assert_eq!(skip_list.next().unwrap(), (6, ()));
        assert_eq!(skip_list.next(), None);

    }
    {
        let mut output: Vec<u8> = Vec::new();
        let mut skip_list_builder: SkipListBuilder = SkipListBuilder::new(2);
        skip_list_builder.insert(2, &());
        skip_list_builder.insert(3, &());
        skip_list_builder.insert(5, &());
        skip_list_builder.insert(7, &());
        skip_list_builder.insert(9, &());
        skip_list_builder.write::<Vec<u8>>(&mut output);
        let mut skip_list: SkipList<()> = SkipList::read(&mut output);
        assert_eq!(skip_list.next().unwrap(), (2, ()));
        skip_list.seek(10);
        assert_eq!(skip_list.next(), None);
    }
    {
        let mut output: Vec<u8> = Vec::new();
        let mut skip_list_builder: SkipListBuilder = SkipListBuilder::new(3);
        for i in (0..1000) {
            skip_list_builder.insert(i, &());
        }
        skip_list_builder.insert(1004, &());
        skip_list_builder.write::<Vec<u8>>(&mut output);
        let mut skip_list: SkipList<()> = SkipList::read(&mut output);
        assert_eq!(skip_list.next().unwrap(), (0, ()));
        skip_list.seek(431);
        assert_eq!(skip_list.next().unwrap(), (431,()) );
        skip_list.seek(1003);
        assert_eq!(skip_list.next().unwrap(), (1004,()) );
        assert_eq!(skip_list.next(), None);
    }
}