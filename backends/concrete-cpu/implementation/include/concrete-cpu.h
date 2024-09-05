// Copyright © 2022 ZAMA.
// All rights reserved.

#ifndef CONCRETE_CPU_FFI_H
#define CONCRETE_CPU_FFI_H

// Warning, this file is autogenerated by cbindgen. Do not modify this manually.

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include "concrete-num_complex.h"


enum Parallelism
#ifdef __cplusplus
  : uint32_t
#endif // __cplusplus
 {
  No = 0,
  Rayon = 1,
};
#ifndef __cplusplus
typedef uint32_t Parallelism;
#endif // __cplusplus

enum ScratchStatus
#ifdef __cplusplus
  : uint32_t
#endif // __cplusplus
 {
  Valid = 0,
  SizeOverflow = 1,
};
#ifndef __cplusplus
typedef uint32_t ScratchStatus;
#endif // __cplusplus

typedef struct Csprng Csprng;

typedef struct EncCsprng EncCsprng;

typedef struct Fft Fft;

typedef struct SecCsprng SecCsprng;

typedef struct Uint128 {
  uint8_t little_endian_bytes[16];
} Uint128;

typedef struct TfhersFheIntDescription {
  size_t width;
  bool is_signed;
  size_t lwe_size;
  size_t n_cts;
  size_t degree;
  size_t noise_level;
  size_t message_modulus;
  size_t carry_modulus;
  bool ks_first;
} TfhersFheIntDescription;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

extern const size_t CONCRETE_FFT_ALIGN;

extern const size_t CONCRETE_FFT_SIZE;

extern const size_t CSPRNG_ALIGN;

extern const size_t CSPRNG_SIZE;

extern const size_t ENCRYPTION_CSPRNG_ALIGN;

extern const size_t ENCRYPTION_CSPRNG_SIZE;

extern const size_t SECRET_CSPRNG_ALIGN;

extern const size_t SECRET_CSPRNG_SIZE;

void concrete_cpu_add_lwe_ciphertext_u64(uint64_t *ct_out,
                                         const uint64_t *ct_in0,
                                         const uint64_t *ct_in1,
                                         size_t lwe_dimension);

void concrete_cpu_add_plaintext_lwe_ciphertext_u64(uint64_t *ct_out,
                                                   const uint64_t *ct_in,
                                                   uint64_t plaintext,
                                                   size_t lwe_dimension);

void concrete_cpu_bootstrap_key_convert_u64_to_fourier(const uint64_t *standard_bsk,
                                                       c64 *fourier_bsk,
                                                       size_t decomposition_level_count,
                                                       size_t decomposition_base_log,
                                                       size_t glwe_dimension,
                                                       size_t polynomial_size,
                                                       size_t input_lwe_dimension,
                                                       const struct Fft *fft,
                                                       uint8_t *stack,
                                                       size_t stack_size);

ScratchStatus concrete_cpu_bootstrap_key_convert_u64_to_fourier_scratch(size_t *stack_size,
                                                                        size_t *stack_align,
                                                                        const struct Fft *fft);

size_t concrete_cpu_bootstrap_key_size_u64(size_t decomposition_level_count,
                                           size_t glwe_dimension,
                                           size_t polynomial_size,
                                           size_t input_lwe_dimension);

void concrete_cpu_bootstrap_lwe_ciphertext_u64(uint64_t *ct_out,
                                               const uint64_t *ct_in,
                                               const uint64_t *accumulator,
                                               const c64 *fourier_bsk,
                                               size_t decomposition_level_count,
                                               size_t decomposition_base_log,
                                               size_t glwe_dimension,
                                               size_t polynomial_size,
                                               size_t input_lwe_dimension,
                                               const struct Fft *fft,
                                               uint8_t *stack,
                                               size_t stack_size);

ScratchStatus concrete_cpu_bootstrap_lwe_ciphertext_u64_scratch(size_t *stack_size,
                                                                size_t *stack_align,
                                                                size_t glwe_dimension,
                                                                size_t polynomial_size,
                                                                const struct Fft *fft);

void concrete_cpu_circuit_bootstrap_boolean_vertical_packing_lwe_ciphertext_u64(uint64_t *ct_out_vec,
                                                                                const uint64_t *ct_in_vec,
                                                                                const uint64_t *lut,
                                                                                const c64 *fourier_bsk,
                                                                                const uint64_t *fpksk,
                                                                                size_t ct_out_dimension,
                                                                                size_t ct_out_count,
                                                                                size_t ct_in_dimension,
                                                                                size_t ct_in_count,
                                                                                size_t lut_size,
                                                                                size_t lut_count,
                                                                                size_t bsk_decomposition_level_count,
                                                                                size_t bsk_decomposition_base_log,
                                                                                size_t bsk_glwe_dimension,
                                                                                size_t bsk_polynomial_size,
                                                                                size_t bsk_input_lwe_dimension,
                                                                                size_t fpksk_decomposition_level_count,
                                                                                size_t fpksk_decomposition_base_log,
                                                                                size_t fpksk_input_dimension,
                                                                                size_t fpksk_output_glwe_dimension,
                                                                                size_t fpksk_output_polynomial_size,
                                                                                size_t _fpksk_count,
                                                                                size_t cbs_decomposition_level_count,
                                                                                size_t cbs_decomposition_base_log,
                                                                                const struct Fft *fft,
                                                                                uint8_t *stack,
                                                                                size_t stack_size);

