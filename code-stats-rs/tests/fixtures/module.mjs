export function greet() {
    console.log("Hello");
}

export const farewell = () => {
    console.log("Goodbye");
};

export default function main() {
    greet();
    farewell();
}

class Greeter {
    greet() {
        return "Hi";
    }
}
