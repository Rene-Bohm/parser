use core::fmt;
use std::collections::HashSet;
use std::collections::HashMap;
use std::fmt::Error;

#[derive(Debug, PartialEq,Clone)]
pub struct Alphabet{
    elements: HashSet<char>,
}


impl Alphabet{

    //This method creates a new instance of Alphabet with an empty set of elements.
    pub fn new ()-> Self{
        Self { elements: HashSet::<char>::new()}
    }

    //This method takes a character as a parameter and returns true if the character 
    //is contained in the set of elements for the Alphabet instance, or false if it is not.
    pub fn contains(&self, character: &char) -> bool{
        self.elements.contains(character)
    }

    //This method takes a character as a parameter and adds it to the set of elements for 
    //the Alphabet instance.
    pub fn insert (&mut self, character: &char) {
        self.elements.insert(character.clone());
    }

    //This method takes a character as a parameter and removes it from the set of elements
    //for the Alphabet instance.
    pub fn delete (&mut self, character: &char) {

        self.elements.remove(character);

    }

    //This method takes another Alphabet instance as a parameter and returns a new Alphabet 
    //instance that contains all the elements from both Alphabet instances.
    pub fn union (&self, other: &Alphabet) -> Self{

        let mut new: HashSet<char> = self.elements.clone();

        for element in &other.elements{
            new.insert(*element);
        }

        Self { elements: new.clone() }

    }

    //This method takes another Alphabet instance as a parameter and returns a new Alphabet 
    //instance that contains only the elements that are present in both Alphabet instances.
    pub fn intersection(&self, other: &Alphabet) -> Self{

        if self.elements.capacity() >= other.elements.capacity(){
            let lh_set: HashSet<char> = other.elements.clone(); // smaller
            let rh_set: HashSet<char> = self.elements.clone(); // bigger
            let mut inter: HashSet<char> = HashSet::new();
            
            for element in lh_set{

                if rh_set.contains(&element){

                    inter.insert(element);
                    
                }
            }
        
            Self { elements: inter.clone()}

        }else{
            let lh_set: HashSet<char> = self.elements.clone();
            let rh_set: HashSet<char> = other.elements.clone();
            let mut inter: HashSet<char> = HashSet::new();
            
            for element in lh_set{

                if rh_set.contains(&element){

                    inter.insert(element);
                    
                }
            }
        
            Self { elements: inter.clone()}
        }

    }



}

macro_rules! alphabet_from_vec {
    ( $vec:expr ) => {
        {
            let mut alphabet = Alphabet::new();
            for c in $vec {
                alphabet.insert(&c);
            }
            alphabet
        }
    };
}

#[test]
fn test_alphabet() {
    // Test that the `new` method creates a new `Alphabet` instance with an empty `elements` field:
    let mut alphabet = Alphabet::new();
    assert!(alphabet.elements.is_empty());

    // Test that the `insert` method adds a character to the `elements` field:
    let c = 'a';
    alphabet.insert(&c);
    assert!(alphabet.elements.contains(&c));

    // Test that the `delete` method removes a character from the `elements` field:
    alphabet.delete(&c);
    assert!(!alphabet.elements.contains(&c));

    // Test that the `insert` method does not add a character that is already present in the `elements` field:
    alphabet.insert(&c);
    alphabet.insert(&c);
    assert_eq!(alphabet.elements.len(), 1);

    // Test that the `delete` method does not remove a character that is not present in the `elements` field:
    alphabet.delete(&c);
    assert!(alphabet.elements.is_empty());

    let alphabet = alphabet_from_vec!(vec!['a', 'b', 'c']);
    assert_eq!(alphabet.elements, HashSet::<char>::from(['a', 'b', 'c']));

    let lh = alphabet_from_vec!(vec!['b', 'c', 'd', 'e']);
    let rh = alphabet_from_vec!(vec!['a', 'b', 'c']);
    let inter = lh.intersection(&rh);

    assert_eq!(inter, alphabet_from_vec!(vec!['b', 'c']));

    let lh = alphabet_from_vec!(vec!['b', 'c', 'd', 'e']);
    let rh = alphabet_from_vec!(vec!['a', 'b', 'F']);
    let union = lh.union(&rh);

    assert_eq!(union, alphabet_from_vec!(vec!['a','b', 'c', 'd', 'e', 'F']));
}

#[derive(Debug, PartialEq,Clone, Eq, Hash)]
pub struct Word{
    word: String,
}

impl Word{

