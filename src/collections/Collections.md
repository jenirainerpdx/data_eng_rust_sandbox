
# Sequences
| Collection      | Best For                           | Fastest Operations                       | Avoid When                                                                         |
|-----------------|------------------------------------|------------------------------------------|------------------------------------------------------------------------------------|
| `Vec<T>`        | Most general sequence storage      | Push/pop at end, random access           | Frequent front/mid insert/remove                                                   |
| `VecDeque<T>`   | Queues, buffers, sliding windows   | Push/pop at both ends                    | Mid-list insert/remove                                                             |
| `LinkedList<T>` | Frequent insert/remove in interior | Insert/remove at known position          | Random access, small lists                                                         |
 | `Array [T; N]`  | Fixed size; known at compile time. | Fast, stack-allocated, fixed-length data | Size is not known at compile time or you need a flexible, growable collection type |


# Maps
| Collection        | Best For                              | Fastest Operations                            | Avoid When                              |
|-------------------|---------------------------------------|-----------------------------------------------|-----------------------------------------|
| `HashMap<K, V>`   | General-purpose key/value storage     | Insert, remove, lookup by key (O(1) avg.)     | Need ordered keys                       |
| `BTreeMap<K, V>`  | Sorted key/value storage              | Range queries, ordered iteration (O(log n))   | Need maximum raw speed, unordered keys  |

# Sets
| Collection        | Best For                              | Fastest Operations                            | Avoid When                              |
|-------------------|---------------------------------------|-----------------------------------------------|-----------------------------------------|
| `HashSet<T>`      | Unordered unique-item storage         | Insert, remove, lookup (O(1) avg.)            | Need sorted iteration                   |
| `BTreeSet<T>`     | Sorted unique-item storage            | Range queries, ordered iteration (O(log n))   | Need maximum unordered speed            |

# Trees
| Collection            | Best For                   | Fastest Operations           | Avoid When                              |
|-----------------------|---------------------------|-----------------------------|-----------------------------------------|
| `BTreeMap/BTreeSet`   | Sorted maps/sets, ranges   | Ordered insert, query        | Only need unordered key/value or set    |
| Custom (e.g. petgraph)| Graph/tree relationships   | Graph/tree traversal         | For just key/value or set storage       |

