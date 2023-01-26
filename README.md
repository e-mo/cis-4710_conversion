# Conversion

The assignment is to write a simple temperature conversion program.
   
The minimum requirement for the undergraduate submission is to read the temperature (number) as a numeric prompt, assume it's a Fahrenheit temperature and convert to Celsius.
   
A graduate credit submission must also allow an optional trailing F or C (upper or lower case) on the temperature (again assuming F if not given) and convert to the other (F->C and C->F).
   
All submissions should break this into 5 functions:
   
main, which does the actual I/O and calls the other functions to do all the work
   
get_inputs, which converts the string passed in into a temperature (number). For the graduate version, this function should also take a default input unit and return a tuple containing the numeric temperature, its unit and the desired output unit. Units should be described as strings (&str). This more advanced version will need to add lifetime indicators on the strings. In either case, the actual return should be an Option, with None being returned if no temperature was present.
   
convert, which takes a input (numeric) temperature, an input unit and an output unit. This function calls the next two functions as appropriate
   
convert_to_f takes a Celsius temperature as a number and returns the equivalent Fahrenheit 
   
convert_to_f takes a Fahrenheit temperature as a number and returns the equivalent Celsius
   
 

You should submit only your main.rs file, which should include the tests as well
   
All of the functions should be appropriately commented
   
You should have tests included for testing the conversion functions. 