ScratchStatus concrete_cpu_circuit_bootstrap_boolean_vertical_packing_lwe_ciphertext_u64_scratch(size_t *stack_size,
                                                                                                 size_t *stack_align,
                                                                                                 size_t ct_out_count,
                                                                                                 size_t ct_in_dimension,
                                                                                                 size_t ct_in_count,
                                                                                                 size_t lut_size,
                                                                                                 size_t lut_count,
                                                                                                 size_t bsk_glwe_dimension,
                                                                                                 size_t bsk_polynomial_size,
                                                                                                 size_t fpksk_output_polynomial_size,
                                                                                                 size_t cbs_decomposition_level_count,
                                                                                                 const struct Fft *fft);

void concrete_cpu_construct_concrete_fft(struct Fft *mem, size_t polynomial_size);

void concrete_cpu_construct_csprng(struct Csprng *mem, struct Uint128 seed);

void concrete_cpu_construct_encryption_csprng(struct EncCsprng *mem, struct Uint128 seed);

void concrete_cpu_construct_secret_csprng(struct SecCsprng *mem, struct Uint128 seed);

int concrete_cpu_crypto_secure_random_128(struct Uint128 *u128);

void concrete_cpu_decompress_seeded_lwe_bootstrap_key_u64(uint64_t *lwe_bsk,
                                                          const uint64_t *seeded_lwe_bsk,
                                                          size_t input_lwe_dimension,
                                                          size_t output_polynomial_size,
                                                          size_t output_glwe_dimension,
                                                          size_t decomposition_level_count,
                                                          size_t decomposition_base_log,
                                                          struct Uint128 compression_seed,
                                                          Parallelism parallelism);

void concrete_cpu_decompress_seeded_lwe_ciphertext_u64(uint64_t *lwe_out,
                                                       const uint64_t *seeded_lwe_in,
                                                       size_t lwe_dimension,
                                                       struct Uint128 compression_seed);

void concrete_cpu_decompress_seeded_lwe_keyswitch_key_u64(uint64_t *lwe_ksk,
                                                          const uint64_t *seeded_lwe_ksk,
                                                          size_t input_lwe_dimension,
                                                          size_t output_lwe_dimension,
                                                          size_t decomposition_level_count,
                                                          size_t decomposition_base_log,
                                                          struct Uint128 compression_seed,
                                                          Parallelism parallelism);

void concrete_cpu_decrypt_glwe_ciphertext_u64(const uint64_t *glwe_sk,
                                              uint64_t *output,
                                              const uint64_t *glwe_ct_in,
                                              size_t glwe_dimension,
                                              size_t polynomial_size);

void concrete_cpu_decrypt_lwe_ciphertext_u64(const uint64_t *lwe_sk,
                                             const uint64_t *lwe_ct_in,
                                             size_t lwe_dimension,
                                             uint64_t *plaintext);

void concrete_cpu_destroy_concrete_fft(struct Fft *mem);

void concrete_cpu_destroy_csprng(struct Csprng *mem);

void concrete_cpu_destroy_encryption_csprng(struct EncCsprng *mem);

void concrete_cpu_destroy_secret_csprng(struct SecCsprng *mem);

void concrete_cpu_encrypt_ggsw_ciphertext_u64(const uint64_t *glwe_sk,
                                              uint64_t *ggsw_out,
                                              uint64_t input,
                                              size_t glwe_dimension,
                                              size_t polynomial_size,
                                              size_t level,
                                              size_t base_log,
                                              double variance,
                                              struct EncCsprng *csprng);

void concrete_cpu_encrypt_lwe_ciphertext_u64(const uint64_t *lwe_sk,
                                             uint64_t *lwe_out,
                                             uint64_t input,
                                             size_t lwe_dimension,
                                             double variance,
                                             struct EncCsprng *csprng);

void concrete_cpu_encrypt_seeded_lwe_ciphertext_u64(const uint64_t *lwe_sk,
                                                    uint64_t *seeded_lwe_out,
                                                    uint64_t input,
                                                    size_t lwe_dimension,
                                                    struct Uint128 compression_seed,
                                                    double variance);

