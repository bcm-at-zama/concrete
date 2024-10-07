#![allow(clippy::boxed_local)]
#![allow(clippy::too_many_arguments)]

use core::panic;

use concrete_optimizer::computing_cost::cpu::CpuComplexity;
use concrete_optimizer::config;
use concrete_optimizer::config::ProcessingUnit;
use concrete_optimizer::dag::operator::{
    self, FunctionTable, LevelledComplexity, OperatorIndex, Precision, Shape,
};
use concrete_optimizer::dag::unparametrized;
use concrete_optimizer::global_parameters::{DEFAULT_DOMAINS, ParameterDomains};
use concrete_optimizer::optimization::config::{Config, SearchSpace};
use concrete_optimizer::optimization::dag::multi_parameters::keys_spec;
use concrete_optimizer::optimization::dag::multi_parameters::keys_spec::CircuitSolution;
use concrete_optimizer::optimization::dag::multi_parameters::optimize::MacroParameters;
use concrete_optimizer::optimization::dag::multi_parameters::partition_cut::PartitionCut;
use concrete_optimizer::optimization::dag::solo_key::optimize_generic::{
    Encoding, Solution as DagSolution,
};
use concrete_optimizer::optimization::decomposition;
use concrete_optimizer::optimization::decomposition::cmux::MaxVarianceError;
use concrete_optimizer::parameters::{
    BrDecompositionParameters, GlweParameters, KsDecompositionParameters,
};
use concrete_optimizer::utils::cache::persistent::default_cache_dir;
use concrete_optimizer::utils::viz::Viz;
use cxx::CxxString;

fn no_solution() -> ffi::Solution {
    ffi::Solution {
        p_error: 1.0, // error probability to signal an impossible solution
        ..ffi::Solution::default()
    }
}

fn no_dag_solution() -> ffi::DagSolution {
    ffi::DagSolution {
        p_error: 1.0, // error probability to signal an impossible solution
        ..ffi::DagSolution::default()
    }
}

fn caches_from(options: &ffi::Options) -> decomposition::PersistDecompCaches {
    if !options.cache_on_disk {
        println!("optimizer: Using stateless cache.");
        let cache_dir = default_cache_dir();
        println!("optimizer: To clear the cache, remove directory {cache_dir}");
    }
    let processing_unit = processing_unit(&options);
    decomposition::cache(
        options.security_level,
        processing_unit,
        Some(ProcessingUnit::Cpu.complexity_model()),
        options.cache_on_disk,
        options.ciphertext_modulus_log,
        options.fft_precision,
    )
}

fn calculate_parameter_domain(options: &ffi::Options) -> ParameterDomains {
    let mut domains = DEFAULT_DOMAINS.clone();

    if !options.parameter_restrictions.glwe_pbs.log2_polynomial_size_min.is_null() {
        domains.glwe_pbs_constrained_cpu.log2_polynomial_size.start = *options.parameter_restrictions.glwe_pbs.log2_polynomial_size_min;
        domains.glwe_pbs_constrained_gpu.log2_polynomial_size.start = *options.parameter_restrictions.glwe_pbs.log2_polynomial_size_min;
    }
    if !options.parameter_restrictions.glwe_pbs.log2_polynomial_size_max.is_null() {
        domains.glwe_pbs_constrained_cpu.log2_polynomial_size.end = *options.parameter_restrictions.glwe_pbs.log2_polynomial_size_max;
        domains.glwe_pbs_constrained_gpu.log2_polynomial_size.end = *options.parameter_restrictions.glwe_pbs.log2_polynomial_size_max;
    }
    if !options.parameter_restrictions.glwe_pbs.glwe_dimension_min.is_null() {
        domains.glwe_pbs_constrained_cpu.glwe_dimension.start = *options.parameter_restrictions.glwe_pbs.glwe_dimension_min;
        domains.glwe_pbs_constrained_gpu.glwe_dimension.start = *options.parameter_restrictions.glwe_pbs.glwe_dimension_min;
    }
    if !options.parameter_restrictions.glwe_pbs.glwe_dimension_max.is_null() {
        domains.glwe_pbs_constrained_cpu.glwe_dimension.end = *options.parameter_restrictions.glwe_pbs.glwe_dimension_max;
        domains.glwe_pbs_constrained_gpu.glwe_dimension.end = *options.parameter_restrictions.glwe_pbs.glwe_dimension_max;
    }
    
    if !options.parameter_restrictions.free_glwe.log2_polynomial_size_min.is_null() {
        domains.free_glwe.log2_polynomial_size.start = *options.parameter_restrictions.free_glwe.log2_polynomial_size_min;
    }
    if !options.parameter_restrictions.free_glwe.log2_polynomial_size_max.is_null() {
        domains.free_glwe.log2_polynomial_size.end = *options.parameter_restrictions.free_glwe.log2_polynomial_size_max;
    }
    if !options.parameter_restrictions.free_glwe.glwe_dimension_min.is_null() {
        domains.free_glwe.glwe_dimension.start = *options.parameter_restrictions.free_glwe.glwe_dimension_min;
    }
    if !options.parameter_restrictions.free_glwe.glwe_dimension_max.is_null() {
        domains.free_glwe.glwe_dimension.end = *options.parameter_restrictions.free_glwe.glwe_dimension_max;
    }
    
    if !options.parameter_restrictions.br_decomposition.log2_base_min.is_null() {
        domains.br_decomposition.log2_base.start = *options.parameter_restrictions.br_decomposition.log2_base_min;
    }
    if !options.parameter_restrictions.br_decomposition.log2_base_max.is_null() {
        domains.br_decomposition.log2_base.end = *options.parameter_restrictions.br_decomposition.log2_base_max;
    }
    if !options.parameter_restrictions.br_decomposition.level_min.is_null() {
        domains.br_decomposition.level.start = *options.parameter_restrictions.br_decomposition.level_min;
    }
    if !options.parameter_restrictions.br_decomposition.level_max.is_null() {
        domains.br_decomposition.level.end = *options.parameter_restrictions.br_decomposition.level_max;
    }
    
    if !options.parameter_restrictions.ks_decomposition.log2_base_min.is_null() {
        domains.ks_decomposition.log2_base.start = *options.parameter_restrictions.ks_decomposition.log2_base_min;
    }
    if !options.parameter_restrictions.ks_decomposition.log2_base_max.is_null() {
        domains.ks_decomposition.log2_base.end = *options.parameter_restrictions.ks_decomposition.log2_base_max;
    }
    if !options.parameter_restrictions.ks_decomposition.level_min.is_null() {
        domains.ks_decomposition.level.start = *options.parameter_restrictions.ks_decomposition.level_min;
    }
    if !options.parameter_restrictions.ks_decomposition.level_max.is_null() {
        domains.ks_decomposition.level.end = *options.parameter_restrictions.ks_decomposition.level_max;
    }
    
    if !options.parameter_restrictions.free_lwe_min.is_null() {
        domains.free_lwe.start = *options.parameter_restrictions.free_lwe_min;
    }
    if !options.parameter_restrictions.free_lwe_max.is_null() {
        domains.free_lwe.end = *options.parameter_restrictions.free_lwe_max;
    }

    domains
}

