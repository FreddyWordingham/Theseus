initSidebarItems({"struct":[["RUNQUEUES","There is one runqueue per core, each core only accesses its own private runqueue and allows the scheduler to select a task from that runqueue to schedule in."],["RoundRobinTaskRef","A cloneable reference to a `Taskref` that exposes more methods related to task scheduling"],["RunQueue","A list of references to `Task`s (`RoundRobinTaskRef`s).  This is used to store the `Task`s (and associated scheduler related data)  that are runnable on a given core. A queue is used for the round robin scheduler. `Runqueue` implements `Deref` and `DerefMut` traits, which dereferences to `VecDeque`."]]});