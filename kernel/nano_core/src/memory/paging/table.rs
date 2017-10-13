// Copyright 2016 Philipp Oppermann. See the README.md
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use memory::paging::entry::*;
use kernel_config::memory::ENTRIES_PER_PAGE_TABLE;
use memory::FrameAllocator;
use core::ops::{Index, IndexMut};
use core::marker::PhantomData;


// Now that we're using the 511th entry of the P4 table for mapping the higher-half kernel, 
// we need to use the 510th entry of P4 instead!
// see this: http://forum.osdev.org/viewtopic.php?f=1&p=176913
//      and: http://forum.osdev.org/viewtopic.php?f=15&t=25545
// NOTE: keep this in sync with the recursive index in kernel_config/memory.rs, and the one in arch/*/boot.asm.
pub const P4: *mut Table<Level4> = 0o177777_776_776_776_776_0000 as *mut _; 
                                         // ^p4 ^p3 ^p2 ^p1 ^offset  
                                         // ^ 0o776 means that we're always looking at the 510th entry recursively

pub struct Table<L: TableLevel> {
    entries: [Entry; ENTRIES_PER_PAGE_TABLE],
    level: PhantomData<L>,
}

impl<L> Table<L>
    where L: TableLevel
{
    pub fn zero(&mut self) {
        for entry in self.entries.iter_mut() {
            entry.set_unused();
        }
    }

    pub fn copy_entry_from_table(&mut self, from_table: &Table<Level4>, index: usize) {
        // bounds check
        assert!(index < ENTRIES_PER_PAGE_TABLE, "copy_entry_from_table: index of was out of bounds!");
        // simply copy the table entry, which is just a u64
        self.entries[index] = from_table[index].copy();
    }

    pub fn clear_entry(&mut self, index: usize) {
        // bounds check
        assert!(index < ENTRIES_PER_PAGE_TABLE, "clear_entry: index of was out of bounds!");
        self.entries[index].set_unused();
    }
}

impl<L> Table<L>
    where L: HierarchicalLevel
{

    /// uses 'index' as an index into this table's list of 512 entries
    /// returns the address of the next lowest page table (so P4 would give P3, P3 -> P2, P2 -> P1) as a raw u64 pointer
    fn next_table_address(&self, index: usize) -> Option<usize> {
        let entry_flags = self[index].flags();
        if entry_flags.contains(PRESENT) && !entry_flags.contains(HUGE_PAGE) {
            let table_address = self as *const _ as usize;
            let retval: usize = (table_address << 9) | (index << 12);
            
            // if bit 47 is zero, then we must sign extend the top 16 bits as zeroes.
            if retval & 0x800000000000 == 0 {
                Some(retval & 0x0000_FFFF_FFFF_FFFF)
            }
            else {
                // otherwise, they should be sign extended as ones.
                Some(retval | 0xFFFF_0000_0000_0000)
            }

            // Some(retval)

        } else {
            None
        }
    }

    /// returns the next lowest page table (so P4 would give P3, P3 -> P2, P2 -> P1)
    pub fn next_table(&self, index: usize) -> Option<&Table<L::NextLevel>> {
        // convert the next table address from a raw pointer back to a Table type
        self.next_table_address(index).map(|address| unsafe { &*(address as *const _) })
    }

    pub fn next_table_mut(&mut self, index: usize) -> Option<&mut Table<L::NextLevel>> {
        self.next_table_address(index).map(|address| unsafe { &mut *(address as *mut _) })
    }

    pub fn next_table_create<A>(&mut self,
                                index: usize,
                                flags: EntryFlags,
                                allocator: &mut A)
                                -> &mut Table<L::NextLevel>
        where A: FrameAllocator
    {
        if self.next_table(index).is_none() {
            assert!(!self.entries[index].flags().contains(HUGE_PAGE),
                    "mapping code does not support huge pages");
            let frame = allocator.allocate_frame().expect("no frames available");
            self.entries[index].set(frame, flags | PRESENT | WRITABLE); // must be PRESENT | WRITABLE
            self.next_table_mut(index).unwrap().zero();
        }
        self.next_table_mut(index).unwrap()
    }
}

impl<L> Index<usize> for Table<L>
    where L: TableLevel
{
    type Output = Entry;

    fn index(&self, index: usize) -> &Entry {
        &self.entries[index]
    }
}

impl<L> IndexMut<usize> for Table<L>
    where L: TableLevel
{
    fn index_mut(&mut self, index: usize) -> &mut Entry {
        &mut self.entries[index]
    }
}

pub trait TableLevel {}

pub enum Level4 {}
#[allow(dead_code)]
pub enum Level3 {}
#[allow(dead_code)]
pub enum Level2 {}
pub enum Level1 {}

impl TableLevel for Level4 {}
impl TableLevel for Level3 {}
impl TableLevel for Level2 {}
impl TableLevel for Level1 {}

pub trait HierarchicalLevel: TableLevel {
    type NextLevel: TableLevel;
}

impl HierarchicalLevel for Level4 {
    type NextLevel = Level3;
}

impl HierarchicalLevel for Level3 {
    type NextLevel = Level2;
}

impl HierarchicalLevel for Level2 {
    type NextLevel = Level1;
}