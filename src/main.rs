use miden_vm::{math::{Felt, FieldElement}, verify, Assembler, DefaultHost, Host, Kernel, ProgramInfo, ProvingOptions, StackInputs};

fn compute_fibonacci(n: usize) -> Felt{

    let mut t0 = Felt::ZERO;
    let mut t1 = Felt::ONE;

    for _ in 0..n{
        t1 = t0+t1;
        core::mem::swap(&mut t0, &mut t1);
    }

    t0

}

fn main(){

    let program = format!(
        "
        begin
            repeat.{}
                swap dup.1 add
            end
        end
        ",
        15
    );

    let prgm = Assembler::default().assemble_program(program).unwrap();
    let expected_result = vec![compute_fibonacci(16)];

    let (mut outputs, proof) = miden_vm::prove(&prgm, StackInputs::try_from_ints([0,1]).unwrap(), DefaultHost::default(), ProvingOptions::default()).unwrap();
    println!("{:?} \n {:?}", outputs.stack_truncated(1), expected_result);

    let kernel = Kernel::default();
    let prgm_info = ProgramInfo::new(prgm.hash(), kernel);

    verify(prgm_info, StackInputs::try_from_ints([0,1]).unwrap(),outputs, proof);

}