#[derive(Clone)]
pub struct ExternalPartition(
    concrete_optimizer::optimization::dag::multi_parameters::partition_cut::ExternalPartition,
);

pub fn get_external_partition(
    name: String,
    log2_polynomial_size: u64,
    glwe_dimension: u64,
    internal_dim: u64,
    max_variance: f64,
    variance: f64,
) -> Box<ExternalPartition> {
    Box::new(ExternalPartition(
        concrete_optimizer::optimization::dag::multi_parameters::partition_cut::ExternalPartition {
            name,
            macro_params: MacroParameters {
                glwe_params: GlweParameters {
                    log2_polynomial_size,
                    glwe_dimension,
                },
                internal_dim,
            },
            max_variance,
            variance,
        },
    ))
}

pub fn get_noise_br(
    options: &ffi::Options,
    log2_polynomial_size: u64,
    glwe_dimension: u64,
    lwe_dim: u64,
    pbs_level: u64,
    pbs_log2_base: u64,
) -> f64 {
    let cache = caches_from(options).caches();
    match decomposition::cmux::get_noise_br(
        cache,
        log2_polynomial_size,
        glwe_dimension,
        lwe_dim,
        pbs_level,
        Some(pbs_log2_base),
    ) {
        Ok(max_variance) => max_variance,
        Err(err) => match err {
            MaxVarianceError::PbsBaseLogNotFound => panic!("pbs base log didn't match"),
            MaxVarianceError::PbsLevelNotFound => panic!("pbs level not found"),
        },
    }
}

fn optimize_bootstrap(precision: u64, noise_factor: f64, options: &ffi::Options) -> ffi::Solution {
    // Support composable since there is no dag
    let processing_unit = processing_unit(&options);

    let config = Config {
        security_level: options.security_level,
        maximum_acceptable_error_probability: options.maximum_acceptable_error_probability,
        key_sharing: options.key_sharing,
        ciphertext_modulus_log: options.ciphertext_modulus_log,
        fft_precision: options.fft_precision,
        complexity_model: &CpuComplexity::default(),
    };

    let sum_size = 1;

    let parameter_restrictions = calculate_parameter_domain(&options);
    let search_space = SearchSpace::default(processing_unit, parameter_restrictions);

    let result = concrete_optimizer::optimization::atomic_pattern::optimize_one(
        sum_size,
        precision,
        config,
        noise_factor,
        &search_space,
        &caches_from(options),
    );
    result
        .best_solution
        .map_or_else(no_solution, |solution| solution.into())
}

fn convert_to_dag_solution(sol: &ffi::Solution) -> ffi::DagSolution {
    sol.into()
}

impl From<&ffi::Solution> for ffi::DagSolution {
    fn from(sol: &ffi::Solution) -> Self {
        Self {
            input_lwe_dimension: sol.input_lwe_dimension,
            internal_ks_output_lwe_dimension: sol.internal_ks_output_lwe_dimension,
            ks_decomposition_level_count: sol.ks_decomposition_level_count,
            ks_decomposition_base_log: sol.ks_decomposition_base_log,
            glwe_polynomial_size: sol.glwe_polynomial_size,
            glwe_dimension: sol.glwe_dimension,
            br_decomposition_level_count: sol.br_decomposition_level_count,
            br_decomposition_base_log: sol.br_decomposition_base_log,
            complexity: sol.complexity,
            noise_max: sol.noise_max,
            p_error: sol.p_error,
            global_p_error: f64::NAN,
            use_wop_pbs: false,
            cb_decomposition_level_count: 0,
            cb_decomposition_base_log: 0,
            pp_decomposition_level_count: 0,
            pp_decomposition_base_log: 0,
            crt_decomposition: vec![],
        }
    }
}

impl From<&ffi::CircuitSolution> for ffi::DagSolution {
    fn from(sol: &ffi::CircuitSolution) -> Self {
        assert!(sol.circuit_keys.secret_keys.len() == 2);
        let keys = &sol.circuit_keys;
        let big_key = &keys.secret_keys[0];
        let small_key = &keys.secret_keys[1];
        let input_lwe_dimension = big_key.polynomial_size * big_key.glwe_dimension;
        let internal_ks_output_lwe_dimension = small_key.polynomial_size * small_key.glwe_dimension;
        let keyswitch_key = &keys.keyswitch_keys[0];
        let bootstrap_key = &keys.bootstrap_keys[0];
        let mut cb_decomposition_level_count = 0;
        let mut cb_decomposition_base_log = 0;
        let mut pp_decomposition_level_count = 0;
        let mut pp_decomposition_base_log = 0;
        let use_wop_pbs = !sol.circuit_keys.circuit_bootstrap_keys.is_empty();
        if use_wop_pbs {
            assert!(sol.circuit_keys.circuit_bootstrap_keys.len() == 1);
            assert!(sol.circuit_keys.private_functional_packing_keys.len() == 1);
            let cb_decomp = &keys.circuit_bootstrap_keys[0].br_decomposition_parameter;
            cb_decomposition_level_count = cb_decomp.level;
            cb_decomposition_base_log = cb_decomp.log2_base;
            let pp_switch_decomp =
                &keys.private_functional_packing_keys[0].br_decomposition_parameter;
            pp_decomposition_level_count = pp_switch_decomp.level;
            pp_decomposition_base_log = pp_switch_decomp.log2_base;
        }
        Self {
            input_lwe_dimension,
            internal_ks_output_lwe_dimension,
            ks_decomposition_level_count: keyswitch_key.ks_decomposition_parameter.level,
            ks_decomposition_base_log: keyswitch_key.ks_decomposition_parameter.log2_base,
            glwe_polynomial_size: big_key.polynomial_size,
            glwe_dimension: big_key.glwe_dimension,
            br_decomposition_level_count: bootstrap_key.br_decomposition_parameter.level,
            br_decomposition_base_log: bootstrap_key.br_decomposition_parameter.log2_base,
            complexity: sol.complexity,
            noise_max: f64::NAN,
            p_error: sol.p_error,
            global_p_error: sol.global_p_error,
            use_wop_pbs,
            cb_decomposition_level_count,
            cb_decomposition_base_log,
            pp_decomposition_level_count,
            pp_decomposition_base_log,
            crt_decomposition: sol.crt_decomposition.clone(),
        }
    }
}

