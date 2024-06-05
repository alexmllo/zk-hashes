use plonky2::{
    field::extension::Extendable,
    hash::hash_types::RichField,
    iop::{ext_target::ExtensionTarget, generator::SimpleGenerator, target::Target, witness::{Witness, WitnessWrite}},
    plonk::circuit_builder::CircuitBuilder, util::serialization::{Read, Write},
};

mod constants;
mod mds;
pub mod rescue_prime;

pub const ALPHA: usize = 7;
pub const ALPHA_INV: usize = 10540996611094048183;

pub const SPONGE_RATE: usize = 8;
pub const SPONGE_CAPACITY: usize = 4;
pub const SPONGE_WIDTH: usize = SPONGE_RATE + SPONGE_CAPACITY;

// Number of rounds calculated using the sage code provided in the rescue-prime paper
pub const NUMBER_OF_ROUNDS: usize = 7;

trait CircuitBuilderExtensions<F: RichField + Extendable<D>, const D: usize> {
    fn exp_inv(&mut self, x: Target) -> Target;
    fn exp_inv_extension(&mut self, x: ExtensionTarget<D>) -> ExtensionTarget<D>;
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilderExtensions<F, D>
    for CircuitBuilder<F, D>
{
    fn exp_inv(&mut self, x: Target) -> Target {
        let x_ext = self.convert_to_ext(x);
        Self::exp_inv_extension(self, x_ext).0[0]
    }

    fn exp_inv_extension(&mut self, x: ExtensionTarget<D>) -> ExtensionTarget<D> {
        let exp_inv = self.add_virtual_extension_target();
        self.add_simple_generator(ExpGeneratorExtension {
            base: x,
            exp_result: exp_inv,
        });

        // Enforce that y^d = x
        let x2 = self.mul_extension(exp_inv, exp_inv);
        let x4 = self.mul_extension(x2, x2);
        let x6 = self.mul_extension(x4, x2);
        let y_inv = self.mul_extension(x6, exp_inv);
        self.connect_extension(y_inv, x);

        exp_inv
    }
}

#[derive(Debug, Default)]
pub struct ExpGeneratorExtension<const D: usize> {
    base: ExtensionTarget<D>,
    exp_result: ExtensionTarget<D>,
}

impl<F: RichField + Extendable<D>, const D: usize> SimpleGenerator<F, D>
    for ExpGeneratorExtension<D>
{
    fn id(&self) -> String {
        "ExpGeneratorExtension".to_string()
    }

    fn dependencies(&self) -> Vec<Target> {
        let deps = self.base.to_target_array().to_vec();
        deps
    }

    fn run_once(
        &self,
        witness: &plonky2::iop::witness::PartitionWitness<F>,
        out_buffer: &mut plonky2::iop::generator::GeneratedValues<F>,
    ) {
        let base = witness.get_extension_target(self.base);
        let mut current_base = base.clone();
        let mut exp = <F as Extendable<D>>::Extension::from(F::ONE);
        let mut power = ALPHA_INV;
        while power > 0 {
            if power % 2 == 1 {
                exp = exp * current_base;
            }
            current_base = current_base * current_base;
            power /= 2;
        }
        out_buffer.set_extension_target(self.exp_result, exp)
    }

    fn serialize(&self, dst: &mut Vec<u8>, _common_data: &plonky2::plonk::circuit_data::CommonCircuitData<F, D>) -> plonky2::util::serialization::IoResult<()> {
        dst.write_target_ext(self.base)?;
        dst.write_target_ext(self.exp_result)
    }

    fn deserialize(src: &mut plonky2::util::serialization::Buffer, _common_data: &plonky2::plonk::circuit_data::CommonCircuitData<F, D>) -> plonky2::util::serialization::IoResult<Self> {
        let base = src.read_target_ext()?;
        let exp = src.read_target_ext()?;
        core::result::Result::Ok(Self {
            base,
            exp_result: exp,
        })
    }     
}
