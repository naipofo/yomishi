-- ------------------------------
-- TABLE: config
-- ------------------------------

DEFINE TABLE config SCHEMAFULL;

DEFINE FIELD value ON config TYPE string;

-- ------------------------------
-- TABLE: dictionary
-- ------------------------------

DEFINE TABLE dictionary SCHEMAFULL;

DEFINE FIELD revision ON dictionary TYPE string;
DEFINE FIELD title ON dictionary TYPE string;

-- ------------------------------
-- TABLE: tag
-- ------------------------------

DEFINE TABLE tag SCHEMAFULL;

DEFINE FIELD id ON tag;
DEFINE FIELD category ON tag TYPE string;
DEFINE FIELD dictionary ON tag TYPE record<dictionary>;
DEFINE FIELD notes ON tag TYPE string;
DEFINE FIELD popularity ON tag TYPE int;
DEFINE FIELD sorting ON tag TYPE int;

-- ------------------------------
-- TABLE: term
-- ------------------------------

DEFINE TABLE term SCHEMAFULL;

DEFINE FIELD definition_tags ON term TYPE string;
DEFINE FIELD dictionary ON term TYPE record<dictionary>;
DEFINE FIELD expression ON term TYPE string;
DEFINE FIELD glossary ON term TYPE string;
DEFINE FIELD reading ON term TYPE string;
DEFINE FIELD rules ON term TYPE string;
DEFINE FIELD score ON term TYPE int;
DEFINE FIELD sequence ON term TYPE int;
DEFINE FIELD tags ON term TYPE array;
DEFINE FIELD tags[*] ON term TYPE record<tag>;

DEFINE INDEX termExpressionIndex ON term FIELDS expression;

-- ------------------------------
-- TABLE: term_meta
-- ------------------------------

DEFINE TABLE term_meta SCHEMAFULL;

DEFINE FIELD dictionary ON term_meta TYPE record<dictionary>;
DEFINE FIELD entry ON term_meta TYPE string;
DEFINE FIELD reading ON term_meta TYPE option<string>;
DEFINE FIELD term ON term_meta TYPE string;

DEFINE INDEX termMetaIndex ON term_meta FIELDS term;