impl From<concrete_optimizer::optimization::atomic_pattern::Solution> for ffi::Solution {
    fn from(a: concrete_optimizer::optimization::atomic_pattern::Solution) -> Self {
        Self {
            input_lwe_dimension: a.input_lwe_dimension,
            internal_ks_output_lwe_dimension: a.internal_ks_output_lwe_dimension,
            ks_decomposition_level_count: a.ks_decomposition_level_count,
            ks_decomposition_base_log: a.ks_decomposition_base_log,
            glwe_polynomial_size: a.glwe_polynomial_size,
            glwe_dimension: a.glwe_dimension,
            br_decomposition_level_count: a.br_decomposition_level_count,
            br_decomposition_base_log: a.br_decomposition_base_log,
            complexity: a.complexity,
            noise_max: a.noise_max,
            p_error: a.p_error,
        }
    }
}

impl From<DagSolution> for ffi::DagSolution {
    fn from(sol: DagSolution) -> Self {
        match sol {
            DagSolution::WpSolution(sol) => Self {
                input_lwe_dimension: sol.input_lwe_dimension,
                internal_ks_output_lwe_dimension: sol.internal_ks_output_lwe_dimension,
                ks_decomposition_level_count: sol.ks_decomposition_level_count,
                ks_decomposition_base_log: sol.ks_decomposition_base_log,
                glwe_polynomial_size: sol.glwe_polynomial_size,
                glwe_dimension: sol.glwe_dimension,
                br_decomposition_level_count: sol.br_decomposition_level_count,
                br_decomposition_base_log: sol.br_decomposition_base_log,
                complexity: sol.complexity,
                noise_max: sol.noise_max,
                p_error: sol.p_error,
                global_p_error: sol.global_p_error,
                use_wop_pbs: false,
                cb_decomposition_level_count: 0,
                cb_decomposition_base_log: 0,
                pp_decomposition_level_count: 0,
                pp_decomposition_base_log: 0,
                crt_decomposition: vec![],
            },
            DagSolution::WopSolution(sol) => Self {
                input_lwe_dimension: sol.input_lwe_dimension,
                internal_ks_output_lwe_dimension: sol.internal_ks_output_lwe_dimension,
                ks_decomposition_level_count: sol.ks_decomposition_level_count,
                ks_decomposition_base_log: sol.ks_decomposition_base_log,
                glwe_polynomial_size: sol.glwe_polynomial_size,
                glwe_dimension: sol.glwe_dimension,
                br_decomposition_level_count: sol.br_decomposition_level_count,
                br_decomposition_base_log: sol.br_decomposition_base_log,
                complexity: sol.complexity,
                noise_max: sol.noise_max,
                p_error: sol.p_error,
                global_p_error: sol.global_p_error,
                use_wop_pbs: true,
                cb_decomposition_level_count: sol.cb_decomposition_level_count,
                cb_decomposition_base_log: sol.cb_decomposition_base_log,
                pp_decomposition_level_count: sol.pp_decomposition_level_count,
                pp_decomposition_base_log: sol.pp_decomposition_base_log,
                crt_decomposition: sol.crt_decomposition,
            },
        }
    }
}

fn convert_to_circuit_solution(sol: &ffi::DagSolution, dag: &Dag) -> ffi::CircuitSolution {
    let big_key = ffi::SecretLweKey {
        identifier: 0,
        polynomial_size: sol.glwe_polynomial_size,
        glwe_dimension: sol.glwe_dimension,
        description: "big representation".into(),
    };
    let small_key = ffi::SecretLweKey {
        identifier: 1,
        polynomial_size: sol.internal_ks_output_lwe_dimension,
        glwe_dimension: 1,
        description: "small representation".into(),
    };
    let keyswitch_key = ffi::KeySwitchKey {
        identifier: 0,
        input_key: big_key.clone(),
        output_key: small_key.clone(),
        ks_decomposition_parameter: ffi::KsDecompositionParameters {
            level: sol.ks_decomposition_level_count,
            log2_base: sol.ks_decomposition_base_log,
        },
        description: "tlu keyswitch".into(),
    };
    let bootstrap_key = ffi::BootstrapKey {
        identifier: 0,
        input_key: small_key.clone(),
        output_key: big_key.clone(),
        br_decomposition_parameter: ffi::BrDecompositionParameters {
            level: sol.br_decomposition_level_count,
            log2_base: sol.br_decomposition_base_log,
        },
        description: "tlu bootstrap".into(),
    };
    let circuit_bootstrap_keys = if sol.use_wop_pbs {
        vec![ffi::CircuitBoostrapKey {
            identifier: 0,
            representation_key: big_key.clone(),
            br_decomposition_parameter: ffi::BrDecompositionParameters {
                level: sol.cb_decomposition_level_count,
                log2_base: sol.cb_decomposition_base_log,
            },
            description: "circuit bootstrap for woppbs".into(),
        }]
    } else {
        vec![]
    };
    let private_functional_packing_keys = if sol.use_wop_pbs {
        vec![ffi::PrivateFunctionalPackingBoostrapKey {
            identifier: 0,
            representation_key: big_key.clone(),
            br_decomposition_parameter: ffi::BrDecompositionParameters {
                level: sol.pp_decomposition_level_count,
                log2_base: sol.pp_decomposition_base_log,
            },
            description: "private functional packing for woppbs".into(),
        }]
    } else {
        vec![]
    };
    let instruction_keys = ffi::InstructionKeys {
        input_key: big_key.identifier,
        tlu_keyswitch_key: keyswitch_key.identifier,
        tlu_bootstrap_key: bootstrap_key.identifier,
        tlu_circuit_bootstrap_key: circuit_bootstrap_keys
            .last()
            .map_or(keys_spec::NO_KEY_ID, |v| v.identifier),
        tlu_private_functional_packing_key: private_functional_packing_keys
            .last()
            .map_or(keys_spec::NO_KEY_ID, |v| v.identifier),
        output_key: big_key.identifier,
        extra_conversion_keys: vec![],
    };
    let instructions_keys = vec![instruction_keys; dag.0.len()];
    let circuit_keys = ffi::CircuitKeys {
        secret_keys: [big_key, small_key].into(),
        keyswitch_keys: [keyswitch_key].into(),
        bootstrap_keys: [bootstrap_key].into(),
        conversion_keyswitch_keys: [].into(),
        circuit_bootstrap_keys,
        private_functional_packing_keys,
    };
    let is_feasible = sol.p_error < 1.0;
    let error_msg = if is_feasible {
        ""
    } else {
        "No crypto-parameters for the given constraints"
    }
    .into();
    ffi::CircuitSolution {
        circuit_keys,
        instructions_keys,
        crt_decomposition: sol.crt_decomposition.clone(),
        complexity: sol.complexity,
        p_error: sol.p_error,
        global_p_error: sol.global_p_error,
        is_feasible,
        error_msg,
    }
}

