package main

import "fmt"

type Person struct {
    Name string
    Age  int
}

func (p Person) Greet() {
    fmt.Printf("Hello, I'm %s\n", p.Name)
}

func main() {
    person := Person{Name: "Alice", Age: 30}
    person.Greet()
}

func add(a, b int) int {
    return a + b
}