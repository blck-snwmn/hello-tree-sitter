class Calculator {
    add(a, b) {
        return a + b;
    }
    
    subtract(a, b) {
        return a - b;
    }
}

function multiply(x, y) {
    return x * y;
}

const divide = (x, y) => {
    return x / y;
};

function main() {
    const calc = new Calculator();
    console.log(calc.add(5, 3));
    console.log(multiply(4, 6));
    console.log(divide(10, 2));
}

main();