# Rust Introduction for .NET and Python Developers

## Getting Started Guide for .NET Developers

### Key Concepts for C# Developers

1. **Ownership & Borrowing** (vs Garbage Collection)
   ```csharp
   // C# - GC handles cleanup
   public void ProcessList(List<string> items) {
       var copy = items;  // Reference copy
       // Both variables can access the list
   }
   ```
   ```rust
   // Rust - Only one owner at a time
   fn process_list(items: Vec<String>) {
       let copy = &items;  // Borrowing reference
       // Original still owns the data
   } // items is dropped here
   ```

2. **Pattern Matching** (similar to C# switch expressions)
   ```csharp
   // C# pattern matching
   string GetMessage(object obj) => obj switch {
       string s => $"String: {s}",
       int i => $"Int: {i}",
       _ => "Unknown"
   };
   ```
   ```rust
   // Rust pattern matching
   fn get_message(obj: &Value) -> String {
       match obj {
           Value::String(s) => format!("String: {}", s),
           Value::Number(n) => format!("Int: {}", n),
           _ => "Unknown".to_string()
       }
   }
   ```

3. **Traits** (like C# interfaces)
   ```csharp
   // C# interface
   public interface ILogger {
       void Log(string message);
   }
   ```
   ```rust
   // Rust trait
   trait Logger {
       fn log(&self, message: &str);
   }
   ```

4. **Option<T>** (vs C# nullable types)
   ```csharp
   // C# nullable
   string? name = GetName();
   if (name != null) {
       Console.WriteLine(name);
   }
   ```
   ```rust
   // Rust Option
   let name: Option<String> = get_name();
   if let Some(name) = name {
       println!("{}", name);
   }
   ```

5. **Memory Safety**
   ```csharp
   // C# - possible null reference
   public void UseArray(int[] arr) {
       arr[0] = 42;  // Could throw NullReferenceException
   }
   ```
   ```rust
   // Rust - compiler prevents null references
   fn use_array(arr: &mut [i32]) {
       arr[0] = 42;  // Guaranteed to be safe if it compiles
   }
   ```

### Common Patterns

1. **Builder Pattern** (similar in both languages)
   ```rust
   // Rust builder
   let command = Command::new("notepad")
       .arg("file.txt")
       .current_dir("C:/temp")
       .spawn()?;
   ```

2. **Error Handling** (vs exceptions)
   ```rust
   // Rust prefers Result over exceptions
   fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
       if b == 0 {
           return Err("division by zero");
       }
       Ok(a / b)
   }
   ```

3. **Async/Await** (similar syntax, different runtime)
   ```rust
   // Rust async
   async fn fetch_data() -> Result<String, Error> {
       let response = client.get("https://api.example.com")
           .send()
           .await?;
       Ok(response.text().await?)
   }
   ```

### Common Gotchas for .NET Developers
1. No null values - use Option<T> instead
2. No inheritance - use traits for shared behavior
3. No exceptions - use Result<T,E> for error handling
4. No implicit type conversion
5. Strict mutability rules

## Getting Started Guide for Python Developers

### Key Differences from Python

1. **Static Typing**
   ```python
   # Python - dynamic typing
   def process_data(data):
       return data.upper()  # Works with any object with upper()
   ```
   ```rust
   // Rust - static typing
   fn process_data(data: String) -> String {
       data.to_uppercase()  // Must be String type
   }
   ```

2. **Memory Management**
   ```python
   # Python - garbage collection
   def process():
       data = [1, 2, 3]
       shared = data  # Reference counting
   ```
   ```rust
   fn process() {
       let data = vec![1, 2, 3];
       let shared = &data;  // Borrowing
   }  // data is dropped here
   ```

3. **Error Handling**
   ```python
   # Python - try/except
   try:
       result = do_something()
   except ValueError as e:
       print(f"Error: {e}")
   ```
   ```rust
   // Rust - Result type
   match do_something() {
       Ok(result) => println!("Success: {}", result),
       Err(e) => println!("Error: {}", e),
   }
   ```

### Common Gotchas for Python Developers
1. Variables are immutable by default
2. No dynamic typing
3. Strict ownership rules
4. No global interpreter lock (GIL)
5. Compile-time checks vs runtime checks
