pub trait Validatable{
    fn validate(&self) -> (bool, Option<&dyn crate::commons::fields::traits::validation_error::ValidationError>);
    fn put_value(&mut self, value: String);
    fn get_raw_value(&self) -> &String;
    fn new(value: String) -> Self where Self: Sized;
    fn cclone(&self) -> Box<dyn Validatable>;
}