// Part of the Concrete Compiler Project, under the BSD3 License with Zama
// Exceptions. See
// https://github.com/zama-ai/concrete-compiler-internal/blob/main/LICENSE.txt
// for license information.

#ifndef CONCRETELANG_SUPPORT_CLIENTPARAMETERS_H_
#define CONCRETELANG_SUPPORT_CLIENTPARAMETERS_H_

#include <mlir/Dialect/LLVMIR/LLVMTypes.h>
#include <mlir/IR/BuiltinOps.h>

#include "concretelang/ClientLib/ClientParameters.h"
#include "concretelang/Support/V0Parameters.h"

namespace mlir {
namespace concretelang {

using ::concretelang::clientlib::ChunkInfo;
using ::concretelang::clientlib::ClientParameters;

llvm::Expected<ClientParameters>
createClientParametersForV0(V0FHEContext context, llvm::StringRef functionName,
                            mlir::ModuleOp module, int bitsOfSecurity,
                            llvm::Optional<ChunkInfo> chunkInfo = llvm::None);

} // namespace concretelang
} // namespace mlir

#endif
