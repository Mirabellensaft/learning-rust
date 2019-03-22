Ownership
=========
## Variable Scope

**1. Explore the scope of a variable.**  
    a. A variable, `a_string`, that refers to a string literal.
    b. Add comments annotating where `a_string` is valid.
    c. Define function `fn change()` to try to change the string literal with the method `.push_str`. Use string literal as parameter and output types.

**2. The `String` type and the string literal**  
    a. Change the string literal in the declaration of `a_string` to the String type.  
    b. Change the parameter and output of `fn change()` to the String type.  
    c. Explain the difference between both concerning mutability and use of memory.  
    d. Explain the 'drop' function.
    e. Add comments annotating where `a_string` is dropped.  

**3. Explore how variables and data interact.**
    a. Define a variable 'x' that refers to an integer.
    b. Define a variable 'y' that refers to x.
    d. Print both
    c. Define a variable 'variable_a' that refers to 'a_string'.
    d. Print 'a_string', and a
    e. Explain the outcome.
    f. Change your code, so the string version works like the integer one.

## Transfering return values

**1. Define two functions:**  
    a. fn gives_ownership() will move its return value, a string from a string literal, into the function that calls it.  
    b. fn takes_and_gives_back() takes a String and returns it.  

**2. Define the following three variables in fn main():**  
    a. gives_ownership moves its return value into a variable_a.  
    b. A a_string gets a string from a string literal.  
    c. The a_string is moved into takes_and_gives_back, which moves its return value into variable_c.   

    d. Find out, where the variables come in and go out of scope. Add comments to every line, where the scope of a variable changes, and where they are moved or dropped.  

**3. Make the following modifications:**  
  a. Define the function calculate_length. The function takes a string and returns its length.  
  b. variable_c is moved into calculate_length, which moves its return value into variable_d. Print its value.  
  c. Explain why the the value of variable_c can no longer be accessed.  
  d. Use a tuple to solve this problem, so that both, the string and its value can be printed.  

## References and Borrowing  
Doing this with tuples is a hassle and the problem can be solved more elegantly.  

**1. Instead of moving a variable, make a reference to it:**  
  a. Revert your program to the version without a tuple.  
  b. Instead of moving variable_c into calculate_length, use &variable_c. Change the parameter of the function to &String.  
  c. Explain the difference between variable_c and &variable_c.  

  d. Define the function change(). The function borrows a string, modifies is with push_str() and returns the reference to the string.  
  e. Explain the output.  

**2. To change the value of a reference, it has to be mutable.**  
  a. Make variable_c into a mutable variable.  
  b. Change the parameter and the output of the function to the mutable refernces &mut String. Move the mutable reference to variable_c to the function.  
  c. Print variable_c.  

**3. Play with mutable and immutable references.**  

  a. Define two different variables, both referencing mutably to variable_c.  
  b. Print both variables.  
  c. Explain a data races.  
  d. Make one of the variables reference immutably to variable_c.  
  e. Explain the outcome.  

**4. Dangling ReferencesS **  
