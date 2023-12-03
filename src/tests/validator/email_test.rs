
#[test]
pub fn test_incorrect_email(){
    use crate::commons::fields::{structs::email::EmailData, traits::validatable::Validatable};

    let test_cases = [
        "alice.smith123@example..com",
        "bob.jones@emailprovider@net",
        "charlie_12@email-service..org",
        "diana-doe1234@examplemail.biz@",
        "eric@samplemail..co.uk",
        "fiona@randomemail@.info",
        "george.smith@mailing-lists.us@",
        "helen-doe@temporarymail@.ru",
        "ian.smith@email-hosting@.tv",
        "julia_jones@emailplace..edu",
        "kyle@example-email@.jobs",
        "lisa.doe@webmail@mobi",
        "matt@emailinbox..travel",
        "nina.jones@testingmail@.name",
        "oliver@emailtest..eu",
        "penny.smith@myemailzone@.asia",
        "quincy@sampleemail@.pro",
        "rachel.doe@emailhub..co",
        "steve-jones@quickmailsolutions@.aero",
        "tina@emailtester..jobs",
        "ulrich@emailcheck@.biz",
        "vanessa_doe@checkmymail..info",
        "william.jones@randommailplace@.us",
        "xander@dummyemail@.tv",
        "yvonne@emailtest@.edu",
        "zane-smith@examplemailinglist@.name",
        "alice.smith@emailhosting@.org",
        "brandon@emailprovider@.tv",
        "cynthia_doe@emailservice..travel",
        "daniel@emailinbox@.jobs",
        "emma@emailtest@.name",
        "fred@emailplace..info",
        "gina-jones@myemailzone@.biz",
        "hank@emailtester@.us",
        "isabel@emailtestplace..edu",
        "jack.doe@webmail..pro",
        "kelly@emailtestmail@.tv",
        "logan-jones@temporarymail@.biz",
        "mia.smith@checkmyemail..info",
        "nathan@emailinbox@co",
        "olivia.doe@emailhub@.travel",
        "paul@emailcheckname@.biz",
        "quinn.jones@sampleemail..co.uk",
        "ryan_doe@quickmailsolutions@.jobs",
        "sophia@emailtestmail..name",
        "tyler@emailinbox@.biz",
        "ursula.doe@randommailplace@.tv",
        "victor@emailservice@.jobs",
        "wendy_jones@examplemail@.info",
        "xavier@emailcheck@.tv",
        "email.@"
    ];

    for &email_str in &test_cases {
        let email_data = EmailData { email: email_str.to_string() };
        assert_eq!(email_data.email, email_str);
        let email_validation_result = email_data.validate();
        assert_eq!(email_validation_result.0, false, "Email falsely validated: {}", email_str);
    }
}

#[test]
fn test_correct_email() {
    use crate::commons::fields::{structs::email::EmailData, traits::validatable::Validatable};

    let test_cases = [
        "alice.smith123@example.com",
        "bob.jones@emailprovider.net",
        "charlie_12@email-service.org",
        "diana-doe1234@examplemail.biz",
        "eric@samplemail.co.uk",
        "fiona@randomemail.info",
        "george.smith@mailing-lists.us",
        "helen-doe@temporarymail.ru",
        "ian.smith@email-hosting.tv",
        "julia_jones@emailplace.edu",
        "kyle@example-email.jobs",
        "lisa.doe@webmail.mobi",
        "matt@emailinbox.travel",
        "nina.jones@testingmail.name",
        "oliver@emailtest.eu",
        "penny.smith@myemailzone.asia",
        "quincy@sampleemail.pro",
        "rachel.doe@emailhub.co",
        "steve-jones@quickmailsolutions.aero",
        "tina@emailtester.jobs",
        "ulrich@emailcheck.biz",
        "vanessa_doe@checkmymail.info",
        "william.jones@randommailplace.us",
        "xander@dummyemail.tv",
        "yvonne@emailtest.edu",
        "zane-smith@examplemailinglist.name",
        "alice.smith@emailhosting.org",
        "brandon@emailprovider.tv",
        "cynthia_doe@emailservice.travel",
        "daniel@emailinbox.jobs",
        "emma@emailtest.name",
        "fred@emailplace.info",
        "gina-jones@myemailzone.biz",
        "hank@emailtester.us",
        "isabel@emailtestplace.edu",
        "jack.doe@webmail.pro",
        "kelly@emailtestmail.tv",
        "logan-jones@temporarymail.biz",
        "mia.smith@checkmyemail.info",
        "nathan@emailinbox.co",
        "olivia.doe@emailhub.travel",
        "paul@emailcheckname.biz",
        "quinn.jones@sampleemail.co.uk",
        "ryan_doe@quickmailsolutions.jobs",
        "sophia@emailtestmail.name",
        "tyler@emailinbox.biz",
        "ursula.doe@randommailplace.tv",
        "victor@emailservice.jobs",
        "wendy_jones@examplemail.info",
        "xavier@emailcheck.tv",
    ];

    for email_str in test_cases{
        let email_data = EmailData {email: email_str.to_string()};
        assert_eq!(email_data.email, email_str);
        let email_validation_result = email_data.validate();
        assert_eq!(email_validation_result.0, true, "Email falsely validated: {} message: {}", email_str, email_validation_result.1.unwrap().description());
    }
}