impl From<CircuitSolution> for ffi::CircuitSolution {
    fn from(v: CircuitSolution) -> Self {
        Self {
            circuit_keys: v.circuit_keys.into(),
            instructions_keys: vec_into(v.instructions_keys),
            crt_decomposition: v.crt_decomposition,
            complexity: v.complexity,
            p_error: v.p_error,
            global_p_error: v.global_p_error,
            is_feasible: v.is_feasible,
            error_msg: v.error_msg,
        }
    }
}

impl ffi::CircuitSolution {
    fn short_dump(&self) -> String {
        let mut new = self.clone();
        new.instructions_keys = vec![];
        new.dump()
    }
    fn dump(&self) -> String {
        format!("{self:#?}")
    }
}

impl From<KsDecompositionParameters> for ffi::KsDecompositionParameters {
    fn from(v: KsDecompositionParameters) -> Self {
        Self {
            level: v.level,
            log2_base: v.log2_base,
        }
    }
}

impl From<BrDecompositionParameters> for ffi::BrDecompositionParameters {
    fn from(v: BrDecompositionParameters) -> Self {
        Self {
            level: v.level,
            log2_base: v.log2_base,
        }
    }
}

impl From<keys_spec::SecretLweKey> for ffi::SecretLweKey {
    fn from(v: keys_spec::SecretLweKey) -> Self {
        Self {
            identifier: v.identifier,
            polynomial_size: v.polynomial_size,
            glwe_dimension: v.glwe_dimension,
            description: v.description,
        }
    }
}

impl From<keys_spec::KeySwitchKey> for ffi::KeySwitchKey {
    fn from(v: keys_spec::KeySwitchKey) -> Self {
        Self {
            identifier: v.identifier,
            input_key: v.input_key.into(),
            output_key: v.output_key.into(),
            ks_decomposition_parameter: v.ks_decomposition_parameter.into(),
            description: v.description,
        }
    }
}

impl From<keys_spec::ConversionKeySwitchKey> for ffi::ConversionKeySwitchKey {
    fn from(v: keys_spec::ConversionKeySwitchKey) -> Self {
        Self {
            identifier: v.identifier,
            input_key: v.input_key.into(),
            output_key: v.output_key.into(),
            ks_decomposition_parameter: v.ks_decomposition_parameter.into(),
            description: v.description,
            fast_keyswitch: v.fast_keyswitch,
        }
    }
}

impl From<keys_spec::BootstrapKey> for ffi::BootstrapKey {
    fn from(v: keys_spec::BootstrapKey) -> Self {
        Self {
            identifier: v.identifier,
            input_key: v.input_key.into(),
            output_key: v.output_key.into(),
            br_decomposition_parameter: v.br_decomposition_parameter.into(),
            description: v.description,
        }
    }
}

impl From<keys_spec::CircuitBoostrapKey> for ffi::CircuitBoostrapKey {
    fn from(v: keys_spec::CircuitBoostrapKey) -> Self {
        Self {
            identifier: v.identifier,
            representation_key: v.representation_key.into(),
            br_decomposition_parameter: v.br_decomposition_parameter.into(),
            description: v.description,
        }
    }
}

impl From<keys_spec::PrivateFunctionalPackingBoostrapKey>
    for ffi::PrivateFunctionalPackingBoostrapKey
{
    fn from(v: keys_spec::PrivateFunctionalPackingBoostrapKey) -> Self {
        Self {
            identifier: v.identifier,
            representation_key: v.representation_key.into(),
            br_decomposition_parameter: v.br_decomposition_parameter.into(),
            description: v.description,
        }
    }
}

impl From<keys_spec::InstructionKeys> for ffi::InstructionKeys {
    fn from(v: keys_spec::InstructionKeys) -> Self {
        Self {
            input_key: v.input_key,
            tlu_keyswitch_key: v.tlu_keyswitch_key,
            tlu_bootstrap_key: v.tlu_bootstrap_key,
            tlu_circuit_bootstrap_key: v.tlu_circuit_bootstrap_key,
            tlu_private_functional_packing_key: v.tlu_private_functional_packing_key,
            output_key: v.output_key,
            extra_conversion_keys: v.extra_conversion_keys,
        }
    }
}

fn vec_into<F, T: std::convert::From<F>>(vec: Vec<F>) -> Vec<T> {
    vec.into_iter().map(|x| x.into()).collect()
}