    //Check if the characters in the `word` field of this `Word` instance are all contained in the given `Alphabet` instance.
    pub fn alphabet_checker(&self, alphabet: &Alphabet) -> bool {
        for chars in self.word.chars(){
            if !alphabet.elements.contains(&chars) {
                return false;
            }
        }
        true
    }

}

impl From<String> for Word{
    fn from(word: String) -> Self {
        Self { word: word.clone() }
    }
}

impl From<&str> for Word{
    fn from(word: &str) -> Self {
        Self { word: word.to_string() }
    }
}

#[test]
fn test_word() {

    let alphabet = alphabet_from_vec!(vec!['a', 'b', 'c']);
    let word: Word = Word::from(String::from("abc"));
    
    assert_eq!(word.alphabet_checker(&alphabet), true);

    let alphabet = alphabet_from_vec!(vec!['a', 'b']);
    let word: Word = Word::from("abc");
    
    assert_eq!(word.alphabet_checker(&alphabet), false);
    
}

#[derive(Debug, PartialEq,Clone)]
pub struct Productions{

    //A mapping from `Word` instances to vectors of `Word` instances, representing the 
    //production rules of a grammar.
    rules: HashMap<Word, Vec<Word>>,
    
}

impl Productions{

    // Create a new instance of `Productions` with an empty map of production rules.
    pub fn new() -> Self{
        Productions { rules: HashMap::<Word, Vec<Word>>::new() }
    }

    // Add a new production rule to the map, with the given `Word` instance as the key 
    //and the given vector of `Word` instances as the value.
    pub fn insert(&mut self, key: &Word, productions: &Vec<Word>) {
        self.rules.insert(key.clone(), productions.clone());
    }

    //Check if all the `Word` instances used in the production rules consist of characters
    //that are within a specific `Alphabet` instance.

    pub fn alphabet_checker(&self, alphabet: &Alphabet) -> bool{

        let mut word_set = HashSet::<Word>::new();

        for (key, productions) in self.rules.iter(){

            word_set.insert(key.clone());

            for word in productions{

                word_set.insert(word.clone());

            }

        }

        for word in &word_set{

            if !word.alphabet_checker(alphabet){
                println!("{:?}",&word);
                return false
            }
        }
        true
    }
}


#[derive(Debug, PartialEq,Clone)]
pub struct Grammar{

    non_terminals: Alphabet,
    terminals: Alphabet,
    start_terminal: char,
    productions: Productions,

}

impl Grammar {
    
    pub fn new(
        non_terminal:&Alphabet, 
        terminal:&Alphabet, 
        start:char, 
        productions: Productions
    ) -> Result<Self, Error>{

        let union = non_terminal.clone().union(&terminal);

        if !non_terminal.intersection(terminal).elements.is_empty(){

            println!("first");
            return Result::Err(fmt::Error)

        }else if !non_terminal.contains(&start) {
            
            println!("second");
            return Result::Err(fmt::Error)

        }else if !productions.alphabet_checker(&union) {
            println!("third");
            return Result::Err(fmt::Error)
        }else{

            Ok(Self{ 
                non_terminals: non_terminal.clone(), 
                terminals: terminal.clone(), 
                start_terminal: start, 
                productions: productions.clone() 
                }
            )
        }
    }

    pub fn apply(&self, key: &Word, production: usize, input: &mut String) -> String{

        let character = &self.productions.rules.get(key).unwrap()[production];

        for chara in input.chars(){

            if key.word = chara.to_string(){

                input.replace(from, to)

            }

        }

        input.to_string()

    }

}

#[test]
fn gram() {

    let terminal = alphabet_from_vec!(vec!['a','b']);
    let non_term = alphabet_from_vec!(vec!['S','N']);
    let mut productions = Productions::new();
    productions.insert(
        &Word::from("S"), 
        &vec![
            Word::from("N"), 
        ]   
    );
    productions.insert(
        &Word::from("N"), 
        &vec![
            Word::from("aNb"), 
            Word::from("ab")
        ]
    );

    println!("{:?}", Grammar::new(&non_term, &terminal, 'S', productions.clone()));

    let terminal = alphabet_from_vec!(vec!['a','b']);
    let non_term = alphabet_from_vec!(vec!['S','N']);
    let mut productions = Productions::new();
    productions.insert(
        &Word::from("S"), 
        &vec![
            Word::from("N"), 
            Word::from("e")
        ]   
    );
    productions.insert(
        &Word::from("N"), 
        &vec![
            Word::from("aNb"), 
            Word::from("ab")
        ]
    );

    println!("{:?}", Grammar::new(&non_term, &terminal, 'S', productions.clone()))

}