#include <concretelang/Runtime/DFRuntime.hpp>
#include <cstdint>
#include <gtest/gtest.h>
#include <tuple>
#include <type_traits>

#include "concretelang/TestLib/TestProgram.h"
#include "end_to_end_jit_test.h"
#include "tests_tools/GtestEnvironment.h"

///////////////////////////////////////////////////////////////////////////////
// Auto-parallelize independent FHE ops /////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////

std::vector<uint64_t> distributed_results;

TEST(Distributed, nn_med_nested) {
  checkedJit(lambda, R"XXX(
func.func @main(%arg0: tensor<200x4x!FHE.eint<4>>) -> tensor<200x8x!FHE.eint<4>> {
  %cst = arith.constant dense<"0x01010100010100000001010101000101010101010101010001000101000001010001010100000101000001000001010001000001010100010001000000010100010001010001000001000101010101000100010001000000000100010001000101000001000101010100010001000000000101000100000000000001000100000100000100000001010000010001000101000100010001000100000100000100010101010000000000000000010001010000000100000100010100000100000000010001000101000100000000000101010101000101010101010100010100010100000000000101010100000100010100000001000101000000010101000101000100000101010100010101010000010101010100010000000000000001010101000100010101000001010001010000010001010101000000000000000001000001000000010100000100000101010100010001000000000000010100010101000000010100000100010001010001000000000100010001000101010100010100000001010100010101010100010100010001000001000000000101000101010001000100000101010100000101010100000100010101000100000101000101010100010001000101010100010001010001010000010000010001010000000001000101010001000000000101000000010000010100010001000001000001010101000100010001010100000101000000010001000000000101000101000000010000000001000101010100010001000000000001010000010001000001010101000101010101010100000000000001000100000100000001000000010101010101000000000101010101000100000101000100000000000001000100000101000101010100010000000101000000000100000100000101010000010100000000010000000000010001000100000101010001010101000000000000010000010101010001000000010001010001010000000000000101000000010101010101000001010101000001000001010100000000010001010100000100000101000101010100010001010001000001000100000101000100010100000100010000000101000000010000010001010101010000000101000000010101000001010100000100010001000000000001010000000100010000000000000000000000000001010101010101010101000001010101000001010100000001000101010101010000010101000101010100010101010000010101010100000100000000000101010000000000010101010000000001000000010100000100000001000101010000000001000001000001010001010000010001000101010001010001010101000100010000000100000100010101000000000101010101010001000100000000000101010000010101000001010001010000000001010100000101000001010000000001010101000100010000010101000000000001000101000001010101000101000001000001000000010100010001000101010100010001010000000101000000010001000001000100000101010001000001000001000101010000010001000001000101000000000000000101010000010000000101010100010100010001010101010000000000010001000101010000000001010100000000010001010100010001000001000101000000010100010000010000010001010100010000010001010100010000010100010101010001000100010100010101000100000101010100000100010100000100000000010101000000010001000001010000000101000100000100010101000000010100000101000001010001010100010000000101010000000001010001000000010100010101010001000100010001000001010101000000010001000100000100010101000000000000010100010000000100000000010100010000000100000101010000010101000100010000010100000001000100000000000100000001010101010101000100010001000000010101010100000001000001000001010001000101010100000001010001010100010101000101000000010001010100010101000100000101000101000001000001000001000101010100010001010000000100000101010100000001000000000000010101000100010001000001000001000000000000010100000100000001"> : tensor<200x8xi5>
  %cst_0 = arith.constant dense<[[1, 0, 0, 0, 1, 0, 0, 1], [0, 0, 1, 1, 0, 0, 0, 0], [1, 1, 0, 1, 1, 0, 1, 1], [1, 1, 0, 0, 1, 0, 1, 1]]> : tensor<4x8xi5>
  %cst_1 = arith.constant dense<[0, 3, 7, 10, 14, 17, 21, 24, 28, 31, 35, 38, 42, 45, 49, 52]> : tensor<16xi64>
  %0 = "FHELinalg.matmul_eint_int"(%arg0, %cst_0) : (tensor<200x4x!FHE.eint<4>>, tensor<4x8xi5>) -> tensor<200x8x!FHE.eint<4>>
  %1 = "FHELinalg.add_eint_int"(%0, %cst) : (tensor<200x8x!FHE.eint<4>>, tensor<200x8xi5>) -> tensor<200x8x!FHE.eint<4>>

  %res = "FHE.zero_tensor"() : () -> tensor<200x8x!FHE.eint<4>>

  %slice_A = tensor.extract_slice %1[0,   0][25, 4][1, 1] : tensor<200x8x!FHE.eint<4>> to tensor<25x4x!FHE.eint<4>>
  %slice_B = tensor.extract_slice %1[25,  0][25, 4][1, 1] : tensor<200x8x!FHE.eint<4>> to tensor<25x4x!FHE.eint<4>>
  %slice_C = tensor.extract_slice %1[50,  0][25, 4][1, 1] : tensor<200x8x!FHE.eint<4>> to tensor<25x4x!FHE.eint<4>>
  %slice_D = tensor.extract_slice %1[75,  0][25, 4][1, 1] : tensor<200x8x!FHE.eint<4>> to tensor<25x4x!FHE.eint<4>>
  %slice_E = tensor.extract_slice %1[100, 0][25, 4][1, 1] : tensor<200x8x!FHE.eint<4>> to tensor<25x4x!FHE.eint<4>>
  %slice_F = tensor.extract_slice %1[125, 0][25, 4][1, 1] : tensor<200x8x!FHE.eint<4>> to tensor<25x4x!FHE.eint<4>>
  %slice_G = tensor.extract_slice %1[150, 0][25, 4][1, 1] : tensor<200x8x!FHE.eint<4>> to tensor<25x4x!FHE.eint<4>>
  %slice_H = tensor.extract_slice %1[175, 0][25, 4][1, 1] : tensor<200x8x!FHE.eint<4>> to tensor<25x4x!FHE.eint<4>>
  %slice_I = tensor.extract_slice %1[0,   4][25, 4][1, 1] : tensor<200x8x!FHE.eint<4>> to tensor<25x4x!FHE.eint<4>>
  %slice_J = tensor.extract_slice %1[25,  4][25, 4][1, 1] : tensor<200x8x!FHE.eint<4>> to tensor<25x4x!FHE.eint<4>>
  %slice_K = tensor.extract_slice %1[50,  4][25, 4][1, 1] : tensor<200x8x!FHE.eint<4>> to tensor<25x4x!FHE.eint<4>>
  %slice_L = tensor.extract_slice %1[75,  4][25, 4][1, 1] : tensor<200x8x!FHE.eint<4>> to tensor<25x4x!FHE.eint<4>>
  %slice_M = tensor.extract_slice %1[100, 4][25, 4][1, 1] : tensor<200x8x!FHE.eint<4>> to tensor<25x4x!FHE.eint<4>>
  %slice_N = tensor.extract_slice %1[125, 4][25, 4][1, 1] : tensor<200x8x!FHE.eint<4>> to tensor<25x4x!FHE.eint<4>>
  %slice_O = tensor.extract_slice %1[150, 4][25, 4][1, 1] : tensor<200x8x!FHE.eint<4>> to tensor<25x4x!FHE.eint<4>>
  %slice_P = tensor.extract_slice %1[175, 4][25, 4][1, 1] : tensor<200x8x!FHE.eint<4>> to tensor<25x4x!FHE.eint<4>>

  %part_A = "FHELinalg.apply_lookup_table"(%slice_A, %cst_1) : (tensor<25x4x!FHE.eint<4>>, tensor<16xi64>) -> tensor<25x4x!FHE.eint<4>>
  %part_B = "FHELinalg.apply_lookup_table"(%slice_B, %cst_1) : (tensor<25x4x!FHE.eint<4>>, tensor<16xi64>) -> tensor<25x4x!FHE.eint<4>>
  %part_C = "FHELinalg.apply_lookup_table"(%slice_C, %cst_1) : (tensor<25x4x!FHE.eint<4>>, tensor<16xi64>) -> tensor<25x4x!FHE.eint<4>>
  %part_D = "FHELinalg.apply_lookup_table"(%slice_D, %cst_1) : (tensor<25x4x!FHE.eint<4>>, tensor<16xi64>) -> tensor<25x4x!FHE.eint<4>>
  %part_E = "FHELinalg.apply_lookup_table"(%slice_E, %cst_1) : (tensor<25x4x!FHE.eint<4>>, tensor<16xi64>) -> tensor<25x4x!FHE.eint<4>>
  %part_F = "FHELinalg.apply_lookup_table"(%slice_F, %cst_1) : (tensor<25x4x!FHE.eint<4>>, tensor<16xi64>) -> tensor<25x4x!FHE.eint<4>>
  %part_G = "FHELinalg.apply_lookup_table"(%slice_G, %cst_1) : (tensor<25x4x!FHE.eint<4>>, tensor<16xi64>) -> tensor<25x4x!FHE.eint<4>>
  %part_H = "FHELinalg.apply_lookup_table"(%slice_H, %cst_1) : (tensor<25x4x!FHE.eint<4>>, tensor<16xi64>) -> tensor<25x4x!FHE.eint<4>>
  %part_I = "FHELinalg.apply_lookup_table"(%slice_I, %cst_1) : (tensor<25x4x!FHE.eint<4>>, tensor<16xi64>) -> tensor<25x4x!FHE.eint<4>>
  %part_J = "FHELinalg.apply_lookup_table"(%slice_J, %cst_1) : (tensor<25x4x!FHE.eint<4>>, tensor<16xi64>) -> tensor<25x4x!FHE.eint<4>>
  %part_K = "FHELinalg.apply_lookup_table"(%slice_K, %cst_1) : (tensor<25x4x!FHE.eint<4>>, tensor<16xi64>) -> tensor<25x4x!FHE.eint<4>>
  %part_L = "FHELinalg.apply_lookup_table"(%slice_L, %cst_1) : (tensor<25x4x!FHE.eint<4>>, tensor<16xi64>) -> tensor<25x4x!FHE.eint<4>>
  %part_M = "FHELinalg.apply_lookup_table"(%slice_M, %cst_1) : (tensor<25x4x!FHE.eint<4>>, tensor<16xi64>) -> tensor<25x4x!FHE.eint<4>>
  %part_N = "FHELinalg.apply_lookup_table"(%slice_N, %cst_1) : (tensor<25x4x!FHE.eint<4>>, tensor<16xi64>) -> tensor<25x4x!FHE.eint<4>>
  %part_O = "FHELinalg.apply_lookup_table"(%slice_O, %cst_1) : (tensor<25x4x!FHE.eint<4>>, tensor<16xi64>) -> tensor<25x4x!FHE.eint<4>>
  %part_P = "FHELinalg.apply_lookup_table"(%slice_P, %cst_1) : (tensor<25x4x!FHE.eint<4>>, tensor<16xi64>) -> tensor<25x4x!FHE.eint<4>>

  %res_A = tensor.insert_slice %part_A into %res  [0,   0][25, 4][1, 1] : tensor<25x4x!FHE.eint<4>> into tensor<200x8x!FHE.eint<4>>
  %res_B = tensor.insert_slice %part_B into %res_A[25,  0][25, 4][1, 1] : tensor<25x4x!FHE.eint<4>> into tensor<200x8x!FHE.eint<4>>
  %res_C = tensor.insert_slice %part_C into %res_B[50,  0][25, 4][1, 1] : tensor<25x4x!FHE.eint<4>> into tensor<200x8x!FHE.eint<4>>
  %res_D = tensor.insert_slice %part_D into %res_C[75,  0][25, 4][1, 1] : tensor<25x4x!FHE.eint<4>> into tensor<200x8x!FHE.eint<4>>
  %res_E = tensor.insert_slice %part_E into %res_D[100, 0][25, 4][1, 1] : tensor<25x4x!FHE.eint<4>> into tensor<200x8x!FHE.eint<4>>
  %res_F = tensor.insert_slice %part_F into %res_E[125, 0][25, 4][1, 1] : tensor<25x4x!FHE.eint<4>> into tensor<200x8x!FHE.eint<4>>
  %res_G = tensor.insert_slice %part_G into %res_F[150, 0][25, 4][1, 1] : tensor<25x4x!FHE.eint<4>> into tensor<200x8x!FHE.eint<4>>
  %res_H = tensor.insert_slice %part_H into %res_G[175, 0][25, 4][1, 1] : tensor<25x4x!FHE.eint<4>> into tensor<200x8x!FHE.eint<4>>
  %res_I = tensor.insert_slice %part_I into %res_H[0,   4][25, 4][1, 1] : tensor<25x4x!FHE.eint<4>> into tensor<200x8x!FHE.eint<4>>
  %res_J = tensor.insert_slice %part_J into %res_I[25,  4][25, 4][1, 1] : tensor<25x4x!FHE.eint<4>> into tensor<200x8x!FHE.eint<4>>
  %res_K = tensor.insert_slice %part_K into %res_J[50,  4][25, 4][1, 1] : tensor<25x4x!FHE.eint<4>> into tensor<200x8x!FHE.eint<4>>
  %res_L = tensor.insert_slice %part_L into %res_K[75,  4][25, 4][1, 1] : tensor<25x4x!FHE.eint<4>> into tensor<200x8x!FHE.eint<4>>
  %res_M = tensor.insert_slice %part_M into %res_L[100, 4][25, 4][1, 1] : tensor<25x4x!FHE.eint<4>> into tensor<200x8x!FHE.eint<4>>
  %res_N = tensor.insert_slice %part_N into %res_M[125, 4][25, 4][1, 1] : tensor<25x4x!FHE.eint<4>> into tensor<200x8x!FHE.eint<4>>
  %res_O = tensor.insert_slice %part_O into %res_N[150, 4][25, 4][1, 1] : tensor<25x4x!FHE.eint<4>> into tensor<200x8x!FHE.eint<4>>
  %res_P = tensor.insert_slice %part_P into %res_O[175, 4][25, 4][1, 1] : tensor<25x4x!FHE.eint<4>> into tensor<200x8x!FHE.eint<4>>

  return %res_P : tensor<200x8x!FHE.eint<4>>
}
)XXX",
             "main", false, true, true, DEFAULT_batchTFHEOps,
             DEFAULT_global_p_error, DEFAULT_chunkedIntegers, DEFAULT_chunkSize,
             DEFAULT_chunkWidth, false);

  const size_t dim0 = 200;
  const size_t dim1 = 4;
  const size_t dim2 = 8;
  const std::vector<size_t> inputShape({dim0, dim1});
  const std::vector<size_t> outputShape({dim0, dim2});
  std::vector<uint64_t> values;
  values.reserve(dim0 * dim1);
  for (size_t i = 0; i < dim0 * dim1; ++i) {
    values.push_back(i % 17 % 4);
  }
  auto input = Tensor<uint64_t>(values, inputShape);

  if (mlir::concretelang::dfr::_dfr_is_root_node()) {
    auto maybeResult = lambda.call({input});
    ASSERT_OUTCOME_HAS_VALUE(maybeResult);
    auto result = maybeResult.value()[0].template getTensor<uint64_t>().value();
    ASSERT_EQ(result.dimensions, outputShape);
    distributed_results = result.values;
  } else {
    ASSERT_OUTCOME_HAS_VALUE(lambda.call({}));
  }
}