impl From<keys_spec::CircuitKeys> for ffi::CircuitKeys {
    fn from(v: keys_spec::CircuitKeys) -> Self {
        Self {
            secret_keys: vec_into(v.secret_keys),
            keyswitch_keys: vec_into(v.keyswitch_keys),
            bootstrap_keys: vec_into(v.bootstrap_keys),
            circuit_bootstrap_keys: vec_into(v.circuit_bootstrap_keys),
            private_functional_packing_keys: vec_into(v.private_functional_packing_keys),
            conversion_keyswitch_keys: vec_into(v.conversion_keyswitch_keys),
        }
    }
}

#[allow(non_snake_case)]
fn NO_KEY_ID() -> u64 {
    keys_spec::NO_KEY_ID
}

pub struct Dag(unparametrized::Dag);

fn empty() -> Box<Dag> {
    Box::new(Dag(unparametrized::Dag::new()))
}

impl Dag {
    fn builder(&mut self, circuit: String) -> Box<DagBuilder<'_>> {
        Box::new(DagBuilder(self.0.builder(circuit)))
    }

    fn dump(&self) -> String {
        self.0.viz_string()
    }

    fn get_input_indices(&self) -> Vec<ffi::OperatorIndex> {
        self.0
            .get_input_operators_iter()
            .map(|n| ffi::OperatorIndex { index: n.id.0 })
            .collect()
    }

    fn get_output_indices(&self) -> Vec<ffi::OperatorIndex> {
        self.0
            .get_output_operators_iter()
            .map(|n| ffi::OperatorIndex { index: n.id.0 })
            .collect()
    }

    fn optimize(&self, options: &ffi::Options) -> ffi::DagSolution {
        let processing_unit = processing_unit(&options);
        let config = Config {
            security_level: options.security_level,
            maximum_acceptable_error_probability: options.maximum_acceptable_error_probability,
            key_sharing: options.key_sharing,
            ciphertext_modulus_log: options.ciphertext_modulus_log,
            fft_precision: options.fft_precision,
            complexity_model: &CpuComplexity::default(),
        };

        let parameter_restrictions = calculate_parameter_domain(&options);
        let search_space = SearchSpace::default(processing_unit, parameter_restrictions);

        let encoding = options.encoding.into();

        if self.0.is_composed() {
            return no_dag_solution();
        }

        let result = concrete_optimizer::optimization::dag::solo_key::optimize_generic::optimize(
            &self.0,
            config,
            &search_space,
            encoding,
            options.default_log_norm2_woppbs,
            &caches_from(options),
        );
        result.map_or_else(no_dag_solution, |solution| solution.into())
    }

    fn get_circuit_count(&self) -> usize {
        self.0.get_circuit_count()
    }

    unsafe fn add_composition<'a>(
        &mut self,
        from_func: &'a CxxString,
        from_pos: usize,
        to_func: &'a CxxString,
        to_pos: usize,
    ) {
        let from_index = self
            .0
            .get_circuit(from_func.to_str().unwrap())
            .get_output_operators_iter()
            .nth(from_pos)
            .unwrap()
            .id;
        let to_index = self
            .0
            .get_circuit(to_func.to_str().unwrap())
            .get_input_operators_iter()
            .nth(to_pos)
            .unwrap()
            .id;
        self.0.add_composition(from_index, to_index);
    }

    fn add_all_compositions(&mut self) {
        let froms = self
            .0
            .get_output_operators_iter()
            .map(|o| o.id)
            .collect::<Vec<_>>();
        let tos = self
            .0
            .get_input_operators_iter()
            .map(|o| o.id)
            .collect::<Vec<_>>();
        self.0.add_compositions(froms, tos);
    }

    fn optimize_multi(&self, options: &ffi::Options) -> ffi::CircuitSolution {
        let processing_unit = processing_unit(&options);
        let config = Config {
            security_level: options.security_level,
            maximum_acceptable_error_probability: options.maximum_acceptable_error_probability,
            key_sharing: options.key_sharing,
            ciphertext_modulus_log: options.ciphertext_modulus_log,
            fft_precision: options.fft_precision,
            complexity_model: &CpuComplexity::default(),
        };
        let parameter_restrictions = calculate_parameter_domain(&options);
        let search_space = SearchSpace::default(processing_unit, parameter_restrictions);

        let encoding = options.encoding.into();
        #[allow(clippy::wildcard_in_or_patterns)]
        let p_cut = match options.multi_param_strategy {
            ffi::MultiParamStrategy::ByPrecisionAndNorm2 => {
                PartitionCut::maximal_partitionning(&self.0)
            }
            ffi::MultiParamStrategy::ByPrecision | _ => PartitionCut::for_each_precision(&self.0),
        };
        let circuit_sol =
            concrete_optimizer::optimization::dag::multi_parameters::optimize_generic::optimize(
                &self.0,
                config,
                &search_space,
                encoding,
                options.default_log_norm2_woppbs,
                &caches_from(options),
                &Some(p_cut),
            );
        circuit_sol.into()
    }
}

pub struct DagBuilder<'dag>(unparametrized::DagBuilder<'dag>);

impl<'dag> DagBuilder<'dag> {
    fn add_input(
        &mut self,
        out_precision: Precision,
        out_shape: &[u64],
        location: &Location,
    ) -> ffi::OperatorIndex {
        let out_shape = Shape {
            dimensions_size: out_shape.to_owned(),
        };

        self.0
            .add_input(out_precision, out_shape, location.0.clone())
            .into()
    }

    fn add_zero_noise(
        &mut self,
        out_precision: Precision,
        out_shape: &[u64],
        location: &Location,
    ) -> ffi::OperatorIndex {
        let out_shape = Shape {
            dimensions_size: out_shape.to_owned(),
        };

        self.0
            .add_zero_noise(out_precision, out_shape, location.0.clone())
            .into()
    }

    fn add_lut(
        &mut self,
        input: ffi::OperatorIndex,
        table: &[u64],
        out_precision: Precision,
        location: &Location,
    ) -> ffi::OperatorIndex {
        let table = FunctionTable {
            values: table.to_owned(),
        };

        self.0
            .add_lut(input.into(), table, out_precision, location.0.clone())
            .into()
    }

