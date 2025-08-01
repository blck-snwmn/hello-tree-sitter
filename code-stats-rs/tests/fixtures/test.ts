// Function declarations
function regularFunction(a: number, b: number): number {
    return a + b;
}

// Arrow functions
const arrowFunction = (x: string): string => {
    return x.toUpperCase();
}

// Method in object
const obj = {
    method(): void {
        console.log("Method in object");
    }
};

// Class with methods
class TypeScriptClass {
    constructor(private name: string) {}
    
    public publicMethod(): string {
        return this.name;
    }
    
    private privateMethod(): void {
        console.log("Private method");
    }
}

// Interface (should be counted as class/struct)
interface Person {
    name: string;
    age: number;
}

// Type alias (not counted)
type Point = {
    x: number;
    y: number;
};

// Function expression
const functionExpression = function(data: any): boolean {
    return !!data;
};