void concrete_cpu_extract_bit_lwe_ciphertext_u64(uint64_t *ct_vec_out,
                                                 const uint64_t *ct_in,
                                                 const c64 *fourier_bsk,
                                                 const uint64_t *ksk,
                                                 size_t ct_out_dimension,
                                                 size_t ct_out_count,
                                                 size_t ct_in_dimension,
                                                 size_t number_of_bits,
                                                 size_t delta_log,
                                                 size_t bsk_decomposition_level_count,
                                                 size_t bsk_decomposition_base_log,
                                                 size_t bsk_glwe_dimension,
                                                 size_t bsk_polynomial_size,
                                                 size_t bsk_input_lwe_dimension,
                                                 size_t ksk_decomposition_level_count,
                                                 size_t ksk_decomposition_base_log,
                                                 size_t ksk_input_dimension,
                                                 size_t ksk_output_dimension,
                                                 const struct Fft *fft,
                                                 uint8_t *stack,
                                                 size_t stack_size);

ScratchStatus concrete_cpu_extract_bit_lwe_ciphertext_u64_scratch(size_t *stack_size,
                                                                  size_t *stack_align,
                                                                  size_t ct_out_dimension,
                                                                  size_t ct_in_dimension,
                                                                  size_t bsk_glwe_dimension,
                                                                  size_t bsk_polynomial_size,
                                                                  const struct Fft *fft);

void concrete_cpu_fill_with_random_gaussian(uint64_t *buffer,
                                            size_t size,
                                            double variance,
                                            struct Csprng *csprng);

size_t concrete_cpu_fourier_bootstrap_key_size_u64(size_t decomposition_level_count,
                                                   size_t glwe_dimension,
                                                   size_t polynomial_size,
                                                   size_t input_lwe_dimension);

size_t concrete_cpu_ggsw_ciphertext_size_u64(size_t glwe_dimension,
                                             size_t polynomial_size,
                                             size_t decomposition_level_count);

size_t concrete_cpu_glwe_ciphertext_size_u64(size_t glwe_dimension, size_t polynomial_size);

size_t concrete_cpu_glwe_secret_key_buffer_size_u64(size_t glwe_dimension, size_t polynomial_size);

size_t concrete_cpu_glwe_secret_key_size_u64(size_t lwe_dimension, size_t polynomial_size);

void concrete_cpu_init_lwe_bootstrap_key_u64(uint64_t *lwe_bsk,
                                             const uint64_t *input_lwe_sk,
                                             const uint64_t *output_glwe_sk,
                                             size_t input_lwe_dimension,
                                             size_t output_polynomial_size,
                                             size_t output_glwe_dimension,
                                             size_t decomposition_level_count,
                                             size_t decomposition_base_log,
                                             double variance,
                                             Parallelism parallelism,
                                             struct EncCsprng *csprng);

void concrete_cpu_init_lwe_circuit_bootstrap_private_functional_packing_keyswitch_keys_u64(uint64_t *lwe_pksk,
                                                                                           const uint64_t *input_lwe_sk,
                                                                                           const uint64_t *output_glwe_sk,
                                                                                           size_t input_lwe_dimension,
                                                                                           size_t output_polynomial_size,
                                                                                           size_t output_glwe_dimension,
                                                                                           size_t decomposition_level_count,
                                                                                           size_t decomposition_base_log,
                                                                                           double variance,
                                                                                           Parallelism parallelism,
                                                                                           struct EncCsprng *csprng);

void concrete_cpu_init_lwe_keyswitch_key_u64(uint64_t *lwe_ksk,
                                             const uint64_t *input_lwe_sk,
                                             const uint64_t *output_lwe_sk,
                                             size_t input_lwe_dimension,
                                             size_t output_lwe_dimension,
                                             size_t decomposition_level_count,
                                             size_t decomposition_base_log,
                                             double variance,
                                             struct EncCsprng *csprng);

void concrete_cpu_init_secret_key_u64(uint64_t *sk, size_t dimension, struct SecCsprng *csprng);

void concrete_cpu_init_seeded_lwe_bootstrap_key_u64(uint64_t *seeded_lwe_bsk,
                                                    const uint64_t *input_lwe_sk,
                                                    const uint64_t *output_glwe_sk,
                                                    size_t input_lwe_dimension,
                                                    size_t output_polynomial_size,
                                                    size_t output_glwe_dimension,
                                                    size_t decomposition_level_count,
                                                    size_t decomposition_base_log,
                                                    struct Uint128 compression_seed,
                                                    double variance,
                                                    Parallelism parallelism);

void concrete_cpu_init_seeded_lwe_keyswitch_key_u64(uint64_t *seeded_lwe_ksk,
                                                    const uint64_t *input_lwe_sk,
                                                    const uint64_t *output_lwe_sk,
                                                    size_t input_lwe_dimension,
                                                    size_t output_lwe_dimension,
                                                    size_t decomposition_level_count,
                                                    size_t decomposition_base_log,
                                                    struct Uint128 compression_seed,
                                                    double variance);

