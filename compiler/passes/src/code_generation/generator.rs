// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::SymbolTable;

use leo_ast::{Function, Identifier};
use leo_errors::emitter::Handler;

use std::collections::HashMap;

pub struct CodeGenerator<'a> {
    /// Symbol table.
    // TODO: Including this for now. Remove if not needed.
    symbol_table: &'a mut SymbolTable<'a>,
    /// Error handler.
    handler: &'a Handler,
    /// A counter to track the next available register.
    pub(crate) next_register: u64,
    /// Reference to the current function.
    pub(crate) current_function: Option<&'a Function>,
    /// Mapping of variables to registers.
    pub(crate) variable_mapping: HashMap<&'a Identifier, String>,
}

impl<'a> CodeGenerator<'a> {
    /// Initializes a new `CodeGenerator`.
    pub fn new(symbol_table: &'a mut SymbolTable<'a>, handler: &'a Handler) -> Self {
        Self {
            symbol_table,
            handler,
            next_register: 0,
            current_function: None,
            variable_mapping: HashMap::new(),
        }
    }
}
