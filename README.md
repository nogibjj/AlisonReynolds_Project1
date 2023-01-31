# Project 1 - Rust Command Line Tool
The goal for this project is to build a random data generator that will generate a few vectors with a given amount of noise.
The first version of this tool allows the user to input a noise value and then generates a vector x with values between 0 and 100, a vector y with values between 0 and 100 with the noise value subtracted, and a vector z = x^2 + y. It will then return a message saying the data has been generated.

The updated version of the product has three parameters: the length of the vectors, the range of the values, and the maximum amount of noise. When the tool is run, it calculates two vectors: x which contains 100 random vectors whose absolute value is less than the provided range value (so if the range is 20, the values will be between -20 and 20) and y which contains the squared values from the x vector with an added random noise value (where the absolute values of the random noise values are all less than the maximum amount of noise). Finally, the function outputs the correlation value between the two vectors. This allows users to examine how correlation can change based on vector size, range, and noise.

## References

* [rust-project-template](https://github.com/nogibjj/rust-new-project-template)
