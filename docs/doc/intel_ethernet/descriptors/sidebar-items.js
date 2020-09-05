initSidebarItems({"constant":[["RX_STATUS_DD","Rx Status: Descriptor Done"],["RX_STATUS_EOP","Rx Status: End of Packet"],["TX_CMD_EOP","Tx Command: End of Packet"],["TX_CMD_IC","Tx Command: Insert Checksum"],["TX_CMD_IDE","Tx Command: Interrupt Delay Enable"],["TX_CMD_IFCS","Tx Command: Insert FCS"],["TX_CMD_RPS","Tx Command: Report Packet Sent"],["TX_CMD_RS","Tx Command: Report Status"],["TX_CMD_VLE","Tx Command: VLAN Packet Enable"],["TX_STATUS_DD","Tx Status: descriptor Done"]],"struct":[["AdvancedRxDescriptor","Advanced Receive Descriptor used in the Ixgbe driver. It has 2 modes: Read and Write Back, both of which use the whole 128 bits.  There is one receive descriptor per receive buffer that can be converted between these 2 modes. Read contains the addresses that the driver writes. Write Back contains information the hardware writes on receiving a packet. More information can be found in the 82599 datasheet."],["LegacyRxDescriptor","This struct is a Legacy Receive Descriptor.  The driver writes to the upper 64 bits, and the NIC writes to the lower 64 bits. It's the descriptor type used in older Intel NICs and the E1000 driver."],["LegacyTxDescriptor","This struct is a Legacy Transmit Descriptor.  It's the descriptor type used in older Intel NICs and the E1000 driver."]],"trait":[["RxDescriptor","A trait for the minimum set of functions needed to receive a packet using one of Intel's receive descriptor types. Receive descriptors contain the physical address where an incoming packet should be stored by the NIC, as well as bits that are updated by the HW once the packet is received. There is one receive descriptor per receive buffer.  Receive functions defined in the Network_Interface_Card crate expect a receive descriptor to implement this trait."],["TxDescriptor","A trait for the minimum set of functions needed to transmit a packet using one of Intel's transmit descriptor types. Transmit descriptors contain the physical address where an outgoing packet is stored, as well as bits that are updated by the HW once the packet is sent. There is one transmit descriptor per transmit buffer. Transmit functions defined in the Network_Interface_Card crate expect a transmit descriptor to implement this trait."]]});