from .rust_calculator import Calculator, add, subtract, multiply, divide, power

__all__ = [
    "Calculator",
    "add",
    "subtract",
    "multiply",
    "divide",
    "power",
    "run_calculator",
]


def run_calculator():
    """
    Run an interactive calculator session using the Rust-powered calculator
    """
    print("Rust-Powered Calculator")
    print("======================")
    print("Commands: +, -, *, /, ^, sqrt, clear, exit")
    print()

    calc = Calculator()
    print(f"Current value: {calc.get_value()}")

    while True:
        try:
            command = input("> ").strip().lower()

            if command in ["exit", "quit"]:
                break

            elif command == "clear":
                calc.clear()

            elif command == "sqrt":
                calc.sqrt()

            elif command.startswith("+"):
                value = float(command[1:])
                calc.add(value)

            elif command.startswith("-"):
                value = float(command[1:])
                calc.subtract(value)

            elif command.startswith("*"):
                value = float(command[1:])
                calc.multiply(value)

            elif command.startswith("/"):
                value = float(command[1:])
                calc.divide(value)

            elif command.startswith("^"):
                value = float(command[1:])
                calc.power(value)

            else:
                print("Unknown command")
                continue

            print(f"Result: {calc.get_value()}")

        except ValueError:
            print("Invalid input")
        except Exception as e:
            print(f"Error: {e}")

    print("Goodbye!")
