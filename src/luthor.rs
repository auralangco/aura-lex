use aura_lex::lexer::lex;

fn main() {
    let src = r#"val x := 10
        fn main := {
            val x := 10
            val y := 20
            val result := x + y
            println(result)
        }

        fn add := (a: I32, b: I32) -> I32 {
            a + b
        }

        type Person := {
            val name: String
            val age: I32
        }

        fn new_person := (name: String, age: I32) -> Person {
            Person { name, age }
        }

        val john := new_person("John Doe", 30)
        println(john.name)
        println(john.age)

        macro print_sum := (a: I32, b: I32) {
            println(a + b)
        }

        print_sum(5, 7)

        tag #example-tag := {
            val description := "This is an example tag"
        }

        object ExampleObject := {
            val id := 1
            val name := "Example"
        }

        import some_module

        val x := 10
        fn main := {
            val x := 10
            val y := 20
            val result := x + y
            println(result)
        }

        fn add := (a: I32, b: I32) -> I32 {
            a + b
        }

        type Person := {
            val name: String
            val age: I32
        }

        fn new_person := (name: String, age: I32) -> Person {
            Person { name, age }
        }

        val john := new_person("John Doe", 30)
        println(john.name)
        println(john.age)

        macro print_sum := (a: I32, b: I32) {
            println(a + b)
        }

        print_sum(5, 7)

        tag #example-tag := {
            val description := "This is an example tag"
        }

        object ExampleObject := {
            val id := 1
            val name := "Example"
        }

        import some_module

        val x := 10
        fn main := {
            val x := 10
            val y := 20
            val result := x + y
            println(result)
        }

        fn add := (a: I32, b: I32) -> I32 {
            a + b
        }

        type Person := {
            val name: String
            val age: I32
        }

        fn new_person := (name: String, age: I32) -> Person {
            Person { name, age }
        }

        val john := new_person("John Doe", 30)
        println(john.name)
        println(john.age)

        macro print_sum := (a: I32, b: I32) {
            println(a + b)
        }

        print_sum(5, 7)

        tag #example-tag := {
            val description := "This is an example tag"
        }

        object ExampleObject := {
            val id := 1
            val name := "Example"
        }

        import some_module

        val x := 10
        fn main := {
            val x := 10
            val y := 20
            val result := x + y
            println(result)
        }

        fn add := (a: I32, b: I32) -> I32 {
            a + b
        }

        type Person := {
            val name: String
            val age: I32
        }

        fn new_person := (name: String, age: I32) -> Person {
            Person { name, age }
        }

        val john := new_person("John Doe", 30)
        println(john.name)
        println(john.age)

        macro print_sum := (a: I32, b: I32) {
            println(a + b)
        }

        print_sum(5, 7)

        tag #example-tag := {
            val description := "This is an example tag"
        }

        object ExampleObject := {
            val id := 1
            val name := "Example"
        }

        import some_module

        val x := 10
        fn main := {
            val x := 10
            val y := 20
            val result := x + y
            println(result)
        }

        fn add := (a: I32, b: I32) -> I32 {
            a + b
        }

        type Person := {
            val name: String
            val age: I32
        }

        fn new_person := (name: String, age: I32) -> Person {
            Person { name, age }
        }

        val john := new_person("John Doe", 30)
        println(john.name)
        println(john.age)

        macro print_sum := (a: I32, b: I32) {
            println(a + b)
        }

        print_sum(5, 7)

        tag #example-tag := {
            val description := "This is an example tag"
        }

        object ExampleObject := {
            val id := 1
            val name := "Example"
        }

        import some_module

        val x := 10
        fn main := {
            val x := 10
            val y := 20
            val result := x + y
            println(result)
        }

        fn add := (a: I32, b: I32) -> I32 {
            a + b
        }

        type Person := {
            val name: String
            val age: I32
        }

        fn new_person := (name: String, age: I32) -> Person {
            Person { name, age }
        }

        val john := new_person("John Doe", 30)
        println(john.name)
        println(john.age)

        macro print_sum := (a: I32, b: I32) {
            println(a + b)
        }

        print_sum(5, 7)

        tag #example-tag := {
            val description := "This is an example tag"
        }

        object ExampleObject := {
            val id := 1
            val name := "Example"
        }

        import some_module

        val x := 10
        fn main := {
            val x := 10
            val y := 20
            val result := x + y
            println(result)
        }

        fn add := (a: I32, b: I32) -> I32 {
            a + b
        }

        type Person := {
            val name: String
            val age: I32
        }

        fn new_person := (name: String, age: I32) -> Person {
            Person { name, age }
        }

        val john := new_person("John Doe", 30)
        println(john.name)
        println(john.age)

        macro print_sum := (a: I32, b: I32) {
            println(a + b)
        }

        print_sum(5, 7)

        tag #example-tag := {
            val description := "This is an example tag"
        }

        object ExampleObject := {
            val id := 1
            val name := "Example"
        }

        import some_module

        val x := 10
        fn main := {
            val x := 10
            val y := 20
            val result := x + y
            println(result)
        }

        fn add := (a: I32, b: I32) -> I32 {
            a + b
        }

        type Person := {
            val name: String
            val age: I32
        }

        fn new_person := (name: String, age: I32) -> Person {
            Person { name, age }
        }

        val john := new_person("John Doe", 30)
        println(john.name)
        println(john.age)

        macro print_sum := (a: I32, b: I32) {
            println(a + b)
        }

        print_sum(5, 7)

        tag #example-tag := {
            val description := "This is an example tag"
        }

        object ExampleObject := {
            val id := 1
            val name := "Example"
        }

        import some_module

        val x := 10
        fn main := {
            val x := 10
            val y := 20
            val result := x + y
            println(result)
        }

        fn add := (a: I32, b: I32) -> I32 {
            a + b
        }

        type Person := {
            val name: String
            val age: I32
        }

        fn new_person := (name: String, age: I32) -> Person {
            Person { name, age }
        }

        val john := new_person("John Doe", 30)
        println(john.name)
        println(john.age)

        macro print_sum := (a: I32, b: I32) {
            println(a + b)
        }

        print_sum(5, 7)

        tag #example-tag := {
            val description := "This is an example tag"
        }

        object ExampleObject := {
            val id := 1
            val name := "Example"
        }

        import some_module

        val x := 10
        fn main := {
            val x := 10
            val y := 20
            val result := x + y
            println(result)
        }

        fn add := (a: I32, b: I32) -> I32 {
            a + b
        }

        type Person := {
            val name: String
            val age: I32
        }

        fn new_person := (name: String, age: I32) -> Person {
            Person { name, age }
        }

        val john := new_person("John Doe", 30)
        println(john.name)
        println(john.age)

        macro print_sum := (a: I32, b: I32) {
            println(a + b)
        }

        print_sum(5, 7)

        tag #example-tag := {
            val description := "This is an example tag"
        }

        object ExampleObject := {
            val id := 1
            val name := "Example"
        }

        import some_module

        val x := 10
        fn main := {
            val x := 10
            val y := 20
            val result := x + y
            println(result)
        }

        fn add := (a: I32, b: I32) -> I32 {
            a + b
        }

        type Person := {
            val name: String
            val age: I32
        }

        fn new_person := (name: String, age: I32) -> Person {
            Person { name, age }
        }

        val john := new_person("John Doe", 30)
        println(john.name)
        println(john.age)

        macro print_sum := (a: I32, b: I32) {
            println(a + b)
        }

        print_sum(5, 7)

        tag #example-tag := {
            val description := "This is an example tag"
        }

        object ExampleObject := {
            val id := 1
            val name := "Example"
        }

        import some_module

        val x := 10
        fn main := {
            val x := 10
            val y := 20
            val result := x + y
            println(result)
        }

        fn add := (a: I32, b: I32) -> I32 {
            a + b
        }

        type Person := {
            val name: String
            val age: I32
        }

        fn new_person := (name: String, age: I32) -> Person {
            Person { name, age }
        }

        val john := new_person("John Doe", 30)
        println(john.name)
        println(john.age)

        macro print_sum := (a: I32, b: I32) {
            println(a + b)
        }

        print_sum(5, 7)

        tag #example-tag := {
            val description := "This is an example tag"
        }

        object ExampleObject := {
            val id := 1
            val name := "Example"
        }

        import some_module

        val x := 10
        fn main := {
            val x := 10
            val y := 20
            val result := x + y
            println(result)
        }

        fn add := (a: I32, b: I32) -> I32 {
            a + b
        }

        type Person := {
            val name: String
            val age: I32
        }

        fn new_person := (name: String, age: I32) -> Person {
            Person { name, age }
        }

        val john := new_person("John Doe", 30)
        println(john.name)
        println(john.age)

        macro print_sum := (a: I32, b: I32) {
            println(a + b)
        }

        print_sum(5, 7)

        tag #example-tag := {
            val description := "This is an example tag"
        }

        object ExampleObject := {
            val id := 1
            val name := "Example"
        }

        import some_module

        val x := 10
        fn main := {
            val x := 10
            val y := 20
            val result := x + y
            println(result)
        }

        fn add := (a: I32, b: I32) -> I32 {
            a + b
        }

        type Person := {
            val name: String
            val age: I32
        }

        fn new_person := (name: String, age: I32) -> Person {
            Person { name, age }
        }

        val john := new_person("John Doe", 30)
        println(john.name)
        println(john.age)

        macro print_sum := (a: I32, b: I32) {
            println(a + b)
        }

        print_sum(5, 7)

        tag #example-tag := {
            val description := "This is an example tag"
        }

        object ExampleObject := {
            val id := 1
            val name := "Example"
        }

        import some_module

        val x := 10
        fn main := {
            val x := 10
            val y := 20
            val result := x + y
            println(result)
        }

        fn add := (a: I32, b: I32) -> I32 {
            a + b
        }

        type Person := {
            val name: String
            val age: I32
        }

        fn new_person := (name: String, age: I32) -> Person {
            Person { name, age }
        }

        val john := new_person("John Doe", 30)
        println(john.name)
        println(john.age)

        macro print_sum := (a: I32, b: I32) {
            println(a + b)
        }

        print_sum(5, 7)

        tag #example-tag := {
            val description := "This is an example tag"
        }

        object ExampleObject := {
            val id := 1
            val name := "Example"
        }

        import some_module

        val x := 10
        fn main := {
            val x := 10
            val y := 20
            val result := x + y
            println(result)
        }

        fn add := (a: I32, b: I32) -> I32 {
            a + b
        }

        type Person := {
            val name: String
            val age: I32
        }

        fn new_person := (name: String, age: I32) -> Person {
            Person { name, age }
        }

        val john := new_person("John Doe", 30)
        println(john.name)
        println(john.age)

        macro print_sum := (a: I32, b: I32) {
            println(a + b)
        }

        print_sum(5, 7)

        tag #example-tag := {
            val description := "This is an example tag"
        }

        object ExampleObject := {
            val id := 1
            val name := "Example"
        }

        import some_module

        val x := 10
        fn main := {
            val x := 10
            val y := 20
            val result := x + y
            println(result)
        }

        fn add := (a: I32, b: I32) -> I32 {
            a + b
        }

        type Person := {
            val name: String
            val age: I32
        }

        fn new_person := (name: String, age: I32) -> Person {
            Person { name, age }
        }

        val john := new_person("John Doe", 30)
        println(john.name)
        println(john.age)

        macro print_sum := (a: I32, b: I32) {
            println(a + b)
        }

        print_sum(5, 7)

        tag #example-tag := {
            val description := "This is an example tag"
        }

        object ExampleObject := {
            val id := 1
            val name := "Example"
        }

        import some_module

        val x := 10
        fn main := {
            val x := 10
            val y := 20
            val result := x + y
            println(result)
        }

        fn add := (a: I32, b: I32) -> I32 {
            a + b
        }

        type Person := {
            val name: String
            val age: I32
        }

        fn new_person := (name: String, age: I32) -> Person {
            Person { name, age }
        }

        val john := new_person("John Doe", 30)
        println(john.name)
        println(john.age)

        macro print_sum := (a: I32, b: I32) {
            println(a + b)
        }

        print_sum(5, 7)

        tag #example-tag := {
            val description := "This is an example tag"
        }

        object ExampleObject := {
            val id := 1
            val name := "Example"
        }

        import some_module

        val x := 10
        fn main := {
            val x := 10
            val y := 20
            val result := x + y
            println(result)
        }

        fn add := (a: I32, b: I32) -> I32 {
            a + b
        }

        type Person := {
            val name: String
            val age: I32
        }

        fn new_person := (name: String, age: I32) -> Person {
            Person { name, age }
        }

        val john := new_person("John Doe", 30)
        println(john.name)
        println(john.age)

        macro print_sum := (a: I32, b: I32) {
            println(a + b)
        }

        print_sum(5, 7)

        tag #example-tag := {
            val description := "This is an example tag"
        }

        object ExampleObject := {
            val id := 1
            val name := "Example"
        }

        import some_module

        val x := 10
        fn main := {
            val x := 10
            val y := 20
            val result := x + y
            println(result)
        }

        fn add := (a: I32, b: I32) -> I32 {
            a + b
        }

        type Person := {
            val name: String
            val age: I32
        }

        fn new_person := (name: String, age: I32) -> Person {
            Person { name, age }
        }

        val john := new_person("John Doe", 30)
        println(john.name)
        println(john.age)

        macro print_sum := (a: I32, b: I32) {
            println(a + b)
        }

        print_sum(5, 7)

        tag #example-tag := {
            val description := "This is an example tag"
        }

        object ExampleObject := {
            val id := 1
            val name := "Example"
        }

        import some_module

        val x := 10
        fn main := {
            val x := 10
            val y := 20
            val result := x + y
            println(result)
        }

        fn add := (a: I32, b: I32) -> I32 {
            a + b
        }

        type Person := {
            val name: String
            val age: I32
        }

        fn new_person := (name: String, age: I32) -> Person {
            Person { name, age }
        }

        val john := new_person("John Doe", 30)
        println(john.name)
        println(john.age)

        macro print_sum := (a: I32, b: I32) {
            println(a + b)
        }

        print_sum(5, 7)

        tag #example-tag := {
            val description := "This is an example tag"
        }

        object ExampleObject := {
            val id := 1
            val name := "Example"
        }

        import some_module
        "#;
    let lexemes = lex(src);
    println!("{:#?}", lexemes);
}