    #[allow(clippy::boxed_local)]
    fn add_dot(
        &mut self,
        inputs: &[ffi::OperatorIndex],
        weights: Box<Weights>,
        location: &Location,
    ) -> ffi::OperatorIndex {
        let inputs: Vec<OperatorIndex> = inputs.iter().copied().map(Into::into).collect();

        self.0.add_dot(inputs, weights.0, location.0.clone()).into()
    }

    fn add_linear_noise(
        &mut self,
        inputs: &[ffi::OperatorIndex],
        lwe_dim_cost_factor: f64,
        fixed_cost: f64,
        weights: &[f64],
        out_shape: &[u64],
        comment: &str,
        location: &Location,
    ) -> ffi::OperatorIndex {
        debug_assert!(weights.len() == inputs.len());
        let inputs: Vec<OperatorIndex> = inputs.iter().copied().map(Into::into).collect();

        let out_shape = Shape {
            dimensions_size: out_shape.to_owned(),
        };

        let complexity = LevelledComplexity {
            lwe_dim_cost_factor,
            fixed_cost,
        };

        self.0
            .add_linear_noise(
                inputs,
                complexity,
                weights,
                out_shape,
                comment,
                location.0.clone(),
            )
            .into()
    }

    fn add_max_noise(
        &mut self,
        inputs: &[ffi::OperatorIndex],
        out_shape: &[u64],
        location: &Location,
    ) -> ffi::OperatorIndex {
        let inputs: Vec<OperatorIndex> = inputs.iter().copied().map(Into::into).collect();

        let out_shape = Shape {
            dimensions_size: out_shape.to_owned(),
        };

        self.0
            .add_max_noise(inputs, out_shape, location.0.clone())
            .into()
    }

    fn add_round_op(
        &mut self,
        input: ffi::OperatorIndex,
        rounded_precision: Precision,
        location: &Location,
    ) -> ffi::OperatorIndex {
        self.0
            .add_round_op(input.into(), rounded_precision, location.0.clone())
            .into()
    }

    fn add_unsafe_cast_op(
        &mut self,
        input: ffi::OperatorIndex,
        new_precision: Precision,
        location: &Location,
    ) -> ffi::OperatorIndex {
        self.0
            .add_unsafe_cast(input.into(), new_precision, location.0.clone())
            .into()
    }

    fn add_change_partition_with_src(
        &mut self,
        input: ffi::OperatorIndex,
        src_partition: &ExternalPartition,
        location: &Location,
    ) -> ffi::OperatorIndex {
        self.0
            .add_change_partition(
                input.into(),
                Some(src_partition.0.clone()),
                None,
                location.0.clone(),
            )
            .into()
    }

    fn add_change_partition_with_dst(
        &mut self,
        input: ffi::OperatorIndex,
        dst_partition: &ExternalPartition,
        location: &Location,
    ) -> ffi::OperatorIndex {
        self.0
            .add_change_partition(
                input.into(),
                None,
                Some(dst_partition.0.clone()),
                location.0.clone(),
            )
            .into()
    }

    fn tag_operator_as_output(&mut self, op: ffi::OperatorIndex) {
        self.0.tag_operator_as_output(op.into());
    }

    fn dump(&self) -> String {
        format!("{}", self.0.get_circuit())
    }
}

#[derive(Clone)]
pub struct Location(operator::Location);

fn location_unknown() -> Box<Location> {
    Box::new(Location(operator::Location::Unknown))
}

fn location_from_string(string: &str) -> Box<Location> {
    let location: Vec<&str> = string.split(':').collect();
    match location[..] {
        [file] => Box::new(Location(operator::Location::File(file.into()))),
        [file, line] => Box::new(Location(operator::Location::Line(
            file.into(),
            line.parse().unwrap(),
        ))),
        [file, line, column] => Box::new(Location(operator::Location::LineColumn(
            file.into(),
            line.parse().unwrap(),
            column.parse().unwrap(),
        ))),
        _ => Box::new(Location(operator::Location::Unknown)),
    }
}

pub struct Weights(operator::Weights);

fn vector(weights: &[i64]) -> Box<Weights> {
    Box::new(Weights(operator::Weights::vector(weights)))
}

fn number(weight: i64) -> Box<Weights> {
    Box::new(Weights(operator::Weights::number(weight)))
}

impl From<OperatorIndex> for ffi::OperatorIndex {
    fn from(oi: OperatorIndex) -> Self {
        Self { index: oi.0 }
    }
}

#[allow(clippy::from_over_into)]
impl Into<OperatorIndex> for ffi::OperatorIndex {
    fn into(self) -> OperatorIndex {
        OperatorIndex(self.index)
    }
}

#[allow(clippy::from_over_into)]
impl Into<Encoding> for ffi::Encoding {
    fn into(self) -> Encoding {
        match self {
            Self::Auto => Encoding::Auto,
            Self::Native => Encoding::Native,
            Self::Crt => Encoding::Crt,
            _ => unreachable!("Internal error: Invalid encoding"),
        }
    }
}

