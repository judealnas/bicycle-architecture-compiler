// Copyright contributors to the Bicycle Architecture Compiler project
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::operation::Operation;

pub trait Architecture {
    fn for_qubits(qubits: usize) -> Self
    where
        Self: Sized;
    fn data_blocks(&self) -> usize;
    fn qubits(&self) -> usize;
    fn validate_operation(&self, op: &Operation) -> bool;
    fn find_path(&self, _start: usize, _end: usize) -> Option<Vec<usize>> {
        // Find a path from one block to another
        None
    }
}

/// Consists of blocks plus one magic state factory at the end of the path
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PathArchitecture {
    pub data_blocks: usize,
}

impl PathArchitecture {
    pub fn for_qubits(qubits: usize) -> Self {
        let data_blocks = qubits.div_ceil(11);

        Self { data_blocks }
    }

    pub fn data_blocks(&self) -> usize {
        self.data_blocks
    }

    pub fn qubits(&self) -> usize {
        self.data_blocks * 11
    }

    pub fn validate_operation(&self, op: &Operation) -> bool {
        // Check that operations act on successive blocks
        if op.len() == 1 {
            true
        } else {
            op[0].0.abs_diff(op[1].0) == 1
        }
    }

    pub fn find_path(&self, start: usize, end: usize) -> Option<Vec<usize>> {
        // NOTE: Intentionally not checking that start and end are valid blocks
        // since magic state factory block assumed to have index `data_blocks`
        // if start >= self.data_blocks() || end >= self.data_blocks() {
        //     return None;
        // }
        Some((start.min(end)..=start.max(end)).collect())
    }
}

impl Architecture for PathArchitecture {
    fn for_qubits(qubits: usize) -> Self {
        PathArchitecture::for_qubits(qubits)
    }

    fn data_blocks(&self) -> usize {
        PathArchitecture::data_blocks(self)
    }

    fn qubits(&self) -> usize {
        PathArchitecture::data_blocks(self) * 11
    }

    fn validate_operation(&self, op: &Operation) -> bool {
        PathArchitecture::validate_operation(self, op)
    }

    fn find_path(&self, start: usize, end: usize) -> Option<Vec<usize>> {
        PathArchitecture::find_path(self, start, end)
    }
}

/// Consists of blocks plus one magic state factory with full connectivity between all blocks
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct FullArchitecture {
    pub data_blocks: usize,
}

impl FullArchitecture {
    pub fn for_qubits(qubits: usize) -> Self {
        let data_blocks = qubits.div_ceil(11);

        Self { data_blocks }
    }

    pub fn data_blocks(&self) -> usize {
        self.data_blocks
    }

    pub fn qubits(&self) -> usize {
        self.data_blocks * 11
    }

    pub fn validate_operation(&self, _op: &Operation) -> bool {
        // All operations are valid on a fully connected architecture
        true
    }

    pub fn find_path(&self, start: usize, end: usize) -> Option<Vec<usize>> {
        // NOTE: Intentionally not checking that start and end are valid blocks
        // since magic state factory block assumed to have index `data_blocks`
        // if start >= self.data_blocks() || end >= self.data_blocks() {
        //     return None;
        // }
        Some(vec![start, end])
    }
}

impl Architecture for FullArchitecture {
    fn for_qubits(qubits: usize) -> Self {
        FullArchitecture::for_qubits(qubits)
    }

    fn data_blocks(&self) -> usize {
        FullArchitecture::data_blocks(self)
    }

    fn qubits(&self) -> usize {
        FullArchitecture::data_blocks(self) * 11
    }

    fn validate_operation(&self, op: &Operation) -> bool {
        FullArchitecture::validate_operation(self, op)
    }

    fn find_path(&self, start: usize, end: usize) -> Option<Vec<usize>> {
        FullArchitecture::find_path(self, start, end)
    }
}
