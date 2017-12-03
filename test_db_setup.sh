#!/bin/bash


# First need a DB user with CREATEDB permission:
# CREATE USER <test_user>;
# ALTER USER <test_user> CREATEDB;

(source test.env && diesel db reset --database-url $TEST_DATABASE_URL)
(source test.env && diesel setup --database-url $TEST_DATABASE_URL)
