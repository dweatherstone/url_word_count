# Challenge: Build a Multi-threaded Web Crawler

## Objectives

Create a Rust program that:

1. Accepts a list of URLs from the user.
2. Spawns multiple threads to fetch the content of these URLs concurrently.
3. Counts the total number of words on each page and prints the results.

## Requirements

1. **Concurrency**: Use `std::thread` or `tokio` for handling multiple threads.
2. **HTTP Requests**: Use the `reqwest` crate for fetching web content.
3. **Synchronization**: Ensure thread safety when aggregating the results. Use a `Mutex` or `Channel` for coordination.
4. **Error Handling**: Gracefully handle unreachable URLs or failed requests.
5. **Input**: Accept URLs from a file or command-line arguments. Fall back to an interactive command prompt if neither are provided.
6. **Ouput**: Print a summary of the word counts for each URL and the total word count.

### Example Input

```plain
https://www.rust-lang.org
https://www.doc.rust-lang.org
https://github.com
```

### Example output

```plain
https://www.rust-lang.org: 1024 words
https://www.doc.rust-lang.org: 1543 words
https://github.com: 867 words

Total: 3434 words
```

## Stretch Goals

1. Add an option to follow links (found in `<a>` tags) to a specified depth.
2. Add a timeout for each request
3. Use a `HashSet` to avoid fetching the same URL multiple times.

## Final Thoughts

This challenge tests your ability to work with Rust's concurrency model, handle real-world web content, and manage errors effectively.