#[allow(
    unused_must_use,
    clippy::needless_lifetimes,
    clippy::needless_maybe_sized
)]
#[cxx::bridge]
mod ffi {
    #[namespace = "concrete_optimizer"]
    extern "Rust" {

        #[namespace = "concrete_optimizer::v0"]
        fn optimize_bootstrap(precision: u64, noise_factor: f64, options: &Options) -> Solution;

        #[namespace = "concrete_optimizer::utils"]
        fn convert_to_dag_solution(solution: &Solution) -> DagSolution;

        #[namespace = "concrete_optimizer::utils"]
        fn convert_to_circuit_solution(solution: &DagSolution, dag: &Dag) -> CircuitSolution;

        type Dag;

        type DagBuilder<'dag>;

        type Location;

        type ExternalPartition;

        #[namespace = "concrete_optimizer::utils"]
        fn location_unknown() -> Box<Location>;

        #[namespace = "concrete_optimizer::utils"]
        fn location_from_string(string: &str) -> Box<Location>;

        #[namespace = "concrete_optimizer::utils"]
        fn get_external_partition(
            name: String,
            log2_polynomial_size: u64,
            glwe_dimension: u64,
            internal_dim: u64,
            max_variance: f64,
            variance: f64,
        ) -> Box<ExternalPartition>;

        #[namespace = "concrete_optimizer::utils"]
        fn get_noise_br(
            options: &Options,
            log2_polynomial_size: u64,
            glwe_dimension: u64,
            lwe_dim: u64,
            pbs_level: u64,
            pbs_log2_base: u64,
        ) -> f64;

        #[namespace = "concrete_optimizer::dag"]
        fn empty() -> Box<Dag>;

        unsafe fn builder(self: &mut Dag, circuit: String) -> Box<DagBuilder<'_>>;

        fn dump(self: &Dag) -> String;

        fn dump(self: &DagBuilder) -> String;

        unsafe fn add_input(
            self: &mut DagBuilder<'_>,
            out_precision: u8,
            out_shape: &[u64],
            location: &Location,
        ) -> OperatorIndex;

        unsafe fn add_zero_noise(
            self: &mut DagBuilder<'_>,
            out_precision: u8,
            out_shape: &[u64],
            location: &Location,
        ) -> OperatorIndex;

        unsafe fn add_lut(
            self: &mut DagBuilder<'_>,
            input: OperatorIndex,
            table: &[u64],
            out_precision: u8,
            location: &Location,
        ) -> OperatorIndex;

        unsafe fn add_dot(
            self: &mut DagBuilder<'_>,
            inputs: &[OperatorIndex],
            weights: Box<Weights>,
            location: &Location,
        ) -> OperatorIndex;

        unsafe fn add_linear_noise(
            self: &mut DagBuilder<'_>,
            inputs: &[OperatorIndex],
            lwe_dim_cost_factor: f64,
            fixed_cost: f64,
            weights: &[f64],
            out_shape: &[u64],
            comment: &str,
            location: &Location,
        ) -> OperatorIndex;

        unsafe fn add_max_noise(
            self: &mut DagBuilder<'_>,
            inputs: &[OperatorIndex],
            out_shape: &[u64],
            location: &Location,
        ) -> OperatorIndex;

        unsafe fn add_round_op(
            self: &mut DagBuilder<'_>,
            input: OperatorIndex,
            rounded_precision: u8,
            location: &Location,
        ) -> OperatorIndex;

        unsafe fn add_unsafe_cast_op(
            self: &mut DagBuilder<'_>,
            input: OperatorIndex,
            rounded_precision: u8,
            location: &Location,
        ) -> OperatorIndex;

        fn add_change_partition_with_src(
            self: &mut DagBuilder<'_>,
            input: OperatorIndex,
            src_partition: &ExternalPartition,
            location: &Location,
        ) -> OperatorIndex;

        fn add_change_partition_with_dst(
            self: &mut DagBuilder<'_>,
            input: OperatorIndex,
            dst_partition: &ExternalPartition,
            location: &Location,
        ) -> OperatorIndex;

        unsafe fn tag_operator_as_output(self: &mut DagBuilder<'_>, op: OperatorIndex);

        fn optimize(self: &Dag, options: &Options) -> DagSolution;

        unsafe fn add_composition<'a>(
            self: &mut Dag,
            from_func: &'a CxxString,
            from_pos: usize,
            to_func: &'a CxxString,
            to_pos: usize,
        );

        fn add_all_compositions(self: &mut Dag);

        #[namespace = "concrete_optimizer::dag"]
        fn dump(self: &CircuitSolution) -> String;

        #[namespace = "concrete_optimizer::dag"]
        fn short_dump(self: &CircuitSolution) -> String;

        type Weights;

        #[namespace = "concrete_optimizer::weights"]
        fn vector(weights: &[i64]) -> Box<Weights>;

        #[namespace = "concrete_optimizer::weights"]
        fn number(weight: i64) -> Box<Weights>;

        fn get_circuit_count(self: &Dag) -> usize;

        fn optimize_multi(self: &Dag, options: &Options) -> CircuitSolution;

        fn get_input_indices(self: &Dag) -> Vec<OperatorIndex>;

        fn get_output_indices(self: &Dag) -> Vec<OperatorIndex>;

        fn NO_KEY_ID() -> u64;
    }

    #[derive(Debug, Clone, Copy)]
    #[namespace = "concrete_optimizer"]
    pub enum Encoding {
        Auto,
        Native,
        Crt,
    }

    #[derive(Clone, Copy)]
    #[namespace = "concrete_optimizer::dag"]
    struct OperatorIndex {
        index: usize,
    }

    #[namespace = "concrete_optimizer::v0"]
    #[derive(Debug, Clone, Copy, Default)]
    pub struct Solution {
        pub input_lwe_dimension: u64,              //n_big
        pub internal_ks_output_lwe_dimension: u64, //n_small
        pub ks_decomposition_level_count: u64,     //l(KS)
        pub ks_decomposition_base_log: u64,        //b(KS)
        pub glwe_polynomial_size: u64,             //N
        pub glwe_dimension: u64,                   //k
        pub br_decomposition_level_count: u64,     //l(BR)
        pub br_decomposition_base_log: u64,        //b(BR)
        pub complexity: f64,
        pub noise_max: f64,
        pub p_error: f64, // error probability
    }

    #[namespace = "concrete_optimizer::dag"]
    #[derive(Debug, Clone, Default)]
    pub struct DagSolution {
        pub input_lwe_dimension: u64,              //n_big
        pub internal_ks_output_lwe_dimension: u64, //n_small
        pub ks_decomposition_level_count: u64,     //l(KS)
        pub ks_decomposition_base_log: u64,        //b(KS)
        pub glwe_polynomial_size: u64,             //N
        pub glwe_dimension: u64,                   //k
        pub br_decomposition_level_count: u64,     //l(BR)
        pub br_decomposition_base_log: u64,        //b(BR)
        pub complexity: f64,
        pub noise_max: f64,
        pub p_error: f64, // error probability
        pub global_p_error: f64,
        pub use_wop_pbs: bool,
        pub cb_decomposition_level_count: u64,
        pub cb_decomposition_base_log: u64,
        pub pp_decomposition_level_count: u64,
        pub pp_decomposition_base_log: u64,
        pub crt_decomposition: Vec<u64>,
    }

