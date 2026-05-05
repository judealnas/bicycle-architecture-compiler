
from qiskit import transpile, QuantumCircuit
from qiskit.quantum_info import get_clifford_gate_names
from qiskit.transpiler.passes import LitinskiTransformation, RemoveBarriers

from scripts.qiskit_parser import iter_qiskit_pbc_circuit


def compile_pbc(circuit: QuantumCircuit):
    """Compile a Qiskit circuit and yield PBC instructions."""
    basis = ["rz", "t", "tdg"] + get_clifford_gate_names()
    tqc = transpile(circuit, basis_gates=basis)

    lit = LitinskiTransformation(fix_clifford=False)
    rb = RemoveBarriers()
    pbc = lit(rb(tqc))

    for inst in iter_qiskit_pbc_circuit(pbc, as_str=True):
        assert isinstance(inst, str)
        yield inst


if __name__ == "__main__":
    # read the number of qubits from the command line (or set to 10 as default)
    import sys
    import argparse
    from pathlib import Path
    from qiskit import QuantumCircuit

    def build_parser():
        parser = argparse.ArgumentParser(
            description="Compile a Qiskit circuit to PBC instructions."
        )
        parser.add_argument(
            "circuit_file",
            type=Path,
            help="Path to the QASM file containing the quantum circuit to compile.",
        )
        parser.add_argument(
            "--output_file",
            "-o",
            type=Path,
            default=None,
            help="Path to the output file where the PBC instructions will be saved. If not provided, the stem of the input file will be used with a .pbc extension.",
        )
        return parser

    parser = build_parser()
    args = parser.parse_args()

    input_file = args.circuit_file
    output_file = args.output_file or input_file.with_suffix(".pbc")

    assert isinstance(input_file, Path)
    if not input_file.is_file():
        print(f"Error: Input file {input_file} does not exist.")
        sys.exit(1)

    assert isinstance(output_file, Path)

    qc = QuantumCircuit.from_qasm_file(input_file)
    with output_file.open("w") as f:
        for inst in compile_pbc(qc):
            f.write(inst + "\n")
