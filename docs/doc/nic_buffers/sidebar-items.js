initSidebarItems({"struct":[["ReceiveBuffer","A buffer that stores a packet (a piece of an Ethernet frame) that has been received from the NIC and is guaranteed to be contiguous in physical memory.  Auto-dereferences into a `MappedPages` object that represents its underlying memory.  When dropped, its underlying memory is automatically returned to the NIC driver for future reuse."],["ReceivedFrame","A network (e.g., Ethernet) frame that has been received by the NIC."],["TransmitBuffer","A buffer that stores a packet to be transmitted through the NIC and is guaranteed to be contiguous in physical memory.  Auto-dereferences into a `MappedPages` object that represents its underlying memory. "]]});