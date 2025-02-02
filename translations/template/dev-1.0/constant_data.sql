PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS "string_data" (
	"id"	INTEGER UNIQUE,
	"string_value"	TEXT UNIQUE,
	PRIMARY KEY("id")
);
INSERT INTO string_data VALUES(0,'Config file is either empty or missing');
INSERT INTO string_data VALUES(1,'Created config');
INSERT INTO string_data VALUES(2,'Creating vault file');
INSERT INTO string_data VALUES(3,'Please enter a secure master password');
INSERT INTO string_data VALUES(4,'enter');
INSERT INTO string_data VALUES(5,'your');
INSERT INTO string_data VALUES(6,'Error message');
INSERT INTO string_data VALUES(7,'Oops, an error occurred');
INSERT INTO string_data VALUES(8,'Key file is either empty or missing');
INSERT INTO string_data VALUES(9,'minimum');
INSERT INTO string_data VALUES(10,'option');
INSERT INTO string_data VALUES(11,'or');
INSERT INTO string_data VALUES(12,'required');
INSERT INTO string_data VALUES(13,'setting');
INSERT INTO string_data VALUES(14,'Success');
INSERT INTO string_data VALUES(15,'try again');
INSERT INTO string_data VALUES(16,'send ''n'' to use default');
INSERT INTO string_data VALUES(17,replace(replace('[IMPORTANT] Remember your MASTER PASSWORD!\n\nPlease READ information below!:\r\0121. This master password is used to access passwords stored in the vault.\r\0122. Keep the master password in a secure place. DON''T store it unsecurely (bad storage example: txt file)\r\0123. There is no recovery option. It''s crucial to memorize or securely store it.','\r',char(13)),'\012',char(10)));
INSERT INTO string_data VALUES(18,replace('HELP: \n RM- Requires Master\n mastervault add - adds information and password into the vault RM\n mastervault config - lists the config and allows for change RM\n mastervault get [name?] - gets a specific password from a name RM\n mastervault update [name?] [password?] - updates a set of information RM\n mastervault delete [name?] - delets a set of information RM\n mastervault list - lists all names (no passwords)\n mastervault search [search?] - searches through all names (no passwords)\n master change-master [old-master?] [new-master?] - attempts to change master (100% requires master)','\n',char(10)));
CREATE TABLE IF NOT EXISTS "other_translations" (
	"id"	INTEGER,
	"setting"	TEXT,
	"error"	TEXT,
	PRIMARY KEY("id")
);
INSERT INTO other_translations VALUES(0,'username','Too short');
INSERT INTO other_translations VALUES(1,'email','Not enough special characters');
INSERT INTO other_translations VALUES(2,'hash length','Not enough lowercase characters');
INSERT INTO other_translations VALUES(3,'memory in kib','Not enough uppercase characters');
INSERT INTO other_translations VALUES(4,'parallelism','Other error');
INSERT INTO other_translations VALUES(5,'algorithm','The password isn''t strong enough');
INSERT INTO other_translations VALUES(6,'iterations','NotFound: The algorithm was not found, existing algorithms: [argon2, argon2i, argon2id] (Recommended: argon2id).');
INSERT INTO other_translations VALUES(7,'not found','DomainElementMissing: TLD (Top Level Domain) or DOMAIN is missing. (NO DOT)');
INSERT INTO other_translations VALUES(8,'This','AtSignMissing: The @ is missing');
INSERT INTO other_translations VALUES(9,NULL,'ConsecutiveDots: The email contains consecutive dots (..), which are not allowed.');
INSERT INTO other_translations VALUES(10,NULL,'AtSignNextToDot: The email contains @ that has dot next to it (@. or .@)');
INSERT INTO other_translations VALUES(11,NULL,'LastOrFirstIsDot: The email begins or ends with a dot (.email or email.)');
INSERT INTO other_translations VALUES(12,NULL,'MultipleAtSigns: The email contains multiple @ signs, which is not allowed.');
INSERT INTO other_translations VALUES(13,NULL,'NotAllowedCharacters: Email contains characters that are not allowed in the local part. Allowed characters: [a-z],[A-Z],[1-9],[.],[-],[_]');
INSERT INTO other_translations VALUES(14,NULL,'ConsecutiveCharacters: Special characters are consecutive.');
INSERT INTO other_translations VALUES(15,NULL,'ParsingFailed: Could not get the value from string.');
INSERT INTO other_translations VALUES(16,NULL,'SettingNotFound: Setting not found in config settings.');
INSERT INTO other_translations VALUES(17,NULL,'U32ConversionFailed: Conversion to number (u32) failed. Please enter a number without any other characters.');
INSERT INTO other_translations VALUES(18,NULL,'ValidationFailed: The validation on this option failed');
INSERT INTO other_translations VALUES(19,NULL,'IsInvalid: some entryies are invalid');
INSERT INTO other_translations VALUES(20,NULL,'OsError: Error relating operating system occurred.');
INSERT INTO other_translations VALUES(21,NULL,'TooSmall: the minimum size of hash_length is 4. (Recommended: 32)');
INSERT INTO other_translations VALUES(22,NULL,'TooSmall: the minimum size of memory_in_kib is 16384. (Recommended: 32768)');
INSERT INTO other_translations VALUES(23,NULL,'TooSmall: the minimum size of parallelism is 1. (Recommended: 1)');
INSERT INTO other_translations VALUES(24,NULL,'TooShort: The username is too short. The length needs to be at least 3 characters long');
COMMIT;
