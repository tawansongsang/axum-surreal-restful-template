USE NS ns_template;
USE DB db_template;

-- Create schemafull users table
DEFINE TABLE users SCHEMAFULL;

-- Define some fields.
DEFINE FIELD username ON TABLE users TYPE string ASSERT string::is::email($value);
DEFINE FIELD email ON TABLE users TYPE string ASSERT string::is::email($value);
DEFINE FIELD email_verified ON TABLE users TYPE option<datetime>;
DEFINE FIELD title ON TABLE users TYPE string ASSERT $value INSIDE ['นาย', 'นางสาว', 'นาง'];
DEFINE FIELD firstname ON TABLE users TYPE string;
DEFINE FIELD middlename ON TABLE users TYPE option<string>;
DEFINE FIELD lastname ON TABLE users TYPE string;
DEFINE FIELD password ON TABLE users TYPE string;
DEFINE FIELD password_salt ON TABLE users TYPE uuid DEFAULT rand::uuid::v4();
DEFINE FIELD token_salt ON TABLE users TYPE uuid DEFAULT rand::uuid::v4();
DEFINE FIELD role ON TABLE users TYPE string DEFAULT 'USER' ASSERT $value INSIDE ['USER', 'ADMIN'];
DEFINE FIELD image ON TABLE users TYPE option<string>;
DEFINE FIELD create_by ON TABLE users TYPE option<record<users>>;
DEFINE FIELD create_on ON TABLE users TYPE datetime DEFAULT time::now();
DEFINE FIELD update_by ON TABLE users TYPE option<record<users>>;
DEFINE FIELD update_on ON TABLE users TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_by ON TABLE users TYPE option<record<users>>;
DEFINE FIELD deleted_on ON TABLE users TYPE option<datetime>;

DEFINE INDEX usernameIndex ON TABLE users COLUMNS username UNIQUE;
DEFINE INDEX emailIndex ON TABLE users COLUMNS email UNIQUE;