    #[derive(Debug, Clone, Copy)]
    #[namespace = "concrete_optimizer"]
    pub enum MultiParamStrategy {
        ByPrecision,
        ByPrecisionAndNorm2,
    }

    #[namespace = "concrete_optimizer"]
    #[derive(Debug, Clone)]
    pub struct GlweParameterRestrictions {
        pub log2_polynomial_size_min: SharedPtr<u64>,
        pub log2_polynomial_size_max: SharedPtr<u64>,
        pub glwe_dimension_min: SharedPtr<u64>,
        pub glwe_dimension_max: SharedPtr<u64>,
    }

    #[namespace = "concrete_optimizer"]
    #[derive(Debug, Clone)]
    pub struct DecompositionParameterRestrictions {
        pub log2_base_min: SharedPtr<u64>,
        pub log2_base_max: SharedPtr<u64>,
        pub level_min: SharedPtr<u64>,
        pub level_max: SharedPtr<u64>,
    }

    #[namespace = "concrete_optimizer"]
    #[derive(Debug, Clone)]
    pub struct ParameterRestrictions {
        pub glwe_pbs: GlweParameterRestrictions,
        pub free_glwe: GlweParameterRestrictions,
        pub br_decomposition: DecompositionParameterRestrictions,
        pub ks_decomposition: DecompositionParameterRestrictions,
        pub free_lwe_min: SharedPtr<u64>,
        pub free_lwe_max: SharedPtr<u64>,
    }

    #[namespace = "concrete_optimizer"]
    #[derive(Debug, Clone)]
    pub struct Options {
        pub security_level: u64,
        pub maximum_acceptable_error_probability: f64,
        pub key_sharing: bool,
        pub multi_param_strategy: MultiParamStrategy,
        pub default_log_norm2_woppbs: f64,
        pub use_gpu_constraints: bool,
        pub encoding: Encoding,
        pub cache_on_disk: bool,
        pub ciphertext_modulus_log: u32,
        pub fft_precision: u32,
        pub parameter_restrictions: ParameterRestrictions,
    }

    #[namespace = "concrete_optimizer::dag"]
    #[derive(Clone, Debug)]
    pub struct BrDecompositionParameters {
        pub level: u64,
        pub log2_base: u64,
    }

    #[namespace = "concrete_optimizer::dag"]
    #[derive(Clone, Debug)]
    pub struct KsDecompositionParameters {
        pub level: u64,
        pub log2_base: u64,
    }

    #[namespace = "concrete_optimizer::dag"]
    #[derive(Debug, Clone)]
    pub struct SecretLweKey {
        /* Big and small secret keys */
        pub identifier: u64,
        pub polynomial_size: u64,
        pub glwe_dimension: u64,
        pub description: String,
    }

    #[namespace = "concrete_optimizer::dag"]
    #[derive(Debug, Clone)]
    pub struct BootstrapKey {
        pub identifier: u64,
        pub input_key: SecretLweKey,
        pub output_key: SecretLweKey,
        pub br_decomposition_parameter: BrDecompositionParameters,
        pub description: String,
    }

    #[namespace = "concrete_optimizer::dag"]
    #[derive(Debug, Clone)]
    pub struct KeySwitchKey {
        pub identifier: u64,
        pub input_key: SecretLweKey,
        pub output_key: SecretLweKey,
        pub ks_decomposition_parameter: KsDecompositionParameters,
        pub description: String,
    }

    #[namespace = "concrete_optimizer::dag"]
    #[derive(Debug, Clone)]
    pub struct ConversionKeySwitchKey {
        pub identifier: u64,
        pub input_key: SecretLweKey,
        pub output_key: SecretLweKey,
        pub ks_decomposition_parameter: KsDecompositionParameters,
        pub fast_keyswitch: bool,
        pub description: String,
    }

    #[namespace = "concrete_optimizer::dag"]
    #[derive(Debug, Clone)]
    pub struct CircuitBoostrapKey {
        pub identifier: u64,
        pub representation_key: SecretLweKey,
        pub br_decomposition_parameter: BrDecompositionParameters,
        pub description: String,
    }

    #[derive(Debug, Clone)]
    pub struct PrivateFunctionalPackingBoostrapKey {
        pub identifier: u64,
        pub representation_key: SecretLweKey,
        pub br_decomposition_parameter: BrDecompositionParameters,
        pub description: String,
    }

    #[derive(Debug, Clone)]
    pub struct CircuitKeys {
        /* All keys used in a circuit */
        pub secret_keys: Vec<SecretLweKey>,
        pub keyswitch_keys: Vec<KeySwitchKey>,
        pub bootstrap_keys: Vec<BootstrapKey>,
        pub conversion_keyswitch_keys: Vec<ConversionKeySwitchKey>,
        pub circuit_bootstrap_keys: Vec<CircuitBoostrapKey>,
        pub private_functional_packing_keys: Vec<PrivateFunctionalPackingBoostrapKey>,
    }

    #[namespace = "concrete_optimizer::dag"]
    #[derive(Debug, Clone)]
    pub struct InstructionKeys {
        pub input_key: u64,
        pub tlu_keyswitch_key: u64,
        pub tlu_bootstrap_key: u64,
        pub tlu_circuit_bootstrap_key: u64,
        pub tlu_private_functional_packing_key: u64,
        pub output_key: u64,
        pub extra_conversion_keys: Vec<u64>,
    }

    #[namespace = "concrete_optimizer::dag"]
    #[derive(Debug, Clone)]
    pub struct CircuitSolution {
        pub circuit_keys: CircuitKeys,
        pub instructions_keys: Vec<InstructionKeys>,
        pub crt_decomposition: Vec<u64>,
        pub complexity: f64,
        pub p_error: f64,
        pub global_p_error: f64,
        pub is_feasible: bool,
        pub error_msg: String,
    }
}

fn processing_unit(options: &ffi::Options) -> ProcessingUnit {
    if options.use_gpu_constraints {
        config::ProcessingUnit::Gpu {
            pbs_type: config::GpuPbsType::Amortized,
            number_of_sm: 1,
        }
    } else {
        config::ProcessingUnit::Cpu
    }
}
