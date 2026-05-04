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
use bicycle_common::{BicycleISA::JointMeasure, Pauli, TwoBases};

pub trait Architecture {
    fn for_qubits(qubits: usize) -> Self
    where
        Self: Sized;
    fn data_blocks(&self) -> usize;
    fn qubits(&self) -> usize;
    fn validate_operation(&self, op: &Operation) -> bool;
    fn ghz_meas(&self, targets: &[usize]) -> Vec<Operation>;
    fn is_magic_block(&self, block_i: usize) -> bool;
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

    fn ghz_meas(&self, targets: &[usize]) -> Vec<Operation> {
        let start = targets.iter().min().copied().unwrap_or(0);
        let end = targets.iter().max().copied().unwrap_or(0);
        let z1 = TwoBases::new(Pauli::Z, Pauli::I).unwrap();

        let mut ops = vec![];
        for i in (start..end).step_by(2).chain((start + 1..end).step_by(2)) {
            let op = vec![(i, JointMeasure(z1)), (i + 1, JointMeasure(z1))];
            ops.push(op);
        }

        ops
    }

    fn is_magic_block(&self, block_i: usize) -> bool {
        block_i == self.data_blocks() - 1
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

    fn ghz_meas(&self, targets: &[usize]) -> Vec<Operation> {
        PathArchitecture::ghz_meas(self, targets)
    }

    fn is_magic_block(&self, block_i: usize) -> bool {
        PathArchitecture::is_magic_block(self, block_i)
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

    fn ghz_meas(&self, targets: &[usize]) -> Vec<Operation> {
        let z1 = TwoBases::new(Pauli::Z, Pauli::I).unwrap();

        let mut ops = vec![];
        for i in (0..targets.len() - 1)
            .step_by(2)
            .chain((1..targets.len() - 1).step_by(2))
        {
            let op = vec![
                (targets[i], JointMeasure(z1)),
                (targets[i + 1], JointMeasure(z1)),
            ];
            ops.push(op);
        }

        ops
    }

    fn is_magic_block(&self, block_i: usize) -> bool {
        true
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

    fn ghz_meas(&self, targets: &[usize]) -> Vec<Operation> {
        FullArchitecture::ghz_meas(self, targets)
    }

    fn is_magic_block(&self, block_i: usize) -> bool {
        FullArchitecture::is_magic_block(self, block_i)
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_ghz_meas_path() {
        let z1 = TwoBases::new(Pauli::Z, Pauli::I).unwrap();
        let arch = PathArchitecture { data_blocks: 4 };

        let ops = arch.ghz_meas(&[0, 1, 2, 3]);
        // One joint operation
        let joint_ops: Vec<_> = ops.iter().filter(|op| op.len() == 2).collect();
        assert_eq!(3, joint_ops.len());

        let zz_meas = vec![
            vec![(0usize, JointMeasure(z1)), (1usize, JointMeasure(z1))],
            vec![(2usize, JointMeasure(z1)), (3usize, JointMeasure(z1))],
            vec![(1usize, JointMeasure(z1)), (2usize, JointMeasure(z1))],
        ];

        for (i, (expected, actual)) in zz_meas.iter().zip(joint_ops.iter().copied()).enumerate() {
            assert_eq!(expected, actual, "mismatch at index {i}");
        }
    }

    #[test]
    fn test_ghz_meas_full() {
        let z1 = TwoBases::new(Pauli::Z, Pauli::I).unwrap();
        let arch = FullArchitecture { data_blocks: 4 };

        let ops = arch.ghz_meas(&[0, 1, 2, 3]);
        println!("GHZ measurement operations: {:?}", ops);

        // Two joint operations
        let joint_ops: Vec<_> = ops.iter().filter(|op| op.len() == 2).cloned().collect();
        assert_eq!(3, joint_ops.len());

        let expected_ops = vec![
            vec![(0usize, JointMeasure(z1)), (1usize, JointMeasure(z1))],
            vec![(2usize, JointMeasure(z1)), (3usize, JointMeasure(z1))],
            vec![(1usize, JointMeasure(z1)), (2usize, JointMeasure(z1))],
        ];

        assert_eq!(expected_ops, joint_ops);
    }
}
