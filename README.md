# Actix-web Hello World Example 🌐

This is a simple "Hello World" example using Actix-web, a powerful and flexible web framework for Rust.

## Getting Started 🚀

To run this example locally, make sure you have Rust installed. Then, follow these steps:

1. Clone this repository:

   ```bash
   git clone https://github.com/itsmohitnarayan/Http-Server-Rust.git
    ```

2. Navigate to the project directory:

   ```bash
   cd Http-Server-Rust
   ```

3. Build and run the project:

   ```bash
   cargo run
   ```

4. Open your web browser and go to [http://localhost:3000](http://localhost:3000). You should see "hello world!".

## Code Explanation ℹ️

This example consists of two main parts:

- `index()` function: This asynchronous function defines the behavior of the '/' route. It simply returns "hello world!" as the response.
  
- `main()` function: This is the entry point of the application. It sets up an Actix-web server with a single route '/' that points to the `index()` function. The server listens on port 3000.

## Dependencies 📦
```
actix-web = "4.5.1"
actix-rt = "2.9.0"
```

## Contributing 🤝

Contributions are welcome! If you find any issues or have suggestions for improvement, feel free to open an issue or submit a pull request.

## License 📄

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---------------------
