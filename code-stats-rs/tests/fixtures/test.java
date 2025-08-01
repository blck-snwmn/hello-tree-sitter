class TestClass {
    private String name;
    
    // Constructor
    public TestClass(String name) {
        this.name = name;
    }
    
    // Public method
    public String getName() {
        return name;
    }
    
    // Private method
    @SuppressWarnings("unused")
    private void privateMethod() {
        System.out.println("Private method");
    }
    
    // Static method
    public static void staticMethod() {
        System.out.println("Static method");
    }
    
    // Inner class
    public class InnerClass {
        public void innerMethod() {
            System.out.println("Inner method");
        }
    }
}

// Interface
interface TestInterface {
    void interfaceMethod();
}

// Abstract class
abstract class AbstractClass {
    public abstract void abstractMethod();
    
    public void concreteMethod() {
        System.out.println("Concrete method");
    }
}