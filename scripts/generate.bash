#!/usr/bin/env bash

set -u

DATABASE="tmp.db"

rm model.rs schema.rs || echo "no files to remove"
rm "${DATABASE}" || echo "no db to remove"
cat create.sql | sqlite3 "${DATABASE}"
diesel print-schema --database-url "${DATABASE}" >schema.rs
echo "schema generated"
cat schema.rs
diesel_ext -s schema.rs -m >model.rs
echo "model generated"
cat model.rs