TEST(Distributed, nn_med_sequential) {
  if (mlir::concretelang::dfr::_dfr_is_root_node()) {
    checkedJit(lambda, R"XXX(
  func.func @main(%arg0: tensor<200x4x!FHE.eint<4>>) -> tensor<200x8x!FHE.eint<4>> {
    %cst = arith.constant dense<"0x01010100010100000001010101000101010101010101010001000101000001010001010100000101000001000001010001000001010100010001000000010100010001010001000001000101010101000100010001000000000100010001000101000001000101010100010001000000000101000100000000000001000100000100000100000001010000010001000101000100010001000100000100000100010101010000000000000000010001010000000100000100010100000100000000010001000101000100000000000101010101000101010101010100010100010100000000000101010100000100010100000001000101000000010101000101000100000101010100010101010000010101010100010000000000000001010101000100010101000001010001010000010001010101000000000000000001000001000000010100000100000101010100010001000000000000010100010101000000010100000100010001010001000000000100010001000101010100010100000001010100010101010100010100010001000001000000000101000101010001000100000101010100000101010100000100010101000100000101000101010100010001000101010100010001010001010000010000010001010000000001000101010001000000000101000000010000010100010001000001000001010101000100010001010100000101000000010001000000000101000101000000010000000001000101010100010001000000000001010000010001000001010101000101010101010100000000000001000100000100000001000000010101010101000000000101010101000100000101000100000000000001000100000101000101010100010000000101000000000100000100000101010000010100000000010000000000010001000100000101010001010101000000000000010000010101010001000000010001010001010000000000000101000000010101010101000001010101000001000001010100000000010001010100000100000101000101010100010001010001000001000100000101000100010100000100010000000101000000010000010001010101010000000101000000010101000001010100000100010001000000000001010000000100010000000000000000000000000001010101010101010101000001010101000001010100000001000101010101010000010101000101010100010101010000010101010100000100000000000101010000000000010101010000000001000000010100000100000001000101010000000001000001000001010001010000010001000101010001010001010101000100010000000100000100010101000000000101010101010001000100000000000101010000010101000001010001010000000001010100000101000001010000000001010101000100010000010101000000000001000101000001010101000101000001000001000000010100010001000101010100010001010000000101000000010001000001000100000101010001000001000001000101010000010001000001000101000000000000000101010000010000000101010100010100010001010101010000000000010001000101010000000001010100000000010001010100010001000001000101000000010100010000010000010001010100010000010001010100010000010100010101010001000100010100010101000100000101010100000100010100000100000000010101000000010001000001010000000101000100000100010101000000010100000101000001010001010100010000000101010000000001010001000000010100010101010001000100010001000001010101000000010001000100000100010101000000000000010100010000000100000000010100010000000100000101010000010101000100010000010100000001000100000000000100000001010101010101000100010001000000010101010100000001000001000001010001000101010100000001010001010100010101000101000000010001010100010101000100000101000101000001000001000001000101010100010001010000000100000101010100000001000000000000010101000100010001000001000001000000000000010100000100000001"> : tensor<200x8xi5>
    %cst_0 = arith.constant dense<[[1, 0, 0, 0, 1, 0, 0, 1], [0, 0, 1, 1, 0, 0, 0, 0], [1, 1, 0, 1, 1, 0, 1, 1], [1, 1, 0, 0, 1, 0, 1, 1]]> : tensor<4x8xi5>
    %0 = "FHELinalg.matmul_eint_int"(%arg0, %cst_0) : (tensor<200x4x!FHE.eint<4>>, tensor<4x8xi5>) -> tensor<200x8x!FHE.eint<4>>
    %1 = "FHELinalg.add_eint_int"(%0, %cst) : (tensor<200x8x!FHE.eint<4>>, tensor<200x8xi5>) -> tensor<200x8x!FHE.eint<4>>
    %cst_1 = arith.constant dense<[0, 3, 7, 10, 14, 17, 21, 24, 28, 31, 35, 38, 42, 45, 49, 52]> : tensor<16xi64>
    %2 = "FHELinalg.apply_lookup_table"(%1, %cst_1) : (tensor<200x8x!FHE.eint<4>>, tensor<16xi64>) -> tensor<200x8x!FHE.eint<4>>
    return %2 : tensor<200x8x!FHE.eint<4>>
  }
)XXX",
               "main", false, false, false, DEFAULT_batchTFHEOps,
               DEFAULT_global_p_error, DEFAULT_chunkedIntegers,
               DEFAULT_chunkSize, DEFAULT_chunkWidth, false);

    const size_t dim0 = 200;
    const size_t dim1 = 4;
    const size_t dim2 = 8;
    const std::vector<size_t> inputShape({dim0, dim1});
    const std::vector<size_t> outputShape({dim0, dim2});
    std::vector<uint64_t> values;
    values.reserve(dim0 * dim1);
    for (size_t i = 0; i < dim0 * dim1; ++i) {
      values.push_back(i % 17 % 4);
    }
    auto input = Tensor<uint64_t>(values, inputShape);

    if (mlir::concretelang::dfr::_dfr_is_root_node()) {
      auto maybeResult = lambda.call({input});
      ASSERT_OUTCOME_HAS_VALUE(maybeResult);
      auto result =
          maybeResult.value()[0].template getTensor<uint64_t>().value();
      for (size_t i = 0; i < dim0 * dim2; i++)
        EXPECT_EQ(distributed_results[i], result.values[i])
            << "result differ at pos " << i;
    }
  }
}
