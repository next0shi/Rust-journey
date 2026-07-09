https://www.youtube.com/watch?v=bum_19loj9A&list=PLBZBJbE_rGRV8D7XZ08LK6z-4zPoWzu5H

### What are Data Structures?

Data structures are essentially different ways of storing data on a computer. They define how information is organized and managed.

- **Example: Neighborhood Map**
  - To represent a neighborhood with locations (home, stores) and streets (one-way, two-way), you need to store connectivity information beyond just coordinates.
  - **Option 1: List of Paths:** Store every possible path between locations. This can be represented similarly to a list or array.
  - **Option 2: Adjacency List:** For each location, list all directly accessible locations. This resembles a hash table or hash map, where each location maps to a list of its neighbors.

### What are Algorithms?

Algorithms are the operations performed on data structures and the sets of instructions for executing them.

- **Example: Finding the Shortest Path**

  - **Problem:** Find the shortest route from home to school in the neighborhood example.
  - **Manual Solution:** Identify all paths, calculate distances using coordinates, and select the shortest.
  - **Algorithmic Approach:** A systematic set of instructions is needed for a computer. This involves:
    1. Identifying all reachable places from the current location.
    2. Tracking the distance traveled for each path.
    3. Repeating this process until the destination (school) is reached.
    4. Comparing distances of all found paths to the school and selecting the shortest.

- **Data Structure Impact on Algorithms:** The choice of data structure can significantly affect how an algorithm is implemented and its efficiency. For instance, finding neighbors in the adjacency list (Option 2) is often quicker than searching through a list of all paths (Option 1).

### Real-World Analogies for Data Structures

- **Array:** Imagine a long box with 100 identical partitions where items are placed in order. Accessing the 98th item is straightforward by calculating its exact position (97 \* partition width).
- **Linked List:** Imagine a series of individual boxes connected by strings. Each box holds an item. Adding new items is easy by adding more boxes and strings. However, finding a specific item (e.g., the 98th) requires traversing the list sequentially from the beginning.

### Choosing the Right Data Structure

The best data structure depends on the specific problem and requirements:

- **Arrays:** Efficient for accessing elements by index when the size is known and stable. Resizing can be costly.
- **Linked Lists:** More flexible for frequent insertions and deletions, especially when the total size is unknown or varies greatly. Accessing elements by index is slower.

### Importance of Data Structures and Algorithms

Understanding data structures and algorithms is crucial for writing efficient software. They can dramatically improve performance, as demonstrated by a case where rewriting code using better data structures reduced execution time from hours to minutes.

### https://www.youtube.com/watch?v=HdTcc4err_I

## Queues: First-In, First-Out Data Structure

A queue operates like a line of customers waiting to pay a cashier. New buyers join the end of the line (enqueue), and the cashier serves the person at the beginning (dequeue). Customers leave the line after being served, embodying the first-in, first-out (FIFO) principle. The core operations are typically called `enqueue` (insert), `dequeue` (delete), and `front` (read).

### Implementing Queues with Arrays

Queues can be implemented using an array of a fixed `capacity`. The `size` of the queue refers to the number of elements currently stored, which may be less than the array's capacity. The `head` points to the first element, and the `tail` points to the slot *after* the last element.

- **Enqueue**: Add an element at the `tail` index and increment `size`. If the array is full (size equals capacity), either raise an error or create a larger array and copy elements.
- **Dequeue**: Move the `head` to the next element and decrement `size`. The element is not explicitly removed from the array.
- **Front**: Return the element at the `head` index.

To handle the array reaching its end, a circular approach is used: when the `tail` or `head` reaches the end, it wraps around to the beginning of the array if space is available. Tracking `size` independently is crucial to differentiate between an empty queue and a full queue, as `head` and `tail` can be the same in both scenarios.

### Queue Use Case: Server Task Management

Queues are useful for scenarios where tasks need to be processed in the order they are received, especially during bursts of activity. A server can enqueue incoming tasks when it's busy and process them from the queue one by one, preventing task loss and maintaining order.

## Stacks: Last-In, First-Out Data Structure

A stack operates like a pile of plates. New plates are added to the top (push), and plates are removed from the top (pop). The `top` operation allows viewing the topmost plate. This follows the last-in, first-out (LIFO) principle.

### Implementing Stacks with Arrays

Similar to queues, stacks can be implemented using an array with a fixed `capacity`. The `size` tracks the number of elements. The bottom of the stack is at index 0, and the top element is at index `size - 1`.

- **Push**: Add an element at the index equal to the current `size` and increment `size`. Handle overflow by raising an error or resizing the array.
- **Pop**: Decrement `size` by one. The element is effectively removed.
- **Top**: Return the element at index `size - 1`.

### Stack Use Case: Browser History

Stacks are ideal for implementing browser back/forward functionality. Two stacks can manage history:

- **Backward Stack**: Stores previously visited pages.
- **Forward Stack**: Stores pages to which the user can navigate forward.

When a new page is opened, the current page is pushed onto the backward stack. Going back involves pushing the active page onto the forward stack and then popping from the backward stack. Going forward is the inverse. If the user navigates to a new page after going back, the forward history is cleared (forward stack is emptied).

## Time and Memory Complexity

Both stack and queue operations (push/pop/top and enqueue/dequeue/front) typically have **constant runtime complexity (O(1))**. They both have **linear memory complexity (O(n))**, where 'n' is the number of elements stored.

## Key Differences and Applications

The fundamental difference lies in their access principles: stacks are LIFO, while queues are FIFO. This makes them suitable for different problems. Stacks are used for tasks like managing function calls, expression evaluation, and browser history. Queues are used for task scheduling, breadth-first searches, and managing requests in a sequential order.
