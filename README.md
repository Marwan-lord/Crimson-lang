# Crimson-lang
An interpreted expression-based functional programming language made in rust  
inspired mainly by monkey-lang from "Writing An Interpreter Book In Go" book 

-----

# Syntax 
```
let name = "Marwan"
let sum = fn (a, b) {
   a + b;
};
sum(12, sum(32, 43))
let greet = fn(name) {
   "hello" + name;
};
greet(name)

let fruits = {
    "apples": "Good",
    "grapes": "Awesome",
    "strawberries": "Mid",
}

fruits["apples"]

let array = [1,2,3,4,5];
print(array)

let x = 12;
let y = 32;

if (x > y) {
   print(" x is larger")
} else {
   print(" y is larger")
};

```


# Inspirations 
[Here](https://github.com/wadackel/rs-monkey-lang) 