size_t concrete_cpu_keyswitch_key_size_u64(size_t decomposition_level_count,
                                           size_t input_dimension,
                                           size_t output_dimension);

void concrete_cpu_keyswitch_lwe_ciphertext_u64(uint64_t *ct_out,
                                               const uint64_t *ct_in,
                                               const uint64_t *keyswitch_key,
                                               size_t decomposition_level_count,
                                               size_t decomposition_base_log,
                                               size_t input_dimension,
                                               size_t output_dimension);

size_t concrete_cpu_lwe_array_to_tfhers_uint8(const uint64_t *lwe_vec_buffer,
                                              uint8_t *fheuint_buffer,
                                              size_t fheuint_buffer_size,
                                              struct TfhersFheIntDescription fheuint_desc);

size_t concrete_cpu_lwe_ciphertext_size_u64(size_t lwe_dimension);

size_t concrete_cpu_lwe_packing_keyswitch_key_size(size_t output_glwe_dimension,
                                                   size_t polynomial_size,
                                                   size_t decomposition_level_count,
                                                   size_t input_lwe_dimension);

size_t concrete_cpu_lwe_secret_key_size_u64(size_t lwe_dimension);

void concrete_cpu_mul_cleartext_lwe_ciphertext_u64(uint64_t *ct_out,
                                                   const uint64_t *ct_in,
                                                   uint64_t cleartext,
                                                   size_t lwe_dimension);

void concrete_cpu_negate_lwe_ciphertext_u64(uint64_t *ct_out,
                                            const uint64_t *ct_in,
                                            size_t lwe_dimension);

size_t concrete_cpu_seeded_bootstrap_key_size_u64(size_t decomposition_level_count,
                                                  size_t glwe_dimension,
                                                  size_t polynomial_size,
                                                  size_t input_lwe_dimension);

size_t concrete_cpu_seeded_keyswitch_key_size_u64(size_t decomposition_level_count,
                                                  size_t input_dimension);

size_t concrete_cpu_serialize_glwe_secret_key_u64(const uint64_t *glwe_sk,
                                                  size_t glwe_dimension,
                                                  size_t polynomial_size,
                                                  uint8_t *out_buffer,
                                                  size_t out_buffer_len);

size_t concrete_cpu_tfhers_fheint_buffer_size_u64(size_t lwe_size, size_t n_cts);

struct TfhersFheIntDescription concrete_cpu_tfhers_uint8_description(const uint8_t *serialized_data_ptr,
                                                                     size_t serialized_data_len);

int64_t concrete_cpu_tfhers_uint8_to_lwe_array(const uint8_t *serialized_data_ptr,
                                               size_t serialized_data_len,
                                               uint64_t *lwe_vec_buffer);

size_t concrete_cpu_tfhers_unknown_noise_level(void);

size_t concrete_cpu_unserialize_glwe_secret_key_u64(const uint8_t *buffer,
                                                    size_t buffer_len,
                                                    uint64_t *glwe_sk,
                                                    size_t glwe_sk_size);

void simulation_circuit_bootstrap_boolean_vertical_packing_lwe_ciphertext_u64(const uint64_t *lwe_list_in,
                                                                              uint64_t *lwe_list_out,
                                                                              size_t ct_in_count,
                                                                              size_t ct_out_count,
                                                                              size_t lut_size,
                                                                              size_t lut_count,
                                                                              const uint64_t *luts,
                                                                              uint64_t glwe_dimension,
                                                                              uint64_t log_poly_size,
                                                                              uint64_t lwe_dimension,
                                                                              uint64_t pbs_level,
                                                                              uint64_t pbs_log_base,
                                                                              uint64_t cb_level,
                                                                              uint64_t cb_log_base,
                                                                              uint64_t pp_level,
                                                                              uint64_t pp_log_base,
                                                                              uint32_t ciphertext_modulus_log,
                                                                              uint64_t security_level,
                                                                              struct Csprng *csprng);

void simulation_extract_bit_lwe_ciphertext_u64(uint64_t *lwe_list_out,
                                               uint64_t lwe_in,
                                               size_t delta_log,
                                               size_t number_of_bits_to_extract,
                                               uint64_t log_poly_size,
                                               uint64_t glwe_dimension,
                                               uint64_t lwe_dimension,
                                               uint64_t ks_log_base,
                                               uint64_t ks_level,
                                               uint64_t br_log_base,
                                               uint64_t br_level,
                                               uint32_t ciphertext_modulus_log,
                                               uint64_t security_level,
                                               struct Csprng *csprng);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* CONCRETE_CPU_FFI_H */
