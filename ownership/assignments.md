Ownership
=========

## Transfering return values

**1. Define three functions:**  
    a. gives_ownership will move its return value, a string from a string literal, into the function that calls it.  
    b. takes_and_gives_back takes a String and returns it.  
    c. fn main()  

**2. Define the following three variables in fn main():**  
    a. gives_ownership moves its return value into a variable_a.  
    b. A variable_b gets a string from a string literal.  
    c. The variable_b is moved into takes_and_gives_back, which moves its return value into variable_c.  

**3. Compile the program.**  

**4. Find out, where the variables come in and go out of scope. Add comments to every line, where the scope of a variable changes, and where they are moved or dropped.**  

**5. Make the following modifications:**  
  a. Define the function calculate_length. The function takes a string and returns its length.  
  b. variable_c is moved into calculate_length, which moves its return value into variable_d. Print its value.  
  c. Explain why the the value of variable_c can no longer be accessed.  
  d. Use a tuple to solve this problem, so that both, the string and its value can be printed.  

## References and Borrowing  
Doing this with tuples is a hassle and the problem can be solved more elegantly.  

**1. Instead of movin a variable, make a reference to it:**  
  a. Revert your program to the version without a tuple.  
  b. Instead of moving variable_c into calculate_length, use &variable_c. Change the parameter of the function to &String.  
  c. Explain the difference between variable_c and &variable_c.  

  d. Define the function change(). The function borrows a string, modifies is with push_str() and returns the reference to the string.  
  e. Explain the output.  

** 2. To change the value of a reference, it has to be mutable.**
  a. Make variable_c into a mutable variable.  
  b. Change the parameter and the output of the function to the mutable refernces &mut String. Move the mutable reference to variable_c to the function.  
  c. Print variable_c.  

** 3. Play with mutable and immutable references.**

  a. Define two different variables, both referencing mutably to variable_c.  
  b. Print both variables.  
  c. Explain a data races.  
  d. Make one of the variables reference immutably to variable_c.  
  e. Explain the outcome.  

** 4. Dangling ReferencesS **
