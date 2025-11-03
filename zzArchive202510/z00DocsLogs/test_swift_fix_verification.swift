// Swift test file for validating entity extraction fix
// This file tests all major Swift entity types

// MARK: - Functions
func greet(name: String) -> String {
    return "Hello, \(name)!"
}

func calculateArea(width: Double, height: Double) -> Double {
    return width * height
}

// MARK: - Classes
class Animal {
    var name: String
    var age: Int

    init(name: String, age: Int) {
        self.name = name
        self.age = age
    }

    func makeSound() {
        print("Some sound")
    }
}

class Dog: Animal {
    func bark() {
        print("Woof!")
    }
}

// MARK: - Structs
struct Rectangle {
    var width: Double
    var height: Double

    func area() -> Double {
        return width * height
    }
}

struct Circle {
    var radius: Double

    var area: Double {
        return Double.pi * radius * radius
    }
}

// MARK: - Enums
enum CompassDirection {
    case north
    case south
    case east
    case west
}

enum Result<T, E> {
    case success(T)
    case failure(E)
}

// MARK: - Protocols
protocol Drawable {
    func draw()
}

protocol Resizable {
    mutating func resize(by factor: Double)
}

// MARK: - Extensions
extension Rectangle: Drawable {
    func draw() {
        print("Drawing rectangle: \(width) x \(height)")
    }
}

extension Circle: Drawable {
    func draw() {
        print("Drawing circle with radius: \(radius)")
    }
}
