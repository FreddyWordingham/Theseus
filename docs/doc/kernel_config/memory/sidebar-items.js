initSidebarItems({"constant":[["ADDRESSABILITY_PER_P4_ENTRY","Value: 512 GiB."],["BYTES_PER_ADDR","64-bit architecture results in 8 bytes per address."],["ENTRIES_PER_PAGE_TABLE","Value: 512. "],["KERNEL_HEAP_INITIAL_SIZE",""],["KERNEL_HEAP_MAX_SIZE","the kernel heap gets the whole 509th P4 entry."],["KERNEL_HEAP_P4_INDEX","Value: 509. The 509th entry is used for the kernel heap"],["KERNEL_HEAP_START","higher-half heap gets 512 GB address range starting at the 509th P4 entry, which is the slot right below the recursive P4 entry (510) actual value: 0o177777_775_000_000_000_0000, or 0xFFFF_FE80_0000_0000"],["KERNEL_OFFSET","The virtual address where the initial kernel (the nano_core) is mapped to. Actual value: 0xFFFFFFFF80000000. i.e., the linear offset between physical memory and kernel memory. So, for example, the VGA buffer will be mapped from 0xb8000 to 0xFFFFFFFF800b8000. This is -2GiB from the end of the 64-bit address space."],["KERNEL_STACK_ALLOCATOR_BOTTOM","the kernel stack allocator gets the 508th P4 entry of addressability. actual value: 0o177777_774_000_000_000_0000, or 0xFFFF_FE00_0000_0000 "],["KERNEL_STACK_ALLOCATOR_TOP_ADDR","the highest actually usuable address in the kernel stack allocator"],["KERNEL_STACK_P4_INDEX","Value: 508. The 508th entry is used for all kernel stacks"],["KERNEL_STACK_SIZE_IN_PAGES","The size in pages of each kernel stack.  If it's too small, complex kernel functions will overflow, causing a page fault / double fault."],["KERNEL_TEXT_MAX_SIZE","The size in bytes, not in pages."],["KERNEL_TEXT_P4_INDEX","Value: 511. The 511th entry is used for kernel text sections"],["KERNEL_TEXT_START","The kernel text region is where we load kernel modules.  It starts at the 511th P4 entry and goes up until the KERNEL_OFFSET, which is where the nano_core itself starts.  actual value: 0o177777_777_000_000_000_0000, or 0xFFFF_FF80_0000_0000"],["MAX_PAGE_NUMBER",""],["MAX_VIRTUAL_ADDRESS",""],["P1_INDEX_SHIFT","Value: 0. Shift the Page number (not the address!) by this to get the P1 index."],["P2_INDEX_SHIFT","Value: 9. Shift the Page number (not the address!) by this to get the P2 index."],["P3_INDEX_SHIFT","Value: 18. Shift the Page number (not the address!) by this to get the P3 index."],["P4_INDEX_SHIFT","Value: 27. Shift the Page number (not the address!) by this to get the P4 index."],["PAGE_SHIFT","The lower 12 bits of a virtual address correspond to the P1 page frame offset. "],["PAGE_SIZE","Page size is 4096 bytes, 4KiB pages."],["RECURSIVE_P4_INDEX","Value: 510. The 510th entry is used for the recursive P4 mapping."],["TEMPORARY_PAGE_VIRT_ADDR",""],["USER_STACK_ALLOCATOR_BOTTOM","the userspace stack allocators (one per userspace task) each get the 507th P4 entry of addressability. actual value: 0o177777_773_000_000_000_0000, or 0xFFFF_FD80_0000_0000  "],["USER_STACK_ALLOCATOR_TOP_ADDR","the highest actually usuable address in each userspace stack allocator"],["USER_STACK_P4_INDEX","Value: 507. The 507th entry is used for all userspace stacks"]]});