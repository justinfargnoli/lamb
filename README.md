# Typed-Lambda-Calculus

This repo aims to implement and build tools around the language described here https://papl.cs.brown.edu/2018/types.html .

`export LLVM_SYS_130_PREFIX="/Users/justinfargnoli/Documents/projects/lamb/llvm/llvm-project-llvmorg-13.0.0/llvm/build"`

`cmake -G "Ninja" -DCMAKE_BUILD_TYPE=Release -DLLVM_TARGETS_TO_BUILD=host -DLLVM_OPTIMIZED_TABLEGEN=ON -DLLVM_ENABLE_ASSERTIONS=ON -DCLANG_ENABLE_STATIC_ANALYZER=OFF`
