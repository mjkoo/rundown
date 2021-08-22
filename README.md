# Rundown

Rundown is a domain specific language for building Runnable Markdown. 

The intent is to make a Literate programming language, where the whole program and text are interwoven in such as way that the code is legible. 

#### Rundown Question

Would you like to hear more about Rundown? [Yes/No]

```rundown
var response = read();
if (response == "no") {
    goto "end";
} 
if (response == "yes" ) {
    goto "first-class-coments";
}
```

## First class comments

In our view, comments are anything in a source file meant not for execution, but rather human readability and understanding. 
In that case, nearly the whole source file is just meant for human reading. 
The code is minimal, and the focus is on readability. 
In fact, the source only serves to modify what is read.

#### First class Question

Do you want to see some examples or have you seen enough? [Examples/End] 

```rundown
var response = read();
if ( response == "examples" ) {
    goto "examples";
} 

if (response == "end") {
    goto "end";
}

goto "first-class-question"

```

## Examples

Being constrained to markdown is no problem. 
Rundown can jump to anywhere in the document. 
Let's look at a couple of examples.

#### Examples Question

You have been here ```rundown static var counter = 0; counter = counter + 1; print(counter);``` times

You have been here 
```rundown static var counter = 0; counter = counter + 1; print(counter);``` 
times

Which example would you like to see? [FizzBuzz/Hello World/End]

``` rundown
var response = read();
if (response == "fizzbuzz") {
    goto "fizzbuzz";
} 

if (response == "hello world") {
    goto "hello-world";
}

if (response == "end") {
    goto "end";
}

goto "examples-question"
```

### FizzBuzz

FizzBuzz can be trivially done using strings all in code. 
However, the point of this language is to jump to points in a markdown file, and display it. 
In this example, we have 3 different headings: Fizz, Buzz, and FizzBuzz.
The code will execute in a goto loop, jumping into the categories and printing the labels. 
The source for fizz buzz is as follows. 

```
static var counter = 0;
counter = counter + 1;
if (counter == 100) {
    goto "examples-question"
}

if (counter % 3 == 0 and counter % 5 == 0) {
    goto "fizzbuzz";
}

if (counter % 3 ) {
    goto "fizz";
}

if (counter % 5 ) {
    goto "buzz";
}

print(counter);
```

The static variable is only set once in a code block, even if that code block is executed multiple times. 
This example works by jumping to a section, printing the text of that section, then jumping back.


### Fake Fizz

```
goto "fizzbuzz-code"
```

Let's see it in action.



#### FizzBuzz Code

```rundown
static var counter = 0;
counter = counter + 1;
if (counter == 100) {
    goto "examples-question"
}

if (counter % 3 == 0 and counter % 5 == 0) {
    goto "fizzbuzz";
}

if (counter % 3 ) {
    goto "fizz";
}

if (counter % 5 ) {
    goto "buzz";
}

print(counter);
```

#### Fizz

Fizz

```rundown 
goto "fizzbuzz-code";
```

#### Buzz

Buzz

```rundown 
goto "fizzbuzz-code";
```

#### FizzBuzz

FizzBuzz


```rundown 
goto "fizzbuzz-code";
```


### Hello World

There are two ways to do Hello World. 
The First is to recognize Rundown programs are Markdown, and simply write `hello world` in a markdown document. 
That is a bit simplistic, let's print it from a rundown code block. 

```
print("hello world");
```

```rundown
print("hello world");
```

We can even print valid markdown!

> Note: You cannot print headings, then goto them

```
print("**hello world**");
```

```rundown
print("**Hello World**");
```

## End

Thanks for checking out **Rundown**!

