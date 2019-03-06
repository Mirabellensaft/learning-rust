Return Values and Scope
=======================
## Transfering return values

#H3 1. Define three functions:
  a) gives_ownership will move its return value, a string from a string literal, into the function that calls it.
  b) takes_and_gives_back takes a String and returns it.
  c) fn main()

#H3 2. Define the following three variables in fn main():
  a) gives_ownership moves its return value into a variable.
  b) A second variable gets a string from a string literal.
  c) The variable from b) is moved into takes_and_gives_back, which moves its return value into the third variable.

#H3 3. Compile the program.

#H3 4. Find out, where the variables come in and go out of scope. Add comments to every line, where the scope of a variable changes, and where they are moved or dropped.

#H3 5. Make the following modifications:
  a. Define the function calculate_length. The function takes a string and returns its length.
  b. variable_c is moved into calculate_length, which moves its return value into variable_d. Print its value.
  c. Explain why the the value of variable_c can no longer be accessed.
  d. Use a tuple to solve this problem, so that both, the string and its value can be printed.

## References and Borrowing
Doing this with tuples is a hassle and the problem can be solved more elegantly.

#H3 1. Make the following modifications:
  a. Revert your program to the version without a tuple.
  b. Instead of moving variable_c into calculate_length, use &variable_c. Change the parameter of the function to &String.
  c. Explain the difference between variable_c and &variable_c.

  d. Define the function change(). The function borrows a string, modifies is with push_str() and returns the reference to the string.
  e. Explain the output.

## Mutable References
  a. Make variable_c into a mutable variable.
  b. Change the parameter and the output of the function to the mutable refernces &mut String. Move the mutable reference to variable_c to the function.
  a. Print variable_c.

## Play with References

  a. Define two different variables, both referencing mutably to variable_c.
  b. Print both variables.
  c. Explain a data races.
  d. Make one of the variables reference immutably to variable_c.
  e. Explain the outcome.

## Dangling References
