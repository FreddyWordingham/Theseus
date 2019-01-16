#![no_std]
#![feature(alloc)]

//! This crate contains an implementation of an in-memory filesystem backed by MappedPages from the memory crate
//! This crate allocates memory at page-size granularity, so it's inefficient with memory when creating small files
//! Currently, the read and write operations of the RamFile follows the interface of the std::io read/write operations of the Rust standard library

#[macro_use] extern crate log;
extern crate alloc;
extern crate spin;
extern crate fs_node;
extern crate memory;
extern crate irq_safety;


// use alloc::vec::Vec;
use core::ops::DerefMut;
use alloc::string::String;
use fs_node::{DirRef, WeakDirRef, File, FsNode};
use memory::{MappedPages, FRAME_ALLOCATOR};
use memory::EntryFlags;
use alloc::sync::{Arc, Weak};
use spin::Mutex;
use alloc::boxed::Box;
use fs_node::FileOrDir;

/// The struct that represents a file in memory that is backed by MappedPages
pub struct MemFile {
    /// The name of the file
    name: String,
    // The size of the file in bytes
    size: usize,
    /// The contents or a seqeunce of bytes as a file: this primitive can be changed into a more complex struct as files become more complex
    contents: MappedPages,
    /// A weak reference to the parent directory
    parent: WeakDirRef,
}

impl MemFile {
    /// Combines file creation and file write into one operation
    pub fn new(name: String, contents: &mut [u8], parent: WeakDirRef) -> Result<(), &'static str> {
        // Obtain the active kernel page table
        let kernel_mmi_ref = memory::get_kernel_mmi_ref().ok_or("KERNEL_MMI was not yet initialized!")?;
        if let memory::PageTable::Active(ref mut active_table) = kernel_mmi_ref.lock().page_table {
            let mut allocator = try!(FRAME_ALLOCATOR.try().ok_or("Couldn't get Frame Allocator")).lock(); 
            // Allocate and map the least number of pages we need to store the information contained in the buffer
            let pages = memory::allocate_pages_by_bytes(contents.len()).ok_or("could not allocate pages")?;
            let mut mapped_pages = active_table.map_allocated_pages(pages,  EntryFlags::WRITABLE, allocator.deref_mut())?;            

            { // scoped this so that the mutable borrow on mapped_pages ends as soon as possible
                // Gets a mutuable reference to the byte portion of the newly mapped pages
                let mut dest_slice = mapped_pages.as_slice_mut::<u8>(0, contents.len())?;
                dest_slice.copy_from_slice(contents); // writes the desired contents into the correct area in the mapped page
            }
            // create and return the newly create MemFile
            let new_file = MemFile {
                name: name, 
                size: contents.len(),
                contents: mapped_pages,
                parent: parent.clone()
            };
            let boxed_file = Arc::new(Mutex::new(Box::new(new_file) as Box<File + Send>));
            let strong_parent = Weak::upgrade(&parent).ok_or("parent possibly doesn't exist for this MemFile")?;
            strong_parent.lock().insert_child(FileOrDir::File(boxed_file))?; // adds the newly created file to the tree
            return Ok(())
        }
        return Err("could not get active table");
    }

    /// Converts a MemFile object --> MappedPages object
    /// Note: This function consumes the MemFile object
    pub fn into_mapped_pages(self) -> MappedPages {
        return self.contents;
    }

    pub fn from_mapped_pages(pages: MappedPages, name: String, size: usize, parent: WeakDirRef) -> MemFile {
        MemFile {
            name: name, 
            size: size, 
            contents: pages, 
            parent: parent, 
        }
    }

}

impl File for MemFile {
    /// Reads the contents of a file
    /// Caller should pass in an empty buffer and the read function will query the size of the buffer
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, &'static str> {
        let num_bytes_read = self.size; // this is the number of bytes of actual information stored in the MappedPage
        // Copies the information from the MappedPage (or at least the section containing the relevant information) to the read-buffer
        buf.copy_from_slice(self.contents.as_slice_mut(0, num_bytes_read)?); 
        return Ok(num_bytes_read);
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize, &'static str> {
        let page_capacity = self.contents.size_in_bytes();
        if buf.len() <= page_capacity {
            { // scoped this so that the mutable borrow on mapped_pages ends as soon as possible
                // Gets a mutuable reference to the byte portion of the newly mapped pages
                let dest_slice = self.contents.as_slice_mut::<u8>(0, buf.len())?;
                dest_slice.copy_from_slice(buf); // writes the desired contents into the correct area in the mapped page
            }    
            return Ok(self.contents.size_in_bytes())
        } else {
            return Err("size of contents to be written exceeds the MappedPages capacity");
        }
    }

    fn seek(&self) { unimplemented!(); }
    fn delete(self) { unimplemented!(); }
    fn size(&self) -> usize {
        return self.size;
    }
}

impl FsNode for MemFile {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    
    /// Returns a pointer to the parent if it exists
    fn get_parent_dir(&self) -> Result<DirRef, &'static str> {
        self.parent.upgrade().ok_or("couldn't upgrade parent")
    }
}
