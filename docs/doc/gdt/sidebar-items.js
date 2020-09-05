initSidebarItems({"enum":[["AvailableSegmentSelector","The GDT SegmentSelectors available in Theseus. Use this type with `get_segment_selector()`. "],["Descriptor","We need 6 GDT segments even for 64-bit: http://tunes.org/~qz/search/?view=1&c=osdev&y=17&m=1&d=2 See more info about GDT here: http://www.flingos.co.uk/docs/reference/Global-Descriptor-Table/                     and here: http://wiki.osdev.org/Global_Descriptor_Table"]],"fn":[["create_tss_gdt","Creates a new GDT, sets up the TSS with the given double fault stack and privilege stack, and then loads that new GDT & TSS."],["get_segment_selector","Stupid hack because SegmentSelector is not Cloneable/Copyable"]],"struct":[["Gdt","The Global Descriptor Table, as specified by the x86_64 architecture."]]});