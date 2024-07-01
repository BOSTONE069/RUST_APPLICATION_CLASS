pub fn matchflowspatterns(){


    #[derive(Debug)]
    enum Language {
        English,
        Spanish,
        Russian,
        Japanese,
    }

    let language = Language::English;

    match language {
        Language::English => println!("Hello World"),
        Language::Spanish => println!("Hola Mundo"),
        Language::Russian => println!("jvhjfjvru"),
        _ => println!("Unsupportted language")
    }


}
