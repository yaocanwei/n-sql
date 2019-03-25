// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use Identifier;

#[derive(Debug)]
pub enum DataType {
    I64,
    F64,
    String,
}

#[derive(Debug)]
pub struct DatabaseInfo {
    name: Identifier,
    schemas: Vec<SchemaInfo>,
}

#[derive(Debug)]
pub struct SchemaInfo {
    name: Identifier,
    tables: Vec<TableInfo>,
}

#[derive(Debug)]
pub struct TableInfo {
    name: Identifier,
    columns: Vec<ColumnInfo>,
}

#[derive(Debug)]
pub struct ColumnInfo {
    name: Identifier,
}

impl DatabaseInfo {
    pub fn new(name: Identifier) -> DatabaseInfo {
        DatabaseInfo {
            name,
            schemas: Vec::new(),
        }
    }
    pub fn name(&self) -> &Identifier {
        &self.name
    }
    pub fn add_schema(&mut self, s: SchemaInfo) {
        self.schemas.push(s)
    }
}

impl SchemaInfo {
    pub fn new(name: Identifier) -> SchemaInfo {
        SchemaInfo {
            name,
            tables: Vec::new(),
        }
    }
    pub fn name(&self) -> &Identifier {
        &self.name
    }

    pub fn add_table(&mut self, t: TableInfo) {
        self.tables.push(t)
    }
}

impl TableInfo {
    pub fn new(name: Identifier) -> TableInfo {
        TableInfo {
            name,
            columns: Vec::new(),
        }
    }

    pub fn name(&self) -> &Identifier {
        &self.name
    }
    pub fn add_column(&mut self, c: ColumnInfo) {
        self.columns.push(c)
    }
}

impl ColumnInfo {
    pub fn new(name: Identifier) -> ColumnInfo {
        ColumnInfo { name }
    }
    pub fn name(&self) -> &Identifier {
        &self.name
    }
}
