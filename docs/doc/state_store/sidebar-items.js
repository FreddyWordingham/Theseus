initSidebarItems({"fn":[["get_state","Returns a Weak reference to the SystemState of the requested type `S`, which the caller can downcast into the specific type `S` by invoking downcast_ref() on the data inside the Weak<> wrapper. It's safe for the caller to save/cache the returned value. "],["init",""],["insert_state","Inserts a new SystemState-implementing type into the map. "]],"struct":[["SSCached","A thread-safe cached reference to a system-wide state. Internally, this contains a Weak pointer to `S`, which is upgraded / updated whenenver the caller invokes `get